use std::env;
use std::path::PathBuf;

use ui_test::color_eyre::Result;
use ui_test::dependencies::DependencyBuilder;
use ui_test::{CommandBuilder, Config};

fn main() -> Result<()> {
	let mut config = Config::rustc("tests/ui");
	// `ui_test` is unable to pick up the workspace target folder:
	// https://github.com/oli-obk/ui_test/issues/362
	config.out_dir = env::current_dir()?
		.parent()
		.unwrap()
		.join("target")
		.join("ui");
	config.comment_defaults.base().set_custom(
		"dependencies",
		DependencyBuilder {
			// No `dev-dependency` support:
			// https://github.com/oli-obk/ui_test/issues/282
			crate_manifest_path: PathBuf::from("tests/ui/Cargo.toml"),
			..DependencyBuilder::default()
		},
	);
	config.skip_files.push(String::from("lib.rs"));
	config.program = CommandBuilder {
		envs: vec![("CARGO_CRATE_NAME".into(), Some("test_crate".into()))],
		..CommandBuilder::rustc()
	};

	if env::var_os("BLESS").filter(|v| v == "1").is_some() {
		config.output_conflict_handling = ui_test::bless_output_files;
	}

	ui_test::run_tests(config)
}
