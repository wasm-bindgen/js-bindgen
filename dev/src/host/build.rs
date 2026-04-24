use anyhow::Result;
use clap::Args;

use super::{HostTarget, HostTargets, metadata};
use crate::command::RunCommand;

#[derive(Args)]
pub struct Build {
	#[arg(long, short, default_value = HostTarget::host().to_clap_arg())]
	targets: Vec<HostTargets>,
}

impl Default for Build {
	fn default() -> Self {
		Self {
			targets: vec![HostTargets::Target(HostTarget::host())],
		}
	}
}

impl Build {
	pub fn new(targets: Vec<HostTargets>) -> Self {
		Self { targets }
	}

	pub fn all() -> Self {
		Self {
			targets: vec![HostTargets::All],
		}
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		let command = RunCommand {
			title: "Build",
			sub_command: "build",
			args: &[],
			envs: &[],
		};
		let targets = HostTarget::from_targets(self.targets.clone())?;
		let duration = metadata::run(&[command], &targets, false, verbose)?;

		println!("-------------------------");
		println!("Total Time: {:.2}s", duration.as_secs_f32());

		Ok(())
	}
}
