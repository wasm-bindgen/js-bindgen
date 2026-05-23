use js_sys::JsValue;
use js_sys::hazard::JsCast;

#[js_sys(namespace = "console")]
extern "js-sys" {
	#[js_sys(js_name = "log")]
	pub fn log0();

	pub fn log<T: JsCast>(#[js_sys(type = &[JsValue])] data: &[T]);

	#[js_sys(js_name = "log")]
	pub fn log2(data1: &JsValue, data2: &JsValue);

	pub fn error(data: &JsValue);
}
