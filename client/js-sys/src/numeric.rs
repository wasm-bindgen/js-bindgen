use core::mem;
#[cfg(target_arch = "wasm64")]
use core::ptr;
use core::ptr::NonNull;

use crate::hazard::{Input, InputAsmConv, InputJsConv, Output, OutputAsmConv, OutputJsConv};
use crate::r#macro::const_concat;
use crate::util::{ASM_PTR_TYPE, ExternValue};

macro_rules! input_output {
	($wasm:literal, $($ty:ty),*) => {$(
		// SAFETY: Implementation.
		unsafe impl Input for $ty {
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
			const ASM_TYPE: &str = <$origin as Input>::ASM_TYPE;
			const JS_CONV: Option<InputJsConv> = <$origin as Input>::JS_CONV;


			type Type = $ty_impl;

			$into_raw
		}

		// SAFETY: Implementation.
		unsafe impl <$($ge)?> Output for $ty {
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
	const ASM_TYPE: &str = "i32";
	const JS_CONV: Option<InputJsConv> = Some(InputJsConv {
		embed: None,
		pre: " = !!",
		post: Some(""),
	});

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		self
	}
}

// SAFETY: Implementation.
unsafe impl Input for u32 {
	const ASM_TYPE: &str = "i32";
	const JS_CONV: Option<InputJsConv> = Some(InputJsConv {
		embed: None,
		pre: " >>>= 0",
		post: None,
	});

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		self
	}
}

// SAFETY: Implementation.
unsafe impl Input for u64 {
	const ASM_TYPE: &str = "i64";
	const JS_CONV: Option<InputJsConv> = Some(InputJsConv {
		embed: None,
		pre: " = BigInt.asUintN(64, ",
		post: Some(")"),
	});

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		self
	}
}

// SAFETY: Implementation.
unsafe impl Input for u128 {
	const ASM_TYPE: &str = Self::Type::ASM_TYPE;
	const ASM_CONV: Option<InputAsmConv> = Self::Type::ASM_CONV;
	const JS_CONV: Option<InputJsConv> = Some(InputJsConv {
		embed: Some(("js_sys", "numeric.u128.decode")),
		pre: " = this.#jsEmbed.js_sys['numeric.u128.decode'](",
		post: Some(")"),
	});

	type Type = ExternValue<AlignedValue>;

	fn into_raw(self) -> Self::Type {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "numeric.u128.decode",
			required_embeds = [("js_sys", "view.getBigUint64")],
			"(ptr) => {{",
			"	const [lo, hi] = this.#jsEmbed.js_sys['view.getBigUint64'](ptr, 2)",
			"	return lo | (hi << 64n)",
			"}}",
		);

		ExternValue::new(AlignedValue(self.to_le_bytes()))
	}
}

// SAFETY: Implementation.
unsafe impl Output for u128 {
	const ASM_TYPE: &str = ASM_PTR_TYPE;
	const ASM_CONV: Option<OutputAsmConv> = Some(OutputAsmConv {
		import: Some(const_concat!(
			"(import \"env\" \"js_sys.numeric.128\" (func $js_sys.numeric.128 (@sym) (param i32 \
			 i32 i32 i32 ",
			ASM_PTR_TYPE,
			")))"
		)),
		direct: false,
		conv: "call $js_sys.numeric.128 (@reloc)",
		r#type: "i32 i32 i32 i32",
	});
	const JS_CONV: Option<OutputJsConv> = Some(OutputJsConv {
		embed: Some(("js_sys", "numeric.128.encode")),
		pre: "this.#jsEmbed.js_sys['numeric.128.encode'](",
		post: ")",
	});

	type Type = Self;

	fn from_raw(raw: Self::Type) -> Self {
		raw
	}
}

// SAFETY: Implementation.
unsafe impl Input for i128 {
	const ASM_TYPE: &str = Self::Type::ASM_TYPE;
	const ASM_CONV: Option<InputAsmConv> = Self::Type::ASM_CONV;
	const JS_CONV: Option<InputJsConv> = Some(InputJsConv {
		embed: Some(("js_sys", "numeric.i128.decode")),
		pre: " = this.#jsEmbed.js_sys['numeric.i128.decode'](",
		post: Some(")"),
	});

	type Type = ExternValue<AlignedValue>;

	fn into_raw(self) -> Self::Type {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "numeric.i128.decode",
			required_embeds = [
				("js_sys", "view.getBigUint64"),
				("js_sys", "view.getBigInt64")
			],
			"(ptr) => {{",
			"	const [lo] = this.#jsEmbed.js_sys['view.getBigUint64'](ptr, 1)",
			"	const [hi] = this.#jsEmbed.js_sys['view.getBigInt64'](ptr + 8, 1)",
			"	return lo | (hi << 64n)",
			"}}",
		);

		ExternValue::new(AlignedValue(self.to_le_bytes()))
	}
}

// SAFETY: Implementation.
unsafe impl Output for i128 {
	const ASM_TYPE: &str = ASM_PTR_TYPE;
	const ASM_CONV: Option<OutputAsmConv> = Some(OutputAsmConv {
		import: Some(const_concat!(
			"(import \"env\" \"js_sys.numeric.128\" (func $js_sys.numeric.128 (@sym) (param i32 \
			 i32 i32 i32 ",
			ASM_PTR_TYPE,
			")))"
		)),
		direct: false,
		conv: "call $js_sys.numeric.128 (@reloc)",
		r#type: "i32 i32 i32 i32",
	});
	const JS_CONV: Option<OutputJsConv> = Some(OutputJsConv {
		embed: Some(("js_sys", "numeric.128.encode")),
		pre: "this.#jsEmbed.js_sys['numeric.128.encode'](",
		post: ")",
	});

	type Type = Self;

	fn from_raw(raw: Self::Type) -> Self {
		raw
	}
}

#[repr(C, align(8))]
pub struct AlignedValue([u8; 16]);

const _: () = {
	debug_assert!(mem::align_of::<ExternValue<AlignedValue>>() == 8);
};

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

js_bindgen::unsafe_embed_asm!(
	"(module (@rwat)",
	#[cfg(target_arch = "wasm64")]
	"  (import \"env\" \"__linear_memory\" (memory i64 0))",
	"  (func $js_sys.numeric.128 (@sym) (param i32 i32 i32 i32 {})",
	"    local.get 4",
	"    local.get 0",
	"    i32.store",
	"    local.get 4",
	"    local.get 1",
	"    i32.store offset=4",
	"    local.get 4",
	"    local.get 2",
	"    i32.store offset=8",
	"    local.get 4",
	"    local.get 3",
	"    i32.store offset=12",
	"  )",
	")",
	interpolate ASM_PTR_TYPE,
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
	ptr.addr() as f64
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
