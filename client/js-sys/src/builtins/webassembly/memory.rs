use super::address::AddressType;
use crate::hazard::JsCast;
use crate::{JsString, JsValue, Object, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type MemoryDescriptor;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory#initial)
	#[must_use]
	#[js_sys(getter = "initial")]
	pub fn initial(self: &MemoryDescriptor) -> JsValue;

	#[js_sys(setter = "initial")]
	fn set_initial32(self: &MemoryDescriptor, value: u32);

	#[js_sys(setter = "initial")]
	fn set_initial64(self: &MemoryDescriptor, value: u64);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory#maximum)
	#[must_use]
	#[js_sys(getter = "maximum")]
	pub fn maximum(self: &MemoryDescriptor) -> Option<JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory#maximum)
	#[js_sys(setter = "maximum")]
	pub fn set_maximum(self: &MemoryDescriptor, value: u32);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory#using_a_64-bit_address)
	#[js_sys(setter = "maximum")]
	pub fn set_maximum64(self: &MemoryDescriptor, value: u64);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory#shared)
	#[must_use]
	#[js_sys(getter = "shared")]
	pub fn shared(self: &MemoryDescriptor) -> Option<bool>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory#shared)
	#[js_sys(setter = "shared")]
	pub fn set_shared(self: &MemoryDescriptor, value: bool);

	#[js_sys(getter = "address")]
	fn address_raw(self: &MemoryDescriptor) -> Option<JsString>;

	#[js_sys(setter = "address")]
	fn set_address_raw(self: &MemoryDescriptor, value: &str);
}

impl MemoryDescriptor {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory)
	#[must_use]
	pub fn new(initial: u32) -> Self {
		let descriptor = Self::unchecked_from(Object::new().into());
		descriptor.set_initial32(initial);
		descriptor
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory#using_a_64-bit_address)
	#[must_use]
	pub fn new64(initial: u64) -> Self {
		let descriptor = Self::unchecked_from(Object::new().into());
		descriptor.set_address(AddressType::I64);
		descriptor.set_initial64(initial);
		descriptor
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory#using_a_64-bit_address)
	#[must_use]
	pub fn address(&self) -> Option<AddressType> {
		self.address_raw()
			.as_ref()
			.and_then(AddressType::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory#using_a_64-bit_address)
	pub fn set_address(&self, value: AddressType) {
		self.set_address_raw(value.as_str());
	}
}

#[js_sys(js_sys = crate, namespace = "WebAssembly")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory)
	#[js_sys(js_name = "Memory", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Memory;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory)
	#[js_sys(constructor)]
	pub fn new(descriptor: &MemoryDescriptor) -> Result<Memory, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/buffer)
	#[must_use]
	#[js_sys(getter)]
	pub fn buffer(self: &Memory) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/grow)
	pub fn grow(self: &Memory, delta: u32) -> Result<u32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/grow)
	#[js_sys(js_name = "grow")]
	pub fn grow64(self: &Memory, delta: u64) -> Result<u64, JsValue>;
}

#[js_sys(js_sys = crate, namespace = "WebAssembly")]
extern "js-sys" {
	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#dom-memory-tofixedlengthbuffer)
	#[must_use]
	#[js_sys(js_name = "toFixedLengthBuffer")]
	pub fn to_fixed_length_buffer(self: &Memory) -> JsValue;

	/// [`WebAssembly` JavaScript interface](https://webassembly.github.io/spec/js-api/#dom-memory-toresizablebuffer)
	#[js_sys(js_name = "toResizableBuffer")]
	pub fn to_resizable_buffer(self: &Memory) -> Result<JsValue, JsValue>;
}
