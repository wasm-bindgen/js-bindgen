#[repr(C)]
pub struct Buffer {
	pub ptr: *mut u8,
	length: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn capture_cov() -> Buffer {
	let mut cov = Vec::new();
	// SAFETY: this function is not thread-safe, but our whole test runner is running single-threaded.
	unsafe {
		minicov::capture_coverage(&mut cov).unwrap();
	}
	let leak = cov.leak();
	Buffer {
		ptr: leak.as_mut_ptr(),
		length: leak.len(),
	}
}

/// # Safety
///
/// The `buffer` must be produced by `capture_cov`,
/// and the same `buffer` must not be freed more than once.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_cov(buffer: Buffer) {
	// SAFETY: see `# Safety`
	unsafe {
		let slice = std::slice::from_raw_parts_mut(buffer.ptr, buffer.length);
		drop(Box::from_raw(slice));
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn module_signature() -> u64 {
	minicov::module_signature()
}

#[macro_export]
macro_rules! ensure_linked {
	() => {
		const _: extern "C" fn() -> wcov::Buffer = wcov::capture_cov;
		const _: unsafe extern "C" fn(wcov::Buffer) = wcov::free_cov;
		const _: extern "C" fn() -> u64 = wcov::module_signature;
	};
}
