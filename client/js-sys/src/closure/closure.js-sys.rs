use crate::JsValue;

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "closure.unref")]
	pub(super) fn closure_unref(callback: &JsValue);
}
