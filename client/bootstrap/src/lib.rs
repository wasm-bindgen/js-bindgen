use std::io::ErrorKind;
use std::path::Path;
use std::{env, fs};

// This communicates the cache directory to the proc-macro.
// Deletes the cache when bootstrapping is enabled.
pub fn bootstrap(cache_dir: &Path) {
	let name = env::var("CARGO_PKG_NAME").expect("`CARGO_PKG_NAME` should be present");
	let version = env::var("CARGO_PKG_VERSION").expect("`CARGO_PKG_VERSION` should be present");

	println!(
		"cargo:rustc-env=JS_BINDGEN_CACHE_DIR_{name}_{version}={}",
		cache_dir.display()
	);

	// When bootstrapping we delete the old cache.
	if env::var_os(format!("JS_BINDGEN_BOOTSTRAP_{name}"))
		.filter(|value| value == "1")
		.is_some()
	{
		match fs::remove_dir_all(cache_dir) {
			Ok(()) => (),
			Err(error) if matches!(error.kind(), ErrorKind::NotFound) => (),
			Err(error) => panic!("{error:?}"),
		}
	}

	println!("cargo:rustc-link-search=native={}", cache_dir.display());
}
