#![no_std]

pub use js_sys;
use js_sys::JsValue;

pub mod console {
	use super::*;

	#[js_bindgen::js_bindgen(namespace = "console")]
	extern "C" {
		pub fn log(data: &JsValue);

		#[js_bindgen(name = "log")]
		pub fn log2(data1: &JsValue, data2: &JsValue);
	}
}
