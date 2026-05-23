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
use wasmparser::Parser;

use crate::binary::BinaryParser;
use crate::run_data::RunData;
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

	let mut binary_parser = Some(BinaryParser::default());
	let mut binary_data = None;
	let mut test_parser = Some(TestParser::default());
	let mut test_data = None;

	for payload in Parser::new(0).parse_all(&wasm_bytes) {
		let payload = payload?;

		if let Some(parser) = test_parser {
			match parser.parse(&payload)? {
				ControlFlow::Continue(parser) => test_parser = Some(parser),
				ControlFlow::Break(data) => {
					test_data = Some(data);
					break;
				}
			}
		}

		if let Some(parser) = binary_parser.take() {
			match parser.parse(&payload)? {
				ControlFlow::Continue(parser) => binary_parser = Some(parser),
				ControlFlow::Break(data) => binary_data = data,
			}
		}

		if test_parser.is_none() && binary_parser.is_none() {
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
		// The JS file has the same name, just a different file extension.
		let imports_path = wasm_path.with_extension("mjs");

		Runner::new(wasm_path, wasm_bytes, imports_path, &run_data).run()?;
	}

	Ok(())
}
