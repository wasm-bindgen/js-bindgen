use std::process::Command;
use std::time::Instant;
use std::{env, slice};

use anyhow::{Result, bail};
use cargo_metadata::{MetadataCommand, TargetKind};
use clap::Args;
use strum::VariantArray;

use crate::command::{self, Group, ToolchainParser};
use crate::permutation::Permutation;
use crate::{Target, TargetFeature};

#[derive(Args)]
pub struct Build {
	#[arg(long)]
	target: Option<Target>,
	#[arg(long, value_delimiter = ',')]
	target_feature: Vec<TargetFeature>,
	#[arg(long, value_parser = ToolchainParser)]
	nightly_toolchain: Option<String>,
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

		let metadata = MetadataCommand::new()
			.current_dir("../client")
			.no_deps()
			.exec()?;

		let mut packages = Vec::new();

		for package in metadata.packages {
			for target in package.targets {
				for kind in target.kind {
					match kind {
						TargetKind::Lib => {
							packages.push((kind, package.name.to_string()));
						}
						TargetKind::Example => {
							packages.push((kind, target.name.clone()));
						}
						_ => (),
					}
				}
			}
		}

		let start = Instant::now();

		if env::var_os("JBG_DEV_TOOLS").is_none_or(|value| value != "1") {
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
		}

		for permutation in Permutation::iter(targets, target_features) {
			for (kind, name) in &packages {
				let mut command =
					command::cargo(&permutation, self.nightly_toolchain.as_deref(), "build");

				let group = match kind {
					TargetKind::Lib => {
						command.args(["-p", name]);
						Group::announce(format!("Build `{name}` - {permutation}").into(), verbose)?
					}
					TargetKind::Example => {
						command.args(["--example", name]);
						Group::announce(
							format!("Build Example `{name}` - {permutation}").into(),
							verbose,
						)?
					}
					_ => unreachable!(),
				};

				if verbose {
					command::print_info(&command);
				}

				let (_, status) = command::run(command, verbose)?;

				if !status.success() {
					bail!("build \"{permutation}\" failed with {status}");
				}

				drop(group);
			}
		}

		println!("-------------------------");
		println!("Total Time: {:.2}s", start.elapsed().as_secs_f32());

		Ok(())
	}
}
