use core::cell::Cell;
#[cfg(target_arch = "wasm64")]
use core::ptr;
use core::ptr::NonNull;

use crate::hazard::{Input, Output};
use crate::util::ExternValue;

macro_rules! input_output {
	($wasm:literal, $($ty:ty),*) => {$(
		// SAFETY: Implementation.
		unsafe impl Input for $ty {
			const ASM_IMPORT_TYPE: &str = $wasm;
			const ASM_TYPE: &str = $wasm;

			type Type = Self;

			fn into_raw(self) -> Self::Type {
				self
			}
		}

		output!($wasm, $ty);
	)*};
}

macro_rules! output {
	($wasm:literal, $($ty:ty),*) => {$(
		// SAFETY: Implementation.
		unsafe impl Output for $ty {
			const ASM_IMPORT_TYPE: &str = $wasm;
			const ASM_TYPE: &str = $wasm;

			type Type = Self;

			fn from_raw(raw: Self::Type) -> Self {
				raw
			}
		}
	)*};
}

macro_rules! delegate {
	($origin:ty, $ty:ty $(:<$ge:tt>)?) => {
		delegate!($origin, $ty $(:<$ge>)?, $ty);
	};
	($origin:ty, $ty:ty $(:<$ge:tt>)?, $ty_impl:ty) => {
		delegate!(
			$origin, $ty $(:<$ge>)?, $ty_impl,
			fn into_raw(self) -> Self::Type {
				self
			}
		);
	};
	($origin:ty, $ty:ty $(:<$ge:tt>)?, $ty_impl:ty, $into_raw:item) => {
		delegate!(
			$origin, $ty $(:<$ge>)?, $ty_impl, $into_raw
			fn from_raw(raw: Self::Type) -> Self {
				raw
			}
		);
	};
	($origin:ty, $ty:ty $(:<$ge:tt>)?, $ty_impl:ty, $into_raw:item $from_raw:item) => {
		// SAFETY: Implementation.
		unsafe impl <$($ge)?> Input for $ty {
			const ASM_IMPORT_TYPE: &str = <$origin as Input>::ASM_IMPORT_TYPE;
			const ASM_TYPE: &str = <$origin as Input>::ASM_TYPE;
			const JS_CONV: Option<(&str, Option<&str>)> = <$origin as Input>::JS_CONV;


			type Type = $ty_impl;

			$into_raw
		}

		// SAFETY: Implementation.
		unsafe impl <$($ge)?> Output for $ty {
			const ASM_IMPORT_TYPE: &str = <$origin as Output>::ASM_IMPORT_TYPE;
			const ASM_TYPE: &str = <$origin as Output>::ASM_TYPE;

			type Type = $ty_impl;

			$from_raw
		}
	};
}

output!("i32", bool);

input_output!("i32", u8, u16);
output!("i32", u32);
output!("i64", u64);
#[cfg(target_arch = "wasm32")]
delegate!(u32, usize);
#[cfg(target_arch = "wasm64")]
delegate!(u64, usize);

input_output!("i32", i8, i16, i32);
input_output!("i64", i64);
#[cfg(target_arch = "wasm32")]
input_output!("i32", isize);
#[cfg(target_arch = "wasm64")]
input_output!("i64", isize);

input_output!("f32", f32);
input_output!("f64", f64);

// SAFETY: Implementation.
unsafe impl Input for bool {
	const ASM_IMPORT_TYPE: &str = "i32";
	const ASM_TYPE: &str = "i32";
	const JS_CONV: Option<(&str, Option<&str>)> = Some((" = !!", Some("")));

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		self
	}
}

// SAFETY: Implementation.
unsafe impl Input for u32 {
	const ASM_IMPORT_TYPE: &str = "i32";
	const ASM_TYPE: &str = "i32";
	const JS_CONV: Option<(&str, Option<&str>)> = Some((" >>>= 0", None));

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		self
	}
}

// SAFETY: Implementation.
unsafe impl Input for u64 {
	const ASM_IMPORT_TYPE: &str = "i64";
	const ASM_TYPE: &str = "i64";
	const JS_CONV: Option<(&str, Option<&str>)> = Some((" = BigInt.asUintN(64, ", Some(")")));

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		self
	}
}

// SAFETY: Implementation.
unsafe impl Input for u128 {
	const ASM_IMPORT_TYPE: &str = Self::Type::ASM_IMPORT_TYPE;
	const ASM_TYPE: &str = Self::Type::ASM_TYPE;
	const ASM_CONV: Option<&str> = Self::Type::ASM_CONV;
	const JS_EMBED: Option<(&str, &str)> = Some(("js_sys", "numeric.u128.decode"));
	const JS_CONV: Option<(&str, Option<&str>)> =
		Some((" = this.#jsEmbed.js_sys['numeric.u128.decode'](", Some(")")));

	type Type = ExternValue<[u8; 16]>;

	fn into_raw(self) -> Self::Type {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "numeric.u128.decode",
			required_embeds = [("js_sys", "extern_value")],
			"(dataPtr) => {{",
			"	const {{ ptr, len }} = this.#jsEmbed.js_sys['extern_value'](dataPtr)",
			"	const view = new DataView(this.#memory.buffer, ptr, len)",
			"	const lo = view.getBigUint64(0, true)",
			"	const hi = view.getBigUint64(8, true)",
			"	return lo | (hi << 64n)",
			"}}",
		);

		ExternValue::new(self.to_le_bytes())
	}
}

// SAFETY: Implementation.
unsafe impl Output for u128 {
	const ASM_IMPORT_FUNC: Option<&str> = Some(".functype js_sys.numeric.u128 (i64, i64) -> ()");
	const ASM_IMPORT_TYPE: &str = "i64, i64";
	const ASM_TYPE: &str = "";
	const ASM_CONV: Option<&str> = Some("call js_sys.numeric.u128");
	const JS_EMBED: Option<(&str, &str)> = Some(("js_sys", "numeric.128.encode"));
	const JS_CONV: Option<(&str, &str)> =
		Some(("this.#jsEmbed.js_sys['numeric.128.encode'](", ")"));

	type Type = ();

	fn from_raw((): Self::Type) -> Self {
		thread_local! {
			static CACHE: Cell<u128> = Cell::new(0);
		}

		#[unsafe(export_name = "js_sys.numeric.u128")]
		fn convert(lo: u64, hi: u64) {
			CACHE.with(|cache| cache.set(u128::from(hi) << 64 | u128::from(lo)));
		}

		CACHE.with(Cell::get)
	}
}

// SAFETY: Implementation.
unsafe impl Input for i128 {
	const ASM_IMPORT_TYPE: &str = Self::Type::ASM_IMPORT_TYPE;
	const ASM_TYPE: &str = Self::Type::ASM_TYPE;
	const ASM_CONV: Option<&str> = Self::Type::ASM_CONV;
	const JS_EMBED: Option<(&str, &str)> = Some(("js_sys", "numeric.i128.decode"));
	const JS_CONV: Option<(&str, Option<&str>)> =
		Some((" = this.#jsEmbed.js_sys['numeric.i128.decode'](", Some(")")));

	type Type = ExternValue<[u8; 16]>;

	fn into_raw(self) -> Self::Type {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "numeric.i128.decode",
			required_embeds = [("js_sys", "extern_value")],
			"(dataPtr) => {{",
			"	const {{ ptr, len }} = this.#jsEmbed.js_sys['extern_value'](dataPtr)",
			"	const view = new DataView(this.#memory.buffer, ptr, len)",
			"	const lo = view.getBigUint64(0, true)",
			"	const hi = view.getBigInt64(8, true)",
			"	return lo | (hi << 64n)",
			"}}",
		);

		ExternValue::new(self.to_le_bytes())
	}
}

// SAFETY: Implementation.
unsafe impl Output for i128 {
	const ASM_IMPORT_FUNC: Option<&str> = Some(".functype js_sys.numeric.i128 (i64, i64) -> ()");
	const ASM_IMPORT_TYPE: &str = "i64, i64";
	const ASM_TYPE: &str = "";
	const ASM_CONV: Option<&str> = Some("call js_sys.numeric.i128");
	const JS_EMBED: Option<(&str, &str)> = Some(("js_sys", "numeric.128.encode"));
	const JS_CONV: Option<(&str, &str)> =
		Some(("this.#jsEmbed.js_sys['numeric.128.encode'](", ")"));

	type Type = ();

	fn from_raw((): Self::Type) -> Self {
		thread_local! {
			static CACHE: Cell<i128> = Cell::new(0);
		}

		#[unsafe(export_name = "js_sys.numeric.i128")]
		fn convert(lo: i64, hi: i64) {
			#[expect(clippy::cast_sign_loss, reason = "avoid sign extending")]
			CACHE.with(|cache| cache.set(i128::from(hi) << 64 | i128::from(lo as u64)));
		}

		CACHE.with(Cell::get)
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "numeric.128.encode",
	required_embeds = [("js_sys", "extern_value")],
	"(value) => {{",
	"	const lo = BigInt.asIntN(64, value & 0xFFFFFFFFFFFFFFFFn)",
	"	const hi = BigInt.asIntN(64, value >> 64n)",
	"	return [lo, hi]",
	"}}",
);

#[cfg(target_arch = "wasm32")]
delegate!(u32, *const T:<T>);
#[cfg(target_arch = "wasm64")]
delegate!(
	f64, *const T:<T>, f64,
	fn into_raw(self) -> Self::Type {
		wasm64_into_raw(self)
	}

	fn from_raw(raw: Self::Type) -> Self {
		wasm64_from_raw(raw)
	}
);

#[cfg(target_arch = "wasm32")]
delegate!(
	u32, *mut T:<T>, *mut T,
	fn into_raw(self) -> Self::Type {
		self
	}
);
#[cfg(target_arch = "wasm64")]
delegate!(
	f64, *mut T:<T>, f64,
	fn into_raw(self) -> Self::Type {
		wasm64_into_raw(self)
	}

	fn from_raw(raw: Self::Type) -> Self {
		wasm64_from_raw(raw)
	}
);

#[cfg(target_arch = "wasm32")]
delegate!(u32, NonNull<T>:<T>);
#[cfg(target_arch = "wasm64")]
delegate!(
	f64, NonNull<T>:<T>, f64,
	fn into_raw(self) -> Self::Type {
		wasm64_into_raw(self.as_ptr())
	}

	fn from_raw(raw: Self::Type) -> Self {
		NonNull::new(wasm64_from_raw(raw)).unwrap()
	}
);

#[cfg(target_arch = "wasm64")]
#[expect(clippy::cast_precision_loss, reason = "checked")]
fn wasm64_into_raw<T>(ptr: *const T) -> f64 {
	let addr = ptr.addr();
	debug_assert!(
		addr < 0x0020_0000_0000_0000,
		"found pointer bigger than `Number.MAX_SAFE_INTEGER`"
	);
	addr as f64
}

#[cfg(target_arch = "wasm64")]
#[expect(
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss,
	reason = "unchecked"
)]
fn wasm64_from_raw<T>(raw: f64) -> *mut T {
	ptr::without_provenance_mut(raw as usize)
}
