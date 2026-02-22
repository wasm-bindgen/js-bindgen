#[rustfmt::skip]
mod r#gen;

pub use self::r#gen::JsArray;
use crate::JsValue;
use crate::util::PtrLength;

impl<T> JsArray<T> {
	#[must_use]
	pub fn into_any(self) -> JsArray {
		JsArray::unchecked_from(self.into())
	}
}

impl From<&[JsValue]> for JsArray<JsValue> {
	fn from(value: &[JsValue]) -> Self {
		js_bindgen::embed_js!(
			name = "array.js_value.encode",
			required_embed = "array.isLittleEndian",
			"(ptr, len) => {{",
			"	const array = new Array(len)",
			"",
			"	if (this.#jsEmbed.js_sys['array.isLittleEndian']) {{",
			"		const view = new Uint32Array(this.#memory.buffer, ptr, len)",
			"		for (let i = 0; i < len; i++) {{",
			"			array[i] = this.#jsEmbed.js_sys['externref.table'].get(view[index])",
			"		}}",
			"	}} else {{",
			"		const view = new DataView(this.#memory.buffer, ptr, len * 4)",
			"		for (let i = 0; i < len; i++) {{",
			"			const index = view.getUint32(i * 4, true)",
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

impl<const N: usize> From<&[u32; N]> for JsArray<u32> {
	fn from(value: &[u32; N]) -> Self {
		value.as_slice().into()
	}
}

impl From<&[u32]> for JsArray<u32> {
	fn from(value: &[u32]) -> Self {
		js_bindgen::embed_js!(
			name = "array.u32.encode",
			required_embed = "array.isLittleEndian",
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
	name = "array.isLittleEndian",
	"(() => {{",
	"	const buffer = new ArrayBuffer(2)",
	"	new DataView(buffer).setInt16(0, 256, true)",
	"	return new Int16Array(buffer)[0] === 256;",
	"}})()",
);
