use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};
use std::{env, thread};

use anyhow::{Context, Result, bail};
use serde_json::{Map, Value as Json};
use url::Url;

pub struct DriverGuard {
	pub url: Url,
	child: Option<Child>,
}

impl Drop for DriverGuard {
	fn drop(&mut self) {
		if let Some(mut child) = self.child.take() {
			let _ = child.kill();
		}
	}
}

pub fn launch_driver(driver: &Driver) -> Result<DriverGuard> {
	let guard = match driver.location() {
		Locate::Remote(url) => DriverGuard {
			url: url.clone(),
			child: None,
		},
		Locate::Local((path, args)) => {
			// Wait for the driver to come online and bind its port before we try to
			// connect to it.
			let start = Instant::now();
			let max = Duration::new(5, 0);

			let (driver_addr, child) = 'outer: loop {
				// Allow tests to run in parallel (in theory) by finding any open port
				// available for our driver. We can't bind the port for the driver, but
				// hopefully the OS gives this invocation unique ports across processes
				let driver_addr = TcpListener::bind("127.0.0.1:0")?.local_addr()?;
				// Spawn the driver binary, collecting its stdout/stderr in separate
				// threads. We'll print this output later.
				let mut cmd = Command::new(path);
				cmd.stdout(Stdio::null())
					.stderr(Stdio::null())
					.args(args)
					.arg(format!("--port={}", driver_addr.port()));
				let mut child = cmd.spawn()?;

				// Wait for the driver to come online and bind its port before we try to
				// connect to it.
				loop {
					if match child.try_wait() {
						Ok(Some(status)) => !status.success(),
						Ok(None) => false,
						Err(_) => true,
					} {
						if start.elapsed() >= max {
							bail!("failed to start driver")
						}

						println!("failed to start driver, trying again ...");

						thread::sleep(Duration::from_millis(100));
						break;
					} else if TcpStream::connect(driver_addr).is_ok() {
						break 'outer (driver_addr, child);
					} else if start.elapsed() >= max {
						bail!("failed to bind driver port during startup")
					} else {
						thread::sleep(Duration::from_millis(100));
					}
				}
			};
			DriverGuard {
				url: Url::parse(&format!("http://{driver_addr}"))?,
				child: Some(child),
			}
		}
	};
	Ok(guard)
}

/// Options that can use to customize and configure a WebDriver session.
pub type Capabilities = Map<String, Json>;

pub fn capabilities(driver: &Driver, mut cap: Capabilities) -> Result<Capabilities> {
	match driver {
		Driver::Gecko(_) => cap
			.entry("moz:firefoxOptions".to_string())
			.or_insert_with(|| Json::Object(serde_json::Map::new()))
			.as_object_mut()
			.expect("moz:firefoxOptions wasn't a JSON object")
			.entry("args".to_string())
			.or_insert_with(|| Json::Array(vec![]))
			.as_array_mut()
			.expect("args wasn't a JSON array")
			.extend(vec![Json::String("-headless".to_string())]),
		Driver::Safari(_) => (),
		Driver::Chrome(_) => {
			cap.entry("goog:chromeOptions".to_string())
				.or_insert_with(|| Json::Object(serde_json::Map::new()))
				.as_object_mut()
				.expect("goog:chromeOptions wasn't a JSON object")
				.entry("args".to_string())
				.or_insert_with(|| Json::Array(vec![]))
				.as_array_mut()
				.expect("args wasn't a JSON array")
				.extend(vec![
					Json::String("headless".to_string()),
					// See https://stackoverflow.com/questions/50642308/
					// for what this funky `disable-dev-shm-usage`
					// option is
					Json::String("disable-dev-shm-usage".to_string()),
					Json::String("no-sandbox".to_string()),
				])
		}
		Driver::Edge(_) => {
			cap.entry("ms:edgeOptions".to_string())
				.or_insert_with(|| Json::Object(serde_json::Map::new()))
				.as_object_mut()
				.expect("ms:edgeOptions wasn't a JSON object")
				.entry("args".to_string())
				.or_insert_with(|| Json::Array(vec![]))
				.as_array_mut()
				.expect("args wasn't a JSON array")
				.extend(vec![
					Json::String("headless".to_string()),
					// See https://stackoverflow.com/questions/50642308/
					// for what this funky `disable-dev-shm-usage`
					// option is
					Json::String("disable-dev-shm-usage".to_string()),
					Json::String("no-sandbox".to_string()),
				])
		}
	}
	Ok(cap)
}

#[derive(Debug)]
pub enum Locate {
	Local((PathBuf, Vec<String>)),
	Remote(Url),
}

#[derive(Debug)]
pub enum Driver {
	Gecko(Locate),
	Safari(Locate),
	Chrome(Locate),
	Edge(Locate),
}

impl Driver {
	/// Attempts to find an appropriate remote WebDriver server or server binary
	/// to execute tests with. Performs a number of heuristics to find one
	/// available, including:
	///
	/// * Env vars like `JBG_TEST_GECKODRIVER_REMOTE` address of remote webdriver.
	/// * Env vars like `JBG_TEST_GECKODRIVER` point to the path to a binary to execute.
	/// * Otherwise, `PATH` is searched for an appropriate binary.
	///
	/// In the last two cases a list of auxiliary arguments is also returned
	/// which is configured through env vars like `JBG_TEST_GECKODRIVER_ARGS` to support
	/// extra arguments to the driver's invocation.
	pub fn find() -> Result<Self> {
		let env_args = |name: &str| {
			let var =
				env::var(format!("JBG_TEST_{}_ARGS", name.to_uppercase())).unwrap_or_default();

			shlex::split(&var)
				.unwrap_or_else(|| var.split_whitespace().map(|s| s.to_string()).collect())
		};

		let drivers = [
			("geckodriver", Driver::Gecko as fn(Locate) -> Self),
			("safaridriver", Driver::Safari as fn(Locate) -> Self),
			("chromedriver", Driver::Chrome as fn(Locate) -> Self),
			("msedgedriver", Driver::Edge as fn(Locate) -> Self),
		];

		// First up, if env vars like JBG_TEST_GECKODRIVER_REMOTE are present, use those
		// to allow forcing usage of a particular remote driver.
		for (driver, ctor) in drivers.iter() {
			let env = format!("JBG_TEST_{}_REMOTE", driver.to_uppercase());
			let url = match env::var(&env) {
				Ok(var) => Url::parse(&var).context(format!("failed to parse `{env}`"))?,
				Err(_) => continue,
			};
			return Ok(ctor(Locate::Remote(url)));
		}

		// Next, if env vars like JBG_TEST_GECKODRIVER are present, use those to
		// allow forcing usage of a particular local driver.
		for (driver, ctor) in drivers.iter() {
			let env = driver.to_uppercase();
			let path = match env::var_os(format!("JBG_TEST_{env}")) {
				Some(path) => path,
				None => continue,
			};
			return Ok(ctor(Locate::Local((path.into(), env_args(driver)))));
		}

		// Next, check PATH. If we can find any supported driver, use that by
		// default.
		for path in env::split_paths(&env::var_os("PATH").unwrap_or_default()) {
			let found = drivers.iter().find(|(name, _)| {
				path.join(name)
					.with_extension(env::consts::EXE_EXTENSION)
					.exists()
			});
			let (driver, ctor) = match found {
				Some(p) => p,
				None => continue,
			};
			return Ok(ctor(Locate::Local((driver.into(), env_args(driver)))));
		}

		bail!(
			"\
failed to find a suitable WebDriver binary or remote running WebDriver to drive
headless \
			 testing; to configure the location of the webdriver binary you can use
environment variables \
			 like `GECKODRIVER=/path/to/geckodriver` or make sure that
the binary is in `PATH`; to configure \
			 the address of remote webdriver you can
use environment variables like `GECKODRIVER_REMOTE=http://remote.host/`This \
			 crate currently supports `geckodriver`, `chromedriver`, `safaridriver`, and
`msedgedriver`, \
			 although more driver support may be added! You can download these at:

    * geckodriver \
			 - https://github.com/mozilla/geckodriver/releases
    * chromedriver - https://chromedriver.chromium.org/downloads* \
			 msedgedriver - \
			 https://developer.microsoft.com/en-us/microsoft-edge/tools/webdriver/* safaridriver \
			 - should be preinstalled on OSX

If you're still having difficulty resolving this error, please feel free to open
an issue against wasm-bindgen/js-bindgen!
    "
		)
	}

	pub fn location(&self) -> &Locate {
		match self {
			Self::Gecko(locate) => locate,
			Self::Safari(locate) => locate,
			Self::Chrome(locate) => locate,
			Self::Edge(locate) => locate,
		}
	}
}
