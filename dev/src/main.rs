mod client;
mod command;
mod features;
mod github;
mod host;

use anyhow::Result;
use clap::{Parser, Subcommand};

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
	All,
	Build,
	Test,
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
			Self::All => {
				Self::Build.execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Test.execute(verbose)?;

				Ok(())
			}
			Self::Build => {
				Client::build().execute(verbose)?;
				println!("-------------------------");
				println!();
				Host::Build.execute(verbose)?;

				Ok(())
			}
			Self::Test => {
				Client::test().execute(verbose)?;
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
