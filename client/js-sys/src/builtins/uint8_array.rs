use super::{Object, Uint8Array};
use crate::hazard::JsCast;
use crate::{JsString, JsValue, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/fromBase64#alphabet)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Base64Alphabet {
	Base64,
	Base64Url,
}

impl Base64Alphabet {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Base64 => "base64",
			Self::Base64Url => "base64url",
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/fromBase64#lastchunkhandling)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Base64LastChunkHandling {
	Loose,
	Strict,
	StopBeforePartial,
}

impl Base64LastChunkHandling {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Loose => "loose",
			Self::Strict => "strict",
			Self::StopBeforePartial => "stop-before-partial",
		}
	}
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/fromBase64#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Base64DecodeOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/toBase64#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Base64EncodeOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/setFromBase64#return_value)
	///
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/setFromHex#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Uint8ArraySetResult;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/setFromBase64#return_value)
	#[must_use]
	#[js_sys(getter = "read")]
	pub fn read(self: &Uint8ArraySetResult) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/setFromBase64#return_value)
	#[must_use]
	#[js_sys(getter = "written")]
	pub fn written(self: &Uint8ArraySetResult) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/fromBase64)
	#[js_sys(static_of = Uint8Array, js_name = "fromBase64")]
	pub fn from_base64(string: &str) -> Result<Uint8Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/fromBase64)
	#[js_sys(static_of = Uint8Array, js_name = "fromBase64")]
	pub fn from_base64_with_options(
		string: &str,
		options: &Base64DecodeOptions,
	) -> Result<Uint8Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/fromHex)
	#[js_sys(static_of = Uint8Array, js_name = "fromHex")]
	pub fn from_hex(string: &str) -> Result<Uint8Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/setFromBase64)
	#[js_sys(js_name = "setFromBase64")]
	pub fn set_from_base64(self: &Uint8Array, string: &str)
	-> Result<Uint8ArraySetResult, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/setFromBase64)
	#[js_sys(js_name = "setFromBase64")]
	pub fn set_from_base64_with_options(
		self: &Uint8Array,
		string: &str,
		options: &Base64DecodeOptions,
	) -> Result<Uint8ArraySetResult, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/setFromHex)
	#[js_sys(js_name = "setFromHex")]
	pub fn set_from_hex(self: &Uint8Array, string: &str) -> Result<Uint8ArraySetResult, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/toBase64)
	#[js_sys(js_name = "toBase64")]
	pub fn to_base64(self: &Uint8Array) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/toBase64)
	#[js_sys(js_name = "toBase64")]
	pub fn to_base64_with_options(
		self: &Uint8Array,
		options: &Base64EncodeOptions,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/toHex)
	#[js_sys(js_name = "toHex")]
	pub fn to_hex(self: &Uint8Array) -> Result<JsString, JsValue>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(setter = "alphabet")]
	fn set_decode_alphabet(self: &Base64DecodeOptions, alphabet: &str);

	#[js_sys(setter = "lastChunkHandling")]
	fn set_last_chunk_handling_raw(self: &Base64DecodeOptions, handling: &str);

	#[js_sys(setter = "alphabet")]
	fn set_encode_alphabet(self: &Base64EncodeOptions, alphabet: &str);

	#[js_sys(setter = "omitPadding")]
	fn set_omit_padding_raw(self: &Base64EncodeOptions, omit_padding: bool);
}

impl Base64DecodeOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/fromBase64#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/fromBase64#alphabet)
	pub fn set_alphabet(&self, alphabet: Base64Alphabet) {
		self.set_decode_alphabet(alphabet.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/fromBase64#lastchunkhandling)
	pub fn set_last_chunk_handling(&self, handling: Base64LastChunkHandling) {
		self.set_last_chunk_handling_raw(handling.as_str());
	}
}

impl Default for Base64DecodeOptions {
	fn default() -> Self {
		Self::new()
	}
}

impl Base64EncodeOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/toBase64#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/toBase64#alphabet)
	pub fn set_alphabet(&self, alphabet: Base64Alphabet) {
		self.set_encode_alphabet(alphabet.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array/toBase64#omitpadding)
	pub fn set_omit_padding(&self, omit_padding: bool) {
		self.set_omit_padding_raw(omit_padding);
	}
}

impl Default for Base64EncodeOptions {
	fn default() -> Self {
		Self::new()
	}
}
