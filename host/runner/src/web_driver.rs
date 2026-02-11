use std::env;
use std::env::VarError;
use std::ffi::OsString;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result, bail};
use js_bindgen_shared::ReadFile;
use serde_json::{Map, Value};
use tokio::net::{TcpListener, TcpStream};
use tokio::process::{Child, Command};
use tokio::sync::oneshot::{self, Receiver};
use tokio::time;
use url::Url;

use crate::util::AtomicFlag;

pub struct WebDriver {
	pub url: Url,
	pub capabilities: Capabilities,
	_child: Option<ChildWrapper>,
}

impl WebDriver {
	pub async fn run() -> Result<Self> {
		let (location, capabilities) = Self::find()?;

		match location {
			Location::Remote(url) => Ok(Self {
				url,
				capabilities,
				_child: None,
			}),
			Location::Local { path, args } => {
				// Wait for the WebDriver to come online and bind its port before we try to
				// connect to it.
				const MAX: Duration = Duration::from_secs(5);
				let start = Instant::now();

				let (driver_addr, child) = 'outer: loop {
					// Get a random open port to allow test runners to run in parallel.
					let driver_addr = TcpListener::bind("127.0.0.1:0").await?.local_addr()?;
					let child = Command::new(&path)
						.stdout(Stdio::piped())
						.stderr(Stdio::piped())
						.args(&args)
						.arg(format!("--port={}", driver_addr.port()))
						.spawn()?;
					let mut child = ChildWrapper::new(child);

					loop {
						let limit = time::sleep(MAX.saturating_sub(start.elapsed()));

						tokio::select! {
							() = limit => {
								child.output_error().await;
								bail!("failed to bind WebDriver port in timeout duration");
							},
							result = TcpStream::connect(driver_addr) => match result {
								Ok(_) => break 'outer (driver_addr, child),
								// Currently unclear which `ErrorKind`s just mean there is no socket to connect to.
								// Related: https://github.com/rust-lang/rust/issues/142557.
								Err(error) if matches!(error.kind(), ErrorKind::ConnectionRefused) => (),
								Err(error) => {
									eprintln!("WebDriver connection failed: {error}");
									eprintln!("trying again ...");
								}
							},
							result = child.wait() => {
								match result {
									Ok(status) => if status.success() {
										eprintln!("WebDriver exited prematurely with success");
									} else {
										eprintln!("WebDriver failed with status: {status}");
									}
									Err(error) => eprintln!("WebDriver failed with error: {error}"),
								}

								child.output_error().await;

								eprintln!("failed to start WebDriver, trying again ...");

								// Back off. If something is really wrong we don't want to re-try in a
								// hot loop.
								time::sleep(Duration::from_millis(100)).await;
								break;
							}
						};
					}
				};

				Ok(Self {
					url: Url::parse(&format!("http://{driver_addr}"))?,
					capabilities,
					_child: Some(child),
				})
			}
		}
	}

	/// Attempts to find a WebDriver.
	///
	/// An explicitly set WebDriver is searched for in `JBG_TEST_<WebDriver>`
	/// and `JBG_TEST_<WebDriver>_REMOTE`. If not found we look for the first
	/// WebDriver in `PATH`.
	fn find() -> Result<(Location, Capabilities)> {
		// Find out if the user is explicitely opting into a specific WebDriver.
		let mut driver = None;

		for kind in WebDriverKind::iter() {
			let env = format!("JBG_TEST_{}_REMOTE", kind.to_str().to_uppercase());
			let url = match env::var(&env) {
				Ok(var) => Url::parse(&var).context(format!("failed to parse `{env}`"))?,
				Err(VarError::NotPresent) => continue,
				Err(VarError::NotUnicode(_)) => bail!("unable to parse `{env}`"),
			};

			if driver.replace((kind, Location::Remote(url))).is_some() {
				bail!("found multiple `JBG_TEST_<WebDriver>` environment variables")
			}
		}

		for kind in WebDriverKind::iter() {
			let env = kind.to_str().to_uppercase();
			let Some(path) = env::var_os(format!("JBG_TEST_{env}")) else {
				continue;
			};

			if driver
				.replace((
					kind,
					Location::Local {
						path: path.into(),
						args: kind.env_args()?,
					},
				))
				.is_some()
			{
				bail!("found multiple `JBG_TEST_<WebDriver>` environment variables")
			}
		}

		// Otherwise pick a default by looking into the users `PATH`.
		if driver.is_none() {
			for path in env::split_paths(&env::var_os("PATH").unwrap_or_default()) {
				let Some(kind) = WebDriverKind::iter().find(|kind| {
					path.join(kind.to_str())
						.with_extension(env::consts::EXE_EXTENSION)
						.exists()
				}) else {
					continue;
				};

				driver = Some((
					kind,
					Location::Local {
						path: kind.to_str().into(),
						args: kind.env_args()?,
					},
				));
				break;
			}
		}

		if let Some((kind, location)) = driver {
			let webdriver_json_path = env::var_os("JBG_TEST_WEBDRIVER_JSON").map(PathBuf::from);
			let webdriver_json_path = webdriver_json_path
				.as_deref()
				.unwrap_or(Path::new("webdriver.json"));
			let capabilities = match ReadFile::new(webdriver_json_path) {
				Ok(file) => serde_json::from_slice(&file)?,
				Err(error) if matches!(error.kind(), ErrorKind::NotFound) => Capabilities::new(),
				Err(error) => return Err(error.into()),
			};
			let capabilities = Self::create_capabilities(kind, capabilities)?;

			Ok((location, capabilities))
		} else {
			bail!(
				"failed to find a suitable WebDriver binary or remote running WebDriver to drive headless testing; \
				to configure the location of the webdriver binary you can use environment variables \
				like `WBG_TEST_<WebDriver>=/path/to/<WebDriver>` or make sure that the binary is in `PATH`; \
				to configure the address of a remote WebDriver you can use environment variables \
				like `WBG_TEST_<WebDriver>_REMOTE=http://remote.host/`; \
				you can download currently supported drivers at:\n\
				* chromedriver - https://googlechromelabs.github.io/chrome-for-testing/\n\
				* geckodriver - https://github.com/mozilla/geckodriver/releases\n\
				* msedgedriver - https://developer.microsoft.com/microsoft-edge/tools/webdriver/\n\
				* safaridriver - pre-installed on MacOS"
			)
		}
	}

	fn create_capabilities(kind: WebDriverKind, mut cap: Capabilities) -> Result<Capabilities> {
		match kind {
			WebDriverKind::Chrome => {
				cap.entry("goog:chromeOptions".to_string())
					.or_insert_with(|| Value::Object(Map::new()))
					.as_object_mut()
					.context("`goog:chromeOptions` isn't a JSON object")?
					.entry("args".to_string())
					.or_insert_with(|| Value::Array(Vec::new()))
					.as_array_mut()
					.context("`args` isn't a JSON array")?
					.extend(vec![
						Value::String("headless".to_string()),
						// See https://stackoverflow.com/questions/50642308/.
						Value::String("disable-dev-shm-usage".to_string()),
						Value::String("no-sandbox".to_string()),
					]);
			}
			WebDriverKind::Edge => {
				cap.entry("ms:edgeOptions".to_string())
					.or_insert_with(|| Value::Object(Map::new()))
					.as_object_mut()
					.context("`ms:edgeOptions` isn't a JSON object")?
					.entry("args".to_string())
					.or_insert_with(|| Value::Array(Vec::new()))
					.as_array_mut()
					.context("`args` isn't a JSON array")?
					.extend(vec![
						Value::String("headless".to_string()),
						// See https://stackoverflow.com/questions/50642308/.
						Value::String("disable-dev-shm-usage".to_string()),
						Value::String("no-sandbox".to_string()),
					]);
			}
			WebDriverKind::Gecko => cap
				.entry("moz:firefoxOptions".to_string())
				.or_insert_with(|| Value::Object(serde_json::Map::new()))
				.as_object_mut()
				.context("`moz:firefoxOptions` isn't a JSON object")?
				.entry("args".to_string())
				.or_insert_with(|| Value::Array(Vec::new()))
				.as_array_mut()
				.context("`args` isn't a JSON array")?
				.extend(vec![Value::String("-headless".to_string())]),
			WebDriverKind::Safari => (),
		}

		Ok(cap)
	}
}

#[derive(Clone, Copy)]
enum WebDriverKind {
	Chrome,
	Edge,
	Gecko,
	Safari,
}

impl WebDriverKind {
	fn to_str(self) -> &'static str {
		match self {
			Self::Chrome => "chromedriver",
			Self::Edge => "msedgedriver",
			Self::Gecko => "geckodriver",
			Self::Safari => "safaridriver",
		}
	}

	fn iter() -> impl Iterator<Item = Self> {
		[Self::Chrome, Self::Edge, Self::Gecko, Self::Safari].into_iter()
	}

	fn env_args(self) -> Result<Vec<OsString>> {
		let key = format!("JBG_TEST_{}_ARGS", self.to_str().to_uppercase());
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
}

enum Location {
	Local { path: PathBuf, args: Vec<OsString> },
	Remote(Url),
}

/// Serialized `webdriver.json` data.
pub type Capabilities = Map<String, Value>;

struct ChildWrapper(Option<ChildInner>);

struct ChildInner {
	child: Child,
	flag: Arc<AtomicFlag>,
	stdout: Receiver<Vec<u8>>,
	stderr: Receiver<Vec<u8>>,
}

impl Drop for ChildWrapper {
	fn drop(&mut self) {
		if let Some(mut inner) = self.0.take() {
			let _ = inner.child.start_kill();
		}
	}
}

impl ChildWrapper {
	fn new(mut child: Child) -> Self {
		let flag = Arc::new(AtomicFlag::new());
		let (stdout_tx, stdout_rx) = oneshot::channel();
		let mut stdout = child.stdout.take().unwrap();
		tokio::spawn({
			let flag = flag.clone();
			async move {
				let mut output = Vec::new();
				tokio::select! {
					() = flag.as_ref() => (),
					_ = tokio::io::copy(&mut stdout, &mut output) => (),
				}
				let _ = stdout_tx.send(output);
			}
		});

		let (stderr_tx, stderr_rx) = oneshot::channel();
		let mut stderr = child.stderr.take().unwrap();
		tokio::spawn({
			let flag = flag.clone();
			async move {
				let mut output = Vec::new();
				tokio::select! {
					() = flag.as_ref() => (),
					_ = tokio::io::copy(&mut stderr, &mut output) => (),
				}
				let _ = stderr_tx.send(output);
			}
		});

		Self(Some(ChildInner {
			child,
			flag,
			stdout: stdout_rx,
			stderr: stderr_rx,
		}))
	}

	async fn wait(&mut self) -> tokio::io::Result<ExitStatus> {
		self.0.as_mut().unwrap().child.wait().await
	}

	async fn output_error(mut self) {
		let ChildInner {
			mut child,
			flag,
			stdout,
			stderr,
		} = self.0.take().unwrap();

		// Wait a moment in hope we can kill the child and get the complete output.
		let _ = tokio::time::timeout(Duration::from_millis(100), child.kill()).await;
		flag.signal();

		let stdout = stdout.await.unwrap();

		if !stdout.is_empty() {
			eprintln!(
				"------ WebDriver stdout ------\n{}",
				String::from_utf8_lossy(&stdout)
			);

			if !stdout.ends_with(b"\n") {
				eprintln!();
			}
		}

		let stderr = stderr.await.unwrap();

		if !stderr.is_empty() {
			eprintln!(
				"------ WebDriver stderr ------\n{}",
				String::from_utf8_lossy(&stderr)
			);

			if !stderr.ends_with(b"\n") {
				eprintln!();
			}
		}
	}
}
