mod config;
mod runner;
mod server;
mod test;

use std::path::PathBuf;
use std::{env, iter, str};

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use js_bindgen_shared::ReadFile;

use crate::runner::Runner;
use crate::test::{TestData, TestEntry};

#[derive(Parser)]
#[command(name = "js-bindgen-runner", version, about, long_about = None)]
struct Cli {
	/// Run ignored and not ignored tests.
	#[arg(long, conflicts_with = "ignored")]
	include_ignored: bool,
	/// Run only ignored tests.
	#[arg(long, conflicts_with = "include_ignored")]
	ignored: bool,
	/// Exactly match filters rather than by substring.
	#[arg(long)]
	exact: bool,
	/// List all tests and benchmarks.
	#[arg(long)]
	list: bool,
	/// don't capture `console.*()` of each task, allow printing directly.
	#[arg(long, alias = "nocapture")]
	no_capture: bool,
	/// Configure formatting of output.
	#[arg(long, value_enum)]
	format: Option<FormatSetting>,
	/// The FILTER string is tested against the name of all tests, and only
	/// those tests whose names contain the filter are run. Multiple filter
	/// strings may be passed, which will run all tests matching any of the
	/// filters.
	filter: Vec<String>,
}

/// Possible values for the `--format` option.
#[derive(Clone, Copy, ValueEnum)]
enum FormatSetting {
	/// Display one character per test
	Terse,
}

fn main() -> Result<()> {
	let mut args = env::args_os();
	let binary = args
		.next()
		.context("expected the first argument to be present")?;
	// We parse the file argument ourselves to prevent it from being shown on the
	// help page.
	let file = args.next();

	let cli = Cli::parse_from(iter::once(binary).chain(args));

	// We delay actually parsing the file to support calling without a file, e.g.
	// `--help`.
	let file = file.context("expected a file to have been passed from `cargo run/test`")?;
	let wasm_path = PathBuf::from(&file);
	let wasm_bytes = ReadFile::new(&wasm_path)
		.with_context(|| format!("failed to read Wasm file: {}", wasm_path.display()))?;

	let (tests, filtered_count) =
		TestEntry::read(&wasm_bytes, cli.filter.as_ref(), cli.ignored, cli.exact)?;

	if cli.list {
		match cli.format {
			Some(FormatSetting::Terse) => {
				for test in &tests {
					println!("{}: test", test.name);
				}
			}
			None => {
				for test in &tests {
					println!("{}: test", test.name);
				}
				println!();
				println!("{} tests, 0 benchmarks", tests.len());
			}
		}
		return Ok(());
	}

	if tests.is_empty() {
		const GREEN: &str = "\u{001b}[32m";
		const RESET: &str = "\u{001b}[0m";

		println!();
		println!("running 0 tests");
		println!();
		println!(
			"test result: {GREEN}ok{RESET}. 0 passed; 0 failed; 0 ignored; 0 measured; \
			 {filtered_count} filtered out; finished in 0.00s"
		);
		println!();
		return Ok(());
	}

	// The JS file has the same name, just a different file extension.
	let imports_path = wasm_path.with_extension("mjs");
	let test_data = TestData {
		no_capture: cli.no_capture,
		filtered_count,
		tests,
	};

	Runner::new(wasm_path, wasm_bytes, imports_path, &test_data).run()
}
