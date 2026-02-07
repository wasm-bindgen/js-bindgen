#![no_std]

pub use js_sys;
use js_sys::JsValue;

/// ```rust
/// # #[js_bindgen_test::test]
/// # fn doctest() {
/// let log = js_sys::JsString::from_str("hello world");
/// web_sys::console::log(&log);
/// # }
/// ````
pub mod console {
	use super::*;

	#[js_sys::js_sys(namespace = "console")]
	extern "C" {
		#[js_sys(js_name = "log")]
		pub fn log0();

		pub fn log(data: &JsValue);

		#[js_sys(js_name = "log")]
		pub fn log2(data1: &JsValue, data2: &JsValue);

		pub fn error(data: &JsValue);
	}
}

#[cfg(test)]
mod tests {
	use js_bindgen_test::test;
	use js_sys::{JsArray, JsString};

	use super::console;

	#[test]
	fn test_console_log() {
		let value = JsString::from_str("hello world");
		console::log(&value);
	}

	#[test]
	fn test_array() {
		let value = JsArray::from([42, 43].as_slice());
		console::log(&value);
	}

	#[test]
	#[ignore = "hah, it works"]
	fn test_ignore() {
		panic!("kaboom");
	}

	#[test]
	#[should_panic(expected = "kaboom")]
	fn test_should_panic() {
		panic!("kaboom");
	}
}
