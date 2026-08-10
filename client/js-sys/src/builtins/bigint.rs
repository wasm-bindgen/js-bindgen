use super::JsString;
use crate::JsValue;

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt)
	#[js_sys(js_name = "BigInt")]
	#[derive(Clone, Debug, PartialEq)]
	pub type BigInt;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt/BigInt)
	#[js_sys(js_name = "BigInt")]
	fn bigint_constructor(value: &JsValue) -> Result<BigInt, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt/asIntN)
	#[js_sys(static_of = BigInt, js_name = "asIntN")]
	pub fn as_int_n(bits: f64, value: &BigInt) -> Result<BigInt, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt/asUintN)
	#[js_sys(static_of = BigInt, js_name = "asUintN")]
	pub fn as_uint_n(bits: f64, value: &BigInt) -> Result<BigInt, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt/toLocaleString)
	#[must_use]
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string(self: &BigInt) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locale(self: &BigInt, locale: &str) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_options(
		self: &BigInt,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &BigInt) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt/toString)
	#[js_sys(js_name = "toString")]
	pub fn to_string_with_radix(self: &BigInt, radix: u8) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt/valueOf)
	#[must_use]
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &BigInt) -> BigInt;
}

impl Eq for BigInt {}

impl BigInt {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt/BigInt)
	pub fn new(value: &JsValue) -> Result<Self, JsValue> {
		bigint_constructor(value)
	}
}
