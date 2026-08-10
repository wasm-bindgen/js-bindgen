use super::Object;
use crate::{JsString, JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean)
	#[js_sys(js_name = "Boolean", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Boolean;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean/Boolean)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> Boolean;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean/Boolean)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_value(value: &JsValue) -> Boolean;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &Boolean) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean/valueOf)
	#[must_use]
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &Boolean) -> bool;
}

impl Eq for Boolean {}

impl Default for Boolean {
	fn default() -> Self {
		Self::new()
	}
}
