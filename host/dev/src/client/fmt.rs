use std::process::Command;
use std::time::Instant;

use anyhow::Result;
use clap::Args;

use crate::{FmtTool, FmtTools, command};

#[derive(Args)]
pub struct Fmt {
	#[arg(long, value_delimiter = ',', default_value = FmtTools::default_arg())]
	tools: Vec<FmtTools>,
}

impl Default for Fmt {
	fn default() -> Self {
		Self {
			tools: vec![FmtTools::default()],
		}
	}
}

impl Fmt {
	pub fn new(tools: Vec<FmtTools>) -> Self {
		Self { tools }
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		let tools = FmtTool::from_tools(self.tools)?;
		let start = Instant::now();

		for tool in tools {
			match tool {
				FmtTool::Rustfmt => {
					let mut command = Command::new("cargo");
					command.current_dir("../client").args(["+nightly", "fmt"]);
					command::run("Rustfmt", command, verbose)?;
				}
				FmtTool::Tombi => {
					let mut command = Command::new("tombi");
					command.current_dir("../client").args(["format", "."]);
					command::run("Tombi Format", command, verbose)?;
				}
				FmtTool::Prettier => {
					let mut command = Command::new("prettier");
					command.current_dir("..").args(["client", "-w"]);
					command::run("Prettier", command, verbose)?;
				}
			}
		}

		println!("-------------------------");
		println!("Total Time: {:.2}s", start.elapsed().as_secs_f32());

		Ok(())
	}
}
