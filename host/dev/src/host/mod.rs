mod build;
mod check;
mod metadata;
mod test;

use std::process::Command;
use std::time::Duration;

use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use strum::{Display, EnumIter};

use self::build::Build;
use self::check::Check;
use crate::{FmtTool, FmtTools, command};

#[derive(Subcommand)]
pub enum Host {
	All {
		#[arg(long, value_delimiter = ',', default_value = FmtTools::default_arg())]
		fmt_tools: Vec<FmtTools>,
		#[command(flatten)]
		check: Check,
	},
	Fmt {
		#[arg(long, value_delimiter = ',', default_value = FmtTools::default_arg())]
		tools: Vec<FmtTools>,
	},
	Build(Build),
	Check(Check),
	Test,
	Audit,
}

impl Host {
	pub fn fmt() -> Self {
		Self::Fmt {
			tools: vec![FmtTools::default()],
		}
	}

	pub fn build(all: bool) -> Self {
		if all {
			Self::Build(Build::all())
		} else {
			Self::Build(Build::default())
		}
	}

	pub fn check(all: bool) -> Self {
		if all {
			Self::Check(Check::all())
		} else {
			Self::Check(Check::default())
		}
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		match self {
			Self::All { fmt_tools, check } => {
				Self::Fmt { tools: fmt_tools }.execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Build(Build::new(check.targets().to_owned())).execute(verbose)?;
				println!("-------------------------");
				println!();
				check.execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Test.execute(verbose)?;

				Ok(())
			}
			Self::Fmt { tools } => {
				let tools = FmtTool::from_tools(tools)?;
				let mut duration = Duration::ZERO;

				for tool in tools {
					match tool {
						FmtTool::Rustfmt => {
							let mut command = Command::new("cargo");
							command.args(["+nightly", "fmt"]);
							duration += command::run("Rustfmt", command, verbose)?;
						}
						FmtTool::Tombi => {
							let mut command = Command::new("tombi");
							command.args(["fmt", "."]);
							duration += command::run("Tombi Fmt", command, verbose)?;
						}
						FmtTool::Prettier => {
							let mut command = Command::new("prettier");
							command.current_dir("..").args(["host", "-w"]);
							duration += command::run("Prettier", command, verbose)?;
						}
					}
				}

				println!("-------------------------");
				println!("Total Time: {:.2}s", duration.as_secs_f32());

				Ok(())
			}
			Self::Build(build) => build.execute(verbose),
			Self::Check(check) => check.execute(verbose),
			Self::Test => test::run(verbose),
			Self::Audit => {
				let mut command = Command::new("cargo");
				command.arg("audit");
				let duration = command::run("RustSec", command, verbose)?;

				println!("-------------------------");
				println!("Total Time: {:.2}s", duration.as_secs_f32());

				Ok(())
			}
		}
	}
}

enum_with_all!(pub enum HostTargets, Target(HostTarget), "targets");

#[derive(Clone, Copy, Default, Display, EnumIter, Eq, PartialEq, ValueEnum)]
pub enum HostTarget {
	#[cfg_attr(target_os = "linux", default)]
	Linux,
	#[cfg_attr(target_os = "macos", default)]
	MacOs,
	#[cfg_attr(target_os = "windows", default)]
	Windows,
}

impl HostTarget {
	fn to_cargo_arg(self) -> &'static str {
		match self {
			Self::Linux => "x86_64-unknown-linux-gnu",
			Self::MacOs => "aarch64-apple-darwin",
			Self::Windows => "x86_64-pc-windows-msvc",
		}
	}

	fn is_host(self) -> bool {
		self == Self::default()
	}

	fn host() -> Self {
		#[cfg(target_os = "linux")]
		return Self::Linux;
		#[cfg(target_os = "macos")]
		return Self::MacOs;
		#[cfg(target_os = "windows")]
		return Self::Windows;
	}
}
