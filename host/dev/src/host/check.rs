use std::iter;
use std::process::Command;
use std::sync::LazyLock;
use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};
use clap::builder::PossibleValue;
use clap::{Args, ValueEnum};
use strum::{EnumIter, IntoEnumIterator};

use super::{HostTarget, HostTargets, metadata};
use crate::command::{self, RunCommand};

#[derive(Args)]
pub struct Check {
	#[arg(long, value_delimiter = ',', default_value = "clippy")]
	tools: Vec<Tools>,
	#[arg(long, short, default_value = HostTarget::host().to_clap_arg())]
	targets: Vec<HostTargets>,
}

#[derive(Clone, Copy)]
enum Tools {
	All,
	Tool(Tool),
}

#[derive(Clone, Copy, EnumIter)]
enum Tool {
	Clippy,
	Tsc,
	EsLint,
	Zizmor,
}

impl Default for Check {
	fn default() -> Self {
		Self {
			tools: vec![Tools::Tool(Tool::Clippy)],
			targets: vec![HostTargets::Target(HostTarget::host())],
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
						RunCommand {
							title: "Check",
							sub_command: "clippy",
							args: &["--keep-going", "--", "-D", "warnings"],
							envs: &[],
						},
						RunCommand {
							title: "Check Tests",
							sub_command: "clippy",
							args: &[
								"--keep-going",
								"--tests",
								"--benches",
								"--",
								"-D",
								"warnings",
							],
							envs: &[],
						},
						RunCommand {
							title: "Check Doc",
							sub_command: "doc",
							args: &["--keep-going", "--no-deps", "--document-private-items"],
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
				Tool::Zizmor => {
					let start = Instant::now();

					let mut command = Command::new("zizmor");
					command.current_dir("../").arg(".");

					command::run("Zizmor", command, verbose)?;

					duration += start.elapsed();
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

impl ValueEnum for Tools {
	fn value_variants<'a>() -> &'a [Self] {
		static VARIANTS: LazyLock<Vec<Tools>> = LazyLock::new(|| {
			iter::once(Tools::All)
				.chain(Tool::iter().map(Tools::Tool))
				.collect()
		});

		&VARIANTS
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Self::All => Some(PossibleValue::new("all")),
			Self::Tool(tool) => Some(PossibleValue::new(tool.to_arg())),
		}
	}
}

impl Tool {
	fn from_tools(cli: Vec<Tools>) -> Result<Vec<Self>> {
		if let [Tools::All] = cli.as_slice() {
			return Ok(Self::iter().collect());
		}

		cli.into_iter()
			.map(|runner| match runner {
				Tools::All => Err(anyhow!("`--tools`s `all` option conflicts with all others")),
				Tools::Tool(tool) => Ok(tool),
			})
			.collect()
	}

	fn to_arg(self) -> &'static str {
		match self {
			Self::Clippy => "clippy",
			Self::Tsc => "tsc",
			Self::EsLint => "es-lint",
			Self::Zizmor => "zizmor",
		}
	}
}
