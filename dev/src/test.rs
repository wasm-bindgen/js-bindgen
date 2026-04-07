use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::{env, slice};

use anyhow::{Result, bail};
use clap::Args;
use strum::{IntoEnumIterator, VariantArray};

use crate::command::{self, Group};
use crate::permutation::Permutation;
use crate::process::ChildWrapper;
use crate::{Runner, Target, TargetFeature, WebDriver};

#[derive(Args)]
pub struct Test {
	#[arg(long)]
	target: Option<Target>,
	#[arg(long, value_delimiter = ',')]
	target_feature: Vec<TargetFeature>,
	#[arg(long, value_delimiter = ',', conflicts_with = "exclude")]
	include: Vec<Runner>,
	#[arg(long, value_delimiter = ',', conflicts_with = "include")]
	exclude: Vec<Runner>,
}

impl Test {
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
		let filter = |runner: &Runner| {
			if !self.include.is_empty() {
				self.include.contains(runner)
			} else if !self.exclude.is_empty() {
				!self.exclude.contains(runner)
			} else {
				true
			}
		};
		let tools_installed = env::var_os("JBG_DEV_TOOLS").is_some_and(|value| value == "1");

		let start = Instant::now();
		let mut build_time = Duration::ZERO;
		let mut test_time = Duration::ZERO;

		let mut web_drivers = Vec::new();
		let mut session_manager_built = tools_installed;

		for web_driver in
			WebDriver::iter().filter(|web_driver| filter(&Runner::WebDriver(*web_driver)))
		{
			if !session_manager_built {
				let group = Group::announce("Build Session Manager".into(), verbose)?;
				let mut command = Command::new("cargo");
				command
					.current_dir("../host")
					.arg("+stable")
					.arg("build")
					.args(["-p", "js-bindgen-web-driver"]);

				let (duration, status) = command::run(command, verbose)?;
				build_time += duration;

				if !status.success() {
					bail!("build Session Manager failed with {status}");
				}

				drop(group);
				session_manager_built = true;
			}

			let mut command = if tools_installed {
				Command::new("js-bindgen-web-driver")
			} else {
				let mut command = Command::new("cargo");
				command
					.current_dir("../host")
					.arg("+stable")
					.arg("run")
					.args(["-p", "js-bindgen-web-driver"])
					.arg("--");
				command
			};

			command
				.stdout(Stdio::null())
				.stderr(Stdio::null())
				.args(["-p", web_driver.port()])
				.arg(web_driver.short_name());

			web_drivers.push(ChildWrapper::new(command)?);
		}

		if !tools_installed {
			let group = Group::announce("Build Linker".into(), verbose)?;
			let mut command = Command::new("cargo");
			command
				.current_dir("../host")
				.env("CI", "true")
				.arg("+stable")
				.arg("build")
				.args(["-p", "js-bindgen-ld"]);

			let (duration, status) = command::run(command, verbose)?;
			build_time += duration;

			if !status.success() {
				bail!("build Linker failed with {status}");
			}

			drop(group);

			let group = Group::announce("Build Runner".into(), verbose)?;
			let mut command = Command::new("cargo");
			command
				.current_dir("../host")
				.env("CI", "true")
				.arg("+stable")
				.arg("build")
				.args(["-p", "js-bindgen-runner"]);

			let (duration, status) = command::run(command, verbose)?;
			build_time += duration;

			if !status.success() {
				bail!("build Runner failed with {status}");
			}

			drop(group);
		}

		for permutation in Permutation::iter(targets, target_features) {
			let mut built = false;

			for test_run in permutation.test_runs(filter) {
				if !built {
					let group =
						Group::announce(format!("Build Tests - {permutation}").into(), verbose)?;
					let mut command = command::cargo(&permutation, "test");
					command.arg("--no-run");

					if verbose {
						command::print_info(&command);
					}

					let (duration, status) = command::run(command, verbose)?;
					build_time += duration;

					if !status.success() {
						bail!("build \"{permutation}\" failed with {status}");
					}

					drop(group);
					built = true;
				}

				let group = Group::announce(format!("Run Tests - {test_run}").into(), verbose)?;
				let mut command = command::cargo(&permutation, "test");
				command.envs(test_run.envs());

				if verbose {
					command::print_info(&command);
				}

				let (duration, status) = command::run(command, verbose)?;
				test_time += duration;

				if !status.success() {
					bail!("test \"{test_run}\" failed with {status}");
				}

				drop(group);
			}
		}

		for web_driver in web_drivers {
			web_driver.shutdown()?;
		}

		println!("-------------------------");
		println!("Build Time: {:.2}s", build_time.as_secs_f32());
		println!("Test Time: {:.2}s", test_time.as_secs_f32());
		println!("Total Time: {:.2}s", start.elapsed().as_secs_f32());

		Ok(())
	}
}
