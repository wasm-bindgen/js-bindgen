use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::{Result, bail};
use cargo_metadata::{MetadataCommand, TargetKind};

use super::HostTarget;
use crate::command::RunCommand;
use crate::features::Features;
use crate::group::Group;
use crate::{command, features};

pub fn run(commands: &[RunCommand], targets: &[HostTarget], verbose: bool) -> Result<Duration> {
	let metadata = MetadataCommand::new()
		.current_dir("../host")
		.no_deps()
		.exec()?;

	let mut packages = Vec::new();

	for package in &metadata.packages {
		if package.publish.as_ref().is_some_and(Vec::is_empty) {
			continue;
		}

		let feature_combinations = features::combinations(package);

		for target in &package.targets {
			for kind in &target.kind {
				if let TargetKind::Lib | TargetKind::ProcMacro | TargetKind::Bin = kind {
					for features in &feature_combinations {
						packages.push((package.name.to_string(), features.clone()));
					}
				}
			}
		}
	}

	let start = Instant::now();

	for (name, features) in packages {
		for target in targets {
			for RunCommand {
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

				command
					.current_dir("../host")
					.arg(sub_command)
					.args(["-p", &name]);

				let target_str = if target.is_host() && targets.len() == 1 {
					String::new()
				} else {
					if target.is_host() {
						command.args(["--target", target.to_cargo_arg()]);
					}

					format!(" - {target}")
				};

				features.args(&mut command);
				command.envs(envs.iter().copied());
				command.args(*args);

				let features_str = match features {
					Features::Default => String::new(),
					_ => format!(" - {features}"),
				};

				let group = Group::announce(
					format!("{title} `{name}`{target_str}{features_str}").into(),
					verbose,
				)?;

				if verbose {
					command::print_info(&command);
				}

				let (_, status) = command::run(command, verbose)?;

				if !status.success() {
					bail!(
						"{} \"`{name}`{features_str}\" failed with {status}",
						title.to_lowercase()
					);
				}

				drop(group);
			}
		}
	}

	Ok(start.elapsed())
}
