#[rustfmt::skip]
#[path ="array.gen.rs"]
mod array;

use core::error::Error;
use core::fmt::{self, Display, Formatter};
use core::mem::MaybeUninit;
use core::ptr;

pub use self::array::JsArray;
use crate::JsValue;
use crate::externref::ExternrefTable;
use crate::hazard::{IntoJS, IntoJsConv, JsCast};
use crate::util::{ExternSlice, PtrConst, PtrLength, PtrMut};

impl<T> JsArray<T> {
	#[must_use]
	pub fn as_any(&self) -> &JsArray {
		JsArray::unchecked_from_ref(self.as_ref())
	}

	#[must_use]
	pub fn into_any(self) -> JsArray {
		JsArray::unchecked_from(self.into())
	}
}

impl<T, const N: usize> From<&[T; N]> for JsArray<T>
where
	Self: for<'a> From<&'a [T]>,
{
	fn from(value: &[T; N]) -> Self {
		value.as_slice().into()
	}
}

// SAFETY: The array delegates to the slice implementation with the same
// element representation.
unsafe impl<'a, T, const N: usize> IntoJS for &'a [T; N]
where
	&'a [T]: IntoJS,
{
	const JS_CONV: Option<IntoJsConv> = <&[T] as IntoJS>::JS_CONV;

	type Abi = <&'a [T] as IntoJS>::Abi;

	fn into_abi(self) -> Self::Abi {
		self.as_slice().into_abi()
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub struct TryFromJsArrayError;

impl Display for TryFromJsArrayError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str("length did not match")
	}
}

impl Error for TryFromJsArrayError {}

impl<T: JsCast> JsArray<T> {
	pub fn to_slice(&self, slice: &mut [T]) -> Result<(), TryFromJsArrayError> {
		let slice = JsValue::from_slice_mut(slice);
		let externref = ExternrefTable::current_ptr();

		// SAFETY: Parameters are correct.
		let result = unsafe {
			array::array_js_value_encode(
				self.as_any(),
				PtrMut::new(slice),
				PtrLength::new(slice),
				externref.ptr,
				externref.len,
			)
		};

		if result {
			ExternrefTable::report_used_slots(slice.len());
			Ok(())
		} else {
			Err(TryFromJsArrayError)
		}
	}

	pub fn to_uninit_slice<'slice>(
		&self,
		slice: &'slice mut [MaybeUninit<T>],
	) -> Result<&'slice mut [T], TryFromJsArrayError> {
		let js_slice = JsValue::from_uninit_slice_mut(slice);
		let externref = ExternrefTable::current_ptr();

		// SAFETY: Parameters are correct.
		let result = unsafe {
			array::array_js_value_encode(
				self.as_any(),
				PtrMut::from_uninit_slice(js_slice),
				PtrLength::from_uninit_slice(js_slice),
				externref.ptr,
				externref.len,
			)
		};

		if result {
			ExternrefTable::report_used_slots(js_slice.len());
			// SAFETY: Correctly initialized in JS.
			Ok(unsafe { assume_init_mut(slice) })
		} else {
			Err(TryFromJsArrayError)
		}
	}

	pub fn to_array<const N: usize>(&self) -> Result<[T; N], TryFromJsArrayError> {
		let mut array: MaybeUninit<[T; N]> = MaybeUninit::uninit();
		let externref = ExternrefTable::current_ptr();
		let js_array = JsValue::from_mut_uninit_array(&mut array);

		// SAFETY: Parameters are correct.
		let result = unsafe {
			array::array_js_value_encode(
				self.as_any(),
				PtrMut::from_uninit_array(js_array),
				PtrLength::from_uninit_array(js_array),
				externref.ptr,
				externref.len,
			)
		};

		if result {
			ExternrefTable::report_used_slots(N);
			// SAFETY: Correctly initialized in JS.
			Ok(unsafe { array.assume_init() })
		} else {
			Err(TryFromJsArrayError)
		}
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "array.js_value.encode",
	required_embeds = [("js_sys", "view.getInt32"), ("js_sys", "view.setInt32")],
	"(array, arrPtr, arrLen, refPtr, refLen) => {{",
	"	if (array.length !== arrLen) return false",
	"",
	"	const table = this.#jsEmbed.js_sys['externref.table']",
	"",
	// Default value helps browsers to optimize.
	"	let tableIndex = 0",
	"	const reused = Math.min(arrLen, refLen)",
	"	const refIndices = this.#jsEmbed.js_sys['view.getInt32'](",
	"		refPtr + (refLen - reused) * 4,",
	"		reused,",
	"	)",
	"	const elemIndices = new Array(arrLen)",
	"	if (arrLen > reused) {{",
	"		tableIndex = table.grow(arrLen - reused)",
	"	}}",
	"",
	"	let refIndex = reused - 1",
	"",
	"	for (let arrayIndex = 0; arrayIndex < arrLen; arrayIndex++) {{",
	"		let elemIndex",
	"",
	"		if (refIndex >= 0) {{",
	"			elemIndex = refIndices[refIndex]",
	"			refIndex--",
	"		}} else {{",
	"			elemIndex = tableIndex",
	"			tableIndex++",
	"		}}",
	"",
	"		table.set(elemIndex, array[arrayIndex])",
	"		elemIndices[arrayIndex] = elemIndex",
	"	}}",
	"",
	"	this.#jsEmbed.js_sys['view.setInt32'](arrPtr, elemIndices)",
	"	return true",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "array.js_value.decode",
	required_embeds = [("js_sys", "view.getInt32")],
	"(ptr, len) => {{",
	"	const array = new Array(len)",
	"	const refIndices = this.#jsEmbed.js_sys['view.getInt32'](ptr, len)",
	"	for (let arrayIndex = 0; arrayIndex < len; arrayIndex++) {{",
	"		array[arrayIndex] = this.#jsEmbed.js_sys['externref.table'].get(refIndices[arrayIndex])",
	"	}}",
	"	return array",
	"}}",
);

impl<T: JsCast> From<&[T]> for JsArray<T> {
	fn from(value: &[T]) -> Self {
		let slice = JsValue::from_slice(value);
		// SAFETY: Parameters are correct.
		let result =
			unsafe { array::array_js_value_decode(PtrConst::new(slice), PtrLength::new(slice)) };

		Self::unchecked_from(result.into())
	}
}

// SAFETY: The two slots point to borrowed `JsValue` table indices, which the
// JavaScript decoder resolves before the import is called.
unsafe impl<T: JsCast> IntoJS for &[T] {
	const JS_CONV: Option<IntoJsConv> = Some(
		IntoJsConv::new("this.#jsEmbed.js_sys['array.js_value.decode']($slot1, $slot2)")
			.with_embed(("js_sys", "array.js_value.decode")),
	);

	type Abi = ExternSlice<JsValue>;

	fn into_abi(self) -> Self::Abi {
		ExternSlice::new(JsValue::from_slice(self))
	}
}

impl JsArray<u32> {
	pub fn to_slice(&self, slice: &mut [u32]) -> Result<(), TryFromJsArrayError> {
		// SAFETY: Parameters are correct.
		let result =
			unsafe { array::array_u32_encode(self, PtrMut::new(slice), PtrLength::new(slice)) };

		if result {
			Ok(())
		} else {
			Err(TryFromJsArrayError)
		}
	}

	pub fn to_uninit_slice<'slice>(
		&self,
		slice: &'slice mut [MaybeUninit<u32>],
	) -> Result<&'slice mut [u32], TryFromJsArrayError> {
		// SAFETY: Parameters are correct.
		let result = unsafe {
			array::array_u32_encode(
				self,
				PtrMut::from_uninit_slice(slice),
				PtrLength::from_uninit_slice(slice),
			)
		};

		if result {
			// SAFETY: Correctly initialized in JS.
			Ok(unsafe { assume_init_mut(slice) })
		} else {
			Err(TryFromJsArrayError)
		}
	}

	#[must_use]
	pub fn to_array<const N: usize>(&self) -> Option<[u32; N]> {
		let mut array: MaybeUninit<[u32; N]> = MaybeUninit::uninit();

		// SAFETY: Parameters are correct.
		let result = unsafe {
			array::array_u32_encode(
				self,
				PtrMut::from_uninit_array(&mut array),
				PtrLength::from_uninit_array(&array),
			)
		};

		if result {
			// SAFETY: Correctly initialized in JS.
			Some(unsafe { array.assume_init() })
		} else {
			None
		}
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "array.u32.encode",
	required_embeds = [("js_sys", "view.setInt32")],
	"(array, ptr, len) => {{",
	"	if (array.length !== len) return false",
	"",
	"	this.#jsEmbed.js_sys['view.setInt32'](ptr, array)",
	"	return true",
	"}}",
);

impl From<&[u32]> for JsArray<u32> {
	fn from(value: &[u32]) -> Self {
		// SAFETY: Parameters are correct.
		unsafe { array::array_u32_decode(PtrConst::new(value), PtrLength::new(value)) }
	}
}

// SAFETY: The two slots describe a borrowed `u32` slice, which JavaScript
// copies into an array before the import is called.
unsafe impl IntoJS for &[u32] {
	const JS_CONV: Option<IntoJsConv> = Some(
		IntoJsConv::new("this.#jsEmbed.js_sys['view.getUint32']($slot1, $slot2)")
			.with_embed(("js_sys", "view.getUint32")),
	);

	type Abi = ExternSlice<u32>;

	fn into_abi(self) -> Self::Abi {
		ExternSlice::new(self)
	}
}

// MSRV: Stable on v1.93.
const unsafe fn assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
	// SAFETY: copied from Std.
	unsafe { &mut *(ptr::from_mut::<[MaybeUninit<T>]>(slice) as *mut [T]) }
}
