use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::{Result, bail};
use clap::{Args, ValueEnum};
use strum::{EnumIter, IntoEnumIterator};

use super::{HostTarget, HostTargets, metadata};
use crate::command;
use crate::command::RunCommand;
use crate::group::Group;

#[derive(Args)]
pub struct Check {
	#[arg(long, value_delimiter = ',', default_value = "clippy")]
	tools: Vec<Tool>,
	#[arg(long, short, default_value = HostTarget::host().to_clap_arg())]
	targets: Vec<HostTargets>,
}

#[derive(Clone, Copy, EnumIter, ValueEnum)]
enum Tool {
	Clippy,
	EsLint,
}

impl Default for Check {
	fn default() -> Self {
		Self {
			tools: vec![Tool::Clippy],
			targets: vec![HostTargets::Target(HostTarget::host())],
		}
	}
}

impl Check {
	pub fn all() -> Self {
		Self {
			tools: Tool::iter().collect(),
			targets: vec![HostTargets::All],
		}
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		let mut duration = Duration::ZERO;

		for tool in self.tools {
			match tool {
				Tool::Clippy => {
					let commands = [
						RunCommand {
							title: "Check",
							sub_command: "clippy",
							args: &["--keep-going", "--", "-D", "warnings"],
							envs: &[],
						},
						RunCommand {
							title: "Doc",
							sub_command: "doc",
							args: &["--keep-going", "--no-deps", "--document-private-items"],
							envs: &[("RUSTDOCFLAGS", "-D warnings")],
						},
					];
					let targets = HostTarget::from_targets(self.targets.clone())?;
					duration += metadata::run(&commands, &targets, verbose)?;
				}
				Tool::EsLint => {
					let start = Instant::now();

					Self::eslint("js-bindgen-ld", "../host/ld/src/js", verbose)?;
					Self::eslint("js-bindgen-runner", "../host/runner/src/js", verbose)?;

					duration += start.elapsed();
				}
			}
		}

		println!("-------------------------");
		println!("Total Time: {:.2}s", duration.as_secs_f32());

		Ok(())
	}

	fn eslint(package: &str, path: &str, verbose: bool) -> Result<()> {
		let group = Group::announce(format!("ESLint `{package}`").into(), verbose)?;

		let mut command = Command::new("npx");
		command.current_dir(path).arg("eslint");

		if verbose {
			command::print_info(&command);
		}

		let (_, status) = command::run(command, verbose)?;

		if !status.success() {
			bail!("ESLint \"`{package}`\" failed with {status}");
		}

		drop(group);

		Ok(())
	}
}
