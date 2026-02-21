use crate::util::PtrLength;

#[js_sys]
extern "js-sys" {
	pub type JsArray<T = JsValue>;

	#[js_sys(js_embed = "array.u32.decode")]
	pub(super) fn array_u32_decode(array: *const u32, len: PtrLength) -> JsArray<u32>;
}
