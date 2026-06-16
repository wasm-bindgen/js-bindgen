mod config;
mod run_data;
mod runner;
mod server;
mod test;

use std::path::PathBuf;
use std::{env, iter};

use anyhow::{Context, Result, bail};
use js_bindgen_cli_lib::{JS_OUTPUT_SECTION, JsOutput};
use js_bindgen_shared::{IS_COMPAT_SECTION, ReadFile};
use wasmparser::{MemoryType, Parser, Payload, TypeRef};

use crate::run_data::{Ctx, RunData};
use crate::runner::Runner;
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

	let mut web = true;
	let mut js_output = None;
	let mut memories = Vec::new();
	let mut test_parser = TestParser::new();

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

		if let Payload::CustomSection(section) = &payload {
			match section.name() {
				IS_COMPAT_SECTION => web = false,
				JS_OUTPUT_SECTION => js_output = Some(postcard::from_bytes(section.data())?),
				_ => (),
			}
		}

		test_parser.parse(&payload)?;

		// We found everything we need.
		if js_output.is_some() && !web && test_parser.found() {
			break;
		}
	}

	let js_output: JsOutput<&str> = js_output.context("unable to find JS output")?;
	let Some(main_memory) = memories.iter().find(|memory| {
		memory.module == js_output.main_memory.module && memory.name == js_output.main_memory.name
	}) else {
		bail!("unable to find main memory as encoded")
	};

	let run_data = if let Some(tests) = test_parser.into_tests() {
		TestCli::run(iter::once(binary).chain(args), tests)
	} else {
		let args = if web {
			iter::once(file)
				.chain(args)
				.map(|arg| {
					arg.into_string().map_err(|arg| {
						anyhow::anyhow!(
							"non-UTF-8 argument passed to JS binary: {}",
							arg.to_string_lossy()
						)
					})
				})
				.collect::<Result<Vec<_>>>()?
		} else {
			Vec::new()
		};

		Some(RunData::Binary {
			ctx: Ctx::new(),
			wasm64: main_memory.data.memory64,
			memory: js_output.main_memory,
			args,
		})
	};

	if let Some(run_data) = run_data {
		let mut js_file = Vec::new();
		js_output.js(&mut js_file, main_memory.data)?;
		drop(js_output);

		let run_data = serde_json::to_string(&run_data).unwrap();

		Runner::new(wasm_path, wasm_bytes, js_file, run_data).run()?;
	}

	Ok(())
}

struct Memory<'a> {
	module: &'a str,
	name: &'a str,
	data: MemoryType,
}
