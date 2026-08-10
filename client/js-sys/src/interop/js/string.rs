use alloc::string::String;
use core::convert::Infallible;
use core::fmt::{self, Display, Formatter};
use core::str::FromStr;

use crate::interop::string::js_string_from_str;
use crate::util::{PtrConst, PtrLength};
use crate::{JsString, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "string.eq")]
	// SAFETY: The pointer and length must describe a valid UTF-8 byte slice.
	#[expect(
		clippy::allow_attributes,
		reason = "the macro emits an unsafe ABI call"
	)]
	#[allow(
		clippy::undocumented_unsafe_blocks,
		reason = "the safety requirement is documented on this declaration"
	)]
	unsafe fn string_eq(string: &JsString, array: PtrConst<u8>, len: PtrLength<u8>) -> bool;

	#[js_sys(js_embed = "string.identity")]
	fn string_from_owned(value: String) -> JsString;

	#[js_sys(js_embed = "string.identity")]
	fn string_to_owned(value: &JsString) -> String;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "string.identity",
	"value => value"
);

impl Display for JsString {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		Display::fmt(&String::from(self), formatter)
	}
}

impl fmt::Debug for JsString {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(&String::from(self), formatter)
	}
}

impl PartialEq<str> for JsString {
	fn eq(&self, other: &str) -> bool {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "string.eq",
			required_embeds = [("js_sys", "string.decode")],
			"(string, ptr, len) => {{",
			"    const other = this.#jsEmbed.js_sys['string.decode'](ptr, len)",
			"    return string === other",
			"}}",
		);

		// SAFETY: Parameters are correct.
		unsafe {
			string_eq(
				self,
				PtrConst::new(other.as_bytes()),
				PtrLength::new(other.as_bytes()),
			)
		}
	}
}

impl PartialEq<&str> for JsString {
	fn eq(&self, other: &&str) -> bool {
		<Self as PartialEq<str>>::eq(self, other)
	}
}

impl PartialEq<String> for JsString {
	fn eq(&self, other: &String) -> bool {
		<Self as PartialEq<str>>::eq(self, other)
	}
}

impl PartialEq<&String> for JsString {
	fn eq(&self, other: &&String) -> bool {
		<Self as PartialEq<str>>::eq(self, other)
	}
}

impl From<&str> for JsString {
	fn from(value: &str) -> Self {
		js_string_from_str(value)
	}
}

impl From<&JsString> for String {
	fn from(value: &JsString) -> Self {
		string_to_owned(value)
	}
}

impl From<JsString> for String {
	fn from(value: JsString) -> Self {
		Self::from(&value)
	}
}

impl From<String> for JsString {
	fn from(value: String) -> Self {
		string_from_owned(value)
	}
}

impl From<char> for JsString {
	fn from(value: char) -> Self {
		let mut buffer = [0; 4];
		let value: &str = value.encode_utf8(&mut buffer);
		Self::from(value)
	}
}

impl Default for JsString {
	fn default() -> Self {
		Self::from("")
	}
}

impl FromStr for JsString {
	type Err = Infallible;

	fn from_str(value: &str) -> Result<Self, Self::Err> {
		Ok(Self::from(value))
	}
}
