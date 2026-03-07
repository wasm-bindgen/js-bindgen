use core::cell::Cell;
use core::mem;
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
			required_embeds = [("js_sys", "isLittleEndian")],
			"(ptr) => {{",
			"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
			"		const view = new BigUint64Array(this.#memory.buffer, ptr, 2)",
			"		return view[0] | (view[1] << 64n)",
			"	}} else {{",
			"		const view = new DataView(this.#memory.buffer, ptr, {})",
			"		const lo = view.getBigUint64(0, true)",
			"		const hi = view.getBigUint64(8, true)",
			"		return lo | (hi << 64n)",
			"	}}",
			"}}",
			const mem::size_of::<[u8; 16]>(),
		);

		ExternValue::new(self.to_le_bytes())
	}
}

// SAFETY: Implementation.
unsafe impl Output for u128 {
	const ASM_IMPORT_FUNC: Option<&str> =
		Some(".functype js_sys.numeric.u128 (i32, i32, i32, i32) -> ()");
	const ASM_IMPORT_TYPE: &str = "i32, i32, i32, i32";
	const ASM_TYPE: &str = "";
	const ASM_CONV: Option<&str> = Some("call js_sys.numeric.u128");
	const JS_EMBED: Option<(&str, &str)> = Some(("js_sys", "numeric.128.encode"));
	const JS_CONV: Option<(&str, &str)> =
		Some(("this.#jsEmbed.js_sys['numeric.128.encode'](", ")"));

	type Type = ();

	fn from_raw((): Self::Type) -> Self {
		thread_local! {
			static CACHE: Cell<[u32; 4]> = Cell::new([0; 4]);
		}

		#[unsafe(export_name = "js_sys.numeric.u128")]
		fn convert(lo_lo: u32, lo_hi: u32, hi_lo: u32, hi_hi: u32) {
			CACHE.with(|cache| cache.set([lo_lo, lo_hi, hi_lo, hi_hi]));
		}

		CACHE.with(|cache| {
			let [lo_lo, lo_hi, hi_lo, hi_hi] = cache.get();
			Self::from(lo_lo)
				| Self::from(lo_hi) << 32
				| Self::from(hi_lo) << 64
				| Self::from(hi_hi) << 96
		})
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
			required_embeds = [("js_sys", "isLittleEndian")],
			"(ptr) => {{",
			"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
			"		const viewU64 = new BigUint64Array(this.#memory.buffer, ptr, 1)",
			"		const viewI64 = new BigInt64Array(this.#memory.buffer, ptr + 8, 1)",
			"		return viewU64[0] | (viewI64[0] << 64n)",
			"	}} else {{",
			"		const view = new DataView(this.#memory.buffer, ptr, {})",
			"		const lo = view.getBigUint64(0, true)",
			"		const hi = view.getBigInt64(8, true)",
			"		return lo | (hi << 64n)",
			"	}}",
			"}}",
			const mem::size_of::<[u8; 16]>(),
		);

		ExternValue::new(self.to_le_bytes())
	}
}

// SAFETY: Implementation.
unsafe impl Output for i128 {
	const ASM_IMPORT_FUNC: Option<&str> =
		Some(".functype js_sys.numeric.i128 (i32, i32, i32, i32) -> ()");
	const ASM_IMPORT_TYPE: &str = "i32, i32, i32, i32";
	const ASM_TYPE: &str = "";
	const ASM_CONV: Option<&str> = Some("call js_sys.numeric.i128");
	const JS_EMBED: Option<(&str, &str)> = Some(("js_sys", "numeric.128.encode"));
	const JS_CONV: Option<(&str, &str)> =
		Some(("this.#jsEmbed.js_sys['numeric.128.encode'](", ")"));

	type Type = ();

	fn from_raw((): Self::Type) -> Self {
		thread_local! {
			static CACHE: Cell<(u32, u32, u32, i32)> = Cell::new((0, 0, 0, 0));
		}

		#[unsafe(export_name = "js_sys.numeric.i128")]
		fn convert(lo_lo: u32, lo_hi: u32, hi_lo: u32, hi_hi: i32) {
			CACHE.with(|cache| cache.set((lo_lo, lo_hi, hi_lo, hi_hi)));
		}

		CACHE.with(|cache| {
			let (lo_lo, lo_hi, hi_lo, hi_hi) = cache.get();
			Self::from(lo_lo)
				| Self::from(lo_hi) << 32
				| Self::from(hi_lo) << 64
				| Self::from(hi_hi) << 96
		})
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "numeric.128.encode",
	"(value) => {{",
	"	const lo_lo = Number(value & 0xFFFFFFFFn)",
	"	const lo_hi = Number((value >> 32n) & 0xFFFFFFFFn)",
	"	const hi_lo = Number((value >> 64n) & 0xFFFFFFFFn)",
	"	const hi_hi = Number((value >> 96n) & 0xFFFFFFFFn)",
	"	return [lo_lo, lo_hi, hi_lo, hi_hi]",
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
