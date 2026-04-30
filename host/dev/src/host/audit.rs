use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use anyhow::Result;
use clap::{Args, ValueEnum};
use strum::EnumIter;

use crate::command;

#[derive(Args)]
pub struct Audit {
	#[arg(long, value_delimiter = ',', default_value = AuditTools::default_arg())]
	tools: Vec<AuditTools>,
}

enum_with_all!(pub enum AuditTools, Tool(AuditTool), "tools");

#[derive(Clone, Copy, Default, EnumIter, Eq, PartialEq, ValueEnum)]
pub enum AuditTool {
	#[default]
	RustSec,
	Npm,
}

impl Audit {
	pub fn new(tools: Vec<AuditTools>) -> Self {
		Self { tools }
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		let tools = AuditTool::from_tools(self.tools)?;
		let mut duration = Duration::ZERO;

		for tool in tools {
			match tool {
				AuditTool::RustSec => {
					let mut command = Command::new("cargo");
					command.arg("audit");
					duration += command::run("RustSec", command, verbose)?;
				}
				AuditTool::Npm => {
					duration += Self::npm("js-bindgen-ld", "ld/src/js", verbose)?;
					duration += Self::npm("js-bindgen-runner", "runner/src/js", verbose)?;
				}
			}
		}

		println!("-------------------------");
		println!("Total Time: {:.2}s", duration.as_secs_f32());

		Ok(())
	}

	fn npm(package: &str, path: &str, verbose: bool) -> Result<Duration> {
		let needs_install = match fs::metadata(Path::new(path).join("package-lock.json")) {
			Ok(meta) => {
				let lock_mtime = meta.modified()?;
				let pkg_mtime = fs::metadata(Path::new(path).join("package.json"))?.modified()?;

				lock_mtime < pkg_mtime
			}
			Err(error) if error.kind() == ErrorKind::NotFound => true,
			Err(error) => return Err(error.into()),
		};

		if needs_install {
			let mut command = Command::new("npm");
			command
				.current_dir(path)
				.arg("install")
				.arg("--package-lock-only")
				.arg("--no-audit")
				.arg("--no-fund");

			command::run(&format!("NPM Install `{package}`"), command, verbose)?;
		}

		let mut command = Command::new("npm");
		command.current_dir(path).arg("audit");
		command::run(&format!("NPM Audit `{package}`"), command, verbose)
	}
}
