use alloc::string::String;
use alloc::vec::Vec;

use super::instance::Instance;
use super::module::Module;
use crate::hazard::JsCast;
use crate::{Array, JsString, JsValue, Object, Promise, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/compile_static)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CompileBuiltin {
	JsString,
}

impl CompileBuiltin {
	const fn as_str(self) -> &'static str {
		match self {
			Self::JsString => "js-string",
		}
	}

	fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"js-string" => Some(Self::JsString),
			_ => None,
		}
	}
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/compile_static)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type CompileOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiate_static)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type InstantiatedSource;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiate_static)
	#[must_use]
	#[js_sys(getter)]
	pub fn module(self: &InstantiatedSource) -> Module;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiate_static)
	#[must_use]
	#[js_sys(getter)]
	pub fn instance(self: &InstantiatedSource) -> Instance;
}

#[js_sys(js_sys = crate, namespace = "WebAssembly")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/compile_static)
	pub fn compile(bytes: &JsValue) -> Promise<Module>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/compile_static)
	#[js_sys(js_name = "compile")]
	pub fn compile_with_options(bytes: &JsValue, options: &CompileOptions) -> Promise<Module>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/compileStreaming_static)
	#[js_sys(js_name = "compileStreaming")]
	pub fn compile_streaming(source: &JsValue) -> Promise<Module>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/compileStreaming_static)
	#[js_sys(js_name = "compileStreaming")]
	pub fn compile_streaming_with_options(
		source: &JsValue,
		options: &CompileOptions,
	) -> Promise<Module>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiate_static)
	#[js_sys(js_name = "instantiate")]
	pub fn instantiate_bytes(bytes: &JsValue) -> Promise<InstantiatedSource>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiate_static)
	#[js_sys(js_name = "instantiate")]
	pub fn instantiate_bytes_with_imports(
		bytes: &JsValue,
		imports: &Object,
	) -> Promise<InstantiatedSource>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiate_static)
	#[js_sys(js_name = "instantiate")]
	pub fn instantiate_bytes_with_imports_and_options(
		bytes: &JsValue,
		imports: &Object,
		options: &CompileOptions,
	) -> Promise<InstantiatedSource>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiate_static)
	#[js_sys(js_name = "instantiate")]
	pub fn instantiate_module(module: &Module) -> Promise<Instance>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiate_static)
	#[js_sys(js_name = "instantiate")]
	pub fn instantiate_module_with_imports(module: &Module, imports: &Object) -> Promise<Instance>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiateStreaming_static)
	#[js_sys(js_name = "instantiateStreaming")]
	pub fn instantiate_streaming(source: &JsValue) -> Promise<InstantiatedSource>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiateStreaming_static)
	#[js_sys(js_name = "instantiateStreaming")]
	pub fn instantiate_streaming_with_imports(
		source: &JsValue,
		imports: &Object,
	) -> Promise<InstantiatedSource>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/instantiateStreaming_static)
	#[js_sys(js_name = "instantiateStreaming")]
	pub fn instantiate_streaming_with_imports_and_options(
		source: &JsValue,
		imports: &Object,
		options: &CompileOptions,
	) -> Promise<InstantiatedSource>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/validate_static)
	pub fn validate(bytes: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/validate_static)
	#[js_sys(js_name = "validate")]
	pub fn validate_with_options(
		bytes: &JsValue,
		options: &CompileOptions,
	) -> Result<bool, JsValue>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(getter = "builtins")]
	pub(crate) fn builtins_raw(self: &CompileOptions) -> Option<Array<JsString>>;

	#[js_sys(setter = "builtins")]
	pub(crate) fn set_builtins_raw(self: &CompileOptions, value: &Array<JsString>);

	#[js_sys(getter = "importedStringConstants")]
	fn imported_string_constants_raw(self: &CompileOptions) -> Option<JsString>;

	#[js_sys(setter = "importedStringConstants")]
	fn set_imported_string_constants_raw(self: &CompileOptions, value: &str);
}

impl CompileOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/compile_static)
	#[must_use]
	pub fn builtins(&self) -> Option<Vec<CompileBuiltin>> {
		self.builtins_raw()?
			.iter()
			.map(|value| CompileBuiltin::from_js_string(&value))
			.collect()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/compile_static)
	pub fn set_builtins(&self, values: &[CompileBuiltin]) {
		let builtins: Array<JsString> = Array::new_typed();
		for value in values {
			let value = JsString::from(value.as_str());
			let _ = builtins.push(&value);
		}
		self.set_builtins_raw(&builtins);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/compile_static)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/compile_static)
	#[must_use]
	pub fn imported_string_constants(&self) -> Option<JsString> {
		self.imported_string_constants_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/compile_static)
	pub fn set_imported_string_constants(&self, value: &str) {
		self.set_imported_string_constants_raw(value);
	}
}

impl Default for CompileOptions {
	fn default() -> Self {
		Self::new()
	}
}
