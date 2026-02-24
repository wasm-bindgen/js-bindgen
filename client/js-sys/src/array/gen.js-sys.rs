use crate::util::PtrLength;

#[js_sys]
extern "js-sys" {
	pub type JsArray<T = JsValue>;

	#[js_sys(property)]
	pub fn length<T>(self: &JsArray<T>) -> u32;

	#[js_sys(js_embed = "array.js_value.encode")]
	pub(super) fn array_js_value_encode(array: *const JsValue, len: PtrLength<JsValue>) -> JsArray<JsValue>;

	#[js_sys(js_embed = "array.js_value.decode")]
	pub(super) fn array_js_value_decode(
		array: &JsArray,
		array_ptr: *mut JsValue,
		array_len: PtrLength<JsValue>,
		externref_ptr: *const i32,
		externref_len: i32,
	) -> bool;

	#[js_sys(js_embed = "array.u32.encode")]
	pub(super) fn array_u32_encode(array: *const u32, len: PtrLength<u32>) -> JsArray<u32>;

	#[js_sys(js_embed = "array.u32.decode")]
	pub(super) fn array_u32_decode(
		array: &JsArray<u32>,
		ptr: *mut u32,
		len: PtrLength<u32>,
	) -> bool;
}
