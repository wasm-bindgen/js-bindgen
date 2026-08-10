use core::marker::PhantomData;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::slice;

use super::externref::{
	WAT_GET_CONV, WAT_INDEX_LOCAL, WAT_INSERT_CONV, WAT_INSERT_IMPORTS, WAT_INSERT_LOCALS,
	WAT_OPTIONAL_INSERT_CONV, WAT_TABLE_IMPORT, WAT_TAKE_CONV, WAT_TAKE_IMPORTS, release,
};
use crate::hazard::{
	FromJS, FromJsConv, IntoJS, IntoJsConv, JsCast, OptionFromAbi, OptionIntoAbi, ReturnAbi,
	ReturnMode, Slot, WatConv,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct JsValue {
	index: i32,
	_local: PhantomData<*const ()>,
}

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "js_value.partial_eq")]
	fn js_value_partial_eq(value1: &JsValue, value2: &JsValue) -> bool;
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
		imports: Some(WAT_TAKE_IMPORTS),
		locals: Some(WAT_INDEX_LOCAL),
		conv: WAT_TAKE_CONV,
		r#type: "externref",
	});
	const FROM_JS_WAT_CONV: Option<WatConv> = Some(WatConv {
		imports: Some(WAT_INSERT_IMPORTS),
		locals: Some(WAT_INSERT_LOCALS),
		conv: WAT_INSERT_CONV,
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
		imports: Some(WAT_TABLE_IMPORT),
		locals: None,
		conv: WAT_GET_CONV,
		r#type: "externref",
	});
}

// SAFETY: `OptionalJsValueAbi` is an `i32` table index. At the JS boundary,
// null is represented by index zero and non-null `externref` values are
// inserted into the `externref` table.
unsafe impl Slot for OptionalJsValueAbi {
	const WAT_TYPE: &'static str = "i32";
	const INTO_JS_WAT_CONV: Option<WatConv> = Some(WatConv {
		imports: Some(WAT_TAKE_IMPORTS),
		locals: Some(WAT_INDEX_LOCAL),
		conv: WAT_TAKE_CONV,
		r#type: "externref",
	});
	const FROM_JS_WAT_CONV: Option<WatConv> = Some(WatConv {
		imports: Some(WAT_INSERT_IMPORTS),
		locals: Some(WAT_INSERT_LOCALS),
		conv: WAT_OPTIONAL_INSERT_CONV,
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

	pub(crate) fn from_slice<T: JsCast>(slice: &[T]) -> &[Self] {
		let ptr: *const Self = slice.as_ptr().cast();
		// SAFETY: `JsCast` assumes that `T` is `#[transparent]` over a `JsValue`.
		unsafe { slice::from_raw_parts(ptr, slice.len()) }
	}

	pub(crate) fn from_slice_mut<T: JsCast>(slice: &mut [T]) -> &mut [Self] {
		let ptr: *mut Self = slice.as_mut_ptr().cast();
		// SAFETY: `JsCast` assumes that `T` is `#[transparent]` over a `JsValue`.
		unsafe { slice::from_raw_parts_mut(ptr, slice.len()) }
	}

	pub(crate) fn from_uninit_slice_mut<T: JsCast>(
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
	#[inline]
	fn clone(&self) -> Self {
		js_bindgen::unsafe_global_wat!(
			"(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym \
			 (name \"js_sys.externref.table\")) 2 externref))",
			"(import \"env\" \"js_sys.externref.next\" (func $js_sys.externref.next (@sym) \
			 (result i32)))",
			"(func $js_sys.js_value.clone (@sym) (param $index i32) (result i32)",
			"  (local $new_index i32)",
			"  call $js_sys.externref.next (@reloc)",
			"  local.tee $new_index",
			"  local.get $index",
			"  table.get $js_sys.import.externref.table (@reloc)",
			"  table.set $js_sys.import.externref.table (@reloc)",
			"  local.get $new_index",
			")",
		);

		unsafe extern "C" {
			#[link_name = "js_sys.js_value.clone"]
			safe fn clone(index: i32) -> i32;
		}

		Self::new(clone(self.index))
	}
}

impl Drop for JsValue {
	#[inline]
	fn drop(&mut self) {
		if u32::from_ne_bytes(self.index.to_ne_bytes()) >= 2 {
			release(self.index);
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

// SAFETY: `None` uses the reserved undefined index, while `Some` preserves the
// borrowed table index produced by the underlying conversion.
unsafe impl<T> OptionIntoAbi<T> for JsValueRefAbi
where
	T: IntoJS<Abi = Self>,
{
	const JS_CONV: Option<IntoJsConv> = T::JS_CONV;

	type Abi = Self;

	fn into_option_abi(value: Option<T>) -> Self::Abi {
		value.map_or(Self(JsValue::UNDEFINED.index), |value| {
			IntoJS::into_abi(value)
		})
	}
}

// SAFETY: `None` becomes the reserved undefined index. A present value
// transfers the owned table index produced by the underlying conversion.
unsafe impl<T> OptionIntoAbi<T> for JsValueAbi
where
	T: IntoJS<Abi = Self>,
{
	const JS_CONV: Option<IntoJsConv> = T::JS_CONV;

	type Abi = OptionalJsValueAbi;

	fn into_option_abi(value: Option<T>) -> Self::Abi {
		match value {
			None => OptionalJsValueAbi(JsValue::UNDEFINED.index),
			Some(value) => {
				let Self(index) = IntoJS::into_abi(value);
				OptionalJsValueAbi(index)
			}
		}
	}
}

// SAFETY: Null or undefined JS values use the reserved undefined index; all
// other values are decoded by the underlying owned table-index conversion.
unsafe impl<T> OptionFromAbi<T> for JsValueAbi
where
	T: FromJS<Abi = Self>,
{
	const JS_CONV: Option<FromJsConv> = Some(FromJsConv::slot1("($value) ?? null"));

	type Abi = OptionalJsValueAbi;

	fn from_option_abi(raw: Self::Abi) -> Option<T> {
		(raw.0 != JsValue::UNDEFINED.index).then(|| T::from_abi(Self(raw.0)))
	}
}

impl PartialEq for JsValue {
	fn eq(&self, other: &Self) -> bool {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "js_value.partial_eq",
			"(value1, value2) => value1 === value2",
		);

		js_value_partial_eq(self, other)
	}
}
