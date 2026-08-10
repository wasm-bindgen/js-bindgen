use crate::{Function, JsString, JsValue, Object, js_sys};

#[expect(non_snake_case, reason = "matches the JavaScript global name")]
pub mod JSON {
	use super::*;

	#[js_sys(js_sys = crate, namespace = "JSON")]
	extern "js-sys" {
		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse)
		pub fn parse(text: &str) -> Result<JsValue, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/parse)
		#[js_sys(js_name = "parse")]
		pub fn parse_with_reviver(text: &str, reviver: &Function) -> Result<JsValue, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/stringify)
		pub fn stringify(value: &JsValue) -> Result<Option<JsString>, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/stringify)
		#[js_sys(js_name = "stringify")]
		pub fn stringify_with_replacer(
			value: &JsValue,
			replacer: &JsValue,
		) -> Result<Option<JsString>, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/stringify)
		#[js_sys(js_name = "stringify")]
		pub fn stringify_with_replacer_and_space(
			value: &JsValue,
			replacer: &JsValue,
			space: &JsValue,
		) -> Result<Option<JsString>, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/isRawJSON)
		#[must_use]
		#[js_sys(js_name = "isRawJSON")]
		pub fn is_raw_json(value: &JsValue) -> bool;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/rawJSON)
		#[js_sys(js_name = "rawJSON")]
		pub fn raw_json(text: &str) -> Result<Object, JsValue>;
	}
}
