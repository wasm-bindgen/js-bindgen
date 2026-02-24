use js_bindgen_test::test;
use js_sys::{JsString, js_sys};
use web_sys::console;

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "test")]
	fn test(value: &str) -> JsString;
}

js_bindgen::embed_js!(module = "string", name = "test", "(value) => value");

#[test]
fn rust_string() {
	const TEST: &str = "Hello, World!";
	console::log(&JsString::from_str(&(TEST.as_ptr().addr()).to_string()));
	let string = test(TEST);
	assert_eq!(String::from(&string), "Hello, World!");
}
