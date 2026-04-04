use std::ffi::OsString;
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use std::{env, fs, process};

use anyhow::{Context, Result};
use fantoccini::ClientBuilder;
use fantoccini::wd::Capabilities;
use js_bindgen_shared::{ReadFile, WebDriver, WebDriverLocation};
use tokio::runtime::Runtime;
use tokio::signal;

use crate::config::{EngineKind, RunnerConfig, WorkerKind};
use crate::server::{HttpServer, Status};
use crate::test::TestData;

const DENO_JS: &str = include_str!("js/deno.mjs");
const NODE_JS_JS: &str = include_str!("js/node-js.mjs");
pub const SHARED_JS: &str = include_str!("js/shared.mjs");
pub const SHARED_TERMINAL_JS: &str = include_str!("js/shared-terminal.mjs");

pub struct Runner {
	wasm_path: PathBuf,
	wasm_bytes: ReadFile,
	imports_path: PathBuf,
	test_data: String,
}

impl Runner {
	pub fn new(
		wasm_path: PathBuf,
		wasm_bytes: ReadFile,
		imports_path: PathBuf,
		test_data: &TestData,
	) -> Self {
		Self {
			wasm_path,
			wasm_bytes,
			imports_path,
			test_data: serde_json::to_string(&test_data).unwrap(),
		}
	}

	pub fn run(self) -> Result<()> {
		match RunnerConfig::search()? {
			RunnerConfig::Engine { kind, path, args } => match kind {
				EngineKind::Deno => self.run_deno(&path, &args),
				EngineKind::NodeJs => self.run_node_js(&path, &args),
			},
			RunnerConfig::WebDriver {
				location,
				capabilities,
				worker,
				..
			} => Runtime::new()?.block_on(self.run_browser(location, capabilities, worker)),
			RunnerConfig::Server { worker } => Runtime::new()?.block_on(self.run_server(worker)),
		}
	}

	fn run_deno(self, binary: &Path, args: &[OsString]) -> Result<()> {
		let dir = tempfile::tempdir()?;

		let runner_path = dir.path().join("runner.mts");
		fs::write(&runner_path, DENO_JS)?;

		fs::write(dir.path().join("test-data.json"), self.test_data)?;
		fs::copy(self.wasm_path, dir.path().join("wasm.wasm"))?;
		fs::copy(self.imports_path, dir.path().join("imports.mjs"))?;
		fs::write(dir.path().join("shared.mjs"), SHARED_JS)?;
		fs::write(dir.path().join("shared-terminal.mjs"), SHARED_TERMINAL_JS)?;

		let status = Command::new(binary)
			.arg("run")
			.args(args)
			.arg("--allow-read")
			.arg(runner_path)
			.status()?;

		if !status.success() {
			process::exit(status.code().unwrap_or(1));
		}

		Ok(())
	}

	fn run_node_js(self, binary: &Path, args: &[OsString]) -> Result<()> {
		let dir = tempfile::tempdir()?;

		let runner_path = dir.path().join("runner.mjs");
		fs::write(&runner_path, NODE_JS_JS)?;

		fs::write(dir.path().join("test-data.json"), self.test_data)?;
		fs::copy(self.wasm_path, dir.path().join("wasm.wasm"))?;
		fs::copy(self.imports_path, dir.path().join("imports.mjs"))?;
		fs::write(dir.path().join("shared.mjs"), SHARED_JS)?;
		fs::write(dir.path().join("shared-terminal.mjs"), SHARED_TERMINAL_JS)?;

		let status = Command::new(binary).args(args).arg(&runner_path).status()?;

		if !status.success() {
			process::exit(status.code().unwrap_or(1));
		}

		Ok(())
	}

	async fn run_browser(
		self,
		location: WebDriverLocation,
		capabilities: Capabilities,
		worker: Option<WorkerKind>,
	) -> Result<()> {
		async fn run(
			server: HttpServer,
			driver: &WebDriver,
			capabilities: Capabilities,
		) -> Result<Status> {
			let client = ClientBuilder::rustls()?
				.capabilities(capabilities)
				.connect(driver.url().as_str())
				.await?;

			client.goto(server.url()).await?;

			let status = server.wait().await;
			client.close().await?;

			Ok(status)
		}

		let server = self.http_server(true, worker).await?;
		let driver = WebDriver::run(location).await?;

		match run(server, &driver, capabilities).await {
			Ok(status) => {
				driver.shutdown().await?;

				match status {
					Status::Ok => Ok(()),
					// See https://github.com/rust-lang/cargo/blob/fa50b03244beda717b3fd2c7a647ba93c0d39e05/src/cargo/ops/cargo_test.rs#L418.
					Status::Failed => process::exit(101),
					Status::Abnormal => process::exit(1),
				}
			}
			Err(error) => {
				driver.output_error().await;
				Err(error)
			}
		}
	}

	async fn run_server(self, worker: Option<WorkerKind>) -> Result<()> {
		let server = self.http_server(false, worker).await?;

		println!("Open this URL in your browser to run tests:");
		println!("{}", server.url());
		println!("Shutdown via CTRL-C");

		signal::ctrl_c().await?;
		server.shutdown().await;

		Ok(())
	}

	async fn http_server(self, headless: bool, worker: Option<WorkerKind>) -> Result<HttpServer> {
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
		HttpServer::start(
			address,
			headless,
			worker,
			self.wasm_bytes,
			&self.imports_path,
			self.test_data,
		)
		.await
	}
}
