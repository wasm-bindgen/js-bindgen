use super::object::Object;
use crate::hazard::JsCast;
use crate::{JsString, JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/Error#cause)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug)]
	pub type ErrorOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause)
	#[must_use]
	#[js_sys(getter)]
	pub fn get_cause(self: &ErrorOptions) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause)
	#[js_sys(setter)]
	pub fn set_cause(self: &ErrorOptions, cause: &JsValue);
}

impl ErrorOptions {
	/// Construct a new `ErrorOptions` dictionary with the given `cause`.
	#[must_use]
	pub fn new(cause: &JsValue) -> Self {
		let ret: Self = JsCast::unchecked_from(Object::new().into());
		ret.set_cause(cause);
		ret
	}
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug)]
	pub type Error;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/Error)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new(message: &str) -> Error;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/Error)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_options(message: &str, options: &ErrorOptions) -> Error;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause)
	#[must_use]
	#[js_sys(getter)]
	pub fn cause(self: &Error) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause)
	#[js_sys(setter)]
	pub fn set_cause(self: &Error, cause: &JsValue);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/message)
	#[must_use]
	#[js_sys(getter)]
	pub fn message(self: &Error) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/message)
	#[js_sys(setter)]
	pub fn set_message(self: &Error, message: &str);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/name)
	#[must_use]
	#[js_sys(getter)]
	pub fn name(self: &Error) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/name)
	#[js_sys(setter)]
	pub fn set_name(self: &Error, name: &str);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &Error) -> JsString;
}
