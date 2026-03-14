use js_bindgen_test::test;
use js_sys::{JsString, js_sys};

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
