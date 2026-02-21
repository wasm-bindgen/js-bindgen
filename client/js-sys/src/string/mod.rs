#[rustfmt::skip]
mod r#gen;

pub use self::r#gen::JsString;
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

		r#gen::string_decode(
			string.as_ptr(),
			PtrLength::new(string.as_ptr(), string.len()),
		)
	}
}
