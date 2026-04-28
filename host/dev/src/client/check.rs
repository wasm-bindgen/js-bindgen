use std::env;
use std::process::Command;
use std::time::Duration;

use anyhow::Result;
use clap::{Args, ValueEnum};
use strum::{EnumIter, IntoEnumIterator};

use super::{ClientArgs, metadata};
use crate::command::{self, CargoCommand};

#[derive(Args)]
pub struct Check {
	#[command(flatten)]
	args: ClientArgs,
	#[arg(long, value_delimiter = ',', default_value = "clippy")]
	tools: Vec<Tool>,
}

#[derive(Clone, Copy, EnumIter, ValueEnum)]
pub enum Tool {
	Clippy,
	CargoJsSys,
}

impl Default for Check {
	fn default() -> Self {
		Self {
			args: ClientArgs::default(),
			tools: vec![Tool::Clippy],
		}
	}
}

impl Check {
	pub fn new(args: ClientArgs, tools: Vec<Tool>) -> Self {
		Self { args, tools }
	}

	pub fn all() -> Self {
		Self {
			args: ClientArgs::all(),
			tools: Tool::iter().collect(),
		}
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		let mut duration = Duration::ZERO;

		for tool in self.tools {
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
							title: "Doc",
							sub_command: "doc",
							args: &["--no-deps", "--document-private-items"],
							envs: &[("RUSTDOCFLAGS", "-D warnings")],
						},
					];
					duration += metadata::run(&self.args, &commands, verbose)?;
				}
				Tool::CargoJsSys => {
					let tools_installed =
						env::var_os("JBG_DEV_TOOLS").is_some_and(|value| value == "1");

					if !tools_installed {
						let mut command = Command::new("cargo");
						command.arg("build").args(["-p", "cargo-js-sys"]);

						command::run("Build `cargo-js-sys`", command, verbose)?;
					}

					duration += Self::cargo_js_sys("js-sys", tools_installed, verbose)?;
					duration += Self::cargo_js_sys("web-sys", tools_installed, verbose)?;
				}
			}
		}

		println!("-------------------------");
		println!("Total Time: {:.2}s", duration.as_secs_f32());

		Ok(())
	}

	fn cargo_js_sys(pkg: &str, tools_installed: bool, verbose: bool) -> Result<Duration> {
		let mut command = if tools_installed {
			Command::new("cargo-js-sys")
		} else {
			let mut command = Command::new("cargo");
			command.arg("run").args(["-p", "cargo-js-sys"]).arg("--");
			command
		};

		command
			.arg("js-sys")
			.args(["--manifest-path", &format!("../client/{pkg}/Cargo.toml")])
			.arg("-c");

		if verbose {
			command.arg("-v");
		}

		command::run(&format!("Check `cargo-js-sys` - `{pkg}`"), command, verbose)
	}
}
