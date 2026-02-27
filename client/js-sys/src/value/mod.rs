#[rustfmt::skip]
#[path ="value.gen.rs"]
mod value;

use core::marker::PhantomData;

use crate::externref::EXTERNREF_TABLE;
use crate::hazard::{Input, Output};

#[derive(Debug)]
#[repr(transparent)]
pub struct JsValue {
	index: i32,
	_local: PhantomData<*const ()>,
}

impl JsValue {
	pub const UNDEFINED: Self = Self::new(0);
	pub const NULL: Self = Self::new(1);

	const fn new(index: i32) -> Self {
		Self {
			index,
			_local: PhantomData,
		}
	}
}

impl Drop for JsValue {
	fn drop(&mut self) {
		if self.index > 1 {
			EXTERNREF_TABLE.with(|table| table.try_borrow_mut().unwrap().remove(self.index));
		}
	}
}

// SAFETY: Implementation for all `JsValue`s.
unsafe impl Input for &JsValue {
	const ASM_IMPORT_FUNC: Option<&'static str> =
		Some(".functype js_sys.externref.get (i32) -> (externref)");
	const ASM_IMPORT_TYPE: &'static str = "externref";
	const ASM_TYPE: &'static str = "i32";
	const ASM_CONV: Option<&'static str> = Some("call js_sys.externref.get");

	type Type = i32;

	fn into_raw(self) -> Self::Type {
		self.index
	}
}

// SAFETY: Implementation for all `JsValue`s.
unsafe impl Output for JsValue {
	const ASM_IMPORT_FUNC: Option<&str> =
		Some(".functype js_sys.externref.insert (externref) -> (i32)");
	const ASM_IMPORT_TYPE: &str = "externref";
	const ASM_TYPE: &str = "i32";
	const ASM_CONV: Option<&str> = Some("call js_sys.externref.insert");

	type Type = i32;

	fn from_raw(raw: Self::Type) -> Self {
		Self::new(raw)
	}
}

impl PartialEq for JsValue {
	fn eq(&self, other: &Self) -> bool {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "js_value.partial_eq",
			"(value1, value2) => value1 === value2",
		);

		value::js_value_partial_eq(self, other)
	}
}
