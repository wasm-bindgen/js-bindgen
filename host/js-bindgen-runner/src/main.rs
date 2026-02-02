mod server;
mod util;
mod web_driver;

use std::env::VarError;
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::str::FromStr;
use std::time::Duration;
use std::{env, fs, iter, str};

use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use fantoccini::ClientBuilder;
use js_bindgen_shared::ReadFile;
use serde::{Serialize, Serializer};
use tokio::runtime::Runtime;
use tokio::{signal, time};
use wasmparser::{Parser as WasmParser, Payload};

use crate::server::{HttpServer, Status};
use crate::web_driver::WebDriver;

const NODE_JS_JS: &str = include_str!("js/node-js.mjs");
const SHARED_JS: &str = include_str!("js/shared.mjs");
const SHARED_TERMINAL_JS: &str = include_str!("js/shared-terminal.mjs");

const DENO_TS: &str = include_str!("js/deno.mts");
const SHARED_TS: &str = include_str!("js/shared.mts");
const SHARED_TERMINAL_TS: &str = include_str!("js/shared-terminal.mts");

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
	let args = TestArgs::new(cli)?;

	let (tests, filtered_count) = TestEntry::read(
		&wasm_bytes,
		args.filter.as_ref(),
		args.ignored_only,
		args.exact,
	)?;

	if args.list_only {
		match args.list_format {
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
		println!();
		println!("running 0 tests");
		println!();
		const GREEN: &str = "\u{001b}[32m";
		const RESET: &str = "\u{001b}[0m";
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
		no_capture: args.no_capture,
		filtered_count,
		tests,
	};
	let test_data_json = serde_json::to_string(&test_data).unwrap();

	let runner = Runner {
		wasm_path,
		imports_path,
		wasm_bytes,
		test_data_json,
	};

	match RunnerConfig::from_env()? {
		RunnerConfig::NodeJs => runner.run_node_js()?,
		RunnerConfig::Deno => runner.run_deno()?,
		RunnerConfig::Browser { worker } => Runtime::new()?.block_on(runner.run_browser(worker))?,
		RunnerConfig::Server { worker } => Runtime::new()?.block_on(runner.run_server(worker))?,
	}

	Ok(())
}

struct TestArgs {
	list_only: bool,
	no_capture: bool,
	filter: Vec<String>,
	list_format: Option<FormatSetting>,
	ignored_only: bool,
	exact: bool,
}

impl TestArgs {
	fn new(cli: Cli) -> Result<Self> {
		let output = Self {
			list_only: cli.list,
			no_capture: cli.no_capture,
			filter: cli.filter,
			list_format: cli.format,
			ignored_only: cli.ignored,
			exact: cli.exact,
		};

		Ok(output)
	}
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TestData {
	no_capture: bool,
	filtered_count: usize,
	tests: Vec<TestEntry>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TestEntry {
	name: String,
	import_name: String,
	#[serde(serialize_with = "option_option_string")]
	ignore: Option<Option<String>>,
	#[serde(serialize_with = "option_option_string")]
	should_panic: Option<Option<String>>,
}

impl TestEntry {
	fn read(
		wasm_bytes: &[u8],
		filter: &[String],
		ignored_only: bool,
		exact: bool,
	) -> Result<(Vec<Self>, usize)> {
		/// - None: `[0]`
		/// - Some(None): `[1]`
		/// - Some(Some(s)): `[2][len(s)][s]`
		fn option_option_string(data: &mut &[u8]) -> Result<Option<Option<String>>> {
			let value = match data
				.split_off_first()
				.context("invalid test flag encoding")?
			{
				0 => None,
				1 => Some(None),
				2 => {
					let len = u16::from_le_bytes(
						data.split_off(..2)
							.context("invalid test flag length encoding")?
							.try_into()?,
					)
					.into();
					let s = str::from_utf8(
						data.split_off(..len)
							.context("invalid test flag reason encoding")?,
					)?
					.to_string();
					Some(Some(s))
				}
				_ => bail!("mismatch flag value"),
			};
			Ok(value)
		}

		let mut tests = Vec::new();
		let mut total = 0;

		for payload in WasmParser::new(0).parse_all(wasm_bytes) {
			if let Payload::CustomSection(section) = payload?
				&& section.name() == "js_bindgen.test"
			{
				let mut data = section.data();

				while !data.is_empty() {
					let len = u32::from_le_bytes(
						data.split_off(..4)
							.context("invalid test encoding")?
							.try_into()?,
					) as usize;
					let mut data = data.split_off(..len).context("invalid test encoding")?;

					let ignore = option_option_string(&mut data)?;
					let should_panic = option_option_string(&mut data)?;
					let import_name = str::from_utf8(data)?;
					let name = import_name
						.split_once("::")
						.unwrap_or_else(|| panic!("unexpected test name: {import_name}"))
						.1;

					total += 1;

					let matches_ignore = !ignored_only || ignore.is_some();
					let matches_filter = filter.is_empty()
						|| filter.iter().any(|filter| {
							if exact {
								filter == name
							} else {
								name.contains(filter)
							}
						});

					if matches_ignore && matches_filter {
						tests.push(Self {
							name: name.to_string(),
							import_name: import_name.to_string(),
							ignore,
							should_panic,
						});
					}
				}

				// Section with the same name can never appear again.
				break;
			}
		}

		tests.sort_unstable_by(|a, b| a.name.cmp(&b.name));
		let filtered_count = total - tests.len();

		Ok((tests, filtered_count))
	}
}

#[derive(Clone, Copy, Debug)]
enum RunnerConfig {
	NodeJs,
	Deno,
	Browser { worker: Option<WorkerKind> },
	Server { worker: Option<WorkerKind> },
}

#[derive(Clone, Copy, Debug)]
enum WorkerKind {
	/// https://developer.mozilla.org/en-US/docs/Web/API/Worker
	Dedicated,
	/// https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker
	Shared,
	/// https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker
	Service,
}

impl WorkerKind {
	fn from_str(worker: &str) -> Result<Self> {
		Ok(match worker {
			"dedicated" => Self::Dedicated,
			"shared" => Self::Shared,
			"service" => Self::Service,
			worker => bail!("unrecognized worker: {worker}"),
		})
	}
}

impl RunnerConfig {
	fn from_env() -> Result<Self> {
		let worker = match env::var("JBG_TEST_WORKER") {
			Ok(worker) => Some(WorkerKind::from_str(&worker)?),
			Err(VarError::NotPresent) => None,
			Err(VarError::NotUnicode(_)) => bail!("unable to parse `JBG_TEST_WORKER`"),
		};

		let config = match env::var("JBG_TEST_RUNNER") {
			Ok(s) => match s.as_str() {
				"browser" => Self::Browser { worker },
				"server" => Self::Server { worker },
				"deno" => Self::Deno,
				"node-js" => Self::NodeJs,
				runner => bail!("unrecognized runner: {runner}"),
			},
			Err(VarError::NotPresent) => {
				const ENGINES: [(&str, RunnerConfig); 2] =
					[("deno", RunnerConfig::Deno), ("node", RunnerConfig::NodeJs)];

				env::var_os("PATH")
					.and_then(|value| {
						env::split_paths(&value).find_map(|path| {
							ENGINES.into_iter().find_map(|(name, value)| {
								path.join(name)
									.with_extension(env::consts::EXE_EXTENSION)
									.exists()
									.then_some(value)
							})
						})
					})
					.context("unable to find a supported JS engine")?
			}
			Err(VarError::NotUnicode(_)) => bail!("unable to parse `JBG_TEST_RUNNER`"),
		};

		if worker.is_some()
			&& let Self::NodeJs { .. } | Self::Deno { .. } = config
		{
			eprintln!(
				"Only browser and server runners support worker types; `JBG_TEST_WORKER` is \
				 ignored",
			)
		}

		Ok(config)
	}
}

struct Runner {
	wasm_path: PathBuf,
	imports_path: PathBuf,
	wasm_bytes: ReadFile,
	test_data_json: String,
}

impl Runner {
	fn run_node_js(self) -> Result<()> {
		let dir = tempfile::tempdir()?;

		let runner_path = dir.path().join("runner.mjs");
		fs::write(&runner_path, NODE_JS_JS)?;

		fs::write(dir.path().join("test-data.json"), self.test_data_json)?;
		fs::copy(self.wasm_path, dir.path().join("wasm.wasm"))?;
		fs::copy(self.imports_path, dir.path().join("imports.mjs"))?;
		fs::write(dir.path().join("shared.mjs"), SHARED_JS)?;
		fs::write(dir.path().join("shared-terminal.mjs"), SHARED_TERMINAL_JS)?;

		let status = Command::new("node").arg(runner_path).status()?;

		if !status.success() {
			process::exit(status.code().unwrap_or(1));
		}

		Ok(())
	}

	fn run_deno(self) -> Result<()> {
		let dir = tempfile::tempdir()?;

		let runner_path = dir.path().join("runner.mts");
		fs::write(&runner_path, DENO_TS)?;

		fs::write(dir.path().join("test-data.json"), self.test_data_json)?;
		fs::copy(self.wasm_path, dir.path().join("wasm.wasm"))?;
		fs::copy(self.imports_path, dir.path().join("imports.mjs"))?;
		fs::write(dir.path().join("shared.mts"), SHARED_TS)?;
		fs::write(dir.path().join("shared-terminal.mts"), SHARED_TERMINAL_TS)?;

		let status = Command::new("deno")
			.arg("run")
			.arg("--allow-read")
			.arg(runner_path)
			.status()?;

		if !status.success() {
			process::exit(status.code().unwrap_or(1));
		}

		Ok(())
	}

	async fn run_browser(self, worker: Option<WorkerKind>) -> Result<()> {
		let server = self.http_server(true, worker).await?;
		let driver = WebDriver::run().await?;
		let client = ClientBuilder::rustls()?
			.capabilities(driver.capabilities)
			.connect(driver.url.as_str())
			.await?;

		client.goto(server.url()).await?;

		let status = server.wait().await;
		client.close().await?;

		match status {
			Status::Ok => Ok(()),
			// See https://github.com/rust-lang/cargo/blob/fa50b03244beda717b3fd2c7a647ba93c0d39e05/src/cargo/ops/cargo_test.rs#L418.
			Status::Failed => process::exit(101),
			Status::Abnormal => process::exit(1),
		}
	}

	async fn run_server(self, worker: Option<WorkerKind>) -> Result<()> {
		let server = self.http_server(false, worker).await?;

		println!("open this URL in your browser to run tests:");
		println!("{}", server.url());
		println!("shutdown via CTRL-C");

		signal::ctrl_c().await?;

		if time::timeout(Duration::from_millis(100), server.shutdown())
			.await
			.is_err()
		{
			eprintln!("failed to shutdown server cleanly");
		}

		Ok(())
	}

	async fn http_server(self, headless: bool, worker: Option<WorkerKind>) -> Result<HttpServer> {
		let assets = BrowserAssets::new(self.wasm_bytes, &self.imports_path, self.test_data_json)?;

		let address = env::var_os("JBG_TEST_SERVER_ADDRESS")
			.map(|var| {
				var.into_string()
					.ok()
					.and_then(|string| {
						SocketAddr::from_str(&string)
							.or_else(|_| {
								IpAddr::from_str(&string).map(|addr| SocketAddr::new(addr, 0))
							})
							.ok()
					})
					.context("unable to parse `JBG_TEST_SERVER_ADDRESS`")
			})
			.transpose()?;
		HttpServer::start(assets, address, headless, worker).await
	}
}

struct BrowserAssets {
	wasm_bytes: ReadFile,
	import_js: ReadFile,
	test_data_json: String,
}

impl BrowserAssets {
	fn new(wasm_bytes: ReadFile, imports_path: &Path, test_data_json: String) -> Result<Self> {
		let import_js = ReadFile::new(imports_path)?;

		Ok(Self {
			wasm_bytes,
			import_js,
			test_data_json,
		})
	}
}

fn option_option_string<S>(value: &Option<Option<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	match value {
		None => serializer.serialize_unit(),
		Some(None) => serializer.serialize_bool(true),
		Some(Some(reason)) => serializer.serialize_str(reason),
	}
}
