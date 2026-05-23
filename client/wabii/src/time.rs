#[link(wasm_import_module = "wabii")]
unsafe extern "C" {
	#[cfg(not(target_feature = "atomics"))]
	#[link_name = "time.performance_now"]
	pub safe fn performance_now() -> f64;
	#[cfg(target_feature = "atomics")]
	#[link_name = "time.atomic_performance_now"]
	pub safe fn performance_now() -> f64;
	#[link_name = "time.date_now"]
	pub safe fn date_now() -> f64;
}

include_wat!("time.wat");

#[cfg(test)]
mod tests {
	use js_bindgen_test::test;

	use super::{date_now, performance_now};

	#[test]
	pub fn test_now() {
		let now1 = performance_now();
		let now2 = date_now();
		assert!(performance_now() - now1 >= 0.0);
		assert!(date_now() - now2 >= 0.0);
	}
}
