#[rustfmt::skip]
mod r#gen;

use core::mem::MaybeUninit;
use core::ptr;

pub use self::r#gen::JsArray;
use crate::JsValue;
use crate::externref::ExternrefTable;
use crate::hazard::Input;
use crate::util::{ExternRef, PtrLength};

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

impl JsArray {
	#[must_use]
	pub fn as_array<const N: usize>(&self) -> Option<[JsValue; N]> {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "array.js_value.encode",
			required_embeds = [("js_sys", "array.isLittleEndian")],
			"(array, arrPtr, arrLen, refPtr, refLen) => {{",
			"	if (array.length !== arrLen) return false",
			"",
			"	const table = this.#jsEmbed.js_sys['externref.table']",
			"	const isLe = this.#jsEmbed.js_sys['array.isLittleEndian']",
			"",
			// Default value helps browsers to optimize.
			"	let tableIndex = 0",
			"	if (arrLen > refLen) {{",
			"		tableIndex = table.grow(arrLen - refLen)",
			"	}}",
			"",
			"	let refIndex = refLen - 1",
			"",
			"	let arrView",
			"	if (isLe)",
			"		arrView = new Int32Array(this.#memory.buffer, arrPtr, arrLen)",
			"	else",
			"		arrView = new DataView(this.#memory.buffer, arrPtr, arrLen * 4)",
			"",
			"	let refView",
			"	if (isLe)",
			"		refView = new Int32Array(this.#memory.buffer, refPtr, refLen)",
			"	else",
			"		refView = new DataView(this.#memory.buffer, refPtr, refLen * 4)",
			"",
			"	for (let i = 0; i < arrLen; i++) {{",
			"		let elemIndex",
			"",
			"		if (refIndex >= 0) {{",
			"			if (isLe)",
			"				elemIndex = refView[refIndex]",
			"			else",
			"				elemIndex = refView.getInt32(refIndex * 4, true)",
			"",
			"			refIndex--",
			"		}} else {{",
			"			elemIndex = tableIndex",
			"			tableIndex++",
			"		}}",
			"",
			"		table.set(elemIndex, array[i])",
			"",
			"		if (isLe)",
			"			arrView[i] = elemIndex",
			"		else",
			"			arrView.setInt32(i * 4, elemIndex, true)",
			"	}}",
			"",
			"	return true",
			"}}",
		);

		let mut array: MaybeUninit<[JsValue; N]> = MaybeUninit::uninit();
		let externref = ExternrefTable::current_ptr();

		let result = r#gen::array_js_value_encode(
			self,
			array.as_mut_ptr().cast(),
			PtrLength::from_uninit_array(&array),
			externref.ptr,
			externref.len,
		);

		if result {
			ExternrefTable::report_used_slots(N);
			// SAFETY: Correctly initialized in JS.
			Some(unsafe { array.assume_init() })
		} else {
			None
		}
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

impl From<&[JsValue]> for JsArray {
	fn from(value: &[JsValue]) -> Self {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "array.js_value.decode",
			required_embeds = [("js_sys", "array.isLittleEndian")],
			"(ptr, len) => {{",
			"	const array = new Array(len)",
			"",
			"	if (this.#jsEmbed.js_sys['array.isLittleEndian']) {{",
			"		const view = new Int32Array(this.#memory.buffer, ptr, len)",
			"		for (let i = 0; i < len; i++) {{",
			"			array[i] = this.#jsEmbed.js_sys['externref.table'].get(view[i])",
			"		}}",
			"	}} else {{",
			"		const view = new DataView(this.#memory.buffer, ptr, len * 4)",
			"		for (let i = 0; i < len; i++) {{",
			"			const index = view.getInt32(i * 4, true)",
			"			array[i] = this.#jsEmbed.js_sys['externref.table'].get(index)",
			"		}}",
			"	}}",
			"",
			"	return array",
			"}}",
		);

		r#gen::array_js_value_decode(value.as_ptr(), PtrLength::new(value))
	}
}

// SAFETY: Implementation.
unsafe impl Input for &[JsValue] {
	const IMPORT_TYPE: &'static str = Self::Type::IMPORT_TYPE;
	const TYPE: &'static str = Self::Type::TYPE;
	const CONV: &'static str = Self::Type::CONV;
	const JS_CONV_EMBED: (&'static str, &'static str) = ("js_sys", "array.rust.js_value");
	const JS_CONV: Option<&'static str> = Some(" = this.#jsEmbed.js_sys['array.rust.js_value'](");
	const JS_CONV_POST: Option<&'static str> = Some(")");

	type Type = ExternRef<JsValue>;

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

		ExternRef::new(self)
	}
}

impl JsArray<u32> {
	#[must_use]
	pub fn as_array<const N: usize>(&self) -> Option<[u32; N]> {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "array.u32.encode",
			required_embeds = [("js_sys", "array.isLittleEndian")],
			"(array, ptr, len) => {{",
			"	if (array.length !== len) return false",
			"",
			"	if (this.#jsEmbed.js_sys['array.isLittleEndian']) {{",
			"		const view = new Uint32Array(this.#memory.buffer, ptr, len)",
			"		view.set(array)",
			"	}} else {{",
			"		const view = new DataView(this.#memory.buffer, ptr, len * 4)",
			"		for (let i = 0; i < len; i++) {{",
			"			view.setUint32(i * 4, array[i], true)",
			"		}}",
			"	}}",
			"",
			"	return true",
			"}}",
		);

		let mut array: MaybeUninit<[u32; N]> = MaybeUninit::uninit();

		let result = r#gen::array_u32_encode(
			self,
			array.as_mut_ptr().cast(),
			PtrLength::from_uninit_array(&array),
		);

		if result {
			// SAFETY: Correctly initialized in JS.
			Some(unsafe { array.assume_init() })
		} else {
			None
		}
	}
}

impl From<&[u32]> for JsArray<u32> {
	fn from(value: &[u32]) -> Self {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "array.u32.decode",
			required_embeds = [("js_sys", "array.isLittleEndian")],
			"(ptr, len) => {{",
			"	if (this.#jsEmbed.js_sys['array.isLittleEndian']) {{",
			"		const view = new Uint32Array(this.#memory.buffer, ptr, len)",
			"		return Array.from(view)",
			"	}} else {{",
			"		const view = new DataView(this.#memory.buffer, ptr, len * 4)",
			"		const array = new Array(len)",
			"		for (let i = 0; i < len; i++) {{",
			"			array[i] = view.getUint32(i * 4, true)",
			"		}}",
			"		return array",
			"	}}",
			"}}",
		);

		r#gen::array_u32_decode(value.as_ptr(), PtrLength::new(value))
	}
}

// SAFETY: Implementation.
unsafe impl Input for &[u32] {
	const IMPORT_TYPE: &'static str = Self::Type::IMPORT_TYPE;
	const TYPE: &'static str = Self::Type::TYPE;
	const CONV: &'static str = Self::Type::CONV;
	const JS_CONV_EMBED: (&'static str, &'static str) = ("js_sys", "array.rust.u32");
	const JS_CONV: Option<&'static str> = Some(" = this.#jsEmbed.js_sys['array.rust.u32'](");
	const JS_CONV_POST: Option<&'static str> = Some(")");

	type Type = ExternRef<u32>;

	fn into_raw(self) -> Self::Type {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "array.rust.u32",
			required_embeds = [("js_sys", "extern_ref"), ("js_sys", "array.u32.decode")],
			"(dataPtr) => {{",
			"	const {{ ptr, len }} = this.#jsEmbed.js_sys['extern_ref'](dataPtr)",
			"	return this.#jsEmbed.js_sys['array.u32.decode'](ptr, len)",
			"}}",
		);

		ExternRef::new(self)
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "array.isLittleEndian",
	"(() => {{",
	"	const buffer = new ArrayBuffer(2)",
	"	new DataView(buffer).setInt16(0, 256, true)",
	"	return new Int16Array(buffer)[0] === 256;",
	"}})()",
);
