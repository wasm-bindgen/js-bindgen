use js_bindgen_test::test;
use js_sys::{JsString, JsValue, js_sys};

js_bindgen::embed_js!(module = "value", name = "nan", "() => NaN");

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "nan")]
	fn nan() -> JsValue;
}

#[test]
fn undefined() {
	let value = JsValue::UNDEFINED.clone();
	let string = JsString::new(&value);
	let string = String::from(&string);

	assert_eq!(string, "undefined");
}

#[test]
fn null() {
	let value = JsValue::NULL.clone();
	let string = JsString::new(&value);
	let string = String::from(&string);

	assert_eq!(string, "null");
}

#[test]
fn clone() {
	let value = JsString::from("Hello, World!");
	let value = value.clone();
	assert_eq!(value, "Hello, World!");
}

#[test]
fn strict_equality_is_not_reflexive() {
	let value = nan();
	assert!(!PartialEq::eq(&value, &value));
}

#[test]
fn many_live_values() {
	let value = JsString::from("Hello, World!");
	let values: Vec<_> = (0..512).map(|_| value.clone()).collect();
	assert_eq!(values.len(), 512);
}
