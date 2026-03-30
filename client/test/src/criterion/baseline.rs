//! Record previous benchmark data

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use js_sys::{JsString, js_sys};
use serde::{Deserialize, Serialize};

use super::SavedSample;
use super::estimate::Estimates;
use crate::console_log;
use crate::utils::LazyCell;

#[cfg_attr(target_feature = "atomics", thread_local)]
static BASELINE: LazyCell<RefCell<BTreeMap<String, BenchmarkBaseline>>> =
	LazyCell::new(|| RefCell::new(BTreeMap::new()));

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct BenchmarkBaseline {
	pub(crate) file: Option<String>,
	pub(crate) module_path: Option<String>,
	pub(crate) iters: Vec<f64>,
	pub(crate) times: Vec<f64>,
	pub(crate) sample: SavedSample,
	pub(crate) estimates: Estimates,
}

/// Write the corresponding benchmark ID and corresponding data into the table.
pub(crate) fn write(id: &str, baseline: BenchmarkBaseline) {
	BASELINE.borrow_mut().insert(id.into(), baseline);
}

/// Read the data corresponding to the benchmark ID from the table.
pub(crate) fn read(id: &str) -> Option<BenchmarkBaseline> {
	BASELINE.borrow().get(id).cloned()
}

#[js_sys]
extern "js-sys" {
	#[js_sys(js_import)]
	fn import_bench_baseline() -> JsString;

	#[js_sys(js_import)]
	fn dump_bench_baseline(baseline: &JsString);
}

/// Used to read previous benchmark data before the benchmark, for later
/// comparison.
pub(crate) fn import_baseline() {
	match serde_json::from_str(&String::from(&import_bench_baseline())) {
		Ok(prev) => {
			*BASELINE.borrow_mut() = prev;
		}
		Err(e) => {
			console_log!("Failed to import previous benchmark {e:?}");
		}
	}
}

/// Used to read benchmark data, and then the runner stores it on the local
/// disk.
pub(crate) fn dump_baseline() {
	let baseline = BASELINE.borrow();
	if !baseline.is_empty() {
		let baseline = JsString::from(
			serde_json::to_string(&*baseline)
				.unwrap_or_default()
				.as_str(),
		);
		dump_bench_baseline(&baseline);
	}
}
