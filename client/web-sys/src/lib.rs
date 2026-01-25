#![no_std]

pub use js_sys;
use js_sys::JsValue;

pub mod console {
	use super::*;

	#[js_sys::js_sys(namespace = "console")]
	extern "C" {
		#[js_sys(js_name = "log")]
		pub fn log0();

		pub fn log(data: &JsValue);

		#[js_sys(js_name = "log")]
		pub fn log2(data1: &JsValue, data2: &JsValue);
	}
}
