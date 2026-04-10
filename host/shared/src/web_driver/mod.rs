mod process;
mod util;

use std::borrow::Cow;
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::process::Stdio;
use std::time::{Duration, Instant};
use std::{fmt, io};

use anyhow::{Result, bail};
use strum::VariantArray;
use tokio::net::{TcpListener, TcpStream};
use tokio::process::Command;
use tokio::time;
use url::Url;

use self::process::ChildWrapper;
pub use self::util::AtomicFlag;

pub struct WebDriver {
	url: Url,
	child: Option<ChildWrapper>,
}

#[derive(Clone, Copy, VariantArray)]
pub enum WebDriverKind {
	Chrome,
	Edge,
	Gecko,
	Safari,
}

pub enum WebDriverLocation {
	Local {
		path: Cow<'static, Path>,
		args: Vec<OsString>,
	},
	Remote(Url),
}

impl WebDriver {
	pub async fn run(location: WebDriverLocation) -> Result<Self> {
		match location {
			WebDriverLocation::Local { path, args } => Self::run_local(&path, &args).await,
			WebDriverLocation::Remote(url) => Ok(Self { url, child: None }),
		}
	}

	pub async fn run_local(path: &Path, args: &[OsString]) -> Result<Self> {
		// Wait for the WebDriver to come online and bind its port before we try to
		// connect to it.
		const MAX: Duration = Duration::from_secs(5);
		let start = Instant::now();

		let (driver_addr, child) = 'outer: loop {
			// Get a random open port to allow test runners to run in parallel.
			let driver_addr = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
				.await?
				.local_addr()?;
			let mut command = Command::new(path);
			command
				.stdout(Stdio::piped())
				.stderr(Stdio::piped())
				.arg(format!("--port={}", driver_addr.port()))
				.args(args);
			let mut child = ChildWrapper::new(command)?;

			loop {
				let limit = time::sleep(MAX.saturating_sub(start.elapsed()));

				tokio::select! {
					() = limit => {
						child.output_error(true).await;
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
					_ = child.wait() => {
						child.output_error(true).await;

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
			child: Some(child),
		})
	}

	#[must_use]
	pub fn url(&self) -> &Url {
		&self.url
	}

	pub async fn output_error(self) {
		if let Some(child) = self.child {
			child.output_error(false).await;
		}
	}

	pub async fn shutdown(self) -> io::Result<()> {
		if let Some(child) = self.child {
			child.shutdown().await
		} else {
			Ok(())
		}
	}
}

impl WebDriverKind {
	#[must_use]
	pub fn to_name(self) -> &'static str {
		match self {
			Self::Chrome => "chrome-driver",
			Self::Edge => "edge-driver",
			Self::Gecko => "gecko-driver",
			Self::Safari => "safari-driver",
		}
	}

	#[must_use]
	pub fn to_env(self) -> &'static str {
		match self {
			Self::Chrome => "CHROME_DRIVER",
			Self::Edge => "EDGE_DRIVER",
			Self::Gecko => "GECKO_DRIVER",
			Self::Safari => "SAFARI_DRIVER",
		}
	}

	#[must_use]
	pub fn to_binary(self) -> &'static str {
		match self {
			Self::Chrome => "chromedriver",
			Self::Edge => "msedgedriver",
			Self::Gecko => "geckodriver",
			Self::Safari => "safaridriver",
		}
	}

	#[must_use]
	pub fn to_download_url(self) -> Option<&'static str> {
		match self {
			Self::Chrome => Some("https://googlechromelabs.github.io/chrome-for-testing/"),
			Self::Edge => Some("https://developer.microsoft.com/microsoft-edge/tools/webdriver/"),
			Self::Gecko => Some("https://github.com/mozilla/geckodriver/releases/"),
			Self::Safari => None,
		}
	}

	#[must_use]
	pub fn multi_session_support(self) -> bool {
		match self {
			Self::Chrome | Self::Edge | Self::Safari => true,
			Self::Gecko => false,
		}
	}

	#[must_use]
	pub fn search_error() -> String {
		format!(
			"to configure the location of a WebDriver binary you can use environment variables \
			like `WBG_TEST_<WebDriver>_PATH=/path/to/<WebDriver>` or make sure that the binary is in `PATH`; \
			to configure the address of a remote WebDriver you can use environment variables \
			like `WBG_TEST_<WebDriver>_REMOTE=http://remote.host/`; \
			you can download supported drivers at:\n\
			* {} - {}\n\
			* {} - {}\n\
			* {} - {}\n\
			* {} - pre-installed on macOS",
			Self::Chrome, Self::Chrome.to_download_url().unwrap(),
			Self::Gecko, Self::Gecko.to_download_url().unwrap(),
			Self::Edge, Self::Edge.to_download_url().unwrap(),
			Self::Safari,
		)
	}
}

impl Display for WebDriverKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let name = match self {
			Self::Chrome => "ChromeDriver",
			Self::Edge => "EdgeDriver",
			Self::Gecko => "GeckoDriver",
			Self::Safari => "SafariDriver",
		};
		f.write_str(name)
	}
}
