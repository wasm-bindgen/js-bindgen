use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

use anyhow::Result;
use js_bindgen_ld_shared::JsBindgenAssemblySectionParser;
use wasmparser::{Parser, Payload};

use crate::js::JsStore;
use crate::wasm_ld::WasmLdArguments;

pub struct PreOutput<'args> {
	pub add_args: Vec<OsString>,
	pub output_path: &'args Path,
	pub main_memory: MainMemory<'args>,
	pub js_store: JsStore,
}

#[derive(Clone, Copy)]
pub struct MainMemory<'a> {
	pub module: &'a str,
	pub name: &'a str,
}

#[derive(Clone, Copy)]
enum Arch {
	Wasm32,
	Wasm64,
}

pub fn processing(args: &[OsString]) -> PreOutput<'_> {
	let wasm_ld_args = WasmLdArguments::new(&args[1..]);

	if wasm_ld_args
		.arg_single("flavor")
		.is_none_or(|v| v != "wasm")
	{
		panic!("the `js-bindgen-ld` should only be used when compiling to a Wasm target")
	}

	let output_path = Path::new(
		wasm_ld_args
			.arg_single("o")
			.expect("output path argument should be present"),
	);

	// With Wasm32 no argument is passed, but Wasm64 requires `-mwasm64`.
	let arch = if let Some(m) = wasm_ld_args.arg_single("m") {
		if m == "wasm32" {
			Arch::Wasm32
		} else if m == "wasm64" {
			Arch::Wasm64
		} else {
			panic!("expected `-m` to either be `wasm32` or `wasm64");
		}
	} else {
		Arch::Wasm32
	};

	// Here we store additional arguments we want to pass to `wasm-ld`.
	let mut add_args: Vec<OsString> = Vec::new();

	// Extract path to the main memory if user-specified, otherwise force export
	// with our own path.
	let main_memory = main_memory(arch, &wasm_ld_args, &mut add_args);

	let mut js_store = JsStore::default();

	// Extract embedded assembly from object files.
	for input in wasm_ld_args.inputs() {
		js_bindgen_ld_shared::ld_input_parser(input, |path, data, object_mtime| {
			process_object(&mut js_store, &mut add_args, path, data, object_mtime)
		})
		.unwrap()
		.unwrap();
	}

	PreOutput {
		add_args,
		output_path,
		main_memory,
		js_store,
	}
}

/// Extracts any assembly instructions from `js-bindgen`, builds object files
/// from them and passes them to the linker.
fn process_object(
	js_store: &mut JsStore,
	add_args: &mut Vec<OsString>,
	archive_path: &Path,
	object: &[u8],
	object_mtime: Option<SystemTime>,
) -> Result<()> {
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
		match &payload {
			Payload::CustomSection(c) if c.name() == "js_bindgen.assembly" => {
				for wat in JsBindgenAssemblySectionParser::new(c) {
					file_counter += 1;
					let wasm_path =
						archive_path.with_added_extension(format!("wasm.{file_counter}.o"));

					// We first use a fingerprint to quickly determine whether `wasm.o` needs to be
					// regenerated: https://doc.rust-lang.org/1.92.0/nightly-rustc/cargo/core/compiler/fingerprint/index.html#fingerprints-and-unithashs
					//
					// Then we compare the `mtime` of the `.o` files with that of `wasm.o`. If it is
					// `None`(should not occur on major platforms), or if the `.o` files are
					// newer than `wasm.o`, we regenerate `wasm.o`.
					if !wasm_path.exists() || {
						js_bindgen_shared::mtime(&std::fs::metadata(&wasm_path)?)?
							.zip(object_mtime)
							.is_none_or(|(t1, t2)| t1 < t2)
					} {
						let asm = js_bindgen_ld_shared::wat_to_object(wat);
						fs::write(&wasm_path, asm).unwrap();
					}

					add_args.push(wasm_path.into());
				}
			}
			// Extract all JS imports.
			Payload::CustomSection(c) if c.name() == "js_bindgen.import" => {
				js_store.add_js_imports(c)?;
			}
			// Extract all JS embeds.
			Payload::CustomSection(c) if c.name() == "js_bindgen.embed" => {
				js_store.add_js_embeds(c)?;
			}
			_ => (),
		}
	}

	Ok(())
}

fn main_memory<'args>(
	arch: Arch,
	wasm_ld_args: &WasmLdArguments<'args>,
	add_args: &mut Vec<OsString>,
) -> MainMemory<'args> {
	// Always force max memory.
	if wasm_ld_args.arg_single("max-memory=").is_none() {
		match arch {
			Arch::Wasm32 => add_args.push(OsString::from("--max-memory=4294967296")),
			Arch::Wasm64 => add_args.push(OsString::from("--max-memory=17179869184")),
		}
	}

	match wasm_ld_args.arg_single("import-memory=") {
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
	}
}
