use std::time::{Duration, Instant};

use anyhow::Result;
use cargo_metadata::{DependencyKind, Metadata, MetadataCommand, Package, TargetKind};

use super::permutation::Permutation;
use super::{ClientArgs, util};
use crate::command::{self, CargoCommand};
use crate::features;
use crate::features::Features;

pub fn run(client_args: &ClientArgs, commands: &[CargoCommand], verbose: bool) -> Result<Duration> {
	let metadata = MetadataCommand::new().current_dir("../client").exec()?;

	let start = Instant::now();

	util::build_linker(verbose)?;

	for CargoTarget {
		kind,
		name,
		features,
		js_sys,
	} in CargoTarget::from_metadata(&metadata)
	{
		for permutation in
			Permutation::iter(&client_args.targets, &client_args.target_features, js_sys)
		{
			for CargoCommand {
				title,
				sub_command,
				envs,
				args,
			} in commands
			{
				let mut command =
					util::cargo(&permutation, &client_args.nightly_toolchain, sub_command);

				let announce = match kind {
					TargetKind::Lib => {
						command.args(["-p", name]);
						*title
					}
					TargetKind::Example => {
						command.args(["--example", name]);
						&format!("{title} Example")
					}
					_ => unreachable!(),
				};

				command.arg("--keep-going");
				command.envs(envs.iter().copied());
				command.args(*args);

				let features_str = match features {
					Features::Default => String::new(),
					_ => format!(" - {features}"),
				};

				command::run(
					&format!("{announce} `{name}`{features_str} - {permutation}"),
					command,
					verbose,
				)?;
			}
		}
	}

	Ok(start.elapsed())
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

			if package
				.targets
				.iter()
				.flat_map(|target| &target.kind)
				.any(|kind| matches!(kind, TargetKind::Lib))
			{
				for features in &feature_combinations {
					targets.push(Self {
						kind: TargetKind::Lib,
						name: &package.name,
						features: features.clone(),
						js_sys,
					});
				}
			}
		}

		for package in &metadata.workspace_packages() {
			let js_sys = Self::js_sys_dependency(metadata, package, true);

			for target in &package.targets {
				for kind in &target.kind {
					if let TargetKind::Example = kind {
						targets.push(Self {
							kind: TargetKind::Example,
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
