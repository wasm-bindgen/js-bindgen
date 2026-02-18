use js_sys_macro::js_sys;

use crate::util::PtrLength;

impl JsString {
	#[expect(
		clippy::should_implement_trait,
		reason = "currently no stable way to unwrap `Infallible`"
	)]
	#[must_use]
	pub fn from_str(string: &str) -> Self {
		js_bindgen::embed_js!(
			name = "string.decode",
			"(ptr, len) => {{",
			"	const decoder = new TextDecoder('utf-8', {{",
			"		fatal: false,",
			"		ignoreBOM: false,",
			"	}})",
			#[cfg(not(target_feature = "atomics"))]
			"	const view = new Uint8Array(this.#memory.buffer, ptr, len)",
			#[cfg(target_feature = "atomics")]
			"	const view = new Uint8Array(this.#memory.buffer).slice(ptr, ptr + len)",
			"",
			"	return decoder.decode(view)",
			"}}",
		);

		string_decode(
			string.as_ptr(),
			PtrLength::new(string.as_ptr(), string.len()),
		)
	}
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	pub type JsString;

	#[js_sys(js_embed = "string.decode")]
	fn string_decode(array: *const u8, len: PtrLength) -> JsString;
}
