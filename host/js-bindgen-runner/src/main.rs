use std::env;
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{
	Arc, Condvar, Mutex,
	atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use js_bindgen_ld_shared::ReadFile;
use wasmparser::{Parser, Payload};

const NODE_RUNNER: &str = "js/node-runner.mjs";
const PLAYWRIGHT_RUNNER: &str = "js/playwright-runner.mjs";
const BROWSER_RUNNER_SOURCE: &str = include_str!("../js/browser-runner.mjs");
const RUNNER_CORE_SOURCE: &str = include_str!("../js/runner-core.mjs");
const SHARED_JS_SOURCE: &str = include_str!("../js/shared.mjs");
const WORKER_RUNNER_SOURCE: &str = include_str!("../js/worker-runner.mjs");
const SERVICE_WORKER_SOURCE: &str = include_str!("../js/service-worker.mjs");
const CONSOLE_HOOK_SOURCE: &str = include_str!("../js/console-hook.mjs");

const GREEN: &str = "\u{001b}[32m";
const RESET: &str = "\u{001b}[0m";

#[derive(Debug, serde::Serialize)]
struct TestEntry {
	name: String,
	ignore: bool,
	ignore_reason: Option<String>,
	should_panic: bool,
	should_panic_reason: Option<String>,
}

fn main() -> Result<()> {
	let mut args = env::args().skip(1);

	let wasm_path = args
		.next()
		.map(PathBuf::from)
		.context("expected a wasm file path")?;

	let args = TestArgs::new(args)?;
	let wasm_bytes = ReadFile::new(&wasm_path)
		.with_context(|| format!("failed to read wasm file: {}", wasm_path.display()))?;

	let mut tests = read_tests(&wasm_bytes)?;
	let filtered_count = apply_filters(&mut tests, &args.filters, args.ignored_only, args.exact);

	if args.list_only {
		match args.list_format {
			ListFormat::Standard => {
				for test in &tests {
					println!("{}: test", test.name);
				}
				println!();
				println!("{} tests, 0 benchmarks", tests.len());
			}
			ListFormat::Terse => {
				for test in &tests {
					println!("{}: test", test.name);
				}
			}
		}
		return Ok(());
	}

	if tests.is_empty() {
		println!();
		println!("running 0 tests");
		println!();
		println!(
			"test result: {GREEN}ok{RESET}. 0 passed; 0 failed; 0 ignored; 0 measured; {filtered_count} filtered out; finished in 0.00s"
		);
		println!();
		return Ok(());
	}

	// `web_sys-4e01138c76cd2a1a.wasm` to `web_sys-4e01138c76cd2a1a.js`
	let imports_path = wasm_path.with_extension("js");
	let tests_json = serde_json::to_string(&tests).expect("checked");

	match args.runner {
		RunnerConfig::Nodejs => run_node(
			&wasm_path,
			&imports_path,
			tests_json,
			filtered_count,
			args.nocapture,
		)?,
		RunnerConfig::Browser { kind, worker } => run_playwright(
			wasm_bytes.to_vec(),
			&imports_path,
			tests_json,
			filtered_count,
			args.nocapture,
			&kind,
			worker.as_ref(),
		)?,
		RunnerConfig::Server { worker } => run_browser_server(
			wasm_bytes.to_vec(),
			&imports_path,
			tests_json,
			filtered_count,
			args.nocapture,
			worker.as_ref(),
		)?,
	}

	Ok(())
}

enum ListFormat {
	Standard,
	Terse,
}

struct TestArgs {
	list_only: bool,
	nocapture: bool,
	filters: Vec<String>,
	list_format: ListFormat,
	ignored_only: bool,
	exact: bool,
	runner: RunnerConfig,
}

impl TestArgs {
	fn new(mut iter: impl Iterator<Item = String>) -> Result<TestArgs> {
		let mut output = TestArgs {
			list_only: false,
			nocapture: false,
			filters: Vec::new(),
			list_format: ListFormat::Standard,
			ignored_only: false,
			exact: false,
			runner: RunnerConfig::from_env()?,
		};

		while let Some(arg) = iter.next() {
			if arg == "--list" {
				output.list_only = true;
			} else if arg == "--nocapture" {
				output.nocapture = true;
			} else if arg == "--ignored" {
				output.ignored_only = true;
			} else if arg == "--exact" {
				output.exact = true;
			} else if let Some(value) = arg.strip_prefix("--format=") {
				if value == "terse" {
					output.list_format = ListFormat::Terse;
				}
			} else if arg == "--format" {
				if let Some(value) = iter.next() {
					if value == "terse" {
						output.list_format = ListFormat::Terse;
					}
				}
			} else if arg.starts_with('-') {
				continue;
			} else {
				output.filters.push(arg);
			}
		}

		Ok(output)
	}
}

#[derive(Debug)]
enum RunnerConfig {
	Nodejs,
	Browser {
		kind: DriverKind,
		worker: Option<WorkerKind>,
	},
	Server {
		worker: Option<WorkerKind>,
	},
}

#[derive(Debug)]
enum WorkerKind {
	// https://developer.mozilla.org/en-US/docs/Web/API/Worker
	Dedicated,
	// https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker
	Shared,
	// https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker
	Service,
}

impl WorkerKind {
	fn from_str(worker: &str) -> Result<Self> {
		Ok(match worker {
			"dedicated" => WorkerKind::Dedicated,
			"shared" => WorkerKind::Shared,
			"service" => WorkerKind::Service,
			worker => bail!("unsupported worker: {worker}"),
		})
	}

	fn as_str(&self) -> &str {
		match self {
			WorkerKind::Dedicated => "dedicated",
			WorkerKind::Shared => "shared",
			WorkerKind::Service => "service",
		}
	}
}

#[derive(Debug)]
enum DriverKind {
	// https://developer.chrome.com/docs/chromedriver
	Chrome,
	// https://github.com/mozilla/geckodriver
	Gecko,
	// https://github.com/WebKit/WebKit
	WebKit,
}

impl DriverKind {
	fn from_str(driver: &str) -> Result<Self> {
		Ok(match driver {
			"chrome" => DriverKind::Chrome,
			"gecko" => DriverKind::Gecko,
			"webkit" => DriverKind::WebKit,
			driver => bail!("unsupported driver: {driver}"),
		})
	}

	fn as_str(&self) -> &str {
		match self {
			DriverKind::Chrome => "chrome",
			DriverKind::Gecko => "gecko",
			DriverKind::WebKit => "webkit",
		}
	}
}

impl RunnerConfig {
	fn from_env() -> Result<Self> {
		let driver = match env::var("JBG_TEST_DRIVER") {
			Ok(driver) => Some(DriverKind::from_str(&driver)?),
			Err(_) => None,
		};

		let worker = match env::var("JBG_TEST_WORKER") {
			Ok(worker) => Some(WorkerKind::from_str(&worker)?),
			Err(_) => None,
		};

		let server = env::var("JBG_TEST_SERVER").is_ok();

		let config = match (server, driver) {
			(true, None) => RunnerConfig::Server { worker },
			(true, Some(d)) => {
				eprintln!("Because a server has been configured, the {d:?} option is not work.");
				RunnerConfig::Server { worker }
			}
			(false, None) => {
				if let Some(worker) = worker {
					eprintln!("The {worker:?} option does not work in Node.js.");
				}
				RunnerConfig::Nodejs
			}
			(false, Some(kind)) => RunnerConfig::Browser { kind, worker },
		};

		Ok(config)
	}
}

fn run_node(
	wasm_path: &Path,
	imports_path: &Path,
	tests_json: String,
	filtered_count: usize,
	nocapture: bool,
) -> Result<()> {
	ensure_module_package(imports_path);
	let runner_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(NODE_RUNNER);
	let mut tests_file = tempfile::NamedTempFile::new()?;
	tests_file.write_all(tests_json.as_bytes())?;
	let tests_path = tests_file.path().to_path_buf();

	let status = Command::new("node")
		.arg(runner_path)
		.env("JS_BINDGEN_WASM", wasm_path)
		.env("JS_BINDGEN_IMPORTS", imports_path)
		.env("JS_BINDGEN_TESTS_PATH", &tests_path)
		.env("JS_BINDGEN_FILTERED", filtered_count.to_string())
		.env("JS_BINDGEN_NOCAPTURE", if nocapture { "1" } else { "0" })
		.status()
		.context("failed to run node")?;

	if !status.success() {
		std::process::exit(status.code().unwrap_or(1));
	}

	Ok(())
}

fn ensure_module_package(imports_path: &Path) {
	let Some(parent) = imports_path.parent() else {
		return;
	};
	let package_json = parent.join("package.json");
	if let Err(err) = fs::write(&package_json, r#"{"type": "module"}"#) {
		eprintln!("failed to write package.json: {err}");
	}
}

fn run_playwright(
	wasm_bytes: Vec<u8>,
	imports_path: &Path,
	tests_json: String,
	filtered_count: usize,
	nocapture: bool,
	driver: &DriverKind,
	worker: Option<&WorkerKind>,
) -> Result<()> {
	let assets = BrowserAssets::new(
		wasm_bytes,
		imports_path,
		&tests_json,
		filtered_count,
		nocapture,
		worker.map(|s| s.as_str()),
	)?;
	let server = HttpServer::start(assets, env::var("JBG_TEST_SERVER_ADDRESS").ok().as_deref())?;
	let url = build_browser_url(server.base_url.as_str());

	let runner_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(PLAYWRIGHT_RUNNER);
	let mut child = Command::new("node")
		.arg(runner_path)
		.env("JBG_TEST_URL", url)
		.env("JBG_TEST_DRIVER", driver.as_str())
		.spawn()
		.context("failed to run node")?;

	let report = server.wait_for_report();
	server.shutdown.store(true, Ordering::Relaxed);

	for line in report.lines {
		println!("{line}");
	}

	let _ = child.kill();
	let _ = child.wait();

	if report.failed > 0 {
		std::process::exit(1);
	}

	Ok(())
}

fn run_browser_server(
	wasm_bytes: Vec<u8>,
	imports_path: &Path,
	tests_json: String,
	filtered_count: usize,
	nocapture: bool,
	worker: Option<&WorkerKind>,
) -> Result<()> {
	let assets = BrowserAssets::new(
		wasm_bytes,
		imports_path,
		&tests_json,
		filtered_count,
		nocapture,
		worker.map(|s| s.as_str()),
	)?;
	let server = HttpServer::start(assets, env::var("JBG_TEST_SERVER_ADDRESS").ok().as_deref())?;
	let url = build_browser_url(server.base_url.as_str());

	println!("open this URL in your browser to run tests:");
	println!("{url}");

	loop {
		let _ = server.wait_for_report();
	}
}

fn build_browser_url(base_url: &str) -> String {
	format!("{base_url}/index.html")
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
	wasm_bytes: Vec<u8>,
	import_js: String,
	tests_json: String,
	index_html: String,
}

impl BrowserAssets {
	fn new(
		wasm_bytes: Vec<u8>,
		imports_path: &Path,
		tests_json: &str,
		filtered_count: usize,
		nocapture: bool,
		worker: Option<&str>,
	) -> Result<Self> {
		let import_js = fs::read_to_string(imports_path)?;

		let index_html = format!(
			include_str!("../js/index.html"),
			filtered_count = filtered_count,
			nocapture_flag = if nocapture { "true" } else { "false" },
			worker = if let Some(worker) = worker {
				format!("{worker:?}")
			} else {
				"null".to_owned()
			},
		);

		Ok(Self {
			wasm_bytes,
			import_js,
			tests_json: tests_json.to_string(),
			index_html,
		})
	}
}

struct HttpServer {
	base_url: String,
	shutdown: Arc<AtomicBool>,
	report_state: Arc<ReportState>,
}

impl HttpServer {
	fn start(assets: BrowserAssets, address: Option<&str>) -> Result<Self> {
		let listener = bind_default_port(address).context("failed to bind server")?;
		let local_addr = listener.local_addr()?;
		let base_url = format!("http://{}:{}", local_addr.ip(), local_addr.port());
		let shutdown = Arc::new(AtomicBool::new(false));
		let shutdown_flag = Arc::clone(&shutdown);
		let assets = Arc::new(assets);
		let report_state = Arc::new(ReportState {
			result: Mutex::new(None),
			signal: Condvar::new(),
		});
		let report_state_thread = Arc::clone(&report_state);

		listener
			.set_nonblocking(true)
			.context("failed to set nonblocking")?;

		thread::spawn(move || {
			while !shutdown_flag.load(Ordering::Relaxed) {
				match listener.accept() {
					Ok((stream, _)) => {
						let assets = Arc::clone(&assets);
						let report_state = Arc::clone(&report_state_thread);
						let _ = handle_connection(stream, assets, report_state);
					}
					Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
						thread::sleep(Duration::from_millis(5));
					}
					Err(_) => break,
				}
			}
		});

		Ok(Self {
			base_url,
			shutdown,
			report_state,
		})
	}

	fn wait_for_report(&self) -> Report {
		let mut guard = self.report_state.result.lock().unwrap();
		loop {
			if let Some(report) = guard.take() {
				return report;
			}
			guard = self.report_state.signal.wait(guard).unwrap();
		}
	}
}

fn bind_default_port(address: Option<&str>) -> Result<TcpListener> {
	let default_addr = address.unwrap_or("127.0.0.1:8000");
	match TcpListener::bind(default_addr) {
		Ok(listener) => Ok(listener),
		Err(err) if err.kind() == ErrorKind::AddrInUse => {
			let fallback_addr = address
				.and_then(|addr| addr.split_once(':'))
				.map(|(ip, _)| format!("{ip}:0"))
				.unwrap_or_else(|| "127.0.0.1:0".to_string());
			TcpListener::bind(&fallback_addr).context("failed to bind fallback port")
		}
		Err(err) => Err(err).context("failed to bind default port"),
	}
}

fn handle_connection(
	mut stream: TcpStream,
	assets: Arc<BrowserAssets>,
	report_state: Arc<ReportState>,
) -> Result<()> {
	let mut buffer = [0u8; 4096];
	let mut request = Vec::new();
	let mut header_end = 0;

	loop {
		let size = stream.read(&mut buffer)?;
		if size == 0 {
			break;
		}
		let len = request.len();
		request.extend_from_slice(&buffer[..size]);

		if let Some(pos) = request[len.saturating_sub(3)..]
			.windows(4)
			.position(|window| window == b"\r\n\r\n")
		{
			header_end = pos;
			break;
		}
	}

	if header_end == 0 {
		return Ok(());
	}

	let (header_bytes, body) = request.split_at(header_end + 4);
	let request_text = String::from_utf8_lossy(header_bytes);
	let mut lines = request_text.lines();
	let Some(request_line) = lines.next() else {
		return Ok(());
	};

	// GET /index.html HTTP/1.1
	let mut parts = request_line.split_whitespace();
	let method = parts.next().unwrap_or_default();
	let path = parts.next().unwrap_or("/");

	let mut content_length = 0usize;
	for line in lines {
		if let Some(value) = line.strip_prefix("Content-Length:") {
			content_length = value.trim().parse().unwrap_or(0);
		}
	}

	if method == "POST" && path.starts_with("/report") {
		let mut body_vec = body.to_vec();
		while body_vec.len() < content_length {
			let size = stream.read(&mut buffer)?;
			if size == 0 {
				break;
			}
			body_vec.extend_from_slice(&buffer[..size]);
		}

		let report = serde_json::from_slice(&body_vec)?;
		*report_state.result.lock().unwrap() = Some(report);
		report_state.signal.notify_all();

		write_response(&mut stream, 200, "text/plain", b"OK")?;
		return Ok(());
	}

	if method == "OPTIONS" {
		write_response(&mut stream, 204, "text/plain", b"")?;
		return Ok(());
	}

	if method != "GET" {
		write_response(&mut stream, 405, "text/plain", b"Method Not Allowed")?;
		return Ok(());
	}

	let (body, content_type, status) = match path {
		"/" | "/index.html" => (assets.index_html.as_bytes(), "text/html", 200),
		"/browser-runner.mjs" => (
			BROWSER_RUNNER_SOURCE.as_bytes(),
			"application/javascript",
			200,
		),
		"/runner-core.mjs" => (RUNNER_CORE_SOURCE.as_bytes(), "application/javascript", 200),
		"/shared.mjs" => (SHARED_JS_SOURCE.as_bytes(), "application/javascript", 200),
		"/worker-runner.mjs" => (
			WORKER_RUNNER_SOURCE.as_bytes(),
			"application/javascript",
			200,
		),
		"/service-worker.mjs" => (
			SERVICE_WORKER_SOURCE.as_bytes(),
			"application/javascript",
			200,
		),
		"/console-hook.mjs" => (
			CONSOLE_HOOK_SOURCE.as_bytes(),
			"application/javascript",
			200,
		),
		"/import.js" => (assets.import_js.as_bytes(), "application/javascript", 200),
		"/tests.json" => (assets.tests_json.as_bytes(), "application/json", 200),
		"/wasm" => (assets.wasm_bytes.as_slice(), "application/wasm", 200),
		_ => (b"Not Found".as_slice(), "text/plain", 404),
	};

	write_response(&mut stream, status, content_type, body)?;
	Ok(())
}

fn write_response(
	stream: &mut TcpStream,
	status: u16,
	content_type: &str,
	body: &[u8],
) -> Result<()> {
	let status_text = match status {
		200 => "OK",
		204 => "No Content",
		404 => "Not Found",
		405 => "Method Not Allowed",
		_ => "OK",
	};
	write!(
		stream,
		"HTTP/1.1 {status} {status_text}\r\nContent-Length: {}\r\nContent-Type: {content_type}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nCross-Origin-Opener-Policy: same-origin\r\nCross-Origin-Embedder-Policy: require-corp\r\n\r\n",
		body.len()
	)?;
	stream.write_all(body)?;
	Ok(())
}

fn read_tests(wasm_bytes: &[u8]) -> Result<Vec<TestEntry>> {
	/// None: `[0]`
	///
	/// Some(None): `[1]`
	///
	/// Some(Some(s)): `[2][len(s)][len]`
	fn option_option_string(
		data: &[u8],
		mut offset: usize,
	) -> Result<(Option<Option<String>>, usize)> {
		offset += 1;
		let value = match data[offset - 1] {
			0 => None,
			1 => Some(None),
			2 => {
				let len = u32::from_le_bytes(
					data[offset..offset + 4]
						.try_into()
						.expect("slice length checked"),
				) as usize;
				offset += 4;
				let s = std::str::from_utf8(&data[offset..offset + len])
					.context("payload is not utf-8")?
					.to_string();
				offset += len;
				Some(Some(s))
			}
			_ => bail!("mismatch flag value"),
		};
		Ok((value, offset))
	}

	let mut tests = Vec::new();

	for payload in Parser::new(0).parse_all(wasm_bytes) {
		if let Payload::CustomSection(section) = payload.context("failed to parse wasm")?
			&& section.name() == "js_bindgen.test"
		{
			let mut offset = 0;
			let data = section.data();

			while offset < data.len() {
				let len = u32::from_le_bytes(
					data[offset..offset + 4]
						.try_into()
						.expect("slice length checked"),
				) as usize;
				offset += 4;
				let end = offset + len;

				let (ignore, offset1) = option_option_string(data, offset)?;
				let (should_panic, offset2) = option_option_string(data, offset1)?;

				let name =
					std::str::from_utf8(&data[offset2..end]).context("test name is not utf-8")?;

				tests.push(TestEntry {
					name: name.to_string(),
					ignore: ignore.is_some(),
					ignore_reason: ignore.and_then(|s| s),
					should_panic: should_panic.is_some(),
					should_panic_reason: should_panic.and_then(|s| s),
				});

				offset = end;
			}
		}
	}

	Ok(tests)
}

fn apply_filters(
	tests: &mut Vec<TestEntry>,
	filters: &[String],
	ignored_only: bool,
	exact: bool,
) -> usize {
	let initial = tests.len();
	tests.retain(|test| {
		let matches_ignore = !ignored_only || test.ignore;
		let matches_filter = if filters.is_empty() {
			true
		} else if exact {
			filters.contains(&test.name)
		} else {
			filters.iter().any(|filter| test.name.contains(filter))
		};
		matches_ignore && matches_filter
	});
	initial - tests.len()
}
