//! This file is not shipped to Crates.io, but it is present when depending on
//! `js-sys` via `git` or `path`.

use std::path::Path;
use std::process::Command;
use std::{env, fs, io, process};

fn main() -> io::Result<()> {
	if option_env!("JS_SYS_LOCAL_DEV").is_some_and(|value| value == "1") {
		search_folder(&env::current_dir()?)?;

		let status = Command::new("cargo")
			.current_dir("../../host")
			.arg("+stable")
			.arg("run")
			.args(["-p", "cargo-js-sys"])
			.arg("--")
			.arg("-q")
			.arg("js-sys")
			.args(["--manifest-path", "../client/js-sys/Cargo.toml"])
			.status()?;

		if !status.success() {
			process::exit(status.code().unwrap_or(1))
		}
	}

	Ok(())
}

fn search_folder(folder: &Path) -> io::Result<()> {
	for entry in fs::read_dir(folder)? {
		let entry = entry?;
		let path = entry.path();

		if path.is_file() && path.as_os_str().as_encoded_bytes().ends_with(b".js-sys.rs") {
			println!("cargo::rerun-if-changed={}", path.display());
		} else if path.is_dir() {
			search_folder(&path)?;
		}
	}

	Ok(())
}
