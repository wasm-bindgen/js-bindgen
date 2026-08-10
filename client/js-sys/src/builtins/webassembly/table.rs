use alloc::string::String;

use super::address::AddressType;
use crate::hazard::JsCast;
use crate::{JsString, JsValue, Object, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/Table#element)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TableElement {
	AnyFunc,
	ExternRef,
}

impl TableElement {
	const fn as_str(self) -> &'static str {
		match self {
			Self::AnyFunc => "anyfunc",
			Self::ExternRef => "externref",
		}
	}

	fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"anyfunc" => Some(Self::AnyFunc),
			"externref" => Some(Self::ExternRef),
			_ => None,
		}
	}
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/Table)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type TableDescriptor;

	#[js_sys(getter = "element")]
	fn element_raw(self: &TableDescriptor) -> JsString;

	#[js_sys(setter = "element")]
	fn set_element_raw(self: &TableDescriptor, value: &str);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/Table#initial)
	#[must_use]
	#[js_sys(getter = "initial")]
	pub fn initial(self: &TableDescriptor) -> JsValue;

	#[js_sys(setter = "initial")]
	fn set_initial32(self: &TableDescriptor, value: u32);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/Table#maximum)
	#[must_use]
	#[js_sys(getter = "maximum")]
	pub fn maximum(self: &TableDescriptor) -> Option<JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/Table#maximum)
	#[js_sys(setter = "maximum")]
	pub fn set_maximum(self: &TableDescriptor, value: u32);

}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	#[js_sys(setter = "initial")]
	fn set_initial64(self: &TableDescriptor, value: u64);

	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	#[js_sys(setter = "maximum")]
	pub fn set_maximum64(self: &TableDescriptor, value: u64);

	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	#[js_sys(getter = "address")]
	fn address_raw(self: &TableDescriptor) -> Option<JsString>;

	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	#[js_sys(setter = "address")]
	fn set_address_raw(self: &TableDescriptor, value: &str);
}

impl TableDescriptor {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/Table)
	#[must_use]
	pub fn new(element: TableElement, initial: u32) -> Self {
		let descriptor = Self::unchecked_from(Object::new().into());
		descriptor.set_element(element);
		descriptor.set_initial32(initial);
		descriptor
	}

	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	#[must_use]
	pub fn new64(element: TableElement, initial: u64) -> Self {
		let descriptor = Self::unchecked_from(Object::new().into());
		descriptor.set_element(element);
		descriptor.set_address(AddressType::I64);
		descriptor.set_initial64(initial);
		descriptor
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/Table#element)
	#[must_use]
	pub fn element(&self) -> Option<TableElement> {
		TableElement::from_js_string(&self.element_raw())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/Table#element)
	pub fn set_element(&self, value: TableElement) {
		self.set_element_raw(value.as_str());
	}

	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	#[must_use]
	pub fn address(&self) -> Option<AddressType> {
		self.address_raw()
			.as_ref()
			.and_then(AddressType::from_js_string)
	}

	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	pub fn set_address(&self, value: AddressType) {
		self.set_address_raw(value.as_str());
	}
}

#[js_sys(js_sys = crate, namespace = "WebAssembly")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table)
	#[js_sys(js_name = "Table", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Table;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/Table)
	#[js_sys(constructor)]
	pub fn new(descriptor: &TableDescriptor) -> Result<Table, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/Table)
	#[js_sys(constructor)]
	pub fn new_with_value(descriptor: &TableDescriptor, value: &JsValue) -> Result<Table, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/length)
	#[must_use]
	#[js_sys(getter)]
	pub fn length(self: &Table) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/get)
	pub fn get(self: &Table, index: u32) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/grow)
	pub fn grow(self: &Table, delta: u32) -> Result<u32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/grow)
	#[js_sys(js_name = "grow")]
	pub fn grow_with_value(self: &Table, delta: u32, value: &JsValue) -> Result<u32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/set)
	pub fn set(self: &Table, index: u32) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Table/set)
	#[js_sys(js_name = "set")]
	pub fn set_with_value(self: &Table, index: u32, value: &JsValue) -> Result<(), JsValue>;
}

#[js_sys(js_sys = crate, namespace = "WebAssembly")]
extern "js-sys" {
	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	#[must_use]
	#[js_sys(getter = "length")]
	pub fn length64(self: &Table) -> u64;

	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	#[js_sys(js_name = "get")]
	pub fn get64(self: &Table, index: u64) -> Result<JsValue, JsValue>;

	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	#[js_sys(js_name = "grow")]
	pub fn grow64(self: &Table, delta: u64) -> Result<u64, JsValue>;

	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	#[js_sys(js_name = "grow")]
	pub fn grow64_with_value(self: &Table, delta: u64, value: &JsValue) -> Result<u64, JsValue>;

	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	#[js_sys(js_name = "set")]
	pub fn set64(self: &Table, index: u64) -> Result<(), JsValue>;

	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#tables)
	#[js_sys(js_name = "set")]
	pub fn set64_with_value(self: &Table, index: u64, value: &JsValue) -> Result<(), JsValue>;
}
