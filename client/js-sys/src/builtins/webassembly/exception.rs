use super::global::ValueType;
use crate::hazard::JsCast;
use crate::{Array, JsString, JsValue, Object, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Tag/Tag)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type TagDescriptor;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Tag/type)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type TagType;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Tag/Tag#parameters)
	#[must_use]
	#[js_sys(getter = "parameters")]
	pub fn parameters(self: &TagDescriptor) -> Array<JsString>;

	#[js_sys(setter = "parameters")]
	pub(crate) fn set_parameters(self: &TagDescriptor, value: &Array<JsString>);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Tag/type)
	#[must_use]
	#[js_sys(getter = "parameters")]
	pub fn parameters(self: &TagType) -> Array<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Exception/Exception#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type ExceptionOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Exception/Exception#options)
	#[must_use]
	#[js_sys(getter = "traceStack")]
	pub fn trace_stack(self: &ExceptionOptions) -> Option<bool>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Exception/Exception#options)
	#[js_sys(setter = "traceStack")]
	pub fn set_trace_stack(self: &ExceptionOptions, value: bool);
}

impl TagDescriptor {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Tag/Tag)
	#[must_use]
	pub fn new(parameters: &[ValueType]) -> Self {
		let values = Array::new_typed();
		for parameter in parameters {
			let _ = values.push(&JsString::from(parameter.as_str()));
		}

		let descriptor = Self::unchecked_from(Object::new().into());
		descriptor.set_parameters(&values);
		descriptor
	}
}

impl ExceptionOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Exception/Exception#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}
}

impl Default for ExceptionOptions {
	fn default() -> Self {
		Self::new()
	}
}

#[js_sys(js_sys = crate, namespace = "WebAssembly")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Tag)
	#[js_sys(js_name = "Tag", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Tag;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Tag/Tag)
	#[js_sys(constructor)]
	pub fn new(descriptor: &TagDescriptor) -> Result<Tag, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Tag/type)
	#[must_use]
	#[js_sys(js_name = "type")]
	pub fn type_(self: &Tag) -> TagType;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Exception)
	#[js_sys(js_name = "Exception", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Exception;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Exception/Exception)
	#[js_sys(constructor)]
	pub fn new(tag: &Tag, payload: &[JsValue]) -> Result<Exception, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Exception/Exception)
	#[js_sys(constructor)]
	pub fn new_with_options(
		tag: &Tag,
		payload: &[JsValue],
		options: &ExceptionOptions,
	) -> Result<Exception, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Exception/is)
	#[must_use]
	pub fn is(self: &Exception, tag: &Tag) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Exception/getArg)
	#[js_sys(js_name = "getArg")]
	pub fn get_arg(self: &Exception, tag: &Tag, index: u32) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Exception/stack)
	#[must_use]
	#[js_sys(getter)]
	pub fn stack(self: &Exception) -> Option<JsString>;
}

#[js_sys(js_sys = crate, namespace = "WebAssembly")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/JSTag_static)
	#[must_use]
	#[js_sys(getter = "JSTag")]
	pub fn js_tag() -> Tag;
}
