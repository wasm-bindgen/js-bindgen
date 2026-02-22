use core::array;

use js_bindgen_test::test;
use js_sys::{JsArray, JsValue};

#[test]
fn js_value() {
	let rust_array = [JsValue::UNDEFINED; 42];
	let js_array = JsArray::from(&rust_array);
	assert_eq!(rust_array.len(), js_array.length().try_into().unwrap());

	let returned_array: [JsValue; 42] = js_array.as_array().unwrap();
	assert!(rust_array == returned_array);
}

#[test]
fn u32() {
	let rust_array: [u32; 42] = array::from_fn(|i| i.try_into().unwrap());
	let js_array = JsArray::from(&rust_array);
	assert_eq!(rust_array.len(), js_array.length().try_into().unwrap());

	let returned_array: [u32; 42] = js_array.as_array().unwrap();
	assert_eq!(rust_array, returned_array);
}
