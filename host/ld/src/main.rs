mod js;
mod post;
mod pre;
mod wasm_ld;

use std::fs::File;
use std::io::BufWriter;
use std::process::{self, Command};
use std::{env, fs};

use js_bindgen_shared::ReadFile;

use crate::pre::PreOutput;

fn main() {
	// Read arguments.
	let args = argfile::expand_args_from(env::args_os(), argfile::parse_response, argfile::PREFIX)
		.unwrap();

	let PreOutput {
		add_args,
		output_path,
		main_memory,
		js_store,
	} = pre::processing(&args);

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
			post::processing(&wasm_input, &mut js_output, main_memory, js_store).unwrap();
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
