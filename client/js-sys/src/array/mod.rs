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
use crate::hazard::{Input, InputAsmConv, InputJsConv};
use crate::util::{ExternSlice, PtrConst, PtrLength, PtrMut};

impl<T> JsArray<T> {
	#[must_use]
	pub fn as_any(&self) -> &JsArray {
		// SAFETY: Only changing the `PhantomData`.
		unsafe { ptr::from_ref(self).cast::<JsArray>().as_ref().unwrap() }
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

// SAFETY: Implementation.
unsafe impl<'a, T, const N: usize> Input for &'a [T; N]
where
	&'a [T]: Input,
{
	const ASM_TYPE: &'static str = <&[T] as Input>::ASM_TYPE;
	const ASM_CONV: Option<InputAsmConv> = <&[T] as Input>::ASM_CONV;
	const JS_CONV: Option<InputJsConv> = <&[T] as Input>::JS_CONV;

	type Type = <&'a [T] as Input>::Type;

	fn into_raw(self) -> Self::Type {
		self.as_slice().into_raw()
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

impl JsArray {
	pub fn to_slice(&self, slice: &mut [JsValue]) -> Result<(), TryFromJsArrayError> {
		let externref = ExternrefTable::current_ptr();

		// SAFETY: Parameters are correct.
		let result = unsafe {
			array::array_js_value_encode(
				self,
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
		slice: &'slice mut [MaybeUninit<JsValue>],
	) -> Result<&'slice mut [JsValue], TryFromJsArrayError> {
		let externref = ExternrefTable::current_ptr();

		// SAFETY: Parameters are correct.
		let result = unsafe {
			array::array_js_value_encode(
				self,
				PtrMut::from_uninit_slice(slice),
				PtrLength::from_uninit_slice(slice),
				externref.ptr,
				externref.len,
			)
		};

		if result {
			ExternrefTable::report_used_slots(slice.len());
			// SAFETY: Correctly initialized in JS.
			Ok(unsafe { assume_init_mut(slice) })
		} else {
			Err(TryFromJsArrayError)
		}
	}

	pub fn to_array<const N: usize>(&self) -> Result<[JsValue; N], TryFromJsArrayError> {
		let mut array: MaybeUninit<[JsValue; N]> = MaybeUninit::uninit();
		let externref = ExternrefTable::current_ptr();

		// SAFETY: Parameters are correct.
		let result = unsafe {
			array::array_js_value_encode(
				self,
				PtrMut::from_uninit_array(&mut array),
				PtrLength::from_uninit_array(&array),
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
	"	if (arrLen > refLen) {{",
	"		tableIndex = table.grow(arrLen - refLen)",
	"	}}",
	"",
	"	let refIndex = refLen - 1",
	"",
	"	for (let arrayIndex = 0; arrayIndex < arrLen; arrayIndex++) {{",
	"		let elemIndex",
	"",
	"		if (refIndex >= 0) {{",
	"			elemIndex = this.#jsEmbed.js_sys['view.getInt32'](refPtr + refIndex * 4, 1)[0]",
	"			refIndex--",
	"		}} else {{",
	"			elemIndex = tableIndex",
	"			tableIndex++",
	"		}}",
	"",
	"		table.set(elemIndex, array[arrayIndex])",
	"		this.#jsEmbed.js_sys['view.setInt32'](arrPtr + arrayIndex * 4, [elemIndex])",
	"	}}",
	"",
	"	return true",
	"}}",
);

impl From<&[JsValue]> for JsArray {
	fn from(value: &[JsValue]) -> Self {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "array.js_value.decode",
			required_embeds = [("js_sys", "view.getInt32")],
			"(ptr, len) => {{",
			"	const array = new Array(len)",
			"	for (let arrayIndex = 0; arrayIndex < len; arrayIndex++) {{",
			"		const [refIndex] = this.#jsEmbed.js_sys['view.getInt32'](ptr + arrayIndex * 4, 1)",
			"		array[arrayIndex] = this.#jsEmbed.js_sys['externref.table'].get(refIndex)",
			"	}}",
			"	return array",
			"}}",
		);

		// SAFETY: Parameters are correct.
		unsafe { array::array_js_value_decode(PtrConst::new(value), PtrLength::new(value)) }
	}
}

// SAFETY: Implementation.
unsafe impl Input for &[JsValue] {
	const ASM_TYPE: &'static str = Self::Type::ASM_TYPE;
	const ASM_CONV: Option<InputAsmConv> = Self::Type::ASM_CONV;
	const JS_CONV: Option<InputJsConv> = Some(InputJsConv {
		embed: Some(("js_sys", "array.rust.js_value")),
		pre: " = this.#jsEmbed.js_sys['array.rust.js_value'](",
		post: Some(")"),
	});

	type Type = ExternSlice<JsValue>;

	fn into_raw(self) -> Self::Type {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "array.rust.js_value",
			required_embeds = [
				("js_sys", "extern_ref"),
				("js_sys", "array.js_value.decode")
			],
			"(dataPtr) => {{",
			"	const {{ ptr, len }} = this.#jsEmbed.js_sys['extern_ref'](dataPtr)",
			"	return this.#jsEmbed.js_sys['array.js_value.decode'](ptr, len)",
			"}}",
		);

		ExternSlice::new(self)
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

// SAFETY: Implementation.
unsafe impl Input for &[u32] {
	const ASM_TYPE: &'static str = Self::Type::ASM_TYPE;
	const ASM_CONV: Option<InputAsmConv> = Self::Type::ASM_CONV;
	const JS_CONV: Option<InputJsConv> = Some(InputJsConv {
		embed: Some(("js_sys", "array.rust.u32")),
		pre: " = this.#jsEmbed.js_sys['array.rust.u32'](",
		post: Some(")"),
	});

	type Type = ExternSlice<u32>;

	fn into_raw(self) -> Self::Type {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "array.rust.u32",
			required_embeds = [("js_sys", "extern_ref"), ("js_sys", "array.u32.decode")],
			"(dataPtr) => {{",
			"	const {{ ptr, len }} = this.#jsEmbed.js_sys.extern_ref(dataPtr)",
			"	return this.#jsEmbed.js_sys['array.u32.decode'](ptr, len)",
			"}}",
		);

		ExternSlice::new(self)
	}
}

// MSRV: Stable on v1.93.
const unsafe fn assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
	// SAFETY: copied from Std.
	unsafe { &mut *(core::ptr::from_mut::<[MaybeUninit<T>]>(slice) as *mut [T]) }
}
