#[macro_use]
mod util;
mod client;
mod command;
mod features;
mod host;

use std::process::Command;
use std::time::Duration;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use strum::EnumIter;

use crate::client::Client;
use crate::host::Host;

#[derive(Parser)]
struct Cli {
	#[arg(short, long, global = true)]
	verbose: bool,
	#[command(subcommand)]
	command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
	All {
		#[arg(long)]
		all: bool,
	},
	Fmt {
		#[arg(long, value_delimiter = ',', default_value = FmtTools::default_arg())]
		tools: Vec<FmtTools>,
	},
	Build {
		#[arg(long)]
		all: bool,
	},
	Check {
		#[arg(long)]
		all: bool,
	},
	Test {
		#[arg(long)]
		all: bool,
	},
	Client {
		#[command(subcommand)]
		client: Client,
	},
	Host {
		#[command(subcommand)]
		host: Host,
	},
}

fn main() -> Result<()> {
	Cli::parse().execute()
}

impl Cli {
	fn execute(self) -> Result<()> {
		self.command.execute(self.verbose)
	}
}

impl CliCommand {
	fn execute(self, verbose: bool) -> Result<()> {
		match self {
			Self::All { all } => {
				let tools = if all {
					FmtTools::all()
				} else {
					vec![FmtTools::default()]
				};
				Self::Fmt { tools }.execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Build { all }.execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Check { all }.execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Test { all }.execute(verbose)?;

				Ok(())
			}
			Self::Fmt { tools } => {
				let tools = FmtTool::from_tools(tools)?;

				if tools.contains(&FmtTool::default()) {
					Client::fmt().execute(verbose)?;
					println!("-------------------------");
					println!();
					Host::fmt().execute(verbose)?;
					println!("-------------------------");
					println!();
				}

				let mut duration = Duration::ZERO;

				for tool in tools {
					match tool {
						FmtTool::Rustfmt => (),
						FmtTool::Taplo => {
							let mut command = Command::new("taplo");
							command.current_dir("..").arg("fmt");
							duration += command::run("Taplo Format", command, verbose)?;
						}
						FmtTool::Prettier => {
							let mut command = Command::new("prettier");
							command.current_dir("..").args([".", "-w"]);
							duration += command::run("Prettier", command, verbose)?;
						}
					}
				}

				println!("-------------------------");
				println!("Total Time: {:.2}s", duration.as_secs_f32());

				Ok(())
			}
			Self::Build { all } => {
				Client::build(all).execute(verbose)?;
				println!("-------------------------");
				println!();
				Host::build(all).execute(verbose)?;

				Ok(())
			}
			Self::Check { all } => {
				Client::check(all).execute(verbose)?;
				println!("-------------------------");
				println!();
				Host::check(all).execute(verbose)?;

				Ok(())
			}
			Self::Test { all } => {
				Client::test(all).execute(verbose)?;
				println!("-------------------------");
				println!();
				Host::Test.execute(verbose)?;

				Ok(())
			}
			Self::Client { client } => client.execute(verbose),
			Self::Host { host } => host.execute(verbose),
		}
	}
}

enum_with_all!(enum FmtTools, Tool(FmtTool), "tools");

#[derive(Clone, Copy, Default, EnumIter, Eq, PartialEq, ValueEnum)]
enum FmtTool {
	#[default]
	Rustfmt,
	Taplo,
	Prettier,
}

impl FmtTools {
	fn all() -> Vec<Self> {
		vec![Self::All]
	}
}
