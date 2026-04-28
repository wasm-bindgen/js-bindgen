//! This file is not shipped to Crates.io, but it is present when depending on
//! `js-bindgen-runner` via `git` or `path`.

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

	if search_dir(&env::current_dir().unwrap().join("src/js"), false) {
		let pkg_mtime = fs::metadata(Path::new("src/js/package.json"))
			.unwrap()
			.modified()
			.unwrap();
		let lock = Path::new("src/js/package-lock.json");
		let lock_mtime = match fs::metadata(lock) {
			Ok(meta) => Some(meta.modified().unwrap()),
			Err(error) if error.kind() == ErrorKind::NotFound => None,
			Err(error) => panic::panic_any(error),
		};

		if lock_mtime.is_none_or(|lock_mtime| lock_mtime < pkg_mtime) {
			let status = Command::new("npm")
				.current_dir("src/js")
				.arg("install")
				.arg("-s")
				.arg("--no-audit")
				.arg("--no-fund")
				.status()
				.unwrap();

			if !status.success() {
				process::exit(status.code().unwrap_or(1));
			}
		}

		let status = Command::new("tsc")
			.current_dir("src/js")
			.arg("--build")
			.status()
			.unwrap();

		if !status.success() {
			process::exit(status.code().unwrap_or(1));
		}
	}
}

fn search_dir(dir: &Path, mut any: bool) -> bool {
	for entry in fs::read_dir(dir).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();

		if path.is_file()
			&& let bytes = path.as_os_str().as_encoded_bytes()
			&& !bytes.ends_with(b".d.mts")
			&& bytes.ends_with(b".mts")
		{
			println!("cargo::rerun-if-changed={}", path.display());

			if !any {
				let ts_mtime = fs::metadata(&path).unwrap().modified().unwrap();
				let js = path.with_extension("").with_extension("mjs");
				let js_mtime = match fs::metadata(js) {
					Ok(meta) => Some(meta.modified().unwrap()),
					Err(error) if error.kind() == ErrorKind::NotFound => None,
					Err(error) => panic::panic_any(error),
				};

				if js_mtime.is_none_or(|js_mtime| js_mtime < ts_mtime) {
					any = true;
				}
			}
		} else if path.is_dir() {
			any |= search_dir(&path, any);
		}
	}

	any
}
