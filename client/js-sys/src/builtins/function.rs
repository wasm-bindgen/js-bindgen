use super::object::Object;
use crate::{JsString, JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug)]
	pub type Function;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/Function)
	#[js_sys(constructor)]
	pub fn new_no_args(body: &str) -> Result<Function, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/Function)
	#[js_sys(constructor)]
	pub fn new_with_args(args: &str, body: &str) -> Result<Function, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/apply)
	pub fn apply(self: &Function, this_arg: &JsValue, args: &JsValue) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/bind)
	#[must_use]
	#[js_sys(variadic)]
	pub fn bind(self: &Function, this_arg: &JsValue, args: &[JsValue]) -> Function;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/call)
	#[js_sys(variadic)]
	pub fn call(self: &Function, this_arg: &JsValue, args: &[JsValue]) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/length)
	#[must_use]
	#[js_sys(getter)]
	pub fn length(self: &Function) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/name)
	#[must_use]
	#[js_sys(getter)]
	pub fn name(self: &Function) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/prototype)
	#[must_use]
	#[js_sys(getter)]
	pub fn prototype(self: &Function) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/prototype)
	#[js_sys(setter)]
	pub fn set_prototype(self: &Function, prototype: &JsValue);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &Function) -> JsString;
}
