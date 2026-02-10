mod wasm_ld;

use std::borrow::Cow;
use std::convert::Infallible;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::process::{self, Command};
use std::{env, fs};

use js_bindgen_ld_lib::MainMemory;
use js_bindgen_ld_shared::JsBindgenAssemblySectionParser;
use js_bindgen_shared::ReadFile;
use wasmparser::{Parser, Payload};

use crate::wasm_ld::WasmLdArguments;

fn main() {
	let args = argfile::expand_args_from(env::args_os(), argfile::parse_response, argfile::PREFIX)
		.unwrap();
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
		});
	}

	let status = Command::new("rust-lld")
		.args(args.iter().skip(1))
		.args(add_args)
		.status()
		.unwrap();

	if status.success() {
		// Unfortunately we don't receive the final output path adjustments Cargo makes.
		// So for the JS file we just figure it out ourselves.
		let package =
			env::var_os("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` should be present");
		let wasm_input = ReadFile::new(output_path).expect("output file should be readable");

		let js_output_path = output_path.with_extension("mjs");
		let mut js_output = BufWriter::new(
			File::create(&js_output_path).expect("output JS file should be writable"),
		);

		let wasm_output =
			js_bindgen_ld_lib::post_processing(&wasm_input, &mut js_output, main_memory);
		drop(wasm_input);

		// We could write into the file directly, but `wasm-encoder` doesn't support
		// `io::Write`: https://github.com/bytecodealliance/wasm-tools/issues/778.
		//
		// When it does, we should rename the old file and write to a new file. This way
		// we can keep parsing and writing at the same time without allocating memory.
		fs::write(output_path, wasm_output).expect("output Wasm file should be writable");

		js_output.into_inner().unwrap().sync_all().unwrap();

		// After the linker is done, Cargo copies the final output to be the name of the
		// package without the fingerprint. We do the same for the JS file. TODO: Skip
		// when detecting test.
		fs::copy(
			js_output_path,
			output_path.with_file_name(package).with_extension("mjs"),
		)
		.expect("copy JS file should be success");
	}

	process::exit(status.code().unwrap_or(1));
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
			for assembly in JsBindgenAssemblySectionParser::new(c) {
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
