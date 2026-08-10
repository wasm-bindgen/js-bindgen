use super::module::Module;
use crate::{JsValue, Object, js_sys};

#[js_sys(js_sys = crate, namespace = "WebAssembly")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Instance)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Instance;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Instance/Instance)
	#[js_sys(constructor)]
	pub fn new(module: &Module) -> Result<Instance, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Instance/Instance)
	#[js_sys(constructor)]
	pub fn new_with_imports(module: &Module, imports: &Object) -> Result<Instance, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Instance/exports)
	#[must_use]
	#[js_sys(getter)]
	pub fn exports(self: &Instance) -> Object;
}
