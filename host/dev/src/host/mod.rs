mod build;
mod check;
mod fmt;
mod metadata;
mod test;

use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use strum::{Display, EnumIter};

use self::build::Build;
use self::check::{Check, Tools};
pub use self::check::{HostTool, Tool};
use self::fmt::Fmt;
use crate::FmtTools;

#[derive(Subcommand)]
pub enum Host {
	All {
		#[arg(long, value_delimiter = ',', default_value = FmtTools::default_arg())]
		fmt_tools: Vec<FmtTools>,
		#[arg(long, value_delimiter = ',', default_value = Tools::default_arg())]
		check_tools: Vec<Tools>,
		#[arg(long, short, default_value = HostTargets::default_arg())]
		targets: Vec<HostTargets>,
	},
	Fmt(Fmt),
	Build(Build),
	Check(Check),
	Test,
}

impl Host {
	pub fn fmt() -> Self {
		Self::Fmt(Fmt::default())
	}

	pub fn build(all: bool) -> Self {
		if all {
			Self::Build(Build::all())
		} else {
			Self::Build(Build::default())
		}
	}

	pub fn check(tools: Vec<Tool>, all: bool) -> Self {
		let targets = if all {
			HostTargets::all()
		} else {
			vec![HostTargets::default()]
		};
		let tools = tools.into_iter().map(Tools::Tool).collect();

		Self::Check(Check::new(tools, targets))
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		match self {
			Self::All {
				fmt_tools,
				check_tools,
				targets,
			} => {
				Self::Fmt(Fmt::new(fmt_tools)).execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Build(Build::new(targets.clone())).execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Check(Check::new(check_tools, targets)).execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Test.execute(verbose)?;

				Ok(())
			}
			Self::Fmt(fmt) => fmt.execute(verbose),
			Self::Build(build) => build.execute(verbose),
			Self::Check(check) => check.execute(verbose),
			Self::Test => test::run(verbose),
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
