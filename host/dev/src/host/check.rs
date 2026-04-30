use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::Result;
use clap::{Args, ValueEnum};
use strum::EnumIter;

use super::{HostTarget, HostTargets, metadata};
use crate::command::{self, CargoCommand};

#[derive(Args)]
pub struct Check {
	#[arg(long, value_delimiter = ',', default_value = Tools::default_arg())]
	tools: Vec<Tools>,
	#[arg(long, short, default_value = HostTargets::default_arg())]
	targets: Vec<HostTargets>,
}

enum_with_all!(enum Tools, Tool(Tool), "tools");

#[derive(Clone, Copy, Default, EnumIter, Eq, PartialEq, ValueEnum)]
enum Tool {
	#[default]
	Clippy,
	Tsc,
	EsLint,
	Tombi,
	Zizmor,
}

impl Default for Check {
	fn default() -> Self {
		Self {
			tools: vec![Tools::default()],
			targets: vec![HostTargets::default()],
		}
	}
}

impl Check {
	pub fn all() -> Self {
		Self {
			tools: vec![Tools::All],
			targets: vec![HostTargets::All],
		}
	}

	pub fn targets(&self) -> &[HostTargets] {
		&self.targets
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		let mut duration = Duration::ZERO;
		let tools = Tool::from_tools(self.tools)?;

		for tool in tools {
			match tool {
				Tool::Clippy => {
					let commands = [
						CargoCommand {
							title: "Check",
							sub_command: "clippy",
							args: &["--", "-D", "warnings"],
							envs: &[],
						},
						CargoCommand {
							title: "Check Tests",
							sub_command: "clippy",
							args: &["--tests", "--benches", "--", "-D", "warnings"],
							envs: &[],
						},
						CargoCommand {
							title: "Check Doc",
							sub_command: "doc",
							args: &["--no-deps", "--document-private-items"],
							envs: &[("RUSTDOCFLAGS", "-D warnings")],
						},
					];
					let targets = HostTarget::from_targets(self.targets.clone())?;
					duration += metadata::run(&commands, &targets, true, verbose)?;
				}
				Tool::Tsc => {
					let start = Instant::now();

					Self::tsc("js-bindgen-ld", "ld/src/js", verbose)?;
					Self::tsc("js-bindgen-runner", "runner/src/js", verbose)?;

					duration += start.elapsed();
				}
				Tool::EsLint => {
					let start = Instant::now();

					Self::eslint("js-bindgen-ld", "ld/src/js", verbose)?;
					Self::eslint("js-bindgen-runner", "runner/src/js", verbose)?;

					duration += start.elapsed();
				}
				Tool::Tombi => {
					let mut command = Command::new("tombi");
					command.args(["lint", "--error-on-warnings", "."]);
					duration += command::run("Tombi Lint", command, verbose)?;
				}
				Tool::Zizmor => {
					let mut command = Command::new("zizmor");
					command.current_dir("../").args(["--pedantic", "."]);
					duration += command::run("Zizmor", command, verbose)?;
				}
			}
		}

		println!("-------------------------");
		println!("Total Time: {:.2}s", duration.as_secs_f32());

		Ok(())
	}

	fn tsc(package: &str, path: &str, verbose: bool) -> Result<()> {
		let mut command = Command::new("tsc");
		command.current_dir(path).arg("-b").arg("--noEmit");

		command::run(&format!("TSC `{package}`"), command, verbose)?;

		Ok(())
	}

	fn eslint(package: &str, path: &str, verbose: bool) -> Result<()> {
		let mut command = Command::new("npx");
		command.current_dir(path).arg("eslint");

		command::run(&format!("ESLint `{package}`"), command, verbose)?;

		Ok(())
	}
}
