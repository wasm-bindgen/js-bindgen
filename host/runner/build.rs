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
		let needs_update = match fs::metadata("src/js/package-lock.json") {
			Ok(meta) => 'outer: {
				let lock_mtime = meta.modified().unwrap();
				let pkg_mtime = fs::metadata("src/js/package.json")
					.unwrap()
					.modified()
					.unwrap();

				if lock_mtime < pkg_mtime {
					break 'outer true;
				}

				match fs::metadata("src/js/node_modules/.package-lock.json") {
					Ok(meta) => meta.modified().unwrap() < pkg_mtime,
					Err(error) if error.kind() == ErrorKind::NotFound => true,
					Err(error) => panic::panic_any(error),
				}
			}
			Err(error) if error.kind() == ErrorKind::NotFound => true,
			Err(error) => panic::panic_any(error),
		};

		if needs_update {
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
				let js = path.with_extension("").with_extension("mjs");

				match fs::metadata(js) {
					Ok(meta) => {
						let js_mtime = meta.modified().unwrap();
						let ts_mtime = fs::metadata(&path).unwrap().modified().unwrap();

						if js_mtime < ts_mtime {
							any = true;
						}
					}
					Err(error) if error.kind() == ErrorKind::NotFound => any = true,
					Err(error) => panic::panic_any(error),
				}
			}
		} else if path.is_dir() {
			any |= search_dir(&path, any);
		}
	}

	any
}
