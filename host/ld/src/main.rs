mod js;
mod ld_args;
mod post;
mod pre;

use std::process::{self, Command};
use std::{env, fs};

use js_bindgen_shared::ReadFile;

use crate::ld_args::LdArguments;
use crate::pre::PreOutput;

fn main() {
	// Read arguments.
	let args = argfile::expand_args_from(env::args_os(), argfile::parse_response, argfile::PREFIX)
		.unwrap();
	let args = LdArguments::new(&args[1..]);

	let PreOutput {
		add_args,
		output_path,
		main_memory,
		js_store,
		is_test,
	} = pre::processing(&args);

	let status = Command::new("rust-lld")
		.args(args.raw_wasm_ld_args())
		.args(add_args)
		.status()
		.unwrap();

	if status.success() {
		let wasm_input = ReadFile::new(output_path).expect("output file should be readable");

		let wasm_output =
			post::processing(&wasm_input, main_memory, js_store, args.web(), is_test).unwrap();
		drop(wasm_input);

		// We could write into the file directly, but `wasm-encoder` doesn't support
		// `io::Write`: https://github.com/bytecodealliance/wasm-tools/issues/778.
		//
		// When it does, we should rename the old file and write to a new file. This way
		// we can keep parsing and writing at the same time without allocating memory.
		fs::write(output_path, wasm_output).expect("output Wasm file should be writable");
	}

	if !status.success() {
		process::exit(status.code().unwrap_or(1));
	}
}
