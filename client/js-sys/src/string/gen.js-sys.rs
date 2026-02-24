use crate::util::PtrLength;

#[js_sys]
extern "js-sys" {
	pub type JsString;

	#[js_sys(js_name = "String")]
	pub(super) fn string_constructor(value: &JsValue) -> JsString;

	#[js_sys(js_embed = "string.decode")]
	pub(super) fn string_decode(array: *const u8, len: PtrLength<u8>) -> JsString;

	#[js_sys(js_embed = "string.utf8_length")]
	pub(super) fn string_utf8_length(string: &JsString) -> f64;

	#[js_sys(js_embed = "string.encode")]
	pub(super) fn string_encode(string: &JsString, array: *mut u8, len: PtrLength<u8>);
}
