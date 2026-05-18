//! Record previous benchmark data

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use js_sys::JsString;
use serde::{Deserialize, Serialize};

use super::SavedSample;
use super::estimate::Estimates;
use crate::console_error;
use crate::context::ctx;
use crate::fs::write_file;
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

#[derive(Deserialize)]
struct BenchCtx {
	path: String,
	baseline: Option<String>,
}

/// Used to read previous benchmark data before the benchmark, for later
/// comparison.
pub(crate) fn import_baseline() {
	fn set() -> serde_json::Result<()> {
		let value = serde_json::from_str::<serde_json::Value>(&ctx().to_string())?;
		let ctx = serde_json::from_value::<BenchCtx>(value)?;
		let Some(baseline) = ctx.baseline else {
			return Ok(());
		};
		let baseline = serde_json::from_str(&baseline)?;
		*BASELINE.borrow_mut() = baseline;
		Ok(())
	}

	if let Err(e) = set() {
		console_error!("Failed to import previous benchmark: {e}");
	}
}

/// Used to read benchmark data, and then the runner stores it on the local
/// disk.
pub(crate) fn dump_baseline() {
	fn path() -> serde_json::Result<String> {
		let value = serde_json::from_str::<serde_json::Value>(&ctx().to_string())?;
		let ctx = serde_json::from_value::<BenchCtx>(value)?;
		Ok(ctx.path)
	}

	let baseline = BASELINE.borrow();
	if let Ok(path) = path() {
		if !baseline.is_empty() {
			let baseline = JsString::from(
				serde_json::to_string(&*baseline)
					.unwrap_or_default()
					.as_str(),
			);
			write_file(&JsString::from(path.as_str()), &baseline);
		}
	}
}
