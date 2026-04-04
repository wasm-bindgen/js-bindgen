use std::borrow::Cow;
use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::io::ErrorKind;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::ops::Deref;
use std::path::Path;
use std::pin::Pin;
use std::process::{ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::{fmt, io};

use anyhow::{Result, bail};
use futures_util::task::AtomicWaker;
use strum::VariantArray;
use tokio::net::{TcpListener, TcpStream};
use tokio::process::{Child, Command};
use tokio::sync::oneshot::{self, Receiver};
use tokio::time;
use url::Url;

pub struct WebDriver {
	url: Url,
	child: Option<ChildWrapper>,
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
			let child = Command::new(path)
				.stdout(Stdio::piped())
				.stderr(Stdio::piped())
				.arg(format!("--port={}", driver_addr.port()))
				.args(args)
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
					_ = child.wait() => {
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
			child: Some(child),
		})
	}

	#[must_use]
	pub fn url(&self) -> &Url {
		&self.url
	}

	#[must_use]
	pub fn is_remote(&self) -> bool {
		self.child.is_none()
	}

	pub async fn output_error(self) {
		if let Some(child) = self.child {
			child.output_error().await;
		}
	}

	pub async fn shutdown(self) -> io::Result<()> {
		if let Some(mut child) = self.child {
			child.0.take().unwrap().child.kill().await
		} else {
			Ok(())
		}
	}
}

struct ChildWrapper(Option<ChildInner>);

struct ChildInner {
	child: Child,
	flag: Arc<AtomicFlag>,
	stdout: Receiver<Vec<u8>>,
	stderr: Receiver<Vec<u8>>,
}

impl Drop for ChildWrapper {
	fn drop(&mut self) {
		if let Some(mut inner) = self.0.take()
			&& let Err(error) = inner.child.start_kill()
		{
			eprintln!("------ WebDriver Process Kill Error ------\n{error}\n");
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
					() = flag.deref() => (),
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
					() = flag.deref() => (),
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

		if let Err(error) = child.kill().await {
			eprintln!("------ WebDriver Process Error ------\n{error}\n");
		}

		match child.try_wait() {
			Ok(Some(status)) => {
				eprintln!("------ WebDriver Process Status ------\n{status}\n");
			}
			Ok(None) => (),
			Err(error) => {
				eprintln!("------ WebDriver Process Status Error ------\n{error}\n");
			}
		}

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

#[derive(Clone, Copy, VariantArray)]
pub enum WebDriverKind {
	Chrome,
	Edge,
	Gecko,
	Safari,
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

#[derive(Default)]
pub struct AtomicFlag {
	waker: AtomicWaker,
	set: AtomicBool,
}

impl AtomicFlag {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	pub fn signal(&self) {
		self.set.store(true, Ordering::Relaxed);
		self.waker.wake();
	}
}

impl Future for &AtomicFlag {
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		// Short-circuit.
		if self.set.load(Ordering::Relaxed) {
			return Poll::Ready(());
		}

		self.waker.register(cx.waker());

		// Need to check condition **after** `register()` to avoid a race condition that
		// would result in lost notifications.
		if self.set.load(Ordering::Relaxed) {
			Poll::Ready(())
		} else {
			Poll::Pending
		}
	}
}
