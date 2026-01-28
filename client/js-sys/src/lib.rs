#![no_std]
#![cfg_attr(target_feature = "atomics", feature(thread_local))]
#![cfg_attr(target_arch = "wasm64", feature(simd_wasm64))]

extern crate alloc;

mod externref;
pub mod hazard;
#[doc(hidden)]
pub mod r#macro;
mod panic;

use core::marker::PhantomData;
use core::ops::Deref;

pub use js_bindgen;
pub use js_sys_macro::js_sys;

use crate::externref::EXTERNREF_TABLE;
use crate::hazard::{Input, Output};
pub use crate::panic::{UnwrapThrowExt, panic};

#[cfg(not(target_feature = "reference-types"))]
compile_error!("`js-sys` requires the `reference-types` target feature");

#[repr(transparent)]
pub struct JsValue {
	index: i32,
	_local: PhantomData<*const ()>,
}

impl JsValue {
	pub const UNDEFINED: Self = Self::new(0);

	const fn new(index: i32) -> Self {
		Self {
			index,
			_local: PhantomData,
		}
	}
}

impl Drop for JsValue {
	fn drop(&mut self) {
		if self.index > 0 {
			EXTERNREF_TABLE.with(|table| table.try_borrow_mut().unwrap().remove(self.index));
		}
	}
}

unsafe impl Input for &JsValue {
	const IMPORT_FUNC: &'static str = ".functype js_sys.externref.get (i32) -> (externref)";
	const IMPORT_TYPE: &'static str = "externref";
	const TYPE: &'static str = "i32";
	const CONV: &'static str = "call js_sys.externref.get";

	type Type = i32;

	fn into_raw(self) -> Self::Type {
		self.index
	}
}

unsafe impl Output for JsValue {
	const IMPORT_FUNC: &str = ".functype js_sys.externref.insert (externref) -> (i32)";
	const IMPORT_TYPE: &str = "externref";
	const TYPE: &str = "i32";
	const CONV: &str = "call js_sys.externref.insert";

	type Type = i32;

	fn from_raw(raw: Self::Type) -> Self {
		Self::new(raw)
	}
}

js_bindgen::embed_js!(
	name = "string.decode",
	"(ptr, len) => {{",
	"	const decoder = new TextDecoder(\"utf-8\", {{",
	"		fatal: false,",
	"		ignoreBOM: false,",
	"	}})",
	#[cfg(not(target_feature = "atomics"))]
	"	const view = new Uint8Array(memory.buffer, ptr, len)",
	#[cfg(target_feature = "atomics")]
	"	const view = new Uint8Array(memory.buffer).slice(ptr, ptr + len)",
	"",
	"	return decoder.decode(view)",
	"}}",
);

#[js_sys(js_sys = crate)]
extern "C" {
	#[cfg(target_arch = "wasm32")]
	#[js_sys(js_embed = "string.decode")]
	fn string_decode(array: *const u8, len: usize) -> JsString;

	#[cfg(target_arch = "wasm64")]
	#[js_sys(js_embed = "string.decode")]
	fn string_decode(array: *const u8, len: f64) -> JsString;
}

#[repr(transparent)]
pub struct JsString(JsValue);

impl JsString {
	#[allow(clippy::should_implement_trait)]
	pub fn from_str(string: &str) -> Self {
		#[cfg(target_arch = "wasm32")]
		let len = string.len();
		#[cfg(target_arch = "wasm64")]
		let len = {
			let len = string.len();
			debug_assert!(
				string.as_ptr() as usize + len < 0x20000000000000,
				"found pointer + string length bigger than `Number.MAX_SAFE_INTEGER`"
			);
			len as f64
		};

		string_decode(string.as_ptr(), len)
	}
}

impl Deref for JsString {
	type Target = JsValue;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

unsafe impl Input for &JsString {
	const IMPORT_FUNC: &'static str = ".functype js_sys.externref.get (i32) -> (externref)";
	const IMPORT_TYPE: &'static str = "externref";
	const TYPE: &'static str = "i32";
	const CONV: &'static str = "call js_sys.externref.get";

	type Type = i32;

	fn into_raw(self) -> Self::Type {
		self.0.into_raw()
	}
}

unsafe impl Output for JsString {
	const IMPORT_FUNC: &str = ".functype js_sys.externref.insert (externref) -> (i32)";
	const IMPORT_TYPE: &str = "externref";
	const TYPE: &str = "i32";
	const CONV: &str = "call js_sys.externref.insert";

	type Type = i32;

	fn from_raw(raw: Self::Type) -> Self {
		Self(JsValue::from_raw(raw))
	}
}

unsafe impl Input for u32 {
	const IMPORT_TYPE: &str = "i32";
	const TYPE: &str = "i32";
	const JS_CONV: &str = " >>>= 0";

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		self
	}
}

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

unsafe impl Input for f64 {
	const IMPORT_TYPE: &str = "f64";
	const TYPE: &str = "f64";

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		self
	}
}

unsafe impl Input for *const u8 {
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
