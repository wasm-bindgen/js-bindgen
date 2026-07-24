#[link(wasm_import_module = "wabii")]
unsafe extern "C" {
	#[cfg(target_feature = "atomics")]
	#[link_name = "random.atomics_fill"]
	pub fn random_fill(ptr: *mut u8, len: usize);
	#[cfg(not(target_feature = "atomics"))]
	#[link_name = "random.fill"]
	pub fn random_fill(ptr: *mut u8, len: usize);
}

#[cfg(target_arch = "wasm32")]
include_wat!("random.wat");
#[cfg(target_arch = "wasm64")]
include_wat!("random.64.wat");

#[cfg(test)]
mod tests {
	use js_bindgen_test::test;

	use super::random_fill;

	#[test]
	pub fn test_random_fill() {
		let mut buf1 = [0; 10];
		let mut buf2 = [0; 10];
		#[expect(clippy::undocumented_unsafe_blocks, reason = "just test")]
		unsafe {
			random_fill(buf1.as_mut_ptr(), buf1.len());
			random_fill(buf2.as_mut_ptr(), buf2.len());
		}
		assert_ne!(buf1, buf2);
	}
}
