use super::JsValue;
use crate::util::PtrLength;

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "js_value.partial_eq")]
	pub(super) fn js_value_partial_eq(value1: &JsValue, value2: &JsValue) -> bool;
}
