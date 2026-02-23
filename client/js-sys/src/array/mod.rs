#[rustfmt::skip]
mod r#gen;

use core::mem::MaybeUninit;

pub use self::r#gen::JsArray;
use crate::JsValue;
use crate::externref::ExternrefTable;
use crate::util::PtrLength;

impl<T> JsArray<T> {
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

impl JsArray {
	#[must_use]
	pub fn as_array<const N: usize>(&self) -> Option<[JsValue; N]> {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "array.js_value.decode",
			required_embeds = ["array.isLittleEndian"],
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
		let externref = ExternrefTable::current_into();

		let result = r#gen::array_js_value_decode(
			self,
			array.as_mut_ptr().cast(),
			PtrLength::from_uninit_array(&array),
			externref.ptr,
			externref.len,
		);

		if result {
			ExternrefTable::report_growth(N);
			// SAFETY: Correctly initialized in JS.
			Some(unsafe { array.assume_init() })
		} else {
			None
		}
	}
}

impl From<&[JsValue]> for JsArray {
	fn from(value: &[JsValue]) -> Self {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "array.js_value.encode",
			required_embeds = ["array.isLittleEndian"],
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

		r#gen::array_js_value_encode(value.as_ptr(), PtrLength::new(value))
	}
}

impl JsArray<u32> {
	#[must_use]
	pub fn as_array<const N: usize>(&self) -> Option<[u32; N]> {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "array.u32.decode",
			required_embeds = ["array.isLittleEndian"],
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

		let result = r#gen::array_u32_decode(
			self,
			array.as_mut_ptr().cast(),
			PtrLength::from_uninit_array(&array),
		);

		if result {
			ExternrefTable::report_growth(N);
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
			name = "array.u32.encode",
			required_embeds = ["array.isLittleEndian"],
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

		r#gen::array_u32_encode(value.as_ptr(), PtrLength::new(value))
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
