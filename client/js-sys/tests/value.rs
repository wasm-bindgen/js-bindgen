use js_bindgen_test::test;
use js_sys::{JsString, JsValue};

#[test]
fn undefined() {
	let string = JsString::new(&JsValue::UNDEFINED);
	let string = String::from(&string);

	assert_eq!(string, "undefined");
}

#[test]
fn null() {
	let string = JsString::new(&JsValue::NULL);
	let string = String::from(&string);

	assert_eq!(string, "null");
}
