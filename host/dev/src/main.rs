mod client;
mod command;
mod features;
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
	All {
		#[arg(long)]
		all: bool,
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
				Self::Build { all }.execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Check { all }.execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Test { all }.execute(verbose)?;

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
