//! ```rust
//! # #[js_bindgen_test::test]
//! # fn doctest() {
//! let log = js_sys::JsString::from("hello world");
//! web_sys::console::log(&log);
//! # }
//! ````

use js_sys::JsValue;

#[js_sys(namespace = "console")]
extern "js-sys" {
	#[js_sys(js_name = "log")]
	pub fn log0();

	pub fn log(data: &JsValue);

	#[js_sys(js_name = "log")]
	pub fn log2(data1: &JsValue, data2: &JsValue);

	pub fn error(data: &JsValue);
}
