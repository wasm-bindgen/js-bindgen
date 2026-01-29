mod wasm_ld;

use std::borrow::Cow;
use std::convert::Infallible;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::{self, Command};
use std::{env, fs};

use hashbrown::{HashMap, HashSet};
use itertools::{Itertools, Position};
use js_bindgen_ld_shared::{CustomSectionParser, ReadFile};
use wasm_encoder::{EntityType, ImportSection, Module, RawSection, Section};
use wasmparser::{Encoding, Parser, Payload, TypeRef};

use crate::wasm_ld::WasmLdArguments;

fn main() {
	let args: Vec<_> = env::args_os().collect();
	let wasm_ld_args = WasmLdArguments::new(&args[1..]);

	if wasm_ld_args
		.arg_single("flavor")
		.filter(|v| *v == "wasm")
		.is_none()
	{
		panic!("the `js-bindgen-ld` should only be used when compiling to a Wasm target")
	}

	// With Wasm32 no argument is passed, but Wasm64 requires `-mwasm64`.
	let arch_str = if let Some(m) = wasm_ld_args.arg_single("m") {
		if m == "wasm32" || m == "wasm64" {
			Cow::Borrowed(m)
		} else {
			panic!("expected `-m` to either be `wasm32` or `wasm64");
		}
	} else {
		Cow::Owned("wasm32".into())
	};

	let output_path = Path::new(
		wasm_ld_args
			.arg_single("o")
			.expect("output path argument should be present"),
	);

	// Here we store additional arguments we want to pass to `wasm-ld`.
	let mut add_args: Vec<OsString> = Vec::new();

	// Extract path to the main memory if user-specified, otherwise force export
	// with our own path.
	let main_memory = match wasm_ld_args.arg_single("import-memory=") {
		Some(arg) => {
			let arg = arg
				.to_str()
				.expect("`--import-memory=` parameters should be valid UTF-8");
			let mut split = arg.splitn(2, ',');

			let module = split.next().expect("should yield something even if empty");

			if let Some(name) = split.next() {
				MainMemory { module, name }
			} else {
				MainMemory { module, name: "" }
			}
		}
		None => {
			if wasm_ld_args.arg_flag("import-memory") {
				eprintln!("found `--import-memory`");
				eprintln!(
					"`js-bindgen` already imports the main memory by default under \
					 `js-bindgen:memory`"
				);
				MainMemory {
					module: "env",
					name: "memory",
				}
			} else {
				add_args.push(OsString::from("--import-memory=js_bindgen,memory"));
				MainMemory {
					module: "js_bindgen",
					name: "memory",
				}
			}
		}
	};

	// Extract embedded assembly from object files.
	for input in wasm_ld_args.inputs() {
		js_bindgen_ld_shared::ld_input_parser::<Infallible>(input, |path, data| {
			process_object(&arch_str, &mut add_args, path, data);
			Ok(())
		})
		.unwrap();
	}

	let status = Command::new("rust-lld")
		.args(args.iter().skip(1))
		.args(add_args)
		.status()
		.unwrap();

	if status.success() {
		let wasm_output = post_processing(output_path, main_memory);

		// We could write into the file directly, but `wasm-encoder` doesn't support
		// `io::Write`: https://github.com/bytecodealliance/wasm-tools/issues/778.
		//
		// When it does, we should rename the old file and write to a new file. This way
		// we can keep parsing and writing at the same time without allocating memory.
		fs::write(output_path, wasm_output).expect("output Wasm file should be writable");
	}

	process::exit(status.code().unwrap_or(1));
}

struct MainMemory<'a> {
	module: &'a str,
	name: &'a str,
}

/// Extracts any assembly instructions from `js-bindgen`, builds object files
/// from them and passes them to the linker.
fn process_object(
	arch_str: &OsStr,
	add_args: &mut Vec<OsString>,
	archive_path: &Path,
	object: &[u8],
) {
	// Multiple files from the same object file need different names.
	let mut file_counter = 0;

	for payload in Parser::new(0).parse_all(object) {
		let payload = match payload {
			Ok(payload) => payload,
			Err(error) => {
				eprintln!("unexpected object file payload: {error}");
				continue;
			}
		};

		// We are only interested in reading custom sections with our name.
		if let Payload::CustomSection(c) = payload
			&& c.name() == "js_bindgen.assembly"
		{
			for assembly in CustomSectionParser::new(c, false) {
				if assembly.is_empty() {
					continue;
				}

				file_counter += 1;
				let asm_path = archive_path.with_added_extension(format!("asm.{file_counter}.o"));

				// Only compile if the file doesn't already exist. Existing fingerprinting
				// ensures freshness:
				// https://doc.rust-lang.org/1.92.0/nightly-rustc/cargo/core/compiler/fingerprint/index.html#fingerprints-and-unithashs
				if !asm_path.exists() {
					let mut asm_file = BufWriter::new(
						File::create(&asm_path).expect("output assembly object should be writable"),
					);

					js_bindgen_ld_shared::assembly_to_object(arch_str, assembly, &mut asm_file)
						.expect("compiling assembly should be valid");

					asm_file.into_inner().unwrap().sync_all().unwrap();
				}

				add_args.push(asm_path.into());
			}
		}
	}
}

/// This removes our custom sections and generates the JS import file.
fn post_processing(output_path: &Path, main_memory: MainMemory<'_>) -> Vec<u8> {
	// Unfortunately we don't receive the final output path adjustments Cargo makes.
	// So for the JS file we just figure it out ourselves.
	let package = env::var_os("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` should be present");
	let wasm_input = ReadFile::new(output_path).expect("output file should be readable");
	let mut wasm_output = Vec::new();

	let mut found_import: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
	let mut expected_import: HashMap<&str, HashSet<&str>> = HashMap::new();
	let mut provided_import: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
	let mut found_embed: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
	let mut expected_embed: HashMap<&str, HashSet<&str>> = HashMap::new();
	let mut provided_embed: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
	let mut memory = None;

	for payload in Parser::new(0).parse_all(&wasm_input) {
		let payload = payload.expect("object file should be valid Wasm");

		match payload {
			Payload::Version { encoding, .. } => wasm_output.extend_from_slice(match encoding {
				Encoding::Module => &Module::HEADER,
				Encoding::Component => {
					unimplemented!("objects with components are not supported")
				}
			}),
			// Read what imports we need. This has already undergone dead-code elimination by LLD.
			Payload::ImportSection(i) => {
				let mut import_section = ImportSection::new();

				for i in i.into_imports() {
					let mut import = i.expect("import should be parsable");

					// This is `llvm-mc` workaround for 32-bit tables when compiling to Wasm64.
					// See https://github.com/llvm/llvm-project/issues/172907.
					// TODO: This linker is supposed to be agnostic towards `js-sys`.
					if let TypeRef::Table(t) = &mut import.ty
						&& t.table64 && import.module == "js_sys"
						&& import.name == "externref.table"
					{
						t.table64 = false;
					}

					import_section.import(
						import.module,
						import.name,
						EntityType::try_from(import.ty)
							.expect("`wasmparser` type should be convertible"),
					);

					// The main memory has its own dedicated output handling.
					if let TypeRef::Memory(m) = import.ty
						&& import.module == main_memory.module
						&& import.name == main_memory.name
					{
						memory = Some(m);
						continue;
					}

					if let Some(code) = provided_import
						.get_mut(import.module)
						.and_then(|names| names.remove(import.name))
					{
						found_import
							.entry(import.module)
							.or_default()
							.insert(import.name, code);
					} else {
						assert!(
							expected_import
								.entry(import.module)
								.or_default()
								.insert(import.name),
							"found duplicate JS import: `{}:{}`",
							import.module,
							import.name
						);
					}
				}

				import_section.append_to(&mut wasm_output);
			}
			// Don't write back our assembly sections.
			Payload::CustomSection(c) if c.name() == "js_bindgen.assembly" => (),
			// Extract all JS imports.
			Payload::CustomSection(c) if c.name().starts_with("js_bindgen.import.") => {
				let stripped = c.name().strip_prefix("js_bindgen.import.").unwrap();
				let (module, name) = stripped.split_once('.').unwrap_or_else(|| {
					panic!("found incorrectly formatted JS import custom section name: {stripped}")
				});

				let mut parser = CustomSectionParser::new(c, true);
				let mut js = parser
					.next()
					.unwrap_or_else(|| panic!("found no JS import for `{module}:{name}`"));

				let js_name = js
					.get(0..2)
					.and_then(|length| {
						let length = usize::from(u16::from_le_bytes(length.try_into().unwrap()));
						js.get(2..2 + length)
					})
					.unwrap_or_else(|| {
						panic!("found invalid JS import encoding `{module}:{name}`")
					});
				let js_name = str::from_utf8(js_name).unwrap_or_else(|e| {
					panic!("found invalid JS import encoding `{module}:{name}`: {e}")
				});
				js = &js[2 + js_name.len()..];
				let js = str::from_utf8(js).unwrap_or_else(|e| {
					panic!("found invalid JS import encoding `{module}:{name}`: {e}")
				});

				if js.is_empty() {
					continue;
				}

				if let Some(new_js) = parser.next() {
					panic!(
						"found multiple JS imports for `{module}:{name}`\n\tJS Import \
						 1:\n{}\n\tJS Import 2:\n{}",
						js,
						String::from_utf8_lossy(new_js),
					);
				}

				if expected_import
					.get_mut(module)
					.map(|names| names.remove(name))
					.unwrap_or_default()
				{
					found_import.entry(module).or_default().insert(name, js);
				} else if let Some(js_old) = provided_import
					.entry_ref(module)
					.or_default()
					.insert(name, js)
				{
					panic!(
						"found multiple JS imports for `{module}:{name}`\n\tJS Import \
						 1:\n{}\n\tJS Import 2:\n{}",
						js_old, js
					);
				}

				if js_name.is_empty() {
					continue;
				}

				if found_embed
					.get_mut(module)
					.map(|names| names.contains_key(js_name))
					.unwrap_or(false)
				{
				} else if let Some(code) = provided_embed
					.get_mut(module)
					.and_then(|names| names.remove(js_name))
				{
					found_embed.entry(module).or_default().insert(js_name, code);
				} else {
					expected_embed.entry(module).or_default().insert(js_name);
				}
			}
			// Extract all JS embeds.
			Payload::CustomSection(c) if c.name().starts_with("js_bindgen.js.") => {
				let stripped = c.name().strip_prefix("js_bindgen.js.").unwrap();
				let (module, name) = stripped.split_once('.').unwrap_or_else(|| {
					panic!("found incorrectly formatted JS import custom section name: {stripped}")
				});

				let mut parser = CustomSectionParser::new(c, false);
				let js = parser
					.next()
					.unwrap_or_else(|| panic!("found no JS embed for `{module}:{name}`"));
				let js = str::from_utf8(js).unwrap_or_else(|e| {
					panic!("found invalid JS import encoding `{module}:{name}`: {e}")
				});

				if js.is_empty() {
					continue;
				}

				if let Some(new_js) = parser.next() {
					panic!(
						"found multiple JS embeds for `{module}:{name}`\n\tJS Embed 1:\n{}\n\tJS \
						 Embed 2:\n{}",
						js,
						String::from_utf8_lossy(new_js),
					);
				}

				if expected_embed
					.get_mut(module)
					.map(|names| names.remove(name))
					.unwrap_or_default()
				{
					found_embed.entry(module).or_default().insert(name, js);
				} else if let Some(js_old) = provided_embed
					.entry_ref(module)
					.or_default()
					.insert(name, js)
				{
					panic!(
						"found multiple JS embeds for `{module}:{name}`\n\tJS Embed 1:\n{}\n\tJS \
						 Embed 2:\n{}",
						js_old, js
					);
				}
			}
			Payload::CodeSectionEntry(_) | Payload::End(_) => (),
			payload => {
				let (id, range) = payload.as_section().unwrap_or_else(|| {
					panic!(
						"expected parsable payload in {}:\n{payload:?}",
						output_path.display()
					)
				});
				RawSection {
					id,
					data: &wasm_input[range],
				}
				.append_to(&mut wasm_output);
			}
		}
	}

	let memory = memory.expect("main memory should be present");
	assert!(
		expected_import.values().all(HashSet::is_empty),
		"missing JS imports: {expected_import:?}"
	);
	assert!(
		expected_embed.values().all(HashSet::is_empty),
		"missing JS embed: {expected_embed:?}"
	);

    // After the linker is done, Cargo copies the final output to be the name of the package without the fingerprint. We do the same for the JS file.
    // TODO: Skip when detecting test.
	let js_output_path = output_path.with_file_name(package).with_extension("js");
	let mut js_output =
		BufWriter::new(File::create(&js_output_path).expect("output JS file should be writable"));

	// Create our `WebAssembly.Memory`.
	js_output
		.write_all(b"const memory = new WebAssembly.Memory({ ")
		.unwrap();

	if memory.memory64 {
		write!(js_output, "initial: {}n", memory.initial).unwrap();
	} else {
		write!(js_output, "initial: {}", memory.initial).unwrap();
	}

	if let Some(max) = memory.maximum {
		if memory.memory64 {
			write!(js_output, ", maximum: {max}n").unwrap();
		} else {
			write!(js_output, ", maximum: {max}").unwrap();
		}
	}

	if memory.memory64 {
		js_output.write_all(b", address: 'i64'").unwrap();
	}

	if memory.shared {
		js_output.write_all(b", shared: true").unwrap();
	}

	js_output.write_all(b" })\n\n").unwrap();

	// Output requested embedded JS.
	if !found_embed.is_empty() {
		js_output.write_all(b"const jsEmbed = {\n").unwrap();

		for (package, embeds) in found_embed {
			writeln!(js_output, "\t{package}: {{").unwrap();

			for (name, js) in embeds {
				write!(js_output, "\t\t\"{name}\": ").unwrap();

				for (position, line) in js.lines().with_position() {
					js_output.write_all(line.as_bytes()).unwrap();

					if let Position::First | Position::Middle = position {
						js_output.write_all(b"\n\t\t").unwrap();
					}
				}

				js_output.write_all(b",\n").unwrap();
			}

			js_output.write_all(b"\t},\n").unwrap();
		}

		js_output.write_all(b"}\n\n").unwrap();
	}

	// Create our `importObject`.
	js_output
		.write_all(b"export const importObject = {\n")
		.unwrap();
	js_output.write_all(b"\tjs_bindgen: { memory },\n").unwrap();

	for (module, names) in found_import {
		writeln!(js_output, "\t{module}: {{").unwrap();

		for (name, js) in names {
			write!(js_output, "\t\t\"{name}\": ").unwrap();

			for (position, line) in js.lines().with_position() {
				js_output.write_all(line.as_bytes()).unwrap();

				if let Position::First | Position::Middle = position {
					js_output.write_all(b"\n\t\t").unwrap();
				}
			}

			js_output.write_all(b",\n").unwrap();
		}

		js_output.write_all(b"\t},\n").unwrap();
	}

	js_output.write_all(b"}\n").unwrap();

	js_output.into_inner().unwrap().sync_all().unwrap();

	// We also need a js file with a fingerprint, otherwise the test files might overwrite each other.
	fs::copy(js_output_path, output_path.with_extension("js"))
		.expect("copy JS file should be success");

	wasm_output
}
