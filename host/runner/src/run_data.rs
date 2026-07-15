use std::path::PathBuf;
use std::{env, process};

use js_bindgen_cli_lib::MainMemory;
use serde::Serialize;

use crate::test::TestEntry;

#[derive(Serialize)]
#[serde(
	tag = "kind",
	rename_all = "camelCase",
	rename_all_fields = "camelCase"
)]
pub enum RunData<'a> {
	Test {
		ctx: Ctx,
		no_capture: bool,
		filtered_count: usize,
		tests: Vec<TestEntry<'a>>,
	},
	Binary {
		ctx: Ctx,
		wasm64: bool,
		memory: MainMemory<'a>,
		args: Vec<String>,
	},
}

#[derive(Serialize)]
pub struct Ctx {
	pid: u32,
	tmpdir: PathBuf,
	#[serde(skip_serializing_if = "Option::is_none")]
	llvm_profile_file: Option<String>,
}

impl Ctx {
	pub fn new() -> Self {
		Self {
			pid: process::id(),
			tmpdir: env::temp_dir(),
			llvm_profile_file: env::var("LLVM_PROFILE_FILE").ok(),
		}
	}
}
