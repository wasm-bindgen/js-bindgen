use crate::hazard::{Input, Output};

// SAFETY: Implementation.
unsafe impl Input for bool {
	const IMPORT_TYPE: &str = "i32";
	const TYPE: &str = "i32";

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		self
	}
}

// SAFETY: Implementation.
unsafe impl Output for bool {
	const IMPORT_TYPE: &str = "i32";
	const TYPE: &str = "i32";

	type Type = Self;

	fn from_raw(raw: Self::Type) -> Self {
		raw
	}
}

// SAFETY: Implementation.
unsafe impl Input for u32 {
	const IMPORT_TYPE: &str = "i32";
	const TYPE: &str = "i32";
	const JS_CONV: &str = " >>>= 0";

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		self
	}
}

// SAFETY: Implementation.
unsafe impl Output for u32 {
	const IMPORT_TYPE: &str = "i32";
	const TYPE: &str = "i32";

	type Type = Self;

	fn from_raw(raw: Self::Type) -> Self {
		raw
	}
}

// SAFETY: Implementation.
unsafe impl Input for usize {
	#[cfg(target_arch = "wasm32")]
	const IMPORT_TYPE: &str = "i32";
	#[cfg(target_arch = "wasm64")]
	const IMPORT_TYPE: &str = "i64";
	#[cfg(target_arch = "wasm32")]
	const TYPE: &str = "i32";
	#[cfg(target_arch = "wasm64")]
	const TYPE: &str = "i64";
	#[cfg(target_arch = "wasm32")]
	const JS_CONV: &str = " >>>= 0";

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		self
	}
}

// SAFETY: Implementation.
unsafe impl Input for f64 {
	const IMPORT_TYPE: &str = "f64";
	const TYPE: &str = "f64";

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		self
	}
}

// SAFETY: Implementation.
unsafe impl<T> Input for *const T {
	#[cfg(target_arch = "wasm32")]
	const IMPORT_TYPE: &str = "i32";
	#[cfg(target_arch = "wasm64")]
	const IMPORT_TYPE: &str = "f64";
	#[cfg(target_arch = "wasm32")]
	const TYPE: &str = "i32";
	#[cfg(target_arch = "wasm64")]
	const TYPE: &str = "f64";
	#[cfg(target_arch = "wasm32")]
	const JS_CONV: &str = " >>>= 0";

	#[cfg(target_arch = "wasm32")]
	type Type = Self;
	#[cfg(target_arch = "wasm64")]
	type Type = f64;

	#[cfg(target_arch = "wasm32")]
	fn into_raw(self) -> Self::Type {
		self
	}

	#[cfg(target_arch = "wasm64")]
	fn into_raw(self) -> Self::Type {
		let addr = self as usize;
		debug_assert!(
			addr < 0x20000000000000,
			"found pointer bigger than `Number.MAX_SAFE_INTEGER`"
		);
		addr as f64
	}
}
