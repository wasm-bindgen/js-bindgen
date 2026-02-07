use crate::hazard::Input;

pub(crate) struct PtrLength(
	#[cfg(target_arch = "wasm32")] usize,
	#[cfg(target_arch = "wasm64")] f64,
);

impl PtrLength {
	pub(crate) fn new<T>(
		#[cfg_attr(target_arch = "wasm32", expect(unused_variables))] ptr: *const T,
		len: usize,
	) -> Self {
		#[cfg(target_arch = "wasm64")]
		let len = {
			debug_assert!(
				ptr as usize + len * core::mem::size_of::<T>() < 0x20000000000000,
				"found pointer + length bigger than `Number.MAX_SAFE_INTEGER`"
			);
			len as f64
		};

		Self(len)
	}
}

unsafe impl Input for PtrLength {
	const IMPORT_TYPE: &str = Self::Type::IMPORT_TYPE;
	const TYPE: &str = Self::Type::TYPE;
	const JS_CONV: &str = Self::Type::JS_CONV;

	#[cfg(target_arch = "wasm32")]
	type Type = usize;
	#[cfg(target_arch = "wasm64")]
	type Type = f64;

	fn into_raw(self) -> Self::Type {
		self.0
	}
}
