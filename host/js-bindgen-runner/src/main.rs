mod driver;

use std::ffi::OsString;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::sync::Arc;
use std::{env, fs, iter, str};

use anyhow::{Context, Result, anyhow, bail};
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Json, Router};
use clap::{Parser, ValueEnum};
use driver::Capabilities;
use fantoccini::ClientBuilder;
use js_bindgen_shared::ReadFile;
use parking_lot::{Condvar, Mutex};
use serde::{Serialize, Serializer};
use tempfile::TempDir;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use wasmparser::{Parser as WasmParser, Payload};

use crate::driver::Driver;

const NODE_RUNNER: &str = include_str!("../js/node-runner.mjs");
const BROWSER_RUNNER_SOURCE: &str = include_str!("../js/browser-runner.mjs");
const RUNNER_CORE_SOURCE: &str = include_str!("../js/runner-core.mjs");
const SHARED_JS_SOURCE: &str = include_str!("../js/shared.mjs");
const WORKER_RUNNER_SOURCE: &str = include_str!("../js/worker-runner.mjs");
const SERVICE_WORKER_SOURCE: &str = include_str!("../js/service-worker.mjs");
const CONSOLE_HOOK_SOURCE: &str = include_str!("../js/console-hook.mjs");

/// Possible values for the `--format` option.
#[derive(Clone, Copy, ValueEnum)]
enum FormatSetting {
	/// Display one character per test
	Terse,
}

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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct TestEntry {
	name: String,
	#[serde(serialize_with = "option_option_string")]
	ignore: Option<Option<String>>,
	#[serde(serialize_with = "option_option_string")]
	should_panic: Option<Option<String>>,
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

	let mut tests = read_tests(&wasm_bytes)?;
	let filtered_count = apply_filters(
		&mut tests,
		args.filter.as_ref(),
		args.ignored_only,
		args.exact,
	);

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
	let tests_json = serde_json::to_string(&tests).unwrap();

	let runner = Runner {
		wasm_path,
		imports_path,
		wasm_bytes,
		tests_json,
		filtered_count,
		no_capture: args.no_capture,
	};

	match RunnerConfig::from_env()? {
		RunnerConfig::Node => runner.run_node()?,
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

#[derive(Clone, Copy, Debug)]
enum RunnerConfig {
	Node,
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
			worker => bail!("unsupported worker: {worker}"),
		})
	}

	fn as_str(self) -> &'static str {
		match self {
			Self::Dedicated => "dedicated",
			Self::Shared => "shared",
			Self::Service => "service",
		}
	}
}

impl RunnerConfig {
	fn from_env() -> Result<Self> {
		let worker = match env::var("JBG_TEST_WORKER") {
			Ok(worker) => Some(WorkerKind::from_str(&worker)?),
			Err(_) => None,
		};

		let config = if let Ok(s) = env::var("JBG_TEST_RUNNER") {
			match s.as_str() {
				"browser" => Self::Browser { worker },
				"server" => Self::Server { worker },
				"node" => Self::Node,
				"deno" => Self::Deno,
				runner => bail!("unsupported runner: {runner}"),
			}
		} else {
			Self::Node
		};

		if worker.is_some()
			&& let Self::Browser { .. } | Self::Server { .. } = config
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
	tests_json: String,
	filtered_count: usize,
	no_capture: bool,
}

impl Runner {
	fn run_node(self) -> Result<()> {
		let node_dir = NodeDir::new(&self.tests_json)?;

		let status = Command::new("node")
			.arg(node_dir.runner)
			.env("JSB_TEST_WASM", self.wasm_path)
			.env("JSB_TEST_IMPORTS", self.imports_path)
			.env("JSB_TEST_TESTS_PATH", node_dir.tests)
			.env("JSB_TEST_FILTERED", self.filtered_count.to_string())
			.env(
				"JSB_TEST_NO_CAPTURE",
				if self.no_capture { "1" } else { "0" },
			)
			.status()?;

		if !status.success() {
			process::exit(status.code().unwrap_or(1));
		}

		Ok(())
	}

	fn run_deno(self) -> Result<()> {
		let node_dir = NodeDir::new(&self.tests_json)?;

		let status = Command::new("deno")
			.arg("run")
			.arg("--allow-env")
			.arg("--allow-read")
			.arg(node_dir.runner)
			.env("JSB_TEST_WASM", self.wasm_path)
			.env("JSB_TEST_IMPORTS", self.imports_path)
			.env("JSB_TEST_TESTS_PATH", node_dir.tests)
			.env("JSB_TEST_FILTERED", self.filtered_count.to_string())
			.env(
				"JSB_TEST_NO_CAPTURE",
				if self.no_capture { "1" } else { "0" },
			)
			.status()?;

		if !status.success() {
			process::exit(status.code().unwrap_or(1));
		}

		Ok(())
	}

	async fn run_browser(self, worker: Option<WorkerKind>) -> Result<()> {
		let assets = BrowserAssets::new(
			self.wasm_bytes,
			&self.imports_path,
			self.tests_json,
			self.filtered_count,
			self.no_capture,
			worker.map(WorkerKind::as_str),
		)?;
		let server = HttpServer::start(assets, Self::server_address()?.as_deref()).await?;
		let url = Self::browser_url(server.base_url.as_str());

		let driver = Driver::find()?;
		let guard = driver::launch_driver(&driver)?;

		let webdriver_json_path = env::var_os("JBG_TEST_WEBDRIVER_JSON").map(PathBuf::from);
		let webdriver_json_path = webdriver_json_path
			.as_deref()
			.unwrap_or(Path::new("webdriver.json"));
		let capabilities = match ReadFile::new(webdriver_json_path) {
			Ok(file) => serde_json::from_slice(&file)?,
			Err(error) if matches!(error.kind(), ErrorKind::NotFound) => Capabilities::new(),
			Err(error) => return Err(error.into()),
		};

		let client = ClientBuilder::rustls()?
			.capabilities(driver::capabilities(&driver, capabilities)?)
			.connect(guard.url.as_str())
			.await
			.context("failed to connect to WebDriver")?;

		client.goto(&url).await?;

		let report = server.wait_for_report();

		client.close().await?;

		for line in report.lines {
			println!("{line}");
		}

		if report.failed > 0 {
			process::exit(1);
		}

		Ok(())
	}

	async fn run_server(self, worker: Option<WorkerKind>) -> Result<()> {
		let assets = BrowserAssets::new(
			self.wasm_bytes,
			&self.imports_path,
			self.tests_json,
			self.filtered_count,
			self.no_capture,
			worker.map(WorkerKind::as_str),
		)?;
		let server = HttpServer::start(assets, Self::server_address()?.as_deref()).await?;
		let url = Self::browser_url(server.base_url.as_str());

		println!("open this URL in your browser to run tests:");
		println!("{url}");

		loop {
			let _ = server.wait_for_report();
		}
	}

	fn server_address() -> Result<Option<String>> {
		env::var_os("JBG_TEST_SERVER_ADDRESS")
			.map(OsString::into_string)
			.transpose()
			.map_err(|_| anyhow!("unable to parse `JBG_TEST_SERVER_ADDRESS`"))
	}

	fn browser_url(base_url: &str) -> String {
		format!("{base_url}/index.html")
	}
}

struct NodeDir {
	_guard: TempDir,
	runner: PathBuf,
	tests: PathBuf,
}

impl NodeDir {
	fn new(tests_json: &str) -> Result<Self> {
		let dir = tempfile::tempdir().unwrap();
		let runner_path = dir.path().join("runner.mjs");
		fs::write(&runner_path, NODE_RUNNER)?;
		let tests_path = dir.path().join("tests.json");
		fs::write(&tests_path, tests_json)?;

		fs::write(dir.path().join("shared.mjs"), SHARED_JS_SOURCE)?;
		fs::write(dir.path().join("runner-core.mjs"), RUNNER_CORE_SOURCE)?;
		fs::write(dir.path().join("console-hook.mjs"), CONSOLE_HOOK_SOURCE)?;

		Ok(Self {
			_guard: dir,
			runner: runner_path,
			tests: tests_path,
		})
	}
}

#[derive(Debug, serde::Deserialize)]
struct Report {
	lines: Vec<String>,
	failed: u64,
}

struct ReportState {
	result: Mutex<Option<Report>>,
	signal: Condvar,
}

struct BrowserAssets {
	wasm_bytes: ReadFile,
	import_js: ReadFile,
	tests_json: String,
	index_html: String,
}

impl BrowserAssets {
	fn new(
		wasm_bytes: ReadFile,
		imports_path: &Path,
		tests_json: String,
		filtered_count: usize,
		no_capture: bool,
		worker: Option<&str>,
	) -> Result<Self> {
		let import_js = ReadFile::new(imports_path)?;

		let index_html = format!(
			include_str!("../js/index.html"),
			filtered_count = filtered_count,
			no_capture_flag = if no_capture { "true" } else { "false" },
			worker = if let Some(worker) = worker {
				format!("\"{worker}\"")
			} else {
				"null".to_owned()
			},
		);

		Ok(Self {
			wasm_bytes,
			import_js,
			tests_json,
			index_html,
		})
	}
}

struct HttpServer {
	base_url: String,
	report_state: Arc<ReportState>,
}

#[derive(Clone)]
struct AppState {
	assets: Arc<BrowserAssets>,
	report_state: Arc<ReportState>,
}

impl HttpServer {
	async fn start(assets: BrowserAssets, address: Option<&str>) -> Result<Self> {
		let listener = bind_default_port(address)
			.await
			.context("failed to bind server")?;
		let local_addr = listener.local_addr()?;
		let base_url = format!("http://{}:{}", local_addr.ip(), local_addr.port());

		let assets = Arc::new(assets);
		let report_state = Arc::new(ReportState {
			result: Mutex::new(None),
			signal: Condvar::new(),
		});
		let state = AppState {
			assets,
			report_state: Arc::clone(&report_state),
		};
		let app = build_router(state);
		let serve = axum::serve(listener, app);

		tokio::spawn(async move {
			if let Err(e) = serve.await {
				panic!("failed to run server: {e:?}");
			}
		});

		Ok(Self {
			base_url,
			report_state,
		})
	}

	fn wait_for_report(&self) -> Report {
		let guard = &mut self.report_state.result.lock();
		loop {
			if let Some(report) = guard.take() {
				return report;
			}
			self.report_state.signal.wait(guard);
		}
	}
}

fn build_router(state: AppState) -> Router {
	Router::new()
		.route("/", get(index_handler))
		.route("/index.html", get(index_handler))
		.route("/browser-runner.mjs", get(browser_runner_handler))
		.route("/runner-core.mjs", get(runner_core_handler))
		.route("/shared.mjs", get(shared_handler))
		.route("/worker-runner.mjs", get(worker_runner_handler))
		.route("/service-worker.mjs", get(service_worker_handler))
		.route("/console-hook.mjs", get(console_hook_handler))
		.route("/import.mjs", get(import_handler))
		.route("/tests.json", get(tests_handler))
		.route("/wasm", get(wasm_handler))
		.route("/report", post(report_handler))
		.with_state(state)
}

async fn bind_default_port(address: Option<&str>) -> Result<TcpListener> {
	let default_addr = address.unwrap_or("127.0.0.1:8000");
	match TcpListener::bind(default_addr).await {
		Ok(listener) => Ok(listener),
		Err(err) if err.kind() == ErrorKind::AddrInUse => {
			let fallback_addr = address
				.and_then(|addr| addr.split_once(':'))
				.map(|(ip, _)| format!("{ip}:0"))
				.unwrap_or_else(|| "127.0.0.1:0".to_string());
			TcpListener::bind(&fallback_addr)
				.await
				.context("failed to bind fallback port")
		}
		Err(err) => Err(err).context("failed to bind default port"),
	}
}

async fn index_handler(State(state): State<AppState>) -> Response {
	bytes_response(
		StatusCode::OK,
		"text/html",
		state.assets.index_html.as_bytes(),
	)
}

async fn browser_runner_handler() -> Response {
	bytes_response(
		StatusCode::OK,
		"application/javascript",
		BROWSER_RUNNER_SOURCE.as_bytes(),
	)
}

async fn runner_core_handler() -> Response {
	bytes_response(
		StatusCode::OK,
		"application/javascript",
		RUNNER_CORE_SOURCE.as_bytes(),
	)
}

async fn shared_handler() -> Response {
	bytes_response(
		StatusCode::OK,
		"application/javascript",
		SHARED_JS_SOURCE.as_bytes(),
	)
}

async fn worker_runner_handler() -> Response {
	bytes_response(
		StatusCode::OK,
		"application/javascript",
		WORKER_RUNNER_SOURCE.as_bytes(),
	)
}

async fn service_worker_handler() -> Response {
	bytes_response(
		StatusCode::OK,
		"application/javascript",
		SERVICE_WORKER_SOURCE.as_bytes(),
	)
}

async fn console_hook_handler() -> Response {
	bytes_response(
		StatusCode::OK,
		"application/javascript",
		CONSOLE_HOOK_SOURCE.as_bytes(),
	)
}

async fn import_handler(State(state): State<AppState>) -> Response {
	bytes_response(
		StatusCode::OK,
		"application/javascript",
		&state.assets.import_js,
	)
}

async fn tests_handler(State(state): State<AppState>) -> Response {
	bytes_response(
		StatusCode::OK,
		"application/json",
		state.assets.tests_json.as_bytes(),
	)
}

async fn wasm_handler(State(state): State<AppState>) -> Response {
	bytes_response(StatusCode::OK, "application/wasm", &state.assets.wasm_bytes)
}

async fn report_handler(State(state): State<AppState>, Json(report): Json<Report>) -> Response {
	*state.report_state.result.lock() = Some(report);
	state.report_state.signal.notify_all();
	bytes_response(StatusCode::OK, "text/plain", "OK".as_bytes())
}

fn bytes_response(status: StatusCode, content_type: &'static str, body: &[u8]) -> Response {
	let mut response = Response::new(Body::from(body.to_vec()));
	*response.status_mut() = status;
	response
		.headers_mut()
		.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
	with_headers(response)
}

fn with_headers(mut response: Response) -> Response {
	let headers = response.headers_mut();
	headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
	headers.insert(
		"Access-Control-Allow-Methods",
		HeaderValue::from_static("GET, POST, OPTIONS"),
	);
	headers.insert(
		"Access-Control-Allow-Headers",
		HeaderValue::from_static("Content-Type"),
	);
	headers.insert(
		"Cross-Origin-Opener-Policy",
		HeaderValue::from_static("same-origin"),
	);
	headers.insert(
		"Cross-Origin-Embedder-Policy",
		HeaderValue::from_static("require-corp"),
	);
	response
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

fn read_tests(wasm_bytes: &[u8]) -> Result<Vec<TestEntry>> {
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
				let len = u32::from_le_bytes(
					data.split_off(..4)
						.context("invalid test flag length encoding")?
						.try_into()?,
				) as usize;
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

	for payload in WasmParser::new(0).parse_all(wasm_bytes) {
		if let Payload::CustomSection(section) = payload?
			&& section.name() == "js_bindgen.test"
		{
			let mut data = section.data();

			while !data.is_empty() {
				let len = u32::from_le_bytes(
					data.split_off(..4)
						.context("invalid test name length encoding")?
						.try_into()?,
				) as usize;

				let ignore = option_option_string(&mut data)?;
				let should_panic = option_option_string(&mut data)?;

				let name = str::from_utf8(
					data.split_off(..len)
						.context("invalid test name encoding")?,
				)?;

				tests.push(TestEntry {
					name: name.to_string(),
					ignore,
					should_panic,
				});
			}

			// Section with the same name can never appear again.
			break;
		}
	}

	Ok(tests)
}

fn apply_filters(
	tests: &mut Vec<TestEntry>,
	filter: &[String],
	ignored_only: bool,
	exact: bool,
) -> usize {
	let initial = tests.len();
	tests.retain(|test| {
		let matches_ignore = !ignored_only || test.ignore.is_some();
		let matches_filter = filter.is_empty()
			|| filter.iter().any(|filter| {
				if exact {
					filter == &test.name
				} else {
					test.name.contains(filter)
				}
			});
		matches_ignore && matches_filter
	});
	initial - tests.len()
}
