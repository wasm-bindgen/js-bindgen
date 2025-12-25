mod lld;

use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::{env, fs};

use hashbrown::{HashMap, HashSet};
use object::read::archive::ArchiveFile;
use wasm_encoder::{EntityType, ImportSection, Module, RawSection, Section};
use wasmparser::{Encoding, Parser, Payload};

use crate::lld::WasmLdArguments;

fn main() {
	let args: Vec<_> = env::args().collect();
	let lld = WasmLdArguments::new(&args[1..]);

	// With Wasm32 no argument is passed, but Wasm64 requires `-mwasm64`.
	let arch = if let Some(m) = lld.table.get("m") {
		m[0]
	} else {
		"wasm32"
	};

	// The output parameter starts with `-o`, the argument after is the path.
	let output_path = Path::new(lld.table["o"][0]);

	// Here we store additional files we want to pass to LLD as arguments.
	let mut asm_args: Vec<PathBuf> = Vec::new();

	for input in &lld.inputs {
		// We found a UNIX archive.
		if input.ends_with(".rlib") {
			let archive_path = Path::new(&input);
			let archive_data = match fs::read(archive_path) {
				Ok(archive_data) => archive_data,
				Err(error) => {
					eprintln!("failed to read archive file, most likely its not one: {error}");
					continue;
				}
			};
			let archive =
				ArchiveFile::parse(&*archive_data).expect("`*.rlib` should be a valid archive");

			for member in archive.members() {
				let member = match member {
					Ok(member) => member,
					Err(error) => {
						eprintln!("unable to parse archive member: {error}");
						continue;
					}
				};
				let name = match str::from_utf8(member.name()) {
					Ok(name) => name.to_owned(),
					Err(error) => {
						eprintln!("unable to convert archive member name to UTF-8: {error}");
						continue;
					}
				};
				let data = match member.data(&*archive_data) {
					Ok(object) => object,
					Err(error) => {
						eprintln!("unable to extract archive member data: {error}");
						continue;
					}
				};

				process_object(
					arch,
					&mut asm_args,
					&archive_path.with_file_name(name),
					data,
				);
			}
		} else if input.ends_with(".o") {
			let object_path = Path::new(&input);
			let object = match fs::read(object_path) {
				Ok(object) => object,
				Err(error) => {
					eprintln!("failed to read object file, most likely its not one: {error}");
					continue;
				}
			};

			process_object(arch, &mut asm_args, object_path, &object);
		}
	}

	let status = Command::new("rust-lld")
		.args(args.iter().skip(1))
		.args(asm_args)
		.status()
		.unwrap();

	if status.success() {
		post_processing(output_path)
	}

	process::exit(status.code().unwrap_or(1));
}

/// Extracts any assembly instructions from `js-bindgen`, builds object files
/// from them and passes them to the linker.
fn process_object(arch: &str, asm_args: &mut Vec<PathBuf>, archive_path: &Path, object: &[u8]) {
	let mut asm_counter = 0;

	for payload in Parser::new(0).parse_all(object) {
		let payload = match payload {
			Ok(payload) => payload,
			Err(error) => {
				eprintln!("unexpected file type in archive: {error}");
				continue;
			}
		};

		// We are only interested in reading custom sections with our name.
		if let Payload::CustomSection(c) = payload
			&& c.name() == "js_bindgen.assembly"
		{
			for assembly in c.data().split(|b| b == &b'\0').filter(|a| !a.is_empty()) {
				let asm_object =
					assembly_to_object(arch, assembly).expect("compiling ASM should be valid");

				let asm_path = archive_path.with_added_extension(format!("asm.{asm_counter}.o"));
				asm_counter += 1;
				fs::write(&asm_path, asm_object).expect("writing ASM object file should succeed");

				asm_args.push(asm_path);
			}
		}
	}
}

/// Currently this simply passes the LLVM s-format assembly to `llvm-mc` to
/// convert to an object file the linker can consume.
fn assembly_to_object(arch: &str, assembly: &[u8]) -> Result<Vec<u8>, Error> {
	let mut child = Command::new("llvm-mc")
		.arg(format!("-arch={arch}"))
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
			"`llvm-mc` process failed with status: {}\n",
			output.status
		)))
	}
}

/// Currently this only entails dropping our custom sections.
fn post_processing(output_path: &Path) {
	let package = env::var_os("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` should be present");
	let wasm_input = fs::read(output_path).expect("output file should be readable");
	let mut wasm_output = Vec::new();

	let mut js_glue: HashMap<&str, HashMap<&str, &[u8]>> = HashMap::new();
	let mut import_names: HashMap<&str, HashSet<&str>> = HashMap::new();
	let mut import_js_glues: HashMap<&str, HashMap<&str, &[u8]>> = HashMap::new();

	for payload in Parser::new(0).parse_all(&wasm_input) {
		let payload = payload.expect("object file should be valid Wasm");

		match payload {
			Payload::Version { encoding, .. } => wasm_output.extend_from_slice(match encoding {
				Encoding::Module => &Module::HEADER,
				Encoding::Component => {
					unimplemented!("objects with components are not supported")
				}
			}),
			Payload::ImportSection(i) => {
				let mut import_section = ImportSection::new();

				for i in i {
					let import = i.expect("import should be parsable");

					import_section.import(
						import.module,
						import.name,
						EntityType::try_from(import.ty)
							.expect("`wasmparser` type should be convertible"),
					);

					if let Some(glue) = import_js_glues
						.get_mut(import.module)
						.and_then(|names| names.remove(import.name))
					{
						js_glue
							.entry(import.module)
							.or_default()
							.insert(import.name, glue);
					} else {
						assert!(
							import_names
								.entry(import.module)
								.or_default()
								.insert(import.name),
							"found duplicate import: {}/{}",
							import.module,
							import.name
						);
					}
				}

				import_section.append_to(&mut wasm_output);
			}
			Payload::CustomSection(c) if c.name() == "js_bindgen.assembly" => (),
			Payload::CustomSection(c) if c.name().starts_with("js_bindgen.import.") => {
				let stripped = c.name().strip_prefix("js_bindgen.import.").unwrap();
				let (module, name) = stripped
					.split_once('.')
					.expect("custom section name should be formatted correctly");

				let mut glues = c.data().split(|b| b == &b'\0');
				let glue = glues
					.next()
					.expect("`js_bindgen.import.*` should contain at least one JS glue code");
				assert_eq!(
					glues.next(),
					Some(b"".as_slice()),
					"`js_bindgen.import.*` should contain at a `\\0` at the end"
				);

				if let Some(other_glue) = glues.next() {
					panic!(
						"found duplicate JS glue for the same import:\n\tImport: \
						 {module}/{name}\n\tJS Glue 1:\n{}\n\tJS Glue 2:\n{}",
						String::from_utf8_lossy(glue),
						String::from_utf8_lossy(other_glue),
					);
				}

				if import_names
					.get_mut(module)
					.map(|names| names.remove(name))
					.unwrap_or_default()
				{
					js_glue.entry(module).or_default().insert(name, glue);
				} else {
					assert!(
						import_js_glues
							.entry_ref(module)
							.or_default()
							.insert(name, glue)
							.is_none(),
						"found duplicate JS glue for the same import:\n\tImport: \
						 {module}/{name}\n\tJS Glue:\n{}",
						String::from_utf8_lossy(glue)
					);
				}
			}
			Payload::CodeSectionEntry(_) | Payload::End(_) => (),
			payload => {
				let (id, range) = payload
					.as_section()
					.unwrap_or_else(|| panic!("payload should be parsable: {payload:?}"));
				RawSection {
					id,
					data: &wasm_input[range],
				}
				.append_to(&mut wasm_output);
			}
		}
	}

	fs::write(output_path, wasm_output).expect("object file should be writable");

	let mut js_output = Vec::new();
	js_output.extend_from_slice(b"export const importObject = {\n");

	for (module, names) in js_glue {
		writeln!(js_output, "\t{module}: {{").unwrap();

		for (name, glue) in names {
			write!(js_output, "\t\t\"{name}\": ").unwrap();
			js_output.extend_from_slice(glue);
			js_output.extend_from_slice(b",\n");
		}

		js_output.extend_from_slice(b"\t},\n");
	}

	js_output.extend_from_slice(b"}\n");

	fs::write(
		output_path.with_file_name(package).with_extension("js"),
		js_output,
	)
	.expect("object file should be writable");
}
