use std::process::Command;
use std::slice;
use std::time::Instant;

use anyhow::{Result, bail};
use clap::Args;
use strum::VariantArray;

use crate::command::{self, Group};
use crate::permutation::Permutation;
use crate::{Target, TargetFeature};

#[derive(Args)]
pub struct Build {
	#[arg(long)]
	target: Option<Target>,
	#[arg(long, value_delimiter = ',')]
	target_feature: Vec<TargetFeature>,
}

impl Build {
	pub fn execute(self, verbose: bool) -> Result<()> {
		let targets = self
			.target
			.as_ref()
			.map_or(Target::VARIANTS, slice::from_ref);
		let target_features = if self.target_feature.is_empty() {
			TargetFeature::VARIANTS
		} else {
			&self.target_feature
		};

		let start = Instant::now();

		let group = Group::announce("Build Linker".into(), verbose)?;
		let mut command = Command::new("cargo");
		command
			.current_dir("../host")
			.env("CI", "true")
			.arg("+stable")
			.arg("build")
			.args(["-p", "js-bindgen-ld"]);

		let (_, status) = command::run(command, verbose)?;

		if !status.success() {
			bail!("build Linker failed with {status}");
		}

		drop(group);

		for permutation in Permutation::iter(targets, target_features) {
			let group = Group::announce(format!("Build - {permutation}").into(), verbose)?;
			let mut command = command::cargo(&permutation, "build");
			command.arg("--examples");

			if verbose {
				command::print_info(&command);
			}

			let (_, status) = command::run(command, verbose)?;

			if !status.success() {
				bail!("build \"{permutation}\" failed with {status}");
			}

			drop(group);
		}

		println!("-------------------------");
		println!("Total Time: {:.2}s", start.elapsed().as_secs_f32());

		Ok(())
	}
}
