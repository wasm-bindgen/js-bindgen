use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::{Result, bail};
use cargo_metadata::{MetadataCommand, TargetKind};
use clap::Subcommand;

use crate::features::Features;
use crate::github::Group;
use crate::{command, features};

#[derive(Subcommand)]
pub enum Host {
	All,
	Build,
	Test,
}

impl Host {
	pub fn execute(self, verbose: bool) -> Result<()> {
		match self {
			Self::All => {
				Self::Build.execute(verbose)?;
				println!("-------------------------");
				println!();
				Self::Test.execute(verbose)?;

				Ok(())
			}
			Self::Build => {
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
							if let TargetKind::Lib | TargetKind::ProcMacro | TargetKind::Bin = kind
							{
								for features in &feature_combinations {
									packages.push((package.name.to_string(), features.clone()));
								}
							}
						}
					}
				}

				let start = Instant::now();

				for (name, features) in packages {
					let mut command = Command::new("cargo");
					command
						.current_dir("../host")
						.env("CI", "true")
						.arg("build")
						.args(["-p", &name]);

					features.args(&mut command);

					let features_str = match features {
						Features::Default => String::new(),
						_ => format!(" - {features}"),
					};

					let group =
						Group::announce(format!("Build `{name}`{features_str}").into(), verbose)?;

					if verbose {
						command::print_info(&command);
					}

					let (_, status) = command::run(command, verbose)?;

					if !status.success() {
						bail!("build \"`{name}`{features_str}\" failed with {status}");
					}

					drop(group);
				}

				println!("-------------------------");
				println!("Total Time: {:.2}s", start.elapsed().as_secs_f32());

				Ok(())
			}
			Self::Test => {
				let start = Instant::now();
				let mut build_time = Duration::ZERO;
				let mut test_time = Duration::ZERO;

				let group = Group::announce("Build Tests".into(), verbose)?;
				let mut command = Command::new("cargo");
				command
					.current_dir("../host")
					.env("CI", "true")
					.arg("test")
					.arg("--all-features")
					.arg("--no-run");

				if verbose {
					command::print_info(&command);
				}

				let (duration, status) = command::run(command, verbose)?;
				build_time += duration;

				if !status.success() {
					bail!("build failed with {status}");
				}

				drop(group);

				let group = Group::announce("Run Tests".into(), verbose)?;
				let mut command = Command::new("cargo");
				command
					.current_dir("../host")
					.env("CI", "true")
					.arg("test")
					.arg("--all-features");

				if verbose {
					command::print_info(&command);
				}

				let (duration, status) = command::run(command, verbose)?;
				test_time += duration;

				if !status.success() {
					bail!("test failed with {status}");
				}

				drop(group);

				println!("-------------------------");
				println!("Build Time: {:.2}s", build_time.as_secs_f32());
				println!("Test Time: {:.2}s", test_time.as_secs_f32());
				println!("Total Time: {:.2}s", start.elapsed().as_secs_f32());

				Ok(())
			}
		}
	}
}
