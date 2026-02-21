#![no_std]

pub mod console;

pub use js_sys;

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
