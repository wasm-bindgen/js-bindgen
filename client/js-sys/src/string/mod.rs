#[rustfmt::skip]
mod r#gen;
mod rust;

use alloc::string::String;
use alloc::vec::Vec;

pub use self::r#gen::JsString;
use crate::JsValue;
use crate::util::PtrLength;

impl JsString {
	#[must_use]
	pub fn new(value: &JsValue) -> Self {
		r#gen::string_constructor(value)
	}

	#[expect(
		clippy::should_implement_trait,
		reason = "currently no stable way to unwrap `Infallible`"
	)]
	#[must_use]
	pub fn from_str(string: &str) -> Self {
		js_bindgen::embed_js!(
			module = "js_sys",
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

		r#gen::string_decode(string.as_ptr(), PtrLength::new(string.as_bytes()))
	}
}

impl From<&JsString> for String {
	fn from(value: &JsString) -> Self {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "string.utf8_length",
			"(string) => new TextEncoder().encode(string).length",
		);

		js_bindgen::embed_js!(
			module = "js_sys",
			name = "string.encode",
			"(string, ptr, len) => {{",
			"	const view = new Uint8Array(this.#memory.buffer, ptr, len)",
			"	new TextEncoder().encodeInto(string, view)",
			"}}",
		);

		let len = r#gen::string_utf8_length(value);
		debug_assert!(
			len < 9_007_199_254_740_992.,
			"found pointer + length bigger than `Number.MAX_SAFE_INTEGER`"
		);
		#[expect(
			clippy::cast_possible_truncation,
			clippy::cast_sign_loss,
			reason = "in practice this is memory constrained"
		)]
		let len = len as usize;

		let mut vec = Vec::with_capacity(len);
		r#gen::string_encode(
			value,
			vec.as_mut_ptr(),
			PtrLength::from_uninit_slice(vec.spare_capacity_mut()),
		);

		// SAFETY:
		unsafe {
			vec.set_len(len);
			Self::from_utf8_unchecked(vec)
		}
	}
}
