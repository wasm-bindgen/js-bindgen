use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::{Result, bail};

use crate::command;
use crate::group::Group;

pub fn run(verbose: bool) -> Result<()> {
	let start = Instant::now();
	let mut build_time = Duration::ZERO;
	let mut test_time = Duration::ZERO;

	let group = Group::announce("Build Tests".into(), verbose)?;
	let mut command = Command::new("cargo");
	command
		.current_dir("../host")
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
