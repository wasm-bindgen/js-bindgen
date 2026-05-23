use std::ffi::OsString;

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
		no_capture: bool,
		filtered_count: usize,
		tests: Vec<TestEntry<'a>>,
	},
	Binary {
		wasm64: bool,
		memory: MainMemory<'a>,
		args: Vec<OsString>,
	},
}
