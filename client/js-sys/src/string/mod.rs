#[rustfmt::skip]
mod r#gen;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{self, Display, Formatter};

pub use self::r#gen::JsString;
use crate::JsValue;
use crate::hazard::Input;
use crate::util::{ExternRef, PtrLength};

impl JsString {
	#[must_use]
	pub fn new(value: &JsValue) -> Self {
		r#gen::string_constructor(value)
	}
}

impl Display for JsString {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", String::from(self))
	}
}

impl PartialEq<&str> for JsString {
	fn eq(&self, other: &&str) -> bool {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "string.eq",
			required_embeds = [("js_sys", "string.decode")],
			"(string, ptr, len) => {{",
			"	const other = this.#jsEmbed.js_sys['string.decode'](ptr, len)",
			"	return string === other",
			"}}",
		);

		r#gen::string_eq(self, other.as_ptr(), PtrLength::new(other.as_bytes()))
	}
}

impl PartialEq<String> for JsString {
	fn eq(&self, other: &String) -> bool {
		self.eq(&other.as_str())
	}
}

impl From<&str> for JsString {
	fn from(value: &str) -> Self {
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

		r#gen::string_decode(value.as_ptr(), PtrLength::new(value.as_bytes()))
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

// SAFETY: Implementation.
unsafe impl Input for &str {
	const IMPORT_TYPE: &'static str = Self::Type::IMPORT_TYPE;
	const TYPE: &'static str = Self::Type::TYPE;
	const CONV: &'static str = Self::Type::CONV;
	const JS_CONV_EMBED: (&'static str, &'static str) = ("js_sys", "string.rust.decode");
	const JS_CONV: Option<&'static str> = Some(" = this.#jsEmbed.js_sys['string.rust.decode'](");
	const JS_CONV_POST: Option<&'static str> = Some(")");

	type Type = ExternRef<u8>;

	fn into_raw(self) -> Self::Type {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "string.rust.decode",
			required_embeds = [("js_sys", "extern_ref"), ("js_sys", "string.decode")],
			"(dataPtr) => {{",
			"	const {{ ptr, len }} = this.#jsEmbed.js_sys['extern_ref'](dataPtr)",
			"	return this.#jsEmbed.js_sys['string.decode'](ptr, len)",
			"}}",
		);

		ExternRef::new(self.as_bytes())
	}
}
