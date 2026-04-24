#[rustfmt::skip]
#[path ="value.gen.rs"]
mod value;

use core::marker::PhantomData;

use crate::externref::EXTERNREF_TABLE;
use crate::hazard::{Input, InputAsmConv, Output, OutputAsmConv};

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

impl Clone for JsValue {
	fn clone(&self) -> Self {
		js_bindgen::unsafe_embed_asm!(
			"(module (@rwat)",
	        #[cfg(target_arch = "wasm64")]
            "  (import \"env\" \"__linear_memory\" (memory i64 0))",
			"  (import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result externref)))",
			"  (import \"env\" \"js_sys.externref.insert\" (func $js_sys.externref.insert (@sym) (param externref) (result i32)))",
			"  (func $js_sys.js_value.clone (@sym) (param i32) (result i32)",
			"    local.get 0",
			"    call $js_sys.externref.get (@reloc)",
			"    call $js_sys.externref.insert (@reloc)",
			"  )",
			")",
		);

		unsafe extern "C" {
			#[link_name = "js_sys.js_value.clone"]
			safe fn clone(size: i32) -> i32;
		}

		Self::new(clone(self.index))
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
	const ASM_TYPE: &'static str = "i32";
	const ASM_CONV: Option<InputAsmConv> = Some(InputAsmConv {
		import: Some(
			"(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result externref)))",
		),
		conv: "call $js_sys.externref.get (@reloc)",
		r#type: "externref",
	});

	type Type = i32;

	fn into_raw(self) -> Self::Type {
		self.index
	}
}

// SAFETY: Implementation for all `JsValue`s.
unsafe impl Output for JsValue {
	const ASM_TYPE: &str = "i32";
	const ASM_CONV: Option<OutputAsmConv> = Some(OutputAsmConv {
		import: Some(
			"(import \"env\" \"js_sys.externref.insert\" (func $js_sys.externref.insert (@sym) (param externref) (result i32)))",
		),
		direct: true,
		conv: "call $js_sys.externref.insert (@reloc)",
		r#type: "externref",
	});

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
