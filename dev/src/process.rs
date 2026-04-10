use std::io;
use std::process::{Child, Command};

pub struct ChildWrapper {
	child: Child,
	shutdown: bool,
}

impl ChildWrapper {
	pub fn new(mut command: Command) -> io::Result<Self> {
		Ok(Self {
			child: command.spawn()?,
			shutdown: true,
		})
	}

	pub fn shutdown(mut self) -> io::Result<()> {
		self.shutdown_internal()
	}

	#[cfg(target_family = "unix")]
	fn shutdown_internal(&mut self) -> io::Result<()> {
		use nix::sys::signal::{self, Signal};
		use nix::unistd::Pid;

		self.shutdown = false;

		let result = signal::kill(
			Pid::from_raw(self.child.id().try_into().unwrap()),
			Signal::SIGINT,
		);

		if let Err(error) = result {
			let _ = self.child.try_wait();
			return Err(error.into());
		}

		self.child.wait()?;
		Ok(())
	}

	#[cfg(target_family = "windows")]
	fn shutdown_internal(&mut self) -> io::Result<()> {
		use windows_sys::Win32::Foundation::FALSE;
		use windows_sys::Win32::System::Console::{self, CTRL_C_EVENT};

		self.shutdown = false;

		// SAFETY: Safe.
		let result = unsafe { Console::GenerateConsoleCtrlEvent(CTRL_C_EVENT, self.child.id()) };

		if result == FALSE {
			return Err(io::Error::last_os_error());
		}

		self.child.wait()?;

		Ok(())
	}
}

impl Drop for ChildWrapper {
	fn drop(&mut self) {
		if self.shutdown {
			let _ = self.shutdown_internal();
		}
	}
}
