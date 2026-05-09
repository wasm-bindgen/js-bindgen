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
use crate::run_data::RunData;
use crate::server::HttpServer;

const DENO_JS: &str = include_str!("js/deno/deno.mjs");
const NODE_JS_JS: &str = include_str!("js/node-js/node-js.mjs");
pub const SHARED_JS: &str = include_str!("js/shared/shared.mjs");
pub const SHARED_TERMINAL_JS: &str = include_str!("js/shared/shared-terminal.mjs");

pub struct Runner {
	wasm_path: PathBuf,
	wasm_bytes: ReadFile,
	imports_path: PathBuf,
	run_data: String,
}

impl Runner {
	pub fn new(
		wasm_path: PathBuf,
		wasm_bytes: ReadFile,
		imports_path: PathBuf,
		run_data: &RunData,
	) -> Self {
		Self {
			wasm_path,
			wasm_bytes,
			imports_path,
			run_data: serde_json::to_string(&run_data).unwrap(),
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

		fs::create_dir(dir.path().join("deno"))?;
		let script_path = dir.path().join("deno/script.mjs");
		fs::write(&script_path, DENO_JS)?;

		fs::write(dir.path().join("run-data.json"), self.run_data)?;
		fs::copy(self.wasm_path, dir.path().join("wasm.wasm"))?;
		fs::copy(self.imports_path, dir.path().join("imports.mjs"))?;
		fs::create_dir(dir.path().join("shared"))?;
		fs::write(dir.path().join("shared/shared.mjs"), SHARED_JS)?;
		fs::write(
			dir.path().join("shared/shared-terminal.mjs"),
			SHARED_TERMINAL_JS,
		)?;

		let status = Command::new(binary)
			.arg("run")
			.args(args)
			.arg("--allow-read")
			.arg(script_path)
			.status()?;

		if !status.success() {
			process::exit(status.code().unwrap_or(1));
		}

		Ok(())
	}

	fn run_node_js(self, binary: &Path, args: &[OsString]) -> Result<()> {
		let dir = tempfile::tempdir()?;

		fs::create_dir(dir.path().join("node-js"))?;
		let script_path: PathBuf = dir.path().join("node-js/script.mjs");
		fs::write(&script_path, NODE_JS_JS)?;

		fs::write(dir.path().join("run-data.json"), self.run_data)?;
		fs::copy(self.wasm_path, dir.path().join("wasm.wasm"))?;
		fs::copy(self.imports_path, dir.path().join("imports.mjs"))?;
		fs::create_dir(dir.path().join("shared"))?;
		fs::write(dir.path().join("shared/shared.mjs"), SHARED_JS)?;
		fs::write(
			dir.path().join("shared/shared-terminal.mjs"),
			SHARED_TERMINAL_JS,
		)?;

		let status = Command::new(binary).args(args).arg(&script_path).status()?;

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
		) -> Result<i32> {
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

				if status == 0 {
					Ok(())
				} else {
					process::exit(status);
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
			self.run_data,
		)
		.await
	}
}
