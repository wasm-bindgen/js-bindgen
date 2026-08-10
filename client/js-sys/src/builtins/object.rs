use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[derive(Clone, Debug)]
	pub type Object<T = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> Object;
}

impl Default for Object {
	fn default() -> Self {
		Self::new()
	}
}
