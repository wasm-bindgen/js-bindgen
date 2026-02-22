use core::array;

use js_bindgen_test::test;
use js_sys::JsArray;

#[test]
fn len() {
	let rust_array: [u32; 42] = array::from_fn(|i| i.try_into().unwrap());
	let js_array = JsArray::from(&rust_array);

	assert_eq!(rust_array.len(), js_array.length().try_into().unwrap());
}
