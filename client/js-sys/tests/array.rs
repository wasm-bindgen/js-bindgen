use core::array;

use js_bindgen_test::test;
use js_sys::{JsArray, JsString, JsValue, js_sys};

js_bindgen::embed_js!(module = "array", name = "test", "(value) => value");
js_bindgen::embed_js!(
	module = "array",
	name = "throwing",
	"(value, len) => new Proxy(new Array(len).fill(value), {{",
	"	get(target, property) {{",
	"		if (property === '1') throw new Error('boom')",
	"		return target[property]",
	"	}}",
	"}})",
);

#[test]
fn js_value() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn js(value: &[JsValue]) -> JsArray<JsValue>;

		#[js_sys(js_embed = "throwing")]
		fn throwing(value: &JsValue, len: u32) -> JsArray<JsValue>;
	}

	let rust_array = [JsValue::UNDEFINED; 42];
	let js_array = JsArray::from(&rust_array);
	assert_eq!(rust_array.len(), js_array.length().try_into().unwrap());

	let ffi_array = js(&rust_array);
	assert_eq!(rust_array.len(), ffi_array.length().try_into().unwrap());

	let mut wrong_length = [JsValue::UNDEFINED; 41];
	assert!(js_array.to_slice(&mut wrong_length).is_err());

	let previous = JsString::from("previous");
	let previous: JsValue = previous.into();
	let mut destination: [JsValue; 42] = array::from_fn(|_| previous.clone());
	let throwing = throwing(&JsValue::NULL, 42);
	assert!(throwing.to_slice(&mut destination).is_err());
	assert!(destination.iter().all(|value| value == &previous));

	js_array.to_slice(&mut destination).unwrap();
	assert_eq!(rust_array, destination);

	let returned_array: [JsValue; 42] = js_array.to_array().unwrap();
	assert_eq!(rust_array, returned_array);

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
