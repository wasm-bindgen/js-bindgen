use std::env;
use std::iter::Copied;
use std::process::Command;
use std::slice::Iter;
use std::sync::LazyLock;
use std::time::Instant;

use anyhow::Result;
use clap::builder::PossibleValue;
use clap::{Args, ValueEnum};
use strum::{EnumIter, IntoEnumIterator};

use super::{ClientArgs, metadata};
use crate::check::CheckTool;
use crate::command::{self, CargoCommand};

#[derive(Args)]
pub struct Check {
	#[command(flatten)]
	args: ClientArgs,
	#[arg(long, value_delimiter = ',', default_value = Tools::default_arg())]
	tools: Vec<Tools>,
}

enum_with_all!(pub enum Tools, Tool(Tool), "tools");

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Tool {
	Shared(CheckTool),
	Client(ClientTool),
}

#[derive(Clone, Copy, EnumIter, Eq, PartialEq, ValueEnum)]
pub enum ClientTool {
	CargoJsSys,
}

impl Default for Check {
	fn default() -> Self {
		Self {
			args: ClientArgs::default(),
			tools: vec![Tools::default()],
		}
	}
}

impl Check {
	pub fn new(args: ClientArgs, tools: Vec<Tools>) -> Self {
		Self { args, tools }
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		let tools = Tool::from_tools(self.tools)?;
		let start = Instant::now();

		for tool in tools {
			match tool {
				Tool::Shared(CheckTool::Clippy) => {
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
							args: &["--tests", "--benches", "--examples", "--", "-D", "warnings"],
							envs: &[],
						},
						CargoCommand {
							title: "Check Doc",
							sub_command: "doc",
							args: &["--no-deps", "--document-private-items"],
							envs: &[("RUSTDOCFLAGS", "-D warnings")],
						},
					];
					metadata::run(self.args.clone(), &commands, false, verbose)?;
				}
				Tool::Client(ClientTool::CargoJsSys) => {
					let tools_installed =
						env::var_os("JBG_DEV_TOOLS").is_some_and(|value| value == "1");

					if !tools_installed {
						let mut command = Command::new("cargo");
						command.arg("build").args(["-p", "cargo-js-sys"]);

						command::run("Build `cargo-js-sys`", command, verbose)?;
					}

					Self::cargo_js_sys("js-sys", tools_installed, verbose)?;
					Self::cargo_js_sys("web-sys", tools_installed, verbose)?;
				}
				Tool::Shared(CheckTool::RustSec) => {
					let mut command = Command::new("cargo");
					command.current_dir("../client").arg("audit");
					command::run("RustSec", command, verbose)?;
				}
				Tool::Shared(CheckTool::Tombi) => {
					let mut command = Command::new("tombi");
					command
						.current_dir("../client")
						.args(["lint", "--error-on-warnings", "."]);
					command::run("Tombi Lint", command, verbose)?;
				}
				Tool::Shared(CheckTool::CargoSpellcheck) => {
					let mut command = Command::new("cargo");
					command
						.current_dir("../client")
						.args(["spellcheck", "-m", "1"]);
					command::run("`cargo-spellcheck`", command, verbose)?;
				}
				Tool::Shared(CheckTool::Typos) => {
					let mut command = Command::new("typos");
					command.current_dir("../client");
					command::run("Typos", command, verbose)?;
				}
			}
		}

		println!("-------------------------");
		println!("Total Time: {:.2}s", start.elapsed().as_secs_f32());

		Ok(())
	}

	fn cargo_js_sys(pkg: &str, tools_installed: bool, verbose: bool) -> Result<()> {
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

		command::run(&format!("Check `cargo-js-sys` - `{pkg}`"), command, verbose)?;

		Ok(())
	}
}

impl Default for Tool {
	fn default() -> Self {
		Self::Shared(CheckTool::default())
	}
}

impl IntoEnumIterator for Tool {
	type Iterator = Copied<Iter<'static, Self>>;

	fn iter() -> Self::Iterator {
		Self::value_variants().iter().copied()
	}
}

impl ValueEnum for Tool {
	fn value_variants<'a>() -> &'a [Self] {
		static VALUES: LazyLock<Vec<Tool>> = LazyLock::new(|| {
			CheckTool::iter()
				.map(Tool::Shared)
				.chain(ClientTool::iter().map(Tool::Client))
				.collect()
		});

		&VALUES
	}

	fn to_possible_value(&self) -> Option<PossibleValue> {
		match self {
			Self::Shared(tool) => tool.to_possible_value(),
			Self::Client(tool) => tool.to_possible_value(),
		}
	}
}
