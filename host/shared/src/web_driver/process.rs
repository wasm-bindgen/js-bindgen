use std::io;
use std::ops::Deref;
use std::process::ExitStatus;
use std::sync::Arc;

#[cfg(target_family = "unix")]
use nix::sys::signal::{self, Signal};
#[cfg(target_family = "unix")]
use nix::unistd::Pid;
use tokio::process::{Child, Command};
use tokio::runtime::Handle;
use tokio::sync::oneshot::{self, Receiver};
use tokio::task;
#[cfg(target_family = "windows")]
use windows_sys::Win32::Foundation::FALSE;
#[cfg(target_family = "windows")]
use windows_sys::Win32::System::Console::{self, CTRL_BREAK_EVENT};
#[cfg(target_family = "windows")]
use windows_sys::Win32::System::Threading::CREATE_NEW_PROCESS_GROUP;

use super::util::AtomicFlag;

pub struct ChildWrapper {
	child: Child,
	shutdown: bool,
	flag: Arc<AtomicFlag>,
	stdout: Receiver<Vec<u8>>,
	stderr: Receiver<Vec<u8>>,
}

impl ChildWrapper {
	pub fn new(mut command: Command) -> io::Result<Self> {
		#[cfg(target_family = "unix")]
		command.process_group(0);
		#[cfg(target_family = "windows")]
		command.creation_flags(CREATE_NEW_PROCESS_GROUP);

		let mut child = command.spawn()?;
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

		Ok(Self {
			child,
			shutdown: true,
			flag,
			stdout: stdout_rx,
			stderr: stderr_rx,
		})
	}

	pub async fn wait(&mut self) -> io::Result<ExitStatus> {
		self.child.wait().await
	}

	fn ctrl_c(&mut self) -> io::Result<()> {
		let Some(id) = self.child.id() else {
			return Ok(());
		};

		#[cfg(target_family = "unix")]
		signal::killpg(Pid::from_raw(id.try_into().unwrap()), Signal::SIGINT)?;

		#[cfg(target_family = "windows")]
		{
			// SAFETY: Safe.
			let result = unsafe { Console::GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, id) };

			if result == FALSE {
				return Err(io::Error::last_os_error());
			}
		}

		Ok(())
	}

	pub async fn shutdown(mut self) -> io::Result<()> {
		self.shutdown_internal().await
	}

	async fn shutdown_internal(&mut self) -> io::Result<()> {
		let result = match self.ctrl_c() {
			Ok(()) => self.wait().await,
			Err(error) => {
				let _ = self.child.try_wait();
				Err(error)
			}
		};
		self.shutdown = false;

		result.map(|_| ())
	}

	pub async fn output_error(mut self, kill: bool) {
		self.shutdown = false;

		let error = if kill {
			self.child.kill().await
		} else {
			self.ctrl_c()
		};

		let status = if let Err(error) = error {
			eprintln!("------ WebDriver Process End Error ------\n{error}\n");
			self.child.try_wait()
		} else {
			self.child.wait().await.map(Some)
		};

		match status {
			Ok(Some(status)) => {
				eprintln!("------ WebDriver Process Status ------\n{status}\n");
			}
			Ok(None) => (),
			Err(error) => {
				eprintln!("------ WebDriver Process Status Error ------\n{error}\n");
			}
		}

		self.flag.signal();

		let stdout = (&mut self.stdout).await.unwrap();

		if !stdout.is_empty() {
			eprintln!(
				"------ WebDriver stdout ------\n{}",
				String::from_utf8_lossy(&stdout)
			);

			if !stdout.ends_with(b"\n") {
				eprintln!();
			}
		}

		let stderr = (&mut self.stderr).await.unwrap();

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

impl Drop for ChildWrapper {
	fn drop(&mut self) {
		if self.shutdown {
			task::block_in_place(move || {
				Handle::current().block_on(async move {
					let _ = self.shutdown_internal().await;
				});
			});
		}
	}
}
