use std::env;
use std::process::{self, Command};

pub fn run(package: &str) {
	let mut command = if env::var_os("JBG_DEV_TOOLS").is_some_and(|value| value == "1") {
		Command::new(package)
	} else {
		let mut command = Command::new("cargo");
		command
			.current_dir("../host")
			.arg("+stable")
			.arg("run")
			.arg("-q")
			.args(["-p", package])
			.arg("--");

		command
	};

	let status = command.args(env::args_os().skip(1)).status().unwrap();

	if !status.success() {
		process::exit(status.code().unwrap_or(1));
	}
}
