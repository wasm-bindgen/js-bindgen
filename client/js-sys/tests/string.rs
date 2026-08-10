use js_bindgen_test::test;
use js_sys::{JsString, JsValue, js_sys};

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "test")]
	fn js_string(value: &str) -> JsString;

	#[js_sys(js_embed = "identity")]
	fn identity(value: String) -> String;

	#[js_sys(js_embed = "identity")]
	fn optional_identity(value: Option<String>) -> Option<String>;

	#[js_sys(js_embed = "identity")]
	fn result_identity(value: String) -> Result<String, JsValue>;

	#[js_sys(js_embed = "throw")]
	fn result_error() -> Result<String, JsValue>;

	#[js_sys(js_embed = "wrong_type")]
	fn wrong_type() -> Result<String, JsValue>;

	#[js_sys(js_embed = "lone_surrogate")]
	fn lone_surrogate() -> String;

	#[js_sys(js_embed = "grow_memory")]
	fn grow_memory(value: String) -> String;
}

js_bindgen::embed_js!(module = "string", name = "test", "(value) => value");
js_bindgen::embed_js!(module = "string", name = "identity", "value => value");
js_bindgen::embed_js!(
	module = "string",
	name = "throw",
	"() => {{ throw 'error' }}"
);
js_bindgen::embed_js!(module = "string", name = "wrong_type", "() => 42");
js_bindgen::embed_js!(
	module = "string",
	name = "lone_surrogate",
	"() => '\\ud800'"
);
js_bindgen::embed_js!(
	module = "string",
	name = "grow_memory",
	"value => {{",
	#[cfg(target_arch = "wasm32")]
	"    this.#memory.grow(1)",
	#[cfg(target_arch = "wasm64")]
	"    this.#memory.grow(1n)",
	"    return value",
	"}}",
);

#[test]
fn borrowed_roundtrip() {
	assert_eq!(js_string("Hello, World!"), "Hello, World!");
}

#[test]
fn owned_roundtrip() {
	for value in [
		"",
		"Hello, World!",
		"a\0b",
		"你好，世界！🦀",
		"\u{feff}leading byte-order mark",
	] {
		assert_eq!(identity(value.to_owned()), value);
	}
}

#[test]
fn optional_owned_roundtrip() {
	assert_eq!(optional_identity(None), None);
	assert_eq!(optional_identity(Some(String::new())), Some(String::new()));
	let value = String::from("optional 🦀");
	assert_eq!(optional_identity(Some(value.clone())), Some(value));
}

#[test]
fn result_owned_roundtrip() {
	let value = String::from("result 🦀");
	assert_eq!(result_identity(value.clone()).unwrap(), value);
	assert_eq!(
		result_error().unwrap_err(),
		JsValue::from(JsString::from("error"))
	);
	assert!(wrong_type().is_err());
}

#[test]
fn rust_conversions() {
	let string = JsString::from(String::from("line\n\"quoted\""));
	assert_eq!(String::from(string.clone()), "line\n\"quoted\"");
	assert_eq!(format!("{string}"), "line\n\"quoted\"");
	assert_eq!(format!("{string:?}"), "\"line\\n\\\"quoted\\\"\"");

	assert_eq!(JsString::from('🦀'), "🦀");
	assert_eq!(JsString::default(), "");
	assert_eq!("parsed".parse::<JsString>().unwrap(), "parsed");
	assert_eq!(lone_surrogate(), "\u{fffd}");
}

#[test]
fn survives_memory_growth() {
	let value = "你好，世界！🦀".repeat(32_768);
	assert_eq!(grow_memory(value.clone()), value);
}
