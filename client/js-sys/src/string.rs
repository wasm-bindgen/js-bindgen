use core::ops::Deref;

use js_sys_macro::js_sys;

use crate::JsValue;
use crate::hazard::{Input, Output};
use crate::util::PtrLength;

#[repr(transparent)]
pub struct JsString(JsValue);

impl JsString {
	#[allow(clippy::should_implement_trait)]
	pub fn from_str(string: &str) -> Self {
		js_bindgen::embed_js!(
			name = "string.decode",
			"(ptr, len) => {{",
			"	const decoder = new TextDecoder(\"utf-8\", {{",
			"		fatal: false,",
			"		ignoreBOM: false,",
			"	}})",
			#[cfg(not(target_feature = "atomics"))]
			"	const view = new Uint8Array(memory.buffer, ptr, len)",
			#[cfg(target_feature = "atomics")]
			"	const view = new Uint8Array(memory.buffer).slice(ptr, ptr + len)",
			"",
			"	return decoder.decode(view)",
			"}}",
		);

		#[js_sys(js_sys = crate)]
		extern "C" {
			#[js_sys(js_embed = "string.decode")]
			fn string_decode(array: *const u8, len: PtrLength) -> JsString;
		}

		string_decode(
			string.as_ptr(),
			PtrLength::new(string.as_ptr(), string.len()),
		)
	}
}

impl Deref for JsString {
	type Target = JsValue;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

unsafe impl Input for &JsString {
	const IMPORT_FUNC: &'static str = ".functype js_sys.externref.get (i32) -> (externref)";
	const IMPORT_TYPE: &'static str = "externref";
	const TYPE: &'static str = "i32";
	const CONV: &'static str = "call js_sys.externref.get";

	type Type = i32;

	fn into_raw(self) -> Self::Type {
		self.0.into_raw()
	}
}

unsafe impl Output for JsString {
	const IMPORT_FUNC: &str = ".functype js_sys.externref.insert (externref) -> (i32)";
	const IMPORT_TYPE: &str = "externref";
	const TYPE: &str = "i32";
	const CONV: &str = "call js_sys.externref.insert";

	type Type = i32;

	fn from_raw(raw: Self::Type) -> Self {
		Self(JsValue::from_raw(raw))
	}
}
