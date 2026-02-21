use crate::util::PtrLength;

#[js_sys]
extern "js-sys" {
	pub type JsString;

	#[js_sys(js_embed = "string.decode")]
	pub(super) fn string_decode(array: *const u8, len: PtrLength) -> JsString;
}
