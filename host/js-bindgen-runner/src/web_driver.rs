use std::env;
use std::env::VarError;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::{ExitStatus, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use serde_json::{Map, Value};
use tokio::net::{TcpListener, TcpStream};
use tokio::process::{Child, Command};
use tokio::sync::oneshot::{self, Receiver, Sender};
use tokio::time;
use url::Url;

pub enum WebDriver {
	Chrome(Location),
	Edge(Location),
	Gecko(Location),
	Safari(Location),
}

pub enum Location {
	Local((PathBuf, Vec<OsString>)),
	Remote(Url),
}

impl WebDriver {
	/// Attempts to find a WebDriver.
	///
	/// An explicitly set WebDriver is searched for in `JBG_TEST_<WebDriver>`
	/// and `JBG_TEST_<WebDriver>_REMOTE`. If not found we look for the first
	/// WebDriver in `PATH`.
	pub fn find() -> Result<Self> {
		fn env_args(name: &str) -> Result<Vec<OsString>> {
			let key = format!("JBG_TEST_{}_ARGS", name.to_uppercase());
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

		let drivers = [
			("chromedriver", Self::Chrome as fn(Location) -> Self),
			("msedgedriver", Self::Edge as _),
			("geckodriver", Self::Gecko as _),
			("safaridriver", Self::Safari as _),
		];

		// Find out if the user is explicitely opting into a specific WebDriver.
		let mut driver = None;

		for (key, ctor) in drivers.iter() {
			let env = format!("JBG_TEST_{}_REMOTE", key.to_uppercase());
			let url = match env::var(&env) {
				Ok(var) => Url::parse(&var).context(format!("failed to parse `{env}`"))?,
				Err(VarError::NotPresent) => continue,
				Err(VarError::NotUnicode(_)) => bail!("unable to parse `{env}`"),
			};

			if driver.replace(ctor(Location::Remote(url))).is_some() {
				bail!("found multiple `JBG_TEST_<WebDriver>` environment variables")
			}
		}

		for (key, ctor) in drivers.iter() {
			let env = key.to_uppercase();
			let Some(path) = env::var_os(format!("JBG_TEST_{env}")) else {
				continue;
			};

			if driver
				.replace(ctor(Location::Local((path.into(), env_args(key)?))))
				.is_some()
			{
				bail!("found multiple `JBG_TEST_<WebDriver>` environment variables")
			}
		}

		if let Some(driver) = driver {
			return Ok(driver);
		}

		// Otherwise pick a default by looking into the users `PATH`.
		for path in env::split_paths(&env::var_os("PATH").unwrap_or_default()) {
			let Some((driver, ctor)) = drivers.iter().find(|(name, _)| {
				path.join(name)
					.with_extension(env::consts::EXE_EXTENSION)
					.exists()
			}) else {
				continue;
			};

			return Ok(ctor(Location::Local((driver.into(), env_args(driver)?))));
		}

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

	pub async fn launch(&self) -> Result<WebDriverGuard> {
		let locate = match self {
			Self::Chrome(locate) => locate,
			Self::Edge(locate) => locate,
			Self::Gecko(locate) => locate,
			Self::Safari(locate) => locate,
		};

		let guard = match locate {
			Location::Remote(url) => WebDriverGuard {
				url: url.clone(),
				_child: None,
			},
			Location::Local((path, args)) => {
				// Wait for the WebDriver to come online and bind its port before we try to
				// connect to it.
				let start = Instant::now();
				const MAX: Duration = Duration::from_secs(5);

				let (driver_addr, child) = loop {
					// Get a random open port to allow test runners to run in parallel.
					let driver_addr = TcpListener::bind("127.0.0.1:0").await?.local_addr()?;
					let child = Command::new(path)
						.stdout(Stdio::piped())
						.stderr(Stdio::piped())
						.args(args)
						.arg(format!("--port={}", driver_addr.port()))
						.spawn()?;
					let mut child = ChildWrapper::new(child);

					let limit = time::sleep(MAX - start.elapsed());

					tokio::select! {
						_ = limit => {
							child.output_error().await;
							bail!("failed to bind WebDriver port in timeout duration");
						},
						_ = TcpStream::connect(driver_addr) => break (driver_addr, child),
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
						}
					};
				};

				WebDriverGuard {
					url: Url::parse(&format!("http://{driver_addr}"))?,
					_child: Some(child),
				}
			}
		};
		Ok(guard)
	}
}

pub struct WebDriverGuard {
	pub url: Url,
	_child: Option<ChildWrapper>,
}

/// Options that can use to customize and configure a WebDriver session.
pub type Capabilities = Map<String, Value>;

pub fn capabilities(driver: &WebDriver, mut cap: Capabilities) -> Result<Capabilities> {
	match driver {
		WebDriver::Chrome(_) => {
			cap.entry("goog:chromeOptions".to_string())
				.or_insert_with(|| Value::Object(serde_json::Map::new()))
				.as_object_mut()
				.expect("goog:chromeOptions wasn't a JSON object")
				.entry("args".to_string())
				.or_insert_with(|| Value::Array(vec![]))
				.as_array_mut()
				.expect("args wasn't a JSON array")
				.extend(vec![
					Value::String("headless".to_string()),
					// See https://stackoverflow.com/questions/50642308/
					// for what this funky `disable-dev-shm-usage`
					// option is
					Value::String("disable-dev-shm-usage".to_string()),
					Value::String("no-sandbox".to_string()),
				])
		}
		WebDriver::Edge(_) => {
			cap.entry("ms:edgeOptions".to_string())
				.or_insert_with(|| Value::Object(serde_json::Map::new()))
				.as_object_mut()
				.expect("ms:edgeOptions wasn't a JSON object")
				.entry("args".to_string())
				.or_insert_with(|| Value::Array(vec![]))
				.as_array_mut()
				.expect("args wasn't a JSON array")
				.extend(vec![
					Value::String("headless".to_string()),
					// See https://stackoverflow.com/questions/50642308/
					// for what this funky `disable-dev-shm-usage`
					// option is
					Value::String("disable-dev-shm-usage".to_string()),
					Value::String("no-sandbox".to_string()),
				])
		}
		WebDriver::Gecko(_) => cap
			.entry("moz:firefoxOptions".to_string())
			.or_insert_with(|| Value::Object(serde_json::Map::new()))
			.as_object_mut()
			.expect("moz:firefoxOptions wasn't a JSON object")
			.entry("args".to_string())
			.or_insert_with(|| Value::Array(vec![]))
			.as_array_mut()
			.expect("args wasn't a JSON array")
			.extend(vec![Value::String("-headless".to_string())]),
		WebDriver::Safari(_) => (),
	}
	Ok(cap)
}

struct ChildWrapper(Option<Inner>);

struct Inner {
	child: Child,
	stdout: (Sender<()>, Receiver<Vec<u8>>),
	stderr: (Sender<()>, Receiver<Vec<u8>>),
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
		let (stdout_kill_tx, stdout_kill_rx) = oneshot::channel();
		let (stdout_out_tx, stdout_out_rx) = oneshot::channel();
		let mut stdout = child.stdout.take().unwrap();
		tokio::spawn(async move {
			let mut output = Vec::new();
			tokio::select! {
				_ = stdout_kill_rx => (),
				_ = tokio::io::copy(&mut stdout, &mut output) => (),
			}
			let _ = stdout_out_tx.send(output);
		});

		let (stderr_kill_tx, stderr_kill_rx) = oneshot::channel();
		let (stderr_out_tx, stderr_out_rx) = oneshot::channel();
		let mut stderr = child.stderr.take().unwrap();
		tokio::spawn(async move {
			let mut output = Vec::new();
			tokio::select! {
				_ = stderr_kill_rx => (),
				_ = tokio::io::copy(&mut stderr, &mut output) => (),
			}
			let _ = stderr_out_tx.send(output);
		});

		Self(Some(Inner {
			child,
			stdout: (stdout_kill_tx, stdout_out_rx),
			stderr: (stderr_kill_tx, stderr_out_rx),
		}))
	}

	async fn wait(&mut self) -> tokio::io::Result<ExitStatus> {
		self.0.as_mut().unwrap().child.wait().await
	}

	async fn output_error(mut self) {
		let Inner {
			mut child,
			stdout,
			stderr,
		} = self.0.take().unwrap();

		// Wait a moment in hope we can kill the child and get the complete output.
		let _ = tokio::time::timeout(Duration::from_millis(100), child.kill()).await;

		let _ = stdout.0.send(());
		let stdout = stdout.1.await.unwrap();

		if !stdout.is_empty() {
			eprintln!(
				"------ WebDriver stdout ------\n{}",
				String::from_utf8_lossy(&stdout)
			);

			if !stdout.ends_with(b"\n") {
				eprintln!();
			}
		}

		let _ = stderr.0.send(());
		let stderr = stderr.1.await.unwrap();

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
