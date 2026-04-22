//! This file is not shipped to Crates.io, but it is present when depending on
//! `web-sys` via `git` or `path`.

use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;
use std::{env, fs, panic, process};

fn main() {
	if option_env!("JBG_DEV").is_none_or(|value| value != "1")
		|| option_env!("CI").is_some_and(|value| value == "true")
	{
		return;
	}

	if search_dir(&env::current_dir().unwrap(), false) {
		let status = Command::new("cargo")
			.env_remove("CARGO_ENCODED_RUSTFLAGS")
			.current_dir("../../host")
			.arg("+stable")
			.arg("run")
			.args(["-p", "cargo-js-sys"])
			.arg("--")
			.arg("-q")
			.arg("js-sys")
			.args(["--manifest-path", "../client/web-sys/Cargo.toml"])
			.status()
			.unwrap();

		if !status.success() {
			process::exit(status.code().unwrap_or(1))
		}
	}
}

fn search_dir(dir: &Path, mut any: bool) -> bool {
	for entry in fs::read_dir(dir).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();

		if path.is_file() && path.as_os_str().as_encoded_bytes().ends_with(b".js-sys.rs") {
			println!("cargo::rerun-if-changed={}", path.display());

			if !any {
				let js_sys_mtime = fs::metadata(&path).unwrap().modified().unwrap();
				let r#gen = path.with_extension("").with_extension("gen.rs");
				let gen_mtime = match fs::metadata(r#gen) {
					Ok(meta) => Some(meta.modified().unwrap()),
					Err(error) if error.kind() == ErrorKind::NotFound => None,
					Err(error) => panic::panic_any(error),
				};

				if gen_mtime.is_none_or(|gen_mtime| gen_mtime < js_sys_mtime) {
					any = true;
				}
			}
		} else if path.is_dir() {
			any |= search_dir(&path, any);
		}
	}

	any
}
