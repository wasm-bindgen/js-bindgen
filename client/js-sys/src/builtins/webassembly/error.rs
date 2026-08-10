use crate::{Error, ErrorOptions, Object, js_sys};

macro_rules! error_types {
	($(
		$type:ident = $js_name:literal {
			type_doc = $type_doc:literal,
			constructor_doc = $constructor_doc:literal,
		}
	)*) => {$(
		#[js_sys(js_sys = crate, namespace = "WebAssembly")]
		extern "js-sys" {
			#[doc = $type_doc]
			#[js_sys(js_name = $js_name, extends = Error, extends = Object)]
			#[derive(Clone, Debug, PartialEq)]
			pub type $type;

			#[doc = $constructor_doc]
			#[must_use]
			#[js_sys(constructor)]
			pub fn new(message: &str) -> $type;

			#[doc = $constructor_doc]
			#[must_use]
			#[js_sys(constructor)]
			pub fn new_with_options(message: &str, options: &ErrorOptions) -> $type;
		}
	)*};
}

error_types! {
	CompileError = "CompileError" {
		type_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/CompileError)",
		constructor_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/CompileError/CompileError)",
	}
	LinkError = "LinkError" {
		type_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/LinkError)",
		constructor_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/LinkError/LinkError)",
	}
	RuntimeError = "RuntimeError" {
		type_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/RuntimeError)",
		constructor_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/RuntimeError/RuntimeError)",
	}
	SuspendError = "SuspendError" {
		type_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/SuspendError)",
		constructor_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/SuspendError/SuspendError)",
	}
}
