use js_bindgen_test::test;
use js_sys::{JsString, js_sys};
use quickcheck::quickcheck;

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "test")]
	fn test(value: &str) -> JsString;
}

js_bindgen::embed_js!(module = "string", name = "test", "(value) => value");

#[test]
fn rust_string() {
	let string = test("Hello, World!");
	assert_eq!(String::from(&string), "Hello, World!");
}

#[test]
fn test_conv_str() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn conv_str(value: &str) -> JsString;
	}
	#[expect(clippy::needless_pass_by_value, reason = "checked")]
	fn prop(val: String) -> bool {
		conv_str(val.as_str()) == val.as_str()
	}
	quickcheck(prop as fn(String) -> bool);
}
