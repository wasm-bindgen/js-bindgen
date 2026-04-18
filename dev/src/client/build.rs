use std::process::Command;
use std::time::Instant;
use std::{env, slice};

use anyhow::{Result, bail};
use cargo_metadata::{DependencyKind, Metadata, MetadataCommand, Package, TargetKind};
use clap::Args;
use strum::VariantArray;

use super::permutation::Permutation;
use super::{ClientArgs, Target, TargetFeature, util};
use crate::features::Features;
use crate::github::Group;
use crate::{command, features};

#[derive(Args, Default)]
pub struct Build {
	#[command(flatten)]
	args: ClientArgs,
}

impl Build {
	pub fn new(args: ClientArgs) -> Self {
		Self { args }
	}

	pub fn execute(self, verbose: bool) -> Result<()> {
		let targets = self
			.args
			.target
			.as_ref()
			.map_or(Target::VARIANTS, slice::from_ref);
		let target_features = if self.args.target_feature.is_empty() {
			TargetFeature::VARIANTS
		} else {
			&self.args.target_feature
		};

		let metadata = MetadataCommand::new().current_dir("../client").exec()?;

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

		for CargoTarget {
			kind,
			name,
			features,
			js_sys,
		} in CargoTarget::from_metadata(&metadata)
		{
			for permutation in Permutation::iter(targets, target_features, js_sys) {
				let mut command = util::cargo(
					&permutation,
					self.args.nightly_toolchain.as_deref(),
					"build",
				);

				let announce = match kind {
					TargetKind::Lib => {
						command.args(["-p", name]);
						"Build"
					}
					TargetKind::Example => {
						command.args(["--example", name]);
						"Build Example"
					}
					_ => unreachable!(),
				};

				let features_str = match features {
					Features::Default => String::new(),
					_ => format!(" - {features}"),
				};

				let group = Group::announce(
					format!("{announce} `{name}`{features_str} - {permutation}").into(),
					verbose,
				)?;

				if verbose {
					command::print_info(&command);
				}

				let (_, status) = command::run(command, verbose)?;

				if !status.success() {
					bail!("build \"`{name}`{features_str} - {permutation}\" failed with {status}");
				}

				drop(group);
			}
		}

		println!("-------------------------");
		println!("Total Time: {:.2}s", start.elapsed().as_secs_f32());

		Ok(())
	}
}

struct CargoTarget<'m> {
	kind: TargetKind,
	name: &'m str,
	features: Features<'m>,
	js_sys: bool,
}

impl<'m> CargoTarget<'m> {
	fn from_metadata(metadata: &'m Metadata) -> Vec<Self> {
		let mut targets = Vec::new();

		for package in &metadata.workspace_packages() {
			let feature_combinations = features::combinations(package);
			let js_sys = Self::js_sys_dependency(metadata, package, false);

			for target in &package.targets {
				for kind in &target.kind {
					if let TargetKind::Lib = kind {
						for features in &feature_combinations {
							targets.push(Self {
								kind: kind.clone(),
								name: &package.name,
								features: features.clone(),
								js_sys,
							});
						}
					}
				}
			}
		}

		for package in &metadata.workspace_packages() {
			let js_sys = Self::js_sys_dependency(metadata, package, true);

			for target in &package.targets {
				for kind in &target.kind {
					if let TargetKind::Example = kind {
						targets.push(Self {
							kind: kind.clone(),
							name: &target.name,
							features: Features::Default,
							js_sys,
						});
					}
				}
			}
		}

		targets
	}

	fn js_sys_dependency(metadata: &'m Metadata, package: &Package, dev: bool) -> bool {
		if package.name == "js-sys" {
			return true;
		}

		package
			.dependencies
			.iter()
			.filter(|dependency| {
				matches!(dependency.kind, DependencyKind::Normal)
					|| (dev && matches!(dependency.kind, DependencyKind::Development))
			})
			.any(|dependency| {
				dependency.name == "js-sys"
					|| metadata
						.packages
						.iter()
						.find(|package| package.name == dependency.name)
						.is_some_and(|package| Self::js_sys_dependency(metadata, package, false))
			})
	}
}
