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
		is_test,
	} = pre::processing(&args);

	let status = Command::new("rust-lld")
		.args(args.iter().skip(1))
		.args(add_args)
		.status()
		.unwrap();

	if status.success() {
		let wasm_input = ReadFile::new(output_path).expect("output file should be readable");

		let output_js = env::var_os("JBG_OUTPUT_JS").is_some_and(|value| value == "1");

		let (wasm_output, main_memory, js_output) =
			post::processing(&wasm_input, main_memory, js_store, is_test, !output_js).unwrap();
		drop(wasm_input);

		// We could write into the file directly, but `wasm-encoder` doesn't support
		// `io::Write`: https://github.com/bytecodealliance/wasm-tools/issues/778.
		//
		// When it does, we should rename the old file and write to a new file. This way
		// we can keep parsing and writing at the same time without allocating memory.
		fs::write(output_path, wasm_output).expect("output Wasm file should be writable");

		if output_js {
			let js_output_path = output_path.with_extension("mjs");
			let mut js_output_file = BufWriter::new(
				File::create(&js_output_path).expect("output JS file should be writable"),
			);

			js_output.js(&mut js_output_file, main_memory).unwrap();

			js_output_file.into_inner().unwrap().sync_all().unwrap();
		}
	}

	if !status.success() {
		process::exit(status.code().unwrap_or(1));
	}
}
