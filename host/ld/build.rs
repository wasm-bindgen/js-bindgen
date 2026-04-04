//! This file is not shipped to Crates.io, but it is present when depending on
//! `js-bindgen-ld` via `git` or `path`.

use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::process::Command;
use std::{env, fs, io, process};

fn main() -> io::Result<()> {
	if option_env!("JBG_LOCAL_DEV").is_some_and(|value| value == "1")
		&& option_env!("CI").is_none_or(|value| value != "true")
	{
		let any = search_folder(&env::current_dir()?.join("src/js"))?;

		if any {
			let status = Command::new("tsc")
				.current_dir("src/js")
				.arg("--build")
				.status()?;

			if !status.success() {
				process::exit(status.code().unwrap_or(1));
			}
		}
	}

	Ok(())
}

fn search_folder(folder: &Path) -> io::Result<bool> {
	let mut any = false;

	for entry in fs::read_dir(folder)? {
		let entry = entry?;
		let path = entry.path();

		if path.is_file()
			&& let bytes = path.as_os_str().as_encoded_bytes()
			&& !bytes.ends_with(b".d.mts")
			&& bytes.ends_with(b".mts")
		{
			println!("cargo::rerun-if-changed={}", path.display());

			let mtime = fs::metadata(&path)?.mtime();
			let mjs = path.with_extension("").with_extension("mjs");

			if !fs::metadata(mjs).is_ok_and(|meta| meta.mtime() >= mtime) {
				any = true;
			}
		} else if path.is_dir() {
			search_folder(&path)?;
		}
	}

	Ok(any)
}
