use crate::{JsString, JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/decodeURI)
	#[js_sys(js_name = "decodeURI")]
	pub fn decode_uri(uri: &str) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/decodeURIComponent)
	#[js_sys(js_name = "decodeURIComponent")]
	pub fn decode_uri_component(component: &str) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURI)
	#[must_use]
	#[js_sys(js_name = "encodeURI")]
	pub fn encode_uri(uri: &str) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent)
	#[must_use]
	#[js_sys(js_name = "encodeURIComponent")]
	pub fn encode_uri_component(component: &str) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval)
	pub fn eval(source: &str) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isFinite)
	#[js_sys(js_name = "isFinite")]
	pub fn is_finite(value: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/isNaN)
	#[js_sys(js_name = "isNaN")]
	pub fn is_nan(value: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/parseFloat)
	#[must_use]
	#[js_sys(js_name = "parseFloat")]
	pub fn parse_float(value: &str) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/parseInt)
	#[must_use]
	#[js_sys(js_name = "parseInt")]
	pub fn parse_int(value: &str) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/parseInt)
	#[must_use]
	#[js_sys(js_name = "parseInt")]
	pub fn parse_int_with_radix(value: &str, radix: u8) -> f64;

}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/globalThis)
	#[must_use]
	#[js_sys(js_embed = "global.this")]
	pub fn global_this() -> JsValue;
}

js_bindgen::embed_js!(module = "js_sys", name = "global.this", "() => globalThis");
