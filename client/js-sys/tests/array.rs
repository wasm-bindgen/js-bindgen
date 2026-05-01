use core::array;

use js_bindgen_test::test;
use js_sys::{JsArray, JsValue, js_sys};

js_bindgen::embed_js!(module = "array", name = "test", "(value) => value");

#[test]
fn js_value() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn js(value: &[JsValue]) -> JsArray<JsValue>;
	}

	let rust_array = [JsValue::UNDEFINED; 42];
	let js_array = JsArray::from(&rust_array);
	assert_eq!(rust_array.len(), js_array.length().try_into().unwrap());

	let ffi_array = js(&rust_array);
	assert_eq!(rust_array.len(), ffi_array.length().try_into().unwrap());

	let returned_array: [JsValue; 42] = js_array.to_array().unwrap();
	assert!(rust_array == returned_array);

	let returned_array: [JsValue; 42] = ffi_array.to_array().unwrap();
	assert_eq!(rust_array, returned_array);
}

#[test]
fn u32() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn u32(value: &[u32]) -> JsArray<u32>;
	}

	let rust_array: [u32; 42] = array::from_fn(|i| i.try_into().unwrap());
	let js_array = JsArray::from(&rust_array);
	assert_eq!(rust_array.len(), js_array.length().try_into().unwrap());

	let ffi_array = u32(&rust_array);
	assert_eq!(rust_array.len(), ffi_array.length().try_into().unwrap());

	let returned_array: [u32; 42] = js_array.to_array().unwrap();
	assert_eq!(rust_array, returned_array);

	let returned_array: [u32; 42] = ffi_array.to_array().unwrap();
	assert_eq!(rust_array, returned_array);
}
