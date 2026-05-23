#[link(wasm_import_module = "wabii")]
unsafe extern "C" {
	#[link_name = "stdio.stdout"]
	pub fn stdout(ptr: *const u8, len: usize);
	#[link_name = "stdio.stderr"]
	pub fn stderr(ptr: *const u8, len: usize);
}

#[cfg(target_arch = "wasm32")]
include_wat!("stdio.wat");
#[cfg(target_arch = "wasm64")]
include_wat!("stdio.64.wat");

#[cfg(test)]
mod tests {
	use js_bindgen_test::test;

	use super::{stderr, stdout};

	#[test]
	pub fn test_stdio() {
		let text = b"hello world\n";
		#[expect(clippy::undocumented_unsafe_blocks, reason = "just test")]
		unsafe {
			stdout(text.as_ptr(), text.len());
			stderr(text.as_ptr(), text.len());
		}
	}
}
