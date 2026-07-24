#[rustfmt::skip]
#[path ="string.gen.rs"]
mod string;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{self, Display, Formatter};

pub use self::string::JsString;
use crate::JsValue;
use crate::util::{PtrConst, PtrLength, PtrMut};

impl JsString {
	#[must_use]
	pub fn new(value: &JsValue) -> Self {
		string::string_constructor(value)
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

		// SAFETY: Parameters are correct.
		unsafe {
			string::string_eq(
				self,
				PtrConst::new(other.as_bytes()),
				PtrLength::new(other.as_bytes()),
			)
		}
	}
}

impl PartialEq<String> for JsString {
	fn eq(&self, other: &String) -> bool {
		self.eq(&other.as_str())
	}
}

impl From<&str> for JsString {
	fn from(value: &str) -> Self {
		// SAFETY: Parameters are correct.
		unsafe {
			string::string_decode(
				PtrConst::new(value.as_bytes()),
				PtrLength::new(value.as_bytes()),
			)
		}
	}
}

impl From<&JsString> for String {
	fn from(value: &JsString) -> Self {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "string.encoder",
			"new TextEncoder()",
		);

		js_bindgen::embed_js!(
			module = "js_sys",
			name = "string.utf8_length",
			required_embeds = [("js_sys", "string.encoder")],
			"(string) => this.#jsEmbed.js_sys['string.encoder'].encode(string).length",
		);

		#[cfg(any(not(target_feature = "atomics"), js_sys_target_feature = "sab"))]
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "string.encode",
			required_embeds = [("js_sys", "string.encoder")],
			"(string, ptr, len) => {{",
			"	const view = new Uint8Array(this.#memory.buffer, ptr, len)",
			"	this.#jsEmbed.js_sys['string.encoder'].encodeInto(string, view)",
			"}}",
		);

		#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "string.encode",
			required_embeds = [("js_sys", "string.encoder"), ("js_sys", "string.sab")],
			"(string, ptr, len) => {{",
			"	if (this.#jsEmbed.js_sys['string.sab']) {{",
			"		const view = new Uint8Array(this.#memory.buffer, ptr, len)",
			"		this.#jsEmbed.js_sys['string.encoder'].encodeInto(string, view)",
			"	}} else {{",
			"		const bytes = this.#jsEmbed.js_sys['string.encoder'].encode(string)",
			"		new Uint8Array(this.#memory.buffer).set(bytes, ptr)",
			"	}}",
			"}}",
		);

		let len = string::string_utf8_length(value);
		#[cfg(target_arch = "wasm32")]
		assert!(
			len < f64::from(u32::MAX),
			"found string length bigger than `usize::MAX`"
		);
		#[expect(
			clippy::cast_possible_truncation,
			clippy::cast_sign_loss,
			reason = "in practice this is memory constrained"
		)]
		let len = len as usize;

		let mut vec = Vec::with_capacity(len);
		// SAFETY: Parameters are correct.
		unsafe {
			string::string_encode(
				value,
				PtrMut::new(&mut vec),
				PtrLength::from_uninit_slice(vec.spare_capacity_mut()),
			);
		}

		// SAFETY: `string.encode` initializes exactly `len` bytes with valid
		// UTF-8 produced by `TextEncoder`.
		unsafe {
			vec.set_len(len);
			Self::from_utf8_unchecked(vec)
		}
	}
}

#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "string.sab",
	"(() => {{",
	"	if (this.#memory.buffer instanceof ArrayBuffer)",
	"		return true",
	"",
	"	const array = new WebAssembly.Memory({{ initial: 0, maximum: 0, shared: true }})",
	"	try {{",
	"		new TextDecoder().decode(array)",
	"		return true",
	"	}} catch {{",
	"		return false",
	"	}}",
	"}})()",
);
