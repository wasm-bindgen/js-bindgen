use alloc::string::String;

use crate::hazard::JsCast;
use crate::{JsString, JsValue, Object, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global/Global#value)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ValueType {
	I32,
	I64,
	F32,
	F64,
	V128,
	ExternRef,
	AnyFunc,
}

impl ValueType {
	pub(crate) const fn as_str(self) -> &'static str {
		match self {
			Self::I32 => "i32",
			Self::I64 => "i64",
			Self::F32 => "f32",
			Self::F64 => "f64",
			Self::V128 => "v128",
			Self::ExternRef => "externref",
			Self::AnyFunc => "anyfunc",
		}
	}

	fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"i32" => Some(Self::I32),
			"i64" => Some(Self::I64),
			"f32" => Some(Self::F32),
			"f64" => Some(Self::F64),
			"v128" => Some(Self::V128),
			"externref" => Some(Self::ExternRef),
			"anyfunc" => Some(Self::AnyFunc),
			_ => None,
		}
	}
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global/Global)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type GlobalDescriptor;

	#[js_sys(getter = "value")]
	fn value_type_raw(self: &GlobalDescriptor) -> JsString;

	#[js_sys(setter = "value")]
	fn set_value_type_raw(self: &GlobalDescriptor, value: &str);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global/Global#mutable)
	#[must_use]
	#[js_sys(getter = "mutable")]
	pub fn mutable(self: &GlobalDescriptor) -> Option<bool>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global/Global#mutable)
	#[js_sys(setter = "mutable")]
	pub fn set_mutable(self: &GlobalDescriptor, value: bool);
}

impl GlobalDescriptor {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global/Global)
	#[must_use]
	pub fn new(value_type: ValueType) -> Self {
		let descriptor = Self::unchecked_from(Object::new().into());
		descriptor.set_value_type(value_type);
		descriptor
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global/Global#value)
	#[must_use]
	pub fn value_type(&self) -> Option<ValueType> {
		ValueType::from_js_string(&self.value_type_raw())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global/Global#value)
	pub fn set_value_type(&self, value: ValueType) {
		self.set_value_type_raw(value.as_str());
	}
}

#[js_sys(js_sys = crate, namespace = "WebAssembly")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global)
	#[js_sys(js_name = "Global", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Global;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global/Global)
	#[js_sys(constructor)]
	pub fn new(descriptor: &GlobalDescriptor) -> Result<Global, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global/Global)
	#[js_sys(constructor)]
	pub fn new_with_value(
		descriptor: &GlobalDescriptor,
		value: &JsValue,
	) -> Result<Global, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global/value)
	#[js_sys(getter)]
	pub fn value(self: &Global) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global/valueOf)
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &Global) -> Result<JsValue, JsValue>;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "webassembly.global.set_value",
	"(global, value) => {{ global.value = value }}",
);

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "webassembly.global.set_value")]
	fn set_global_value(global: &Global, value: &JsValue) -> Result<(), JsValue>;
}

impl Global {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Global/value)
	pub fn set_value(&self, value: &JsValue) -> Result<(), JsValue> {
		set_global_value(self, value)
	}
}
