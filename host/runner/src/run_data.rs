use std::ffi::OsString;

use serde::Serialize;

use crate::binary::MainMemory;
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
		memory: MainMemory,
		args: Vec<OsString>,
	},
}
