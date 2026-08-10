use super::Object;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView)
	#[js_sys(js_name = "DataView", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DataView;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/DataView)
	#[js_sys(constructor)]
	pub fn new(buffer: &JsValue) -> Result<DataView, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/DataView)
	#[js_sys(constructor)]
	pub fn new_with_offset(buffer: &JsValue, byte_offset: f64) -> Result<DataView, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/DataView)
	#[js_sys(constructor)]
	pub fn new_with_offset_and_length(
		buffer: &JsValue,
		byte_offset: f64,
		byte_length: f64,
	) -> Result<DataView, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/buffer)
	#[must_use]
	#[js_sys(getter)]
	pub fn buffer(self: &DataView) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/byteLength)
	#[js_sys(getter = "byteLength")]
	pub fn byte_length(self: &DataView) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/byteOffset)
	#[js_sys(getter = "byteOffset")]
	pub fn byte_offset(self: &DataView) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getBigInt64)
	#[js_sys(js_name = "getBigInt64")]
	pub fn get_big_int64(self: &DataView, byte_offset: f64) -> Result<i64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getBigInt64)
	#[js_sys(js_name = "getBigInt64")]
	pub fn get_big_int64_endian(
		self: &DataView,
		byte_offset: f64,
		little_endian: bool,
	) -> Result<i64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getBigUint64)
	#[js_sys(js_name = "getBigUint64")]
	pub fn get_big_uint64(self: &DataView, byte_offset: f64) -> Result<u64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getBigUint64)
	#[js_sys(js_name = "getBigUint64")]
	pub fn get_big_uint64_endian(
		self: &DataView,
		byte_offset: f64,
		little_endian: bool,
	) -> Result<u64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat16)
	#[js_sys(js_name = "getFloat16")]
	pub fn get_float16_as_f32(self: &DataView, byte_offset: f64) -> Result<f32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat16)
	#[js_sys(js_name = "getFloat16")]
	pub fn get_float16_endian_as_f32(
		self: &DataView,
		byte_offset: f64,
		little_endian: bool,
	) -> Result<f32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat32)
	#[js_sys(js_name = "getFloat32")]
	pub fn get_float32(self: &DataView, byte_offset: f64) -> Result<f32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat32)
	#[js_sys(js_name = "getFloat32")]
	pub fn get_float32_endian(
		self: &DataView,
		byte_offset: f64,
		little_endian: bool,
	) -> Result<f32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat64)
	#[js_sys(js_name = "getFloat64")]
	pub fn get_float64(self: &DataView, byte_offset: f64) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getFloat64)
	#[js_sys(js_name = "getFloat64")]
	pub fn get_float64_endian(
		self: &DataView,
		byte_offset: f64,
		little_endian: bool,
	) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt8)
	#[js_sys(js_name = "getInt8")]
	pub fn get_int8(self: &DataView, byte_offset: f64) -> Result<i8, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt16)
	#[js_sys(js_name = "getInt16")]
	pub fn get_int16(self: &DataView, byte_offset: f64) -> Result<i16, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt16)
	#[js_sys(js_name = "getInt16")]
	pub fn get_int16_endian(
		self: &DataView,
		byte_offset: f64,
		little_endian: bool,
	) -> Result<i16, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt32)
	#[js_sys(js_name = "getInt32")]
	pub fn get_int32(self: &DataView, byte_offset: f64) -> Result<i32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getInt32)
	#[js_sys(js_name = "getInt32")]
	pub fn get_int32_endian(
		self: &DataView,
		byte_offset: f64,
		little_endian: bool,
	) -> Result<i32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint8)
	#[js_sys(js_name = "getUint8")]
	pub fn get_uint8(self: &DataView, byte_offset: f64) -> Result<u8, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint16)
	#[js_sys(js_name = "getUint16")]
	pub fn get_uint16(self: &DataView, byte_offset: f64) -> Result<u16, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint16)
	#[js_sys(js_name = "getUint16")]
	pub fn get_uint16_endian(
		self: &DataView,
		byte_offset: f64,
		little_endian: bool,
	) -> Result<u16, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint32)
	#[js_sys(js_name = "getUint32")]
	pub fn get_uint32(self: &DataView, byte_offset: f64) -> Result<u32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/getUint32)
	#[js_sys(js_name = "getUint32")]
	pub fn get_uint32_endian(
		self: &DataView,
		byte_offset: f64,
		little_endian: bool,
	) -> Result<u32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setBigInt64)
	#[js_sys(js_name = "setBigInt64")]
	pub fn set_big_int64(self: &DataView, byte_offset: f64, value: i64) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setBigInt64)
	#[js_sys(js_name = "setBigInt64")]
	pub fn set_big_int64_endian(
		self: &DataView,
		byte_offset: f64,
		value: i64,
		little_endian: bool,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setBigUint64)
	#[js_sys(js_name = "setBigUint64")]
	pub fn set_big_uint64(self: &DataView, byte_offset: f64, value: u64) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setBigUint64)
	#[js_sys(js_name = "setBigUint64")]
	pub fn set_big_uint64_endian(
		self: &DataView,
		byte_offset: f64,
		value: u64,
		little_endian: bool,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat16)
	#[js_sys(js_name = "setFloat16")]
	pub fn set_float16_from_f32(
		self: &DataView,
		byte_offset: f64,
		value: f32,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat16)
	#[js_sys(js_name = "setFloat16")]
	pub fn set_float16_endian_from_f32(
		self: &DataView,
		byte_offset: f64,
		value: f32,
		little_endian: bool,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat32)
	#[js_sys(js_name = "setFloat32")]
	pub fn set_float32(self: &DataView, byte_offset: f64, value: f32) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat32)
	#[js_sys(js_name = "setFloat32")]
	pub fn set_float32_endian(
		self: &DataView,
		byte_offset: f64,
		value: f32,
		little_endian: bool,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat64)
	#[js_sys(js_name = "setFloat64")]
	pub fn set_float64(self: &DataView, byte_offset: f64, value: f64) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setFloat64)
	#[js_sys(js_name = "setFloat64")]
	pub fn set_float64_endian(
		self: &DataView,
		byte_offset: f64,
		value: f64,
		little_endian: bool,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt8)
	#[js_sys(js_name = "setInt8")]
	pub fn set_int8(self: &DataView, byte_offset: f64, value: i8) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt16)
	#[js_sys(js_name = "setInt16")]
	pub fn set_int16(self: &DataView, byte_offset: f64, value: i16) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt16)
	#[js_sys(js_name = "setInt16")]
	pub fn set_int16_endian(
		self: &DataView,
		byte_offset: f64,
		value: i16,
		little_endian: bool,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt32)
	#[js_sys(js_name = "setInt32")]
	pub fn set_int32(self: &DataView, byte_offset: f64, value: i32) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setInt32)
	#[js_sys(js_name = "setInt32")]
	pub fn set_int32_endian(
		self: &DataView,
		byte_offset: f64,
		value: i32,
		little_endian: bool,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint8)
	#[js_sys(js_name = "setUint8")]
	pub fn set_uint8(self: &DataView, byte_offset: f64, value: u8) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint16)
	#[js_sys(js_name = "setUint16")]
	pub fn set_uint16(self: &DataView, byte_offset: f64, value: u16) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint16)
	#[js_sys(js_name = "setUint16")]
	pub fn set_uint16_endian(
		self: &DataView,
		byte_offset: f64,
		value: u16,
		little_endian: bool,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint32)
	#[js_sys(js_name = "setUint32")]
	pub fn set_uint32(self: &DataView, byte_offset: f64, value: u32) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView/setUint32)
	#[js_sys(js_name = "setUint32")]
	pub fn set_uint32_endian(
		self: &DataView,
		byte_offset: f64,
		value: u32,
		little_endian: bool,
	) -> Result<(), JsValue>;
}
