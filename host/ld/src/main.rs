mod js;
mod post;
mod pre;
mod wasm_ld;

use std::ffi::OsString;
use std::process::{self, Command};
use std::{env, fs};

use clap::Parser;
use js_bindgen_shared::ReadFile;

use crate::pre::PreOutput;

#[derive(Parser)]
struct Args {
	#[arg(long)]
	web: bool,
	#[arg(allow_hyphen_values = true, trailing_var_arg = true)]
	lld: Vec<OsString>,
}

fn main() {
	// Read arguments.
	let args = argfile::expand_args_from(env::args_os(), argfile::parse_response, argfile::PREFIX)
		.unwrap();
	let args = Args::parse_from(args);

	let PreOutput {
		add_args,
		output_path,
		main_memory,
		js_store,
		is_test,
	} = pre::processing(&args);

	let status = Command::new("rust-lld")
		.args(&args.lld)
		.args(add_args)
		.status()
		.unwrap();

	if status.success() {
		let wasm_input = ReadFile::new(output_path).expect("output file should be readable");

		let wasm_output =
			post::processing(&wasm_input, main_memory, js_store, args.web, is_test).unwrap();
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
