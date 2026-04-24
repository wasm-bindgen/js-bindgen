mod check;
mod metadata;
mod test;

use std::iter;
use std::sync::LazyLock;

use anyhow::{Result, anyhow};
use clap::builder::PossibleValue;
use clap::{Subcommand, ValueEnum};
use strum::{Display, EnumIter, IntoEnumIterator};

use self::check::Check;
use crate::command::RunCommand;

#[derive(Subcommand)]
pub enum Host {
	All(Check),
	Build,
	Check(Check),
	Test,
}

impl Host {
	pub fn check(all: bool) -> Self {
		if all {
			Self::Check(Check::all())
		} else {
			Self::Check(Check::default())
		}
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		match self {
			Self::All(check) => {
				Self::Build.execute(verbose)?;
				println!("-------------------------");
				println!();
				check.execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Test.execute(verbose)?;

				Ok(())
			}
			Self::Build => {
				let command = RunCommand {
					title: "Build",
					sub_command: "build",
					args: &[],
					envs: &[],
				};
				let duration = metadata::run(&[command], &[HostTarget::host()], false, verbose)?;

				println!("-------------------------");
				println!("Total Time: {:.2}s", duration.as_secs_f32());

				Ok(())
			}
			Self::Check(check) => check.execute(verbose),
			Self::Test => test::run(verbose),
		}
	}
}

#[derive(Clone, Copy)]
pub enum HostTargets {
	All,
	Target(HostTarget),
}

#[derive(Clone, Copy, Display, EnumIter, Eq, PartialEq)]
pub enum HostTarget {
	Linux,
	MacOs,
	Windows,
}

impl ValueEnum for HostTargets {
	fn value_variants<'a>() -> &'a [Self] {
		static VARIANTS: LazyLock<Vec<HostTargets>> = LazyLock::new(|| {
			iter::once(HostTargets::All)
				.chain(HostTarget::iter().map(HostTargets::Target))
				.collect()
		});

		&VARIANTS
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Self::All => Some(PossibleValue::new("all")),
			Self::Target(target) => Some(PossibleValue::new(target.to_clap_arg())),
		}
	}
}

impl HostTarget {
	fn from_targets(cli: Vec<HostTargets>) -> Result<Vec<Self>> {
		if let [HostTargets::All] = cli.as_slice() {
			return Ok(Self::iter().collect());
		}

		cli.into_iter()
			.map(|runner| match runner {
				HostTargets::All => Err(anyhow!(
					"`--targets`s `all` option conflicts with all others"
				)),
				HostTargets::Target(target) => Ok(target),
			})
			.collect()
	}

	fn to_clap_arg(self) -> &'static str {
		match self {
			Self::Linux => "linux",
			Self::MacOs => "mac-os",
			Self::Windows => "windows",
		}
	}

	fn to_cargo_arg(self) -> &'static str {
		match self {
			Self::Linux => "x86_64-unknown-linux-gnu",
			Self::MacOs => "aarch64-apple-darwin",
			Self::Windows => "x86_64-pc-windows-msvc",
		}
	}

	fn is_host(self) -> bool {
		self == Self::host()
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
