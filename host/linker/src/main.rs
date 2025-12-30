mod lld;

use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::io::{Error, Write};
use std::path::Path;
use std::process::{self, Command, Stdio};
use std::{env, fs};

use hashbrown::{HashMap, HashSet};
use object::read::archive::ArchiveFile;
use wasm_encoder::{EntityType, ImportSection, Module, RawSection, Section};
use wasmparser::{Encoding, Parser, Payload, TypeRef};

use crate::lld::WasmLdArguments;

fn main() {
	let args: Vec<_> = env::args_os().collect();
	let lld_args = WasmLdArguments::new(&args[1..]);

	if lld_args
		.arg_single("flavor")
		.filter(|v| *v == "wasm")
		.is_none()
	{
		panic!("the `js-bindgen-linker` should only be used when compiling to a Wasm target")
	}

	// With Wasm32 no argument is passed, but Wasm64 requires `-mwasm64`.
	let arch_str = if let Some(m) = lld_args.arg_single("m") {
		if m == "wasm32" || m == "wasm64" {
			Cow::Borrowed(m)
		} else {
			panic!("expected `-m` to either be `wasm32` or `wasm64");
		}
	} else {
		Cow::Owned("wasm32".into())
	};

	let output_path = Path::new(
		lld_args
			.arg_single("o")
			.expect("output path argument should be present"),
	);

	// Here we store additional arguments we want to pass to LLD.
	let mut add_args: Vec<OsString> = Vec::new();

	// Extract path to the main memory if user-specified, otherwise force export
	// with our own path.
	let main_memory = match lld_args.arg_single("import-memory=") {
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
			if lld_args.arg_flag("import-memory") {
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
	for input in lld_args.inputs() {
		// We found a UNIX archive.
		if input.as_encoded_bytes().ends_with(b".rlib") {
			let archive_path = Path::new(&input);
			let archive_data = match fs::read(archive_path) {
				Ok(archive_data) => archive_data,
				Err(error) => {
					eprintln!(
						"failed to read archive file {}:\n{error}",
						archive_path.display()
					);
					continue;
				}
			};
			let archive = match ArchiveFile::parse(&*archive_data) {
				Ok(archive_data) => archive_data,
				Err(error) => {
					eprintln!(
						"failed to parse archive file {}:\n{error}",
						archive_path.display()
					);
					continue;
				}
			};

			for member in archive.members() {
				let member = match member {
					Ok(member) => member,
					Err(error) => {
						eprintln!(
							"unable to parse archive member in {}:\n{error}",
							archive_path.display()
						);
						continue;
					}
				};
				let name = match str::from_utf8(member.name()) {
					Ok(name) => name.to_owned(),
					Err(error) => {
						eprintln!(
							"unable to convert archive member name to UTF-8 in {}:\n{error}",
							archive_path.display()
						);
						continue;
					}
				};
				let data = match member.data(&*archive_data) {
					Ok(object) => object,
					Err(error) => {
						eprintln!(
							"unable to extract archive member data from {}:\n{error}",
							archive_path.display()
						);
						continue;
					}
				};

				process_object(
					&arch_str,
					&mut add_args,
					&archive_path.with_file_name(name),
					data,
				);
			}
		} else if input.as_encoded_bytes().ends_with(b".o") {
			let object_path = Path::new(&input);
			let object = match fs::read(object_path) {
				Ok(object) => object,
				Err(error) => {
					eprintln!(
						"failed to read object file {}:\n{error}",
						object_path.display()
					);
					continue;
				}
			};

			process_object(&arch_str, &mut add_args, object_path, &object);
		}
	}

	let status = Command::new("rust-lld")
		.args(args.iter().skip(1))
		.args(add_args)
		.status()
		.unwrap();

	if status.success() {
		post_processing(output_path, main_memory)
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
			let mut data = c.data();

			while let Some(length) = data.get(..4) {
				data = &data[4..];
				let length = u32::from_le_bytes(length.try_into().unwrap()) as usize;

				if length == 0 {
					continue;
				}

				let assembly = data
					.get(..length)
					.expect("invalid length encoding for assembly");
				data = &data[length..];

				let asm_object = assembly_to_object(arch_str, assembly)
					.expect("compiling assembly should be valid");

				let asm_path = archive_path.with_added_extension(format!("asm.{file_counter}.o"));
				file_counter += 1;
				fs::write(&asm_path, asm_object)
					.expect("writing assembly object file should succeed");

				add_args.push(asm_path.into());
			}

			assert!(
				data.is_empty(),
				"found left over bytes in assembly: {data:?}"
			);
		}
	}
}

/// Currently this simply passes the LLVM s-format assembly to `llvm-mc` to
/// convert to an object file the linker can consume.
fn assembly_to_object(arch_str: &OsStr, assembly: &[u8]) -> Result<Vec<u8>, Error> {
	let mut child = Command::new("llvm-mc")
		.arg(format!("-arch={}", arch_str.display()))
		// In the future we will switch to something supporting auto-detection.
		.arg("-mattr=+reference-types,+call-indirect-overlong")
		.arg("-filetype=obj")
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.stdin(Stdio::piped())
		.spawn()?;

	let stdin = child
		.stdin
		.as_mut()
		.ok_or_else(|| Error::other("`llvm-mc` process should have `stdin`"))?;
	stdin.write_all(assembly)?;

	let output = child.wait_with_output()?;

	if output.status.success() {
		Ok(output.stdout)
	} else {
		eprintln!(
			"------ llvm-mc input -------\n{}",
			String::from_utf8_lossy(assembly)
		);

		if !output.stdout.is_empty() {
			eprintln!(
				"------ llvm-mc stdout ------\n{}",
				String::from_utf8_lossy(&output.stdout)
			);

			if !output.stdout.ends_with(b"\n") {
				eprintln!();
			}
		}

		if !output.stderr.is_empty() {
			eprintln!(
				"------ llvm-mc stderr ------\n{}",
				String::from_utf8_lossy(&output.stderr)
			);

			if !output.stderr.ends_with(b"\n") {
				eprintln!();
			}
		}

		Err(Error::other(format!(
			"`llvm-mc` process failed with status: {}",
			output.status
		)))
	}
}

/// This removes our custom sections and generates the JS import file.
fn post_processing(output_path: &Path, main_memory: MainMemory<'_>) {
	// Unfortunately we don't receive the final output path adjustments Cargo makes.
	// So for the JS file we just figure it out ourselves.
	let package = env::var_os("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` should be present");
	let wasm_input = fs::read(output_path).expect("output file should be readable");
	let mut wasm_output = Vec::new();

	let mut found_import: HashMap<&str, HashMap<&str, &[u8]>> = HashMap::new();
	let mut expected_import: HashMap<&str, HashSet<&str>> = HashMap::new();
	let mut provided_import: HashMap<&str, HashMap<&str, &[u8]>> = HashMap::new();
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

				for i in i {
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
							"found duplicate import: `{}:{}`",
							import.module,
							import.name
						);
					}
				}

				import_section.append_to(&mut wasm_output);
			}
			// Don't write back our assembly sections.
			Payload::CustomSection(c) if c.name() == "js_bindgen.assembly" => (),
			// Extract all embedded imports.
			Payload::CustomSection(c) if c.name().starts_with("js_bindgen.import.") => {
				let stripped = c.name().strip_prefix("js_bindgen.import.").unwrap();
				let (module, name) = stripped
					.split_once('.')
					.expect("custom section name should be formatted correctly");

				let mut data = c.data();
				let length = data
					.get(..4)
					.unwrap_or_else(|| panic!("found no JS import for `{module}:{name}`"));
				data = &data[4..];
				let length = u32::from_le_bytes(length.try_into().unwrap()) as usize;

				if length == 0 {
					panic!("found empty JS import for `{module}:{name}`")
				}

				let js = data.get(..length).unwrap_or_else(|| {
					panic!("invalid length encoding in JS import for `{module}:{name}`")
				});
				data = &data[length..];

				assert!(
					data.is_empty(),
					"found multiple JS imports for `{module}:{name}`\n\tJS Import 1:\n{}\n\tJS \
					 Import 2:\n{}",
					String::from_utf8_lossy(js),
					String::from_utf8_lossy(data),
				);

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
						String::from_utf8_lossy(js_old),
						String::from_utf8_lossy(js)
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

	fs::write(output_path, wasm_output).expect("output Wasm file should be writable");

	let mut js_output = Vec::new();

	// Create our `WebAssembly.Memory`.
	js_output.extend_from_slice(b"const memory = new WebAssembly.Memory({ ");

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
		js_output.extend_from_slice(b", address: 'i64'");
	}

	if memory.shared {
		js_output.extend_from_slice(b", shared: true");
	}

	js_output.extend_from_slice(b" })\n");

	// Create our `importObject`.
	js_output.extend_from_slice(b"export const importObject = {\n");
	js_output.extend_from_slice(b"\tjs_bindgen: { memory },");

	for (module, names) in found_import {
		writeln!(js_output, "\t{module}: {{").unwrap();

		for (name, js) in names {
			write!(js_output, "\t\t\"{name}\": ").unwrap();
			js_output.extend_from_slice(js);
			js_output.extend_from_slice(b",\n");
		}

		js_output.extend_from_slice(b"\t},\n");
	}

	js_output.extend_from_slice(b"}\n");

	fs::write(
		output_path.with_file_name(package).with_extension("js"),
		js_output,
	)
	.expect("output JS file should be writable");
}
