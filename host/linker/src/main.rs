use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::io::{Error, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::{env, fs};

use hashbrown::{HashMap, HashSet};
use object::read::archive::ArchiveFile;
use wasm_encoder::{EntityType, ImportSection, Module, RawSection, Section};
use wasmparser::{Encoding, Parser, Payload};

/// `wasm-ld`'s options and kinds.
///
/// How to generate:
///
/// ```sh
/// llvm-tblgen --dump-json lld/wasm/Options.td -o options.json -I llvm/include
/// ```
///
/// ```ignore
/// let opt_table: BTreeMap<String, Value> =
///     serde_json::from_slice(&std::fs::read("options.json").unwrap()).unwrap();
/// let mut vec = Vec::new();
/// for option in opt_table.values() {
///     if let Some(name) = option.get("Name").and_then(|n| n.as_str())
///         && let Some(kind) = option
///             .get("Kind")
///             .and_then(|def| def.get("def"))
///             .and_then(|s| s.as_str())
///     {
///         if kind == "KIND_INPUT" || kind == "KIND_UNKNOWN" {
///             continue;
///         }
///         vec.push((name, format!("OptKind::{kind}")));
///     }
/// }
///
/// println!("{vec:#?}");
/// ```
///
/// And copy to here.
static OPT_KIND: [(&str, OptKind); 149] = [
	("Bdynamic", OptKind::KIND_FLAG),
	("Bstatic", OptKind::KIND_FLAG),
	("Bsymbolic", OptKind::KIND_FLAG),
	("Map", OptKind::KIND_SEPARATE),
	("Map=", OptKind::KIND_JOINED),
	("O", OptKind::KIND_JOINED_OR_SEPARATE),
	("allow-multiple-definition", OptKind::KIND_FLAG),
	("allow-undefined", OptKind::KIND_FLAG),
	("allow-undefined-file=", OptKind::KIND_JOINED),
	("allow-undefined-file", OptKind::KIND_SEPARATE),
	("e", OptKind::KIND_JOINED_OR_SEPARATE),
	("entry=", OptKind::KIND_JOINED),
	("library=", OptKind::KIND_JOINED),
	("library-path", OptKind::KIND_SEPARATE),
	("library-path=", OptKind::KIND_JOINED),
	("M", OptKind::KIND_FLAG),
	("r", OptKind::KIND_FLAG),
	("s", OptKind::KIND_FLAG),
	("S", OptKind::KIND_FLAG),
	("t", OptKind::KIND_FLAG),
	("y", OptKind::KIND_JOINED_OR_SEPARATE),
	("u", OptKind::KIND_JOINED_OR_SEPARATE),
	("call_shared", OptKind::KIND_FLAG),
	("V", OptKind::KIND_FLAG),
	("dy", OptKind::KIND_FLAG),
	("dn", OptKind::KIND_FLAG),
	("non_shared", OptKind::KIND_FLAG),
	("static", OptKind::KIND_FLAG),
	("E", OptKind::KIND_FLAG),
	("i", OptKind::KIND_FLAG),
	("library", OptKind::KIND_SEPARATE),
	("build-id", OptKind::KIND_FLAG),
	("build-id=", OptKind::KIND_JOINED),
	("check-features", OptKind::KIND_FLAG),
	("color-diagnostics", OptKind::KIND_FLAG),
	("color-diagnostics=", OptKind::KIND_JOINED),
	("compress-relocations", OptKind::KIND_FLAG),
	("demangle", OptKind::KIND_FLAG),
	("disable-verify", OptKind::KIND_FLAG),
	("emit-relocs", OptKind::KIND_FLAG),
	("end-lib", OptKind::KIND_FLAG),
	("entry", OptKind::KIND_SEPARATE),
	("error-limit", OptKind::KIND_SEPARATE),
	("error-limit=", OptKind::KIND_JOINED),
	("error-unresolved-symbols", OptKind::KIND_FLAG),
	("experimental-pic", OptKind::KIND_FLAG),
	("export", OptKind::KIND_SEPARATE),
	("export-all", OptKind::KIND_FLAG),
	("export-dynamic", OptKind::KIND_FLAG),
	("export=", OptKind::KIND_JOINED),
	("export-if-defined", OptKind::KIND_SEPARATE),
	("export-if-defined=", OptKind::KIND_JOINED),
	("export-memory", OptKind::KIND_FLAG),
	("export-memory=", OptKind::KIND_JOINED),
	("export-table", OptKind::KIND_FLAG),
	("extra-features=", OptKind::KIND_COMMAJOINED),
	("fatal-warnings", OptKind::KIND_FLAG),
	("features=", OptKind::KIND_COMMAJOINED),
	("gc-sections", OptKind::KIND_FLAG),
	("global-base=", OptKind::KIND_JOINED),
	("growable-table", OptKind::KIND_FLAG),
	("help", OptKind::KIND_FLAG),
	("import-memory", OptKind::KIND_FLAG),
	("import-memory=", OptKind::KIND_JOINED),
	("import-table", OptKind::KIND_FLAG),
	("import-undefined", OptKind::KIND_FLAG),
	("initial-heap=", OptKind::KIND_JOINED),
	("initial-memory=", OptKind::KIND_JOINED),
	("keep-section", OptKind::KIND_SEPARATE),
	("keep-section=", OptKind::KIND_JOINED),
	("l", OptKind::KIND_JOINED_OR_SEPARATE),
	("L", OptKind::KIND_JOINED_OR_SEPARATE),
	("lto-CGO", OptKind::KIND_JOINED),
	("lto-O", OptKind::KIND_JOINED),
	("lto-debug-pass-manager", OptKind::KIND_FLAG),
	("lto-obj-path=", OptKind::KIND_JOINED),
	("lto-partitions=", OptKind::KIND_JOINED),
	("m", OptKind::KIND_JOINED_OR_SEPARATE),
	("max-memory=", OptKind::KIND_JOINED),
	("merge-data-segments", OptKind::KIND_FLAG),
	("mllvm", OptKind::KIND_SEPARATE),
	("mllvm=", OptKind::KIND_JOINED),
	("no-allow-multiple-definition", OptKind::KIND_FLAG),
	("no-check-features", OptKind::KIND_FLAG),
	("no-color-diagnostics", OptKind::KIND_FLAG),
	("no-demangle", OptKind::KIND_FLAG),
	("no-entry", OptKind::KIND_FLAG),
	("no-export-dynamic", OptKind::KIND_FLAG),
	("no-fatal-warnings", OptKind::KIND_FLAG),
	("no-gc-sections", OptKind::KIND_FLAG),
	("no-growable-memory", OptKind::KIND_FLAG),
	("no-merge-data-segments", OptKind::KIND_FLAG),
	("no-pie", OptKind::KIND_FLAG),
	("no-print-gc-sections", OptKind::KIND_FLAG),
	("no-shlib-sigcheck", OptKind::KIND_FLAG),
	("no-stack-first", OptKind::KIND_FLAG),
	("no-whole-archive", OptKind::KIND_FLAG),
	("noinhibit-exec", OptKind::KIND_FLAG),
	("o", OptKind::KIND_JOINED_OR_SEPARATE),
	("page-size=", OptKind::KIND_JOINED),
	("pie", OptKind::KIND_FLAG),
	("print-gc-sections", OptKind::KIND_FLAG),
	("print-map", OptKind::KIND_FLAG),
	("relocatable", OptKind::KIND_FLAG),
	("reproduce", OptKind::KIND_SEPARATE),
	("reproduce=", OptKind::KIND_JOINED),
	("rpath", OptKind::KIND_SEPARATE),
	("rpath=", OptKind::KIND_JOINED),
	("rsp-quoting", OptKind::KIND_SEPARATE),
	("rsp-quoting=", OptKind::KIND_JOINED),
	("save-temps", OptKind::KIND_FLAG),
	("shared", OptKind::KIND_FLAG),
	("shared-memory", OptKind::KIND_FLAG),
	("soname", OptKind::KIND_SEPARATE),
	("soname=", OptKind::KIND_JOINED),
	("stack-first", OptKind::KIND_FLAG),
	("start-lib", OptKind::KIND_FLAG),
	("strip-all", OptKind::KIND_FLAG),
	("strip-debug", OptKind::KIND_FLAG),
	("table-base=", OptKind::KIND_JOINED),
	("thinlto-cache-dir=", OptKind::KIND_JOINED),
	("thinlto-cache-policy", OptKind::KIND_SEPARATE),
	("thinlto-cache-policy=", OptKind::KIND_JOINED),
	("thinlto-emit-imports-files", OptKind::KIND_FLAG),
	("thinlto-emit-index-files", OptKind::KIND_FLAG),
	("thinlto-index-only", OptKind::KIND_FLAG),
	("thinlto-index-only=", OptKind::KIND_JOINED),
	("thinlto-jobs=", OptKind::KIND_JOINED),
	("thinlto-object-suffix-replace=", OptKind::KIND_JOINED),
	("thinlto-prefix-replace=", OptKind::KIND_JOINED),
	("threads", OptKind::KIND_SEPARATE),
	("threads=", OptKind::KIND_JOINED),
	("trace", OptKind::KIND_FLAG),
	("trace-symbol", OptKind::KIND_SEPARATE),
	("trace-symbol=", OptKind::KIND_JOINED),
	("undefined", OptKind::KIND_SEPARATE),
	("undefined=", OptKind::KIND_JOINED),
	("unresolved-symbols", OptKind::KIND_SEPARATE),
	("unresolved-symbols=", OptKind::KIND_JOINED),
	("v", OptKind::KIND_FLAG),
	("verbose", OptKind::KIND_FLAG),
	("version", OptKind::KIND_FLAG),
	("warn-unresolved-symbols", OptKind::KIND_FLAG),
	("whole-archive", OptKind::KIND_FLAG),
	("why-extract=", OptKind::KIND_JOINED),
	("wrap", OptKind::KIND_SEPARATE),
	("wrap=", OptKind::KIND_JOINED),
	("z", OptKind::KIND_JOINED_OR_SEPARATE),
	// rust-lld
	("flavor", OptKind::KIND_SEPARATE),
];

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
enum OptKind {
	// --extra-features=a,b,c
	KIND_COMMAJOINED,
	// --gc-sections
	KIND_FLAG,
	// --export=xx
	KIND_JOINED,
	// -ofoo.wasm OR -o foo.wasm
	KIND_JOINED_OR_SEPARATE,
	// --export xx
	KIND_SEPARATE,
}

#[derive(Debug)]
struct WasmLdArguments<'a> {
	table: HashMap<&'a str, Vec<&'a OsStr>>,
	inputs: Vec<&'a OsString>,
}

fn main() {
	let args: Vec<_> = env::args_os().collect();
	let lld = extract_inputs(&args[1..]);

	// With Wasm32 no argument is passed, but Wasm64 requires `-mwasm64`.
	let arch = if let Some(m) = lld.table.get("m") {
		m[0].to_str().expect("-m value should be utf8")
	} else {
		"wasm32"
	};

	// The output parameter starts with `-o`, the argument after is the path.
	let output_path = Path::new(lld.table["o"][0]);

	// Here we store additional files we want to pass to LLD as arguments.
	let mut asm_args: Vec<PathBuf> = Vec::new();

	for input in &lld.inputs {
		// We found a UNIX archive.
		if input.as_encoded_bytes().ends_with(b".rlib") {
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
		} else if input.as_encoded_bytes().ends_with(b".o") {
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

fn extract_inputs(args: &[OsString]) -> WasmLdArguments<'_> {
	let mut args = args.iter();
	let mut lld = WasmLdArguments {
		table: HashMap::new(),
		inputs: Vec::new(),
	};

	let option_table = HashMap::from(OPT_KIND);

	loop {
		let Some(arg) = args.next() else {
			break;
		};

		let bytes = arg.as_encoded_bytes();

		// In the LLVM Parser, if a value does not start with `-`, it is treated as INPUT.
		let Some(stripped) = bytes
			.strip_prefix(b"--")
			.or_else(|| bytes.strip_prefix(b"-"))
		else {
			lld.inputs.push(arg);
			continue;
		};

		// Find the longest substring and the option kind.
		//
		// TODO: optimize with binary search
		for end in (0..=stripped.len()).rev() {
			if let Ok(sub) = str::from_utf8(&stripped[0..end])
				&& let Some(kind) = option_table.get(sub)
			{
				let mut next = || {
					args.next()
						.expect("separate kind should be have value")
						.as_os_str()
				};
				let remain = || OsStr::from_bytes(&stripped[end..]);
				let value = match kind {
					OptKind::KIND_FLAG => None,
					OptKind::KIND_SEPARATE => Some(next()),
					OptKind::KIND_COMMAJOINED | OptKind::KIND_JOINED => Some(remain()),
					OptKind::KIND_JOINED_OR_SEPARATE => Some(if stripped.len() == end {
						next()
					} else {
						remain()
					}),
				};
				if let Some(value) = value {
					lld.table.entry(sub).or_default().push(value);
				} else {
					lld.table.insert(sub, Vec::new());
				}
				break;
			}
		}
	}

	lld
}
