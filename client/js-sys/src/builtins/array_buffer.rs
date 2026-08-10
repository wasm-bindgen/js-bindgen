use super::Object;
use crate::hazard::JsCast;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/ArrayBuffer)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type ArrayBufferOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/ArrayBuffer#maxbytelength)
	#[must_use]
	#[js_sys(getter = "maxByteLength")]
	pub fn max_byte_length(self: &ArrayBufferOptions) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/ArrayBuffer#maxbytelength)
	#[js_sys(setter = "maxByteLength")]
	pub fn set_max_byte_length(self: &ArrayBufferOptions, max_byte_length: f64);
}

impl ArrayBufferOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/ArrayBuffer#maxbytelength)
	#[must_use]
	pub fn new(max_byte_length: f64) -> Self {
		let options = Self::unchecked_from(Object::new().into());
		options.set_max_byte_length(max_byte_length);
		options
	}
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer)
	#[js_sys(js_name = "ArrayBuffer", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type ArrayBuffer;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/ArrayBuffer)
	#[js_sys(constructor)]
	pub fn new(byte_length: f64) -> Result<ArrayBuffer, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/ArrayBuffer)
	#[js_sys(constructor)]
	pub fn new_with_options(
		byte_length: f64,
		options: &ArrayBufferOptions,
	) -> Result<ArrayBuffer, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/byteLength)
	#[must_use]
	#[js_sys(getter = "byteLength")]
	pub fn byte_length(self: &ArrayBuffer) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/detached)
	#[must_use]
	#[js_sys(getter)]
	pub fn detached(self: &ArrayBuffer) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/isView)
	#[must_use]
	#[js_sys(static_of = ArrayBuffer, js_name = "isView")]
	pub fn is_view(value: &JsValue) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/maxByteLength)
	#[must_use]
	#[js_sys(getter = "maxByteLength")]
	pub fn max_byte_length(self: &ArrayBuffer) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/resizable)
	#[must_use]
	#[js_sys(getter)]
	pub fn resizable(self: &ArrayBuffer) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/resize)
	pub fn resize(self: &ArrayBuffer, new_byte_length: f64) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/slice)
	pub fn slice(self: &ArrayBuffer, begin: f64) -> Result<ArrayBuffer, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/slice)
	#[js_sys(js_name = "slice")]
	pub fn slice_range(self: &ArrayBuffer, begin: f64, end: f64) -> Result<ArrayBuffer, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/transfer)
	pub fn transfer(self: &ArrayBuffer) -> Result<ArrayBuffer, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/transfer)
	#[js_sys(js_name = "transfer")]
	pub fn transfer_with_length(
		self: &ArrayBuffer,
		new_byte_length: f64,
	) -> Result<ArrayBuffer, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/transferToFixedLength)
	#[js_sys(js_name = "transferToFixedLength")]
	pub fn transfer_to_fixed_length(self: &ArrayBuffer) -> Result<ArrayBuffer, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer/transferToFixedLength)
	#[js_sys(js_name = "transferToFixedLength")]
	pub fn transfer_to_fixed_length_with_length(
		self: &ArrayBuffer,
		new_byte_length: f64,
	) -> Result<ArrayBuffer, JsValue>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer)
	#[js_sys(js_name = "SharedArrayBuffer", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type SharedArrayBuffer;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/SharedArrayBuffer)
	#[js_sys(constructor)]
	pub fn new(byte_length: f64) -> Result<SharedArrayBuffer, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/SharedArrayBuffer)
	#[js_sys(constructor)]
	pub fn new_with_options(
		byte_length: f64,
		options: &ArrayBufferOptions,
	) -> Result<SharedArrayBuffer, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/byteLength)
	#[must_use]
	#[js_sys(getter = "byteLength")]
	pub fn byte_length(self: &SharedArrayBuffer) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/growable)
	#[must_use]
	#[js_sys(getter)]
	pub fn growable(self: &SharedArrayBuffer) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/maxByteLength)
	#[must_use]
	#[js_sys(getter = "maxByteLength")]
	pub fn max_byte_length(self: &SharedArrayBuffer) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/grow)
	pub fn grow(self: &SharedArrayBuffer, new_byte_length: f64) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/slice)
	pub fn slice(self: &SharedArrayBuffer, begin: f64) -> Result<SharedArrayBuffer, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer/slice)
	#[js_sys(js_name = "slice")]
	pub fn slice_range(
		self: &SharedArrayBuffer,
		begin: f64,
		end: f64,
	) -> Result<SharedArrayBuffer, JsValue>;
}
