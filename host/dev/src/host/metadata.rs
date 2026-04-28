use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::Result;
use cargo_metadata::{MetadataCommand, TargetKind};

use super::HostTarget;
use crate::command::{self, CargoCommand};
use crate::features;
use crate::features::Features;

pub fn run(
	commands: &[CargoCommand],
	targets: &[HostTarget],
	private: bool,
	verbose: bool,
) -> Result<Duration> {
	let metadata = MetadataCommand::new().no_deps().exec()?;

	let mut packages = Vec::new();

	for package in &metadata.packages {
		if !private && package.publish.as_ref().is_some_and(Vec::is_empty) {
			continue;
		}

		let feature_combinations = features::combinations(package);

		if package
			.targets
			.iter()
			.flat_map(|target| &target.kind)
			.any(|kind| {
				matches!(
					kind,
					TargetKind::Lib | TargetKind::ProcMacro | TargetKind::Bin
				)
			}) {
			for features in &feature_combinations {
				packages.push((package.name.to_string(), features.clone()));
			}
		}
	}

	let start = Instant::now();

	for (name, features) in packages {
		for target in targets {
			for CargoCommand {
				title,
				sub_command,
				envs,
				args,
			} in commands
			{
				let mut command = Command::new("cargo");

				if let HostTarget::Windows = target
					&& !target.is_host()
				{
					command.arg("xwin");
				}

				command.arg(sub_command).args(["-p", &name]);

				let target_str = if target.is_host() && targets.len() == 1 {
					String::new()
				} else {
					if target.is_host() {
						command.args(["--target", target.to_cargo_arg()]);
					}

					format!(" - {target}")
				};

				features.args(&mut command);
				command.arg("--keep-going");
				command.envs(envs.iter().copied());
				command.args(*args);

				let features_str = match features {
					Features::Default => String::new(),
					_ => format!(" - {features}"),
				};

				command::run(
					&format!("{title} `{name}`{target_str}{features_str}"),
					command,
					verbose,
				)?;
			}
		}
	}

	Ok(start.elapsed())
}
