use core::marker::PhantomData;
use core::ops::Deref;

use js_sys_macro::js_sys;

use crate::JsValue;
use crate::hazard::{Input, Output};
use crate::util::PtrLength;

pub struct JsArray<T = JsValue> {
	value: JsValue,
	_type: PhantomData<T>,
}

impl From<&[u32]> for JsArray<u32> {
	fn from(value: &[u32]) -> Self {
		js_bindgen::embed_js!(
			name = "array.u32.decode",
			js_embed = "array.isLittleEndian",
			"(ptr, len) => {{",
			"	if (jsEmbed.js_sys[\"array.isLittleEndian\"]) {{",
			"		const view = new Uint32Array(memory.buffer, ptr, len)",
			"		return Array.from(view)",
			"	}} else {{",
			"		const view = new DataView(memory.buffer, ptr, len * 4)",
			"		const array = new Array(len)",
			"		for (let i = 0; i < len; i++) {{",
			"			array[i] = view.getUint32(i * 4, true)",
			"		}}",
			"		return array",
			"	}}",
			"}}",
		);

		#[js_sys(js_sys = crate)]
		extern "C" {
			#[js_sys(js_embed = "array.u32.decode")]
			fn array_u32_decode(array: *const u32, len: PtrLength) -> JsArray<u32>;
		}

		array_u32_decode(value.as_ptr(), PtrLength::new(value.as_ptr(), value.len()))
	}
}

impl<T> Deref for JsArray<T> {
	type Target = JsValue;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

unsafe impl<T> Input for &JsArray<T> {
	const IMPORT_FUNC: &'static str = ".functype js_sys.externref.get (i32) -> (externref)";
	const IMPORT_TYPE: &'static str = "externref";
	const TYPE: &'static str = "i32";
	const CONV: &'static str = "call js_sys.externref.get";

	type Type = i32;

	fn into_raw(self) -> Self::Type {
		self.value.into_raw()
	}
}

unsafe impl<T> Output for JsArray<T> {
	const IMPORT_FUNC: &str = ".functype js_sys.externref.insert (externref) -> (i32)";
	const IMPORT_TYPE: &str = "externref";
	const TYPE: &str = "i32";
	const CONV: &str = "call js_sys.externref.insert";

	type Type = i32;

	fn from_raw(raw: Self::Type) -> Self {
		Self {
			value: JsValue::from_raw(raw),
			_type: PhantomData,
		}
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
