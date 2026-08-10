use std::process::Command;
use std::time::Instant;

use anyhow::Result;
use clap::Args;

use super::permutation::Profile;
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

pub type Tool = CheckTool;

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
		let tools = CheckTool::from_tools(self.tools)?;
		let start = Instant::now();

		for tool in tools {
			match tool {
				CheckTool::Clippy => {
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
					metadata::run(self.args.clone(), &commands, Profile::Dev, verbose)?;
				}
				CheckTool::RustSec => {
					let mut command = Command::new("cargo");
					command.current_dir("../client").arg("audit");
					command::run("RustSec", command, verbose)?;
				}
				CheckTool::Tombi => {
					let mut command = Command::new("tombi");
					command
						.current_dir("../client")
						.args(["lint", "--error-on-warnings", "."]);
					command::run("Tombi Lint", command, verbose)?;
				}
				CheckTool::CargoSpellcheck => {
					let mut command = Command::new("cargo");
					command
						.current_dir("../client")
						.args(["spellcheck", "-m", "1"]);
					command::run("`cargo-spellcheck`", command, verbose)?;
				}
				CheckTool::Typos => {
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
}
