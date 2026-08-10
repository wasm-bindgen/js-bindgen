use crate::{Function, JsValue, Object, js_sys};

#[js_sys(js_sys = crate, namespace = "WebAssembly")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Suspending)
	#[js_sys(js_name = "Suspending", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Suspending;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Suspending/Suspending)
	#[js_sys(constructor)]
	pub fn new(function: &Function) -> Result<Suspending, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/promising_static)
	pub fn promising(function: &Function) -> Result<Function, JsValue>;
}
