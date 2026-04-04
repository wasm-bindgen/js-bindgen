use std::borrow::Cow;
use std::env::{self, VarError};
use std::ffi::OsString;
use std::fmt::{self, Display, Formatter};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error, Result, anyhow, bail};
use fantoccini::wd::Capabilities;
use js_bindgen_shared::{ReadFile, WebDriverKind, WebDriverLocation};
use serde_json::{Map, Value};
use strum::VariantArray;
use url::Url;

pub enum RunnerConfig {
	Engine {
		kind: EngineKind,
		path: Cow<'static, Path>,
		args: Vec<OsString>,
	},
	WebDriver {
		kind: WebDriverKind,
		location: WebDriverLocation,
		capabilities: Capabilities,
		worker: Option<WorkerKind>,
	},
	Server {
		worker: Option<WorkerKind>,
	},
}

#[derive(Clone, Copy)]
enum RunnerKind {
	Engine(Option<EngineKind>),
	WebDriver(Option<WebDriverKind>),
	Server,
}

#[derive(Clone, Copy, VariantArray)]
pub enum EngineKind {
	Deno,
	NodeJs,
}

#[derive(Clone, Copy, Debug)]
pub enum WorkerKind {
	/// <https://developer.mozilla.org/en-US/docs/Web/API/Worker>
	Dedicated,
	/// <https://developer.mozilla.org/en-US/docs/Web/API/SharedWorker>
	Shared,
	/// <https://developer.mozilla.org/en-US/docs/Web/API/ServiceWorker>
	Service,
}

impl RunnerConfig {
	pub fn search() -> Result<Self> {
		fn search_engine(config: &mut Option<RunnerConfig>, engines: &[EngineKind]) -> Result<()> {
			for engine in engines.iter().copied() {
				let Some(path) = env::var_os(format!("JBG_TEST_{}_PATH", engine.to_env())) else {
					continue;
				};

				if let Some(first) = config.replace(RunnerConfig::Engine {
					kind: engine,
					path: Cow::Owned(path.into()),
					args: env_args(engine.to_env())?,
				}) {
					return Err(multiple_error(&first, engine.to_env()));
				}
			}

			Ok(())
		}

		fn search_web_driver(
			config: &mut Option<RunnerConfig>,
			web_drivers: &[WebDriverKind],
		) -> Result<()> {
			for web_driver in web_drivers.iter().copied() {
				let Some(location) = web_driver_location_from_env(web_driver)? else {
					continue;
				};

				if let Some(first) = config.replace(RunnerConfig::WebDriver {
					kind: web_driver,
					location,
					capabilities: create_capabilities(web_driver)?,
					worker: WorkerKind::from_env()?,
				}) {
					return Err(multiple_error(&first, web_driver.to_env()));
				}
			}

			Ok(())
		}

		fn multiple_error(first: &RunnerConfig, second: &str) -> Error {
			let first = match first {
				RunnerConfig::Engine { kind, .. } => kind.to_env(),
				RunnerConfig::WebDriver { kind, .. } => kind.to_env(),
				RunnerConfig::Server { .. } => unreachable!(),
			};

			anyhow!(
				"found incompatible `JBG_TEST_{first}_PATH` and `JBG_TEST_{second}_PATH` \
				 environment variables"
			)
		}

		fn search_path(
			engines: &[EngineKind],
			web_drivers: &[WebDriverKind],
		) -> Result<Option<RunnerConfig>> {
			for path in env::split_paths(&env::var_os("PATH").unwrap_or_default()) {
				for engine in engines {
					if path
						.join(engine.to_binary())
						.with_extension(env::consts::EXE_EXTENSION)
						.exists()
					{
						return Ok(Some(RunnerConfig::Engine {
							kind: *engine,
							path: Cow::Borrowed(Path::new(engine.to_binary())),
							args: env_args(engine.to_env())?,
						}));
					}
				}

				for web_driver in web_drivers {
					if path
						.join(web_driver.to_binary())
						.with_extension(env::consts::EXE_EXTENSION)
						.exists()
					{
						return Ok(Some(RunnerConfig::WebDriver {
							kind: *web_driver,
							location: WebDriverLocation::Local {
								path: Cow::Borrowed(Path::new(web_driver.to_binary())),
								args: env_args(web_driver.to_env())?,
							},
							capabilities: create_capabilities(*web_driver)?,
							worker: WorkerKind::from_env()?,
						}));
					}
				}
			}

			Ok(None)
		}

		let kind = RunnerKind::from_env()?;
		let mut config = None;

		if let Some(kind) = kind {
			match kind {
				RunnerKind::Engine(None) => {
					search_engine(&mut config, EngineKind::VARIANTS)?;
				}
				RunnerKind::Engine(Some(engine)) => {
					search_engine(&mut config, &[engine])?;
				}
				RunnerKind::WebDriver(None) => {
					search_web_driver(&mut config, WebDriverKind::VARIANTS)?;
				}
				RunnerKind::WebDriver(Some(web_driver)) => {
					search_web_driver(&mut config, &[web_driver])?;
				}
				RunnerKind::Server => {
					config = Some(Self::Server {
						worker: WorkerKind::from_env()?,
					});
				}
			}
		} else {
			search_engine(&mut config, EngineKind::VARIANTS)?;
			search_web_driver(&mut config, WebDriverKind::VARIANTS)?;
		}

		if let Some(config) = config {
			Ok(config)
		} else {
			let config = if let Some(kind) = kind {
				match kind {
					RunnerKind::Engine(None) => search_path(EngineKind::VARIANTS, &[])?,
					RunnerKind::Engine(Some(engine)) => search_path(&[engine], &[])?,
					RunnerKind::WebDriver(None) => search_path(&[], WebDriverKind::VARIANTS)?,
					RunnerKind::WebDriver(Some(web_driver)) => search_path(&[], &[web_driver])?,
					RunnerKind::Server => unreachable!(),
				}
			} else {
				search_path(EngineKind::VARIANTS, WebDriverKind::VARIANTS)?
			};

			if let Some(config) = config {
				Ok(config)
			} else if let Some(kind) = kind {
				match kind {
					RunnerKind::Engine(None) => bail!(
						"failed to find a suitable JS engine binary; {}",
						EngineKind::search_error()
					),
					RunnerKind::Engine(Some(engine)) => bail!(
						"failed to find a suitable {engine} binary; to configure the location of \
						 the {engine} binary you can use environment variable \
						 `WBG_TEST_{env}_PATH=/path/to/{binary}` or make sure that the binary is \
						 in `PATH`; you can download {engine} at: {url}",
						env = engine.to_env(),
						binary = engine.to_binary(),
						url = engine.to_download_url(),
					),
					RunnerKind::WebDriver(None) => bail!(
						"failed to find a suitable WebDriver binary or remote running WebDriver; \
						 {}",
						WebDriverKind::search_error()
					),
					RunnerKind::WebDriver(Some(web_driver)) => bail!(
						"failed to find a suitable {web_driver} binary or remote running {web_driver}; \
						to configure the location of the WebDriver binary you can use environment variable \
						`WBG_TEST_{env}_PATH=/path/to/{binary}` or make sure that the binary is in `PATH`; \
						to configure the address of a remote {web_driver} you can use environment variable \
						`WBG_TEST_{env}_REMOTE=http://remote.host/`{url}",
						env = web_driver.to_env(),
						binary = web_driver.to_binary(),
						url = if let Some(url) = web_driver.to_download_url() {
							format!("; you can download {web_driver} at: {url}")
						} else {
							String::new()
						},
					),
					RunnerKind::Server => unreachable!(),
				}
			} else {
				bail!(
					"failed to find a suitable JS engine or WebDriver binary\n\n{}\n\n{}",
					EngineKind::search_error(),
					WebDriverKind::search_error()
				)
			}
		}
	}
}

impl RunnerKind {
	fn from_env() -> Result<Option<Self>> {
		Ok(match env::var("JBG_TEST_RUNNER") {
			Ok(runner) => Some(match runner.as_str() {
				"engine" => Self::Engine(None),
				"web-driver" => Self::WebDriver(None),
				"server" => Self::Server,
				runner => 'runner: {
					for engine in EngineKind::VARIANTS {
						if runner == engine.to_name() {
							break 'runner Self::Engine(Some(*engine));
						}
					}

					for web_driver in WebDriverKind::VARIANTS {
						if runner == web_driver.to_name() {
							break 'runner Self::WebDriver(Some(*web_driver));
						}
					}

					bail!("unrecognized runner: {runner}")
				}
			}),
			Err(VarError::NotPresent) => None,
			Err(VarError::NotUnicode(_)) => bail!("unable to parse `JBG_TEST_RUNNER`"),
		})
	}
}

impl EngineKind {
	fn to_name(self) -> &'static str {
		match self {
			Self::Deno => "deno",
			Self::NodeJs => "node-js",
		}
	}

	fn to_env(self) -> &'static str {
		match self {
			Self::Deno => "DENO",
			Self::NodeJs => "NODE_JS",
		}
	}

	fn to_binary(self) -> &'static str {
		match self {
			Self::Deno => "deno",
			Self::NodeJs => "node",
		}
	}

	fn to_download_url(self) -> &'static str {
		match self {
			Self::Deno => "https://deno.com/",
			Self::NodeJs => "https://nodejs.org/en/download",
		}
	}

	fn search_error() -> String {
		format!(
			"to configure the location of a JS engine binary you can use environment variables \
			 like `WBG_TEST_<engine>_PATH=/path/to/<engine>` or make sure that the binary is in \
			 `PATH`; you can download supported Js engines at:\n* {} - {}\n* {} - {}",
			Self::Deno,
			Self::Deno.to_download_url(),
			Self::NodeJs,
			Self::NodeJs.to_download_url(),
		)
	}
}

impl Display for EngineKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let name = match self {
			Self::Deno => "Deno",
			Self::NodeJs => "Node.js",
		};
		f.write_str(name)
	}
}

impl WorkerKind {
	fn from_env() -> Result<Option<Self>> {
		Ok(match env::var("JBG_TEST_WORKER") {
			Ok(worker) => Some(match worker.as_str() {
				"dedicated" => Self::Dedicated,
				"shared" => Self::Shared,
				"service" => Self::Service,
				worker => bail!("unrecognized worker: {worker}"),
			}),
			Err(VarError::NotPresent) => None,
			Err(VarError::NotUnicode(_)) => bail!("unable to parse `JBG_TEST_WORKER`"),
		})
	}
}

fn web_driver_location_from_env(kind: WebDriverKind) -> Result<Option<WebDriverLocation>> {
	let mut location = None;

	let local_env = format!("JBG_TEST_{}_PATH", kind.to_env());
	if let Some(path) = env::var_os(&local_env) {
		location = Some(WebDriverLocation::Local {
			path: Cow::Owned(path.into()),
			args: env_args(kind.to_env())?,
		});
	}

	let remote_env = format!("JBG_TEST_{}_REMOTE", kind.to_env());
	match env::var(&remote_env) {
		Ok(var) => {
			let url = Url::parse(&var).context(format!("failed to parse `{remote_env}`"))?;

			if location.replace(WebDriverLocation::Remote(url)).is_some() {
				bail!("found incompatible `{local_env}` and `{remote_env}` environment variables")
			}
		}
		Err(VarError::NotPresent) => (),
		Err(VarError::NotUnicode(_)) => bail!("unable to parse `{remote_env}`"),
	}

	Ok(location)
}

pub fn env_args(name: &str) -> Result<Vec<OsString>> {
	let key = format!("JBG_TEST_{name}_ARGS");

	let Some(var) = env::var_os(&key) else {
		return Ok(Vec::new());
	};

	let Some(args) = shlex::bytes::split(var.as_encoded_bytes()) else {
		bail!("failed to parse `{key}`");
	};

	Ok(args
		.into_iter()
		.map(|arg|
					// SAFETY: original source is a `OsString`.
					unsafe { OsString::from_encoded_bytes_unchecked(arg) })
		.collect())
}

fn create_capabilities(kind: WebDriverKind) -> Result<Capabilities> {
	fn v8_options(object: &str, capabilities: &mut Capabilities) -> Result<()> {
		capabilities
			.entry(String::from(object))
			.or_insert_with(|| Value::Object(Map::new()))
			.as_object_mut()
			.with_context(|| format!("`{object}` isn't a JSON object"))?
			.entry(String::from("args"))
			.or_insert_with(|| Value::Array(Vec::new()))
			.as_array_mut()
			.context("`args` isn't a JSON array")?
			.extend(vec![
				Value::String(String::from("headless")),
				// See https://stackoverflow.com/questions/50642308/.
				Value::String(String::from("disable-dev-shm-usage")),
				Value::String(String::from("no-sandbox")),
			]);

		Ok(())
	}

	let webdriver_json_path = env::var_os("JBG_TEST_WEBDRIVER_JSON").map(PathBuf::from);
	let webdriver_json_path = webdriver_json_path
		.as_deref()
		.unwrap_or(Path::new("webdriver.json"));
	let mut capabilities = match ReadFile::new(webdriver_json_path) {
		Ok(file) => serde_json::from_slice(&file)?,
		Err(error) if matches!(error.kind(), ErrorKind::NotFound) => Capabilities::new(),
		Err(error) => return Err(error.into()),
	};

	match kind {
		WebDriverKind::Chrome => v8_options("goog:chromeOptions", &mut capabilities)?,
		WebDriverKind::Edge => v8_options("ms:edgeOptions", &mut capabilities)?,
		WebDriverKind::Gecko => capabilities
			.entry(String::from("moz:firefoxOptions"))
			.or_insert_with(|| Value::Object(Map::new()))
			.as_object_mut()
			.context("`moz:firefoxOptions` isn't a JSON object")?
			.entry(String::from("args"))
			.or_insert_with(|| Value::Array(Vec::new()))
			.as_array_mut()
			.context("`args` isn't a JSON array")?
			.extend(vec![Value::String(String::from("-headless"))]),
		WebDriverKind::Safari => (),
	}

	Ok(capabilities)
}
