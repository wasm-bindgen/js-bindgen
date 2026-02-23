use core::mem::MaybeUninit;

use crate::hazard::Input;

pub(crate) struct PtrLength(
	#[cfg(target_arch = "wasm32")] usize,
	#[cfg(target_arch = "wasm64")] f64,
);

impl PtrLength {
	pub(crate) fn new<T>(value: &[T]) -> Self {
		Self::internal(value.as_ptr(), value.len())
	}

	pub(crate) fn from_uninit_array<T, const N: usize>(value: &MaybeUninit<[T; N]>) -> Self {
		Self::internal(value.as_ptr(), N)
	}

	fn internal<T>(
		#[cfg_attr(
			not(target_arch = "wasm64"),
			expect(unused_variables, reason = "only needed for Wasm64")
		)]
		ptr: *const T,
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

// SAFETY: Delegated to already implemented types.
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
