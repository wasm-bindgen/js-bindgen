use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::Result;

use crate::command;

pub fn run(verbose: bool) -> Result<()> {
	let start = Instant::now();
	let mut build_time = Duration::ZERO;
	let mut test_time = Duration::ZERO;

	let mut command = Command::new("cargo");
	command
		.arg("test")
		.arg("--workspace")
		.arg("--all-features")
		.arg("--no-run");

	build_time += command::run("Build Tests", command, verbose)?;

	let mut command = Command::new("cargo");
	command.arg("test").arg("--workspace").arg("--all-features");

	test_time += command::run("Run Tests", command, verbose)?;

	println!("-------------------------");
	println!("Build Time: {:.2}s", build_time.as_secs_f32());
	println!("Test Time: {:.2}s", test_time.as_secs_f32());
	println!("Total Time: {:.2}s", start.elapsed().as_secs_f32());

	Ok(())
}
