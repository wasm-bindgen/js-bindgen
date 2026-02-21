#[cfg(feature = "js-sys")]
mod js_sys;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use clap_cargo::style::CLAP_STYLING;

#[cfg(feature = "js-sys")]
use crate::js_sys::JsSys;

#[cfg(not(any(feature = "js-sys")))]
compile_error!("pick at least one crate feature");

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo", version, about, long_about = None, styles = CLAP_STYLING)]
struct Cli {
	#[command(flatten)]
	global_args: GlobalArgs,
	#[command(subcommand)]
	commands: Commands,
}

#[derive(Args, Clone, Copy)]
struct GlobalArgs {
	#[arg(global = true, short, long, conflicts_with = "verbose")]
	quiet: bool,
	#[arg(global = true, short, long)]
	verbose: bool,
	#[arg(global = true, short = 'n', long)]
	dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
	#[cfg(feature = "js-sys")]
	JsSys(JsSys),
}

fn main() -> Result<()> {
	let cli = Cli::parse();

	match cli.commands {
		#[cfg(feature = "js-sys")]
		Commands::JsSys(js_sys) => js_sys.run(cli.global_args)?,
	}

	Ok(())
}
