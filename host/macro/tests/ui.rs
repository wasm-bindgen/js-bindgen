use std::env;
use std::path::PathBuf;

use ui_test::Config;
use ui_test::color_eyre::Result;
use ui_test::custom_flags::rustfix::RustfixMode;
use ui_test::dependencies::DependencyBuilder;

fn main() -> Result<()> {
	let mut config = Config::rustc("tests/ui");
	// `ui_test` is unable to pick up the workspace target folder:
	// https://github.com/oli-obk/ui_test/issues/362
	config.out_dir = env::current_dir()?
		.parent()
		.unwrap()
		.join("target")
		.join("ui");
	let base = config.comment_defaults.base();
	base.set_custom(
		"dependencies",
		DependencyBuilder {
			// No `dev-dependency` support:
			// https://github.com/oli-obk/ui_test/issues/282
			crate_manifest_path: PathBuf::from("tests/ui/Cargo.toml"),
			..DependencyBuilder::default()
		},
	);
	base.set_custom("rustfix", RustfixMode::Disabled);
	config.skip_files.push(String::from("lib.rs"));

	if env::var_os("BLESS").filter(|v| v == "1").is_some() {
		config.output_conflict_handling = ui_test::bless_output_files;
	}

	let result = ui_test::run_tests(config);

	if cfg!(coverage_nightly) {
		Ok(())
	} else {
		result
	}
}
