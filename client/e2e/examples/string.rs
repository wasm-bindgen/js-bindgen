#[rustfmt::skip]
fn main() {
	// ;; exports["rust_string"]()
	// ;; exports["rust_js_string"]() === "Hello from Rust! 🦀"
	// ;; exports["identity"]("Hello from JavaScript! 🦀") === "Hello from JavaScript! 🦀"
	// ;; exports["borrowed_js_string"]("borrowed") === true
	// ;; exports["borrowed_js_value"]("value") === true
	// ;; exports["borrowed_js_array"]([1, 2, 3]) === 3
	// ;; exports["optional_js_string"](false) === undefined
	// ;; exports["optional_js_string"](true) === "optional"
	// ;; exports["roundtrip"]("", "")
	// ;; exports["roundtrip"]("Hello, World!", "Hello, World!")
	// ;; exports["roundtrip"]("你好，世界！🦀", "你好，世界！🦀")
	// ;; exports["roundtrip"]("a\0b", "a\0b")
	// ;; exports["roundtrip"]("\ud800", "\ufffd")
	// ;; (() => { const value = "js-bindgen 🦀 ".repeat(8_192); return exports["roundtrip"](value, value) })()
	// ;; exports["result_js_string"](true) === "ok"
	// ;; (() => { try { exports["result_js_string"](false); return false } catch (error) { return error === "error" } })()
	// ;; exports["import_result_js_string"]("ok") === "ok!"
	// ;; (() => { try { exports["import_result_js_string"]("error"); return false } catch (error) { return error === "string error" } })()
}

use js_sys::{JsArray, JsString, JsValue, js_sys};

js_sys::js_bindgen::embed_js!(
	module = "string",
	name = "result.js_string",
	"(value) => {{",
	"	if (value === 'error') throw 'string error'",
	"	return `${{value}}!`",
	"}}",
);

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "result.js_string")]
	fn import_result_js_string_raw(value: JsString) -> Result<JsString, JsValue>;
}

#[expect(clippy::cmp_owned, reason = "checked")]
#[js_sys]
fn rust_string() -> bool {
	JsString::from("Hello from Rust! 🦀") == "Hello from Rust! 🦀"
}

#[js_sys]
fn rust_js_string() -> JsString {
	JsString::from("Hello from Rust! 🦀")
}

#[js_sys]
fn identity(value: JsString) -> JsString {
	value
}

#[js_sys]
fn borrowed_js_string(value: &JsString) -> bool {
	value.eq(&"borrowed")
}

#[js_sys]
fn borrowed_js_value(value: &JsValue) -> bool {
	let expected = JsString::from("value");
	value == expected.as_ref()
}

#[js_sys]
fn borrowed_js_array(value: &JsArray) -> u32 {
	value.length()
}

#[js_sys]
fn optional_js_string(some: bool) -> Option<JsString> {
	some.then(|| JsString::from("optional"))
}

#[expect(clippy::cmp_owned, reason = "checked")]
#[js_sys]
fn roundtrip(value: JsString, expected: JsString) -> bool {
	let rust_value = String::from(&value);
	let rust_expected = String::from(&expected);
	drop(value);
	drop(expected);
	JsString::from(rust_value.as_str()) == rust_expected
}

#[js_sys]
fn result_js_string(ok: bool) -> Result<JsString, JsString> {
	if ok {
		Ok(JsString::from("ok"))
	} else {
		Err(JsString::from("error"))
	}
}

#[js_sys]
fn import_result_js_string(value: JsString) -> Result<JsString, JsValue> {
	import_result_js_string_raw(value)
}
