use std::io;
use std::io::ErrorKind;
use std::ops::Deref;
use std::process::{ExitStatus, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Result, bail};
use tokio::net::{TcpListener, TcpStream};
use tokio::process::{Child, Command};
use tokio::sync::oneshot::{self, Receiver};
use tokio::time;
use url::Url;

use crate::config::WebDriverLocation;
use crate::util::AtomicFlag;

pub struct WebDriver {
	pub url: Url,
	child: Option<ChildWrapper>,
}

impl WebDriver {
	pub async fn run(location: WebDriverLocation) -> Result<Self> {
		match location {
			WebDriverLocation::Remote(url) => Ok(Self { url, child: None }),
			WebDriverLocation::Local { path, args } => {
				// Wait for the WebDriver to come online and bind its port before we try to
				// connect to it.
				const MAX: Duration = Duration::from_secs(5);
				let start = Instant::now();

				let (driver_addr, child) = 'outer: loop {
					// Get a random open port to allow test runners to run in parallel.
					let driver_addr = TcpListener::bind("127.0.0.1:0").await?.local_addr()?;
					let child = Command::new(path.deref())
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
					child: Some(child),
				})
			}
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

		if let Err(error) = child.kill().await {
			eprintln!("------ WebDriver Process Error ------\n{error}\n");
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
