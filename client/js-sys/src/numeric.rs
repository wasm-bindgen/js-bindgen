use core::mem;

use crate::hazard::{Input, InputJsConv, InputWatConv, Output, OutputJsConv, OutputWatConv};
use crate::r#macro::const_concat;
use crate::util::{ExternValue, WAT_PTR_TYPE};

macro_rules! input_output {
	($wasm:literal, $($ty:ty),*) => {$(
		// SAFETY: Implementation.
		unsafe impl Input for $ty {
			const WAT_TYPE: &str = $wasm;

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
			const WAT_TYPE: &str = $wasm;

			type Type = Self;

			fn from_raw(raw: Self::Type) -> Self {
				raw
			}
		}
	)*};
}

output!("i32", bool);

input_output!("i32", u8, u16);
output!("i32", u32);
output!("i64", u64);

input_output!("i32", i8, i16, i32);
input_output!("i64", i64);

input_output!("f32", f32);
input_output!("f64", f64);

// SAFETY: Implementation.
unsafe impl Input for bool {
	const WAT_TYPE: &str = "i32";
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
	const WAT_TYPE: &str = "i32";
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
	const WAT_TYPE: &str = "i64";
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
	const WAT_TYPE: &str = Self::Type::WAT_TYPE;
	const WAT_CONV: Option<InputWatConv> = Self::Type::WAT_CONV;
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
	const WAT_TYPE: &str = WAT_PTR_TYPE;
	const WAT_CONV: Option<OutputWatConv> = Some(OutputWatConv {
		import: Some(const_concat!(
			"(import \"env\" \"js_sys.numeric.128\" (func $js_sys.numeric.128 (@sym) (param i32 \
			 i32 i32 i32 ",
			WAT_PTR_TYPE,
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
	const WAT_TYPE: &str = Self::Type::WAT_TYPE;
	const WAT_CONV: Option<InputWatConv> = Self::Type::WAT_CONV;
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
	const WAT_TYPE: &str = WAT_PTR_TYPE;
	const WAT_CONV: Option<OutputWatConv> = Some(OutputWatConv {
		import: Some(const_concat!(
			"(import \"env\" \"js_sys.numeric.128\" (func $js_sys.numeric.128 (@sym) (param i32 \
			 i32 i32 i32 ",
			WAT_PTR_TYPE,
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

js_bindgen::unsafe_global_wat!(
	"(func $js_sys.numeric.128 (@sym) (param i32 i32 i32 i32 {})",
	"  local.get 4",
	"  local.get 0",
	"  i32.store",
	"  local.get 4",
	"  local.get 1",
	"  i32.store offset=4",
	"  local.get 4",
	"  local.get 2",
	"  i32.store offset=8",
	"  local.get 4",
	"  local.get 3",
	"  i32.store offset=12",
	")",
	interpolate WAT_PTR_TYPE,
);
