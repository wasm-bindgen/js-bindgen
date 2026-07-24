#[rustfmt::skip]
#[path ="value.gen.rs"]
mod value;

use core::marker::PhantomData;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::slice;

use crate::externref::EXTERNREF_TABLE;
use crate::hazard::{
	FromJS, FromJsConv, IntoJS, JsCast, OptionIntoJS, ReturnAbi, ReturnMode, Slot, WatConv,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct JsValue {
	index: i32,
	_local: PhantomData<*const ()>,
}

/// The Wasm `ABI` carrier for an owned `externref` table index.
#[doc(hidden)]
#[repr(transparent)]
pub struct JsValueAbi(i32);

/// The Wasm `ABI` carrier for a borrowed `externref` table index.
#[doc(hidden)]
#[repr(transparent)]
pub struct JsValueRefAbi(i32);

/// The Wasm `ABI` carrier for an optional `externref` table index.
#[doc(hidden)]
#[repr(transparent)]
pub struct OptionalJsValueAbi(i32);

impl Default for JsValueAbi {
	fn default() -> Self {
		Self(JsValue::UNDEFINED.index)
	}
}

// SAFETY: `JsValueAbi` transfers ownership of an `i32` table index across the
// JS boundary.
unsafe impl Slot for JsValueAbi {
	const WAT_TYPE: &'static str = "i32";
	const INTO_JS_WAT_CONV: Option<WatConv> = Some(WatConv {
		import: Some(
			"(import \"env\" \"js_sys.externref.take\" (func $js_sys.externref.take (@sym) (param \
			 i32) (result externref)))",
		),
		conv: "call $js_sys.externref.take (@reloc)",
		r#type: "externref",
	});
	const FROM_JS_WAT_CONV: Option<WatConv> = Some(WatConv {
		import: Some(
			"(import \"env\" \"js_sys.externref.insert\" (func $js_sys.externref.insert (@sym) \
			 (param externref) (result i32)))",
		),
		conv: "call $js_sys.externref.insert (@reloc)",
		r#type: "externref",
	});
}

// SAFETY: A transparent `i32` carrier is returned directly.
unsafe impl ReturnAbi for JsValueAbi {
	const MODE: ReturnMode = ReturnMode::Direct;
}

// SAFETY: `JsValueRefAbi` borrows an `externref` table entry for the duration
// of the JS call.
unsafe impl Slot for JsValueRefAbi {
	const WAT_TYPE: &'static str = "i32";
	const INTO_JS_WAT_CONV: Option<WatConv> = Some(WatConv {
		import: Some(
			"(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param \
			 i32) (result externref)))",
		),
		conv: "call $js_sys.externref.get (@reloc)",
		r#type: "externref",
	});
}

// SAFETY: `OptionalJsValueAbi` is an `i32` table index. At the JS boundary,
// null is represented by index zero and non-null `externref` values are
// inserted into the `externref` table.
unsafe impl Slot for OptionalJsValueAbi {
	const WAT_TYPE: &'static str = "i32";
	const INTO_JS_WAT_CONV: Option<WatConv> = Some(WatConv {
		import: Some(
			"(import \"env\" \"js_sys.externref.take\" (func $js_sys.externref.take (@sym) (param \
			 i32) (result externref)))",
		),
		conv: "call $js_sys.externref.take (@reloc)",
		r#type: "externref",
	});
	const FROM_JS_WAT_CONV: Option<WatConv> = Some(WatConv {
		import: Some(
			"(import \"env\" \"js_sys.optional.js_value\" (func $js_sys.optional.js_value (@sym) \
			 (param externref) (result i32)))",
		),
		conv: "call $js_sys.optional.js_value (@reloc)",
		r#type: "externref",
	});
}

// SAFETY: A transparent `i32` carrier is returned directly.
unsafe impl ReturnAbi for OptionalJsValueAbi {
	const MODE: ReturnMode = ReturnMode::Direct;
}

impl JsValue {
	pub const UNDEFINED: Self = Self::new(0);
	pub const NULL: Self = Self::new(1);

	pub(crate) const fn new(index: i32) -> Self {
		Self {
			index,
			_local: PhantomData,
		}
	}

	pub fn from_slice<T: JsCast>(slice: &[T]) -> &[Self] {
		let ptr: *const Self = slice.as_ptr().cast();
		// SAFETY: `JsCast` assumes that `T` is `#[transparent]` over a `JsValue`.
		unsafe { slice::from_raw_parts(ptr, slice.len()) }
	}

	pub fn from_slice_mut<T: JsCast>(slice: &mut [T]) -> &mut [Self] {
		let ptr: *mut Self = slice.as_mut_ptr().cast();
		// SAFETY: `JsCast` assumes that `T` is `#[transparent]` over a `JsValue`.
		unsafe { slice::from_raw_parts_mut(ptr, slice.len()) }
	}

	pub fn from_uninit_slice_mut<T: JsCast>(
		slice: &mut [MaybeUninit<T>],
	) -> &mut [MaybeUninit<Self>] {
		let ptr: *mut MaybeUninit<Self> = slice.as_mut_ptr().cast();
		// SAFETY: `JsCast` assumes that `T` is `#[transparent]` over a `JsValue`.
		unsafe { slice::from_raw_parts_mut(ptr, slice.len()) }
	}

	// MSRV: This functionality will be removed in v1.95 when the standard library
	// has more convenient functions to cast `MaybeUninit` arrays.
	pub(crate) fn from_mut_uninit_array<T: JsCast, const N: usize>(
		array: &mut MaybeUninit<[T; N]>,
	) -> &mut MaybeUninit<[Self; N]> {
		let ptr: *mut MaybeUninit<[Self; N]> = array.as_mut_ptr().cast();
		// SAFETY: `JsCast` assumes that `T` is `#[transparent]` over a `JsValue`.
		unsafe { ptr.as_mut() }.unwrap()
	}
}

impl Clone for JsValue {
	fn clone(&self) -> Self {
		js_bindgen::unsafe_global_wat!(
			"(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param \
			 i32) (result externref)))",
			"(import \"env\" \"js_sys.externref.insert\" (func $js_sys.externref.insert (@sym) \
			 (param externref) (result i32)))",
			"(func $js_sys.js_value.clone (@sym) (param $index i32) (result i32)",
			"  local.get $index",
			"  call $js_sys.externref.get (@reloc)",
			"  call $js_sys.externref.insert (@reloc)",
			")",
		);

		unsafe extern "C" {
			#[link_name = "js_sys.js_value.clone"]
			safe fn clone(size: i32) -> i32;
		}

		if self.index > 1 {
			Self::new(clone(self.index))
		} else {
			Self::new(self.index)
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

// SAFETY: `JsCast` guarantees that `T` is transparent over `JsValue`, so a
// shared reference has the same `externref` table index `ABI`.
unsafe impl<T: JsCast> IntoJS for &T {
	type Abi = JsValueRefAbi;

	fn into_abi(self) -> Self::Abi {
		JsValueRefAbi(self.unchecked_as_ref().index)
	}
}

// SAFETY: `JsValue` is transparently represented by itself.
unsafe impl JsCast for JsValue {}

// SAFETY: The owned table index is transferred to JavaScript and recycled
// after the WAT shim has loaded its `externref`.
unsafe impl IntoJS for JsValue {
	type Abi = JsValueAbi;

	fn into_abi(self) -> Self::Abi {
		let value = ManuallyDrop::new(self);
		JsValueAbi(value.index)
	}
}

// SAFETY: `JsCast` guarantees that `T` is transparent over `JsValue`, so an
// `externref` table index can be reconstructed as any `T: JsCast`.
unsafe impl<T: JsCast> FromJS for T {
	type Abi = JsValueAbi;

	fn from_abi(raw: Self::Abi) -> Self {
		T::unchecked_from(JsValue::new(raw.0))
	}
}

// SAFETY: `None` uses the reserved undefined index, which no borrowed
// `JsValue` can produce.
unsafe impl<T: JsCast> OptionIntoJS for &T {
	type OptionAbi = JsValueRefAbi;

	fn option_into_abi(value: Option<Self>) -> Self::OptionAbi {
		value.map_or(JsValueRefAbi(JsValue::UNDEFINED.index), |value| {
			IntoJS::into_abi(value.unchecked_as_ref())
		})
	}
}

// SAFETY: `None` becomes index zero. A present value transfers its owned table
// index to JavaScript.
unsafe impl OptionIntoJS for JsValue {
	type OptionAbi = OptionalJsValueAbi;

	fn option_into_abi(value: Option<Self>) -> Self::OptionAbi {
		match value {
			None => OptionalJsValueAbi(Self::UNDEFINED.index),
			Some(value) => {
				let JsValueAbi(index) = IntoJS::into_abi(value);
				OptionalJsValueAbi(index)
			}
		}
	}
}

// SAFETY: Null or undefined JS values become index zero; all other `externref`
// values are inserted into the `externref` table and reconstructed as `T`.
unsafe impl<T: JsCast> FromJS for Option<T> {
	const JS_CONV: Option<FromJsConv> = Some(FromJsConv::slot1("($value) ?? null"));

	type Abi = OptionalJsValueAbi;

	fn from_abi(raw: Self::Abi) -> Self {
		(raw.0 != JsValue::UNDEFINED.index).then(|| T::unchecked_from(JsValue::new(raw.0)))
	}
}

js_bindgen::unsafe_global_wat!(
	"(import \"env\" \"js_sys.externref.insert\" (func $js_sys.externref.insert (@sym) (param \
	 externref) (result i32)))",
	"(func $js_sys.optional.js_value (@sym) (param $value externref) (result i32)",
	"  local.get $value",
	"  ref.is_null",
	"  if (result i32)",
	"    i32.const 0",
	"  else",
	"    local.get $value",
	"    call $js_sys.externref.insert (@reloc)",
	"  end",
	")",
);

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
