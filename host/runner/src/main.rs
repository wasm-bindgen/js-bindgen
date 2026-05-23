mod binary;
mod config;
mod run_data;
mod runner;
mod server;
mod test;

use std::ops::ControlFlow;
use std::path::PathBuf;
use std::{env, iter};

use anyhow::{Context, Result, bail};
use js_bindgen_shared::ReadFile;
use wasmparser::{MemoryType, Parser, Payload, TypeRef};

use crate::binary::BinaryParser;
use crate::run_data::RunData;
use crate::runner::{JsOutput, Runner};
use crate::test::{TestCli, TestParser};

fn main() -> Result<()> {
	// We change the current directory when going through the local development
	// cargo shim. To work correctly with relative path, we reset to the original
	// directory the runner was called from.
	if let Some(path) = env::var_os("JBG_DEV_CWD") {
		env::set_current_dir(path)?;
	}

	let mut args = env::args_os();
	let binary = args
		.next()
		.context("expected the first argument to be present")?;
	// We parse the file argument ourselves to prevent it from being shown on the
	// help page.
	let file = args
		.next()
		.context("expected a file to have been passed from `cargo run/test`")?;
	let wasm_path = PathBuf::from(&file);
	let wasm_bytes = ReadFile::new(&wasm_path)
		.with_context(|| format!("failed to read Wasm file: {}", wasm_path.display()))?;

	let mut js_output = None;
	let mut memories = Vec::new();
	let mut binary_parser = Some(BinaryParser::default());
	let mut binary_data = None;
	let mut test_parser = Some(TestParser::default());
	let mut test_data = None;

	for payload in Parser::new(0).parse_all(&wasm_bytes) {
		let payload = payload?;

		if let Payload::ImportSection(section) = &payload {
			for import in section.clone().into_imports() {
				let import = import?;

				if let TypeRef::Memory(memory) = import.ty {
					memories.push(Memory {
						module: import.module,
						name: import.name,
						data: memory,
					});
				}
			}
		}

		if let Payload::CustomSection(section) = &payload
			&& section.name() == js_bindgen_cli_lib::JS_OUTPUT_SECTION
		{
			let raw: js_bindgen_cli_lib::JsOutput<&str> = postcard::from_bytes(section.data())?;
			let mut output = Vec::new();

			let Some(main_memory) = memories.iter().find(|memory| {
				memory.module == raw.main_memory.module && memory.name == raw.main_memory.name
			}) else {
				bail!("unable to find main memory as encoded")
			};

			raw.js(&mut output, main_memory.data)?;
			js_output = Some(output);
		}

		if let Some(parser) = test_parser.take() {
			match parser.parse(&payload)? {
				ControlFlow::Continue(parser) => test_parser = Some(parser),
				ControlFlow::Break(data) => test_data = Some(data),
			}
		}

		if let Some(parser) = binary_parser.take() {
			match parser.parse(&payload)? {
				ControlFlow::Continue(parser) => binary_parser = Some(parser),
				ControlFlow::Break(data) => binary_data = data,
			}
		}

		if js_output.is_some()
			&& (test_data.is_some() || (test_parser.is_none() && binary_parser.is_none()))
		{
			break;
		}
	}

	let run_data = if let Some(tests) = test_data {
		TestCli::run(iter::once(binary).chain(args), tests)
	} else if let Some(main) = binary_data {
		Some(RunData::Binary {
			wasm64: main.wasm64,
			memory: main.memory,
			args: iter::once(file).chain(args).collect(),
		})
	} else {
		bail!("no target found to run")
	};

	if let Some(run_data) = run_data {
		let js_output = if let Some(js_output) = js_output {
			JsOutput::Output(js_output)
		} else {
			// The JS file has the same name, just a different file extension.
			let path = wasm_path.with_extension("mjs");
			println!("{}", path.display());
			JsOutput::File {
				file: ReadFile::new(&path)?,
				path,
			}
		};
		let run_data = serde_json::to_string(&run_data).unwrap();

		Runner::new(wasm_path, wasm_bytes, js_output, run_data).run()?;
	}

	Ok(())
}

struct Memory<'a> {
	module: &'a str,
	name: &'a str,
	data: MemoryType,
}
