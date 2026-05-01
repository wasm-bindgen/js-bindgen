use crate::util::{PtrConst, PtrLength, PtrMut};

#[js_sys]
extern "js-sys" {
	pub type JsArray<T = JsValue>;

	#[js_sys(property)]
	pub fn length<T>(self: &JsArray<T>) -> u32;

	#[js_sys(js_embed = "array.js_value.decode")]
	pub(super) unsafe fn array_js_value_decode(
		array: PtrConst<JsValue>,
		len: PtrLength<JsValue>,
	) -> JsArray<JsValue>;

	#[js_sys(js_embed = "array.js_value.encode")]
	pub(super) unsafe fn array_js_value_encode(
		array: &JsArray,
		array_ptr: PtrMut<JsValue>,
		array_len: PtrLength<JsValue>,
		externref_ptr: PtrConst<i32>,
		externref_len: i32,
	) -> bool;

	#[js_sys(js_embed = "view.getUint32")]
	pub(super) unsafe fn array_u32_decode(
		array: PtrConst<u32>,
		len: PtrLength<u32>,
	) -> JsArray<u32>;

	#[js_sys(js_embed = "array.u32.encode")]
	pub(super) unsafe fn array_u32_encode(
		array: &JsArray<u32>,
		ptr: PtrMut<u32>,
		len: PtrLength<u32>,
	) -> bool;
}
