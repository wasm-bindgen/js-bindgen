use super::JsString;
use crate::JsValue;

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number)
	#[js_sys(js_name = "Number")]
	#[derive(Clone, Debug, PartialEq)]
	pub type Number<T = f64>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/Number)
	#[js_sys(js_name = "Number")]
	fn number_constructor(value: &JsValue) -> Result<Number, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/isFinite)
	#[must_use]
	#[js_sys(static_of = Number, js_name = "isFinite")]
	pub fn is_finite(value: &JsValue) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/isInteger)
	#[must_use]
	#[js_sys(static_of = Number, js_name = "isInteger")]
	pub fn is_integer(value: &JsValue) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/isNaN)
	#[must_use]
	#[js_sys(static_of = Number, js_name = "isNaN")]
	pub fn is_nan(value: &JsValue) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/isSafeInteger)
	#[must_use]
	#[js_sys(static_of = Number, js_name = "isSafeInteger")]
	pub fn is_safe_integer(value: &JsValue) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/parseFloat)
	#[must_use]
	#[js_sys(static_of = Number, js_name = "parseFloat")]
	pub fn parse_float(value: &str) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/parseInt)
	#[must_use]
	#[js_sys(static_of = Number, js_name = "parseInt")]
	pub fn parse_int(value: &str, radix: u8) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toExponential)
	#[must_use]
	#[js_sys(js_name = "toExponential")]
	pub fn to_exponential<T>(self: &Number<T>) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toExponential)
	#[js_sys(js_name = "toExponential")]
	pub fn to_exponential_with_digits<T>(
		self: &Number<T>,
		fraction_digits: u8,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toFixed)
	#[must_use]
	#[js_sys(js_name = "toFixed")]
	pub fn to_fixed<T>(self: &Number<T>) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toFixed)
	#[js_sys(js_name = "toFixed")]
	pub fn to_fixed_with_digits<T>(self: &Number<T>, digits: u8) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toLocaleString)
	#[must_use]
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string<T>(self: &Number<T>) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locale<T>(
		self: &Number<T>,
		locale: &str,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_options<T>(
		self: &Number<T>,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toPrecision)
	#[must_use]
	#[js_sys(js_name = "toPrecision")]
	pub fn to_precision<T>(self: &Number<T>) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toPrecision)
	#[js_sys(js_name = "toPrecision")]
	pub fn to_precision_with_digits<T>(
		self: &Number<T>,
		precision: u8,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string<T>(self: &Number<T>) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toString)
	#[js_sys(js_name = "toString")]
	pub fn to_string_with_radix<T>(self: &Number<T>, radix: u8) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/valueOf)
	#[must_use]
	#[js_sys(js_name = "valueOf")]
	pub fn value_of<T>(self: &Number<T>) -> f64;
}

impl Number {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/Number)
	pub fn new(value: &JsValue) -> Result<Self, JsValue> {
		number_constructor(value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/EPSILON)
	pub const EPSILON: f64 = f64::EPSILON;
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/MAX_SAFE_INTEGER)
	pub const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/MAX_VALUE)
	pub const MAX_VALUE: f64 = f64::MAX;
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/MIN_SAFE_INTEGER)
	pub const MIN_SAFE_INTEGER: f64 = -9_007_199_254_740_991.0;
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/MIN_VALUE)
	pub const MIN_VALUE: f64 = f64::from_bits(1);
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/NaN)
	pub const NAN: f64 = f64::NAN;
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/NEGATIVE_INFINITY)
	pub const NEGATIVE_INFINITY: f64 = f64::NEG_INFINITY;
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/POSITIVE_INFINITY)
	pub const POSITIVE_INFINITY: f64 = f64::INFINITY;
}
