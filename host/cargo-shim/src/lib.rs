use std::env;
use std::process::{self, Command};

pub fn run(package: &str) {
	let status = Command::new("cargo")
		.current_dir("../host")
		.arg("+stable")
		.arg("run")
		.arg("-q")
		.args(["-p", package])
		.arg("--")
		.args(env::args_os().skip(1))
		.status()
		.unwrap();

	if !status.success() {
		process::exit(status.code().unwrap_or(1));
	}
}
