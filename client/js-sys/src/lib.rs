#![no_std]
#![cfg_attr(target_feature = "atomics", feature(thread_local))]
#![cfg_attr(target_arch = "wasm64", feature(simd_wasm64))]

extern crate alloc;

mod array;
mod externref;
pub mod hazard;
#[doc(hidden)]
pub mod r#macro;
mod numeric;
mod panic;
mod string;
mod util;

use core::marker::PhantomData;

pub use js_bindgen;
pub use js_sys_macro::js_sys;

pub use crate::array::JsArray;
use crate::externref::EXTERNREF_TABLE;
use crate::hazard::{Input, Output};
pub use crate::panic::{UnwrapThrowExt, panic};
pub use crate::string::JsString;

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
