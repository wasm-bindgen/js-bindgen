use js_bindgen_test::test;
use js_sys::{JsString, JsValue, js_sys};

js_bindgen::embed_js!(module = "value", name = "nan", "() => NaN");
js_bindgen::embed_js!(
	module = "value",
	name = "externref_length",
	required_embeds = [("js_sys", "externref.table")],
	"() => this.#jsEmbed.js_sys['externref.table'].length",
);
#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "nan")]
	fn nan() -> JsValue;

	#[js_sys(js_embed = "externref_length")]
	fn externref_length() -> u32;
}

#[test]
fn undefined() {
	let value = JsValue::UNDEFINED.clone();
	let string = JsString::new(&value).unwrap();
	let string = String::from(&string);

	assert_eq!(string, "undefined");
}

#[test]
fn null() {
	let value = JsValue::NULL.clone();
	let string = JsString::new(&value).unwrap();
	let string = String::from(&string);

	assert_eq!(string, "null");
}

#[test]
fn nan_strict_equality() {
	let value = nan();
	assert!(!PartialEq::eq(&value, &value));
}

#[test]
fn externref_reuse() {
	let value = JsString::from("Hello, World!");
	let values: Vec<_> = (0..512).map(|_| value.clone()).collect();
	assert!(values.iter().all(|candidate| candidate == &value));
	let grown_length = externref_length();
	drop(values);

	let reused: Vec<_> = (0..512).map(|_| value.clone()).collect();
	assert!(reused.iter().all(|candidate| candidate == &value));
	assert_eq!(externref_length(), grown_length);
}
