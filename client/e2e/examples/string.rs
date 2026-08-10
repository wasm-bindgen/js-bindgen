#[rustfmt::skip]
fn main() {
	// ;; exports["rust_js_string"]() === "Hello from Rust! 🦀"
	// ;; exports["identity"]("Hello from JavaScript! 🦀") === "Hello from JavaScript! 🦀"
	// ;; exports["borrowed_js_string"]("borrowed") === true
	// ;; exports["optional_js_string"](false) === undefined
	// ;; exports["optional_js_string"](true) === "optional"
	// ;; exports["owned_roundtrip"]("") === ""
	// ;; exports["owned_roundtrip"]("a\0b 你好 🦀") === "a\0b 你好 🦀"
	// ;; exports["owned_roundtrip"]("\ud800") === "\ufffd"
	// ;; (() => { const value = "owned 🦀 ".repeat(32_768); return exports["owned_roundtrip"](value) === value })()
	// ;; exports["optional_owned_roundtrip"](undefined) === undefined
	// ;; exports["optional_owned_roundtrip"](null) === undefined
	// ;; exports["optional_owned_roundtrip"]("optional 🦀") === "optional 🦀"
	// ;; exports["result_owned_string"](true) === "ok"
	// ;; (() => { try { exports["result_owned_string"](false); return false } catch (error) { return error === "owned error" } })()
	// ;; exports["result_js_string"](true) === "ok"
	// ;; (() => { try { exports["result_js_string"](false); return false } catch (error) { return error === "error" } })()
	// ;; exports["import_result_js_string"]("ok") === "ok!"
	// ;; (() => { try { exports["import_result_js_string"]("error"); return false } catch (error) { return error === "string error" } })()
	// ;; exports["import_result_string"]("ok") === "ok!"
	// ;; (() => { try { exports["import_result_string"]("error"); return false } catch (error) { return error === "string error" } })()
}

use js_sys::{JsString, JsValue, js_sys};

js_sys::js_bindgen::embed_js!(
	module = "string",
	name = "result.js_string",
	"(value) => {{",
	"    if (value === 'error') throw 'string error'",
	"    return `${{value}}!`",
	"}}",
);

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "result.js_string")]
	fn import_result_js_string_raw(value: JsString) -> Result<JsString, JsValue>;

	#[js_sys(js_embed = "result.js_string")]
	fn import_result_string_raw(value: String) -> Result<String, JsValue>;
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
fn optional_js_string(some: bool) -> Option<JsString> {
	some.then(|| JsString::from("optional"))
}

#[js_sys]
fn owned_roundtrip(value: String) -> String {
	value
}

#[js_sys]
fn optional_owned_roundtrip(value: Option<String>) -> Option<String> {
	value
}

#[js_sys]
fn result_owned_string(ok: bool) -> Result<String, JsString> {
	if ok {
		Ok(String::from("ok"))
	} else {
		Err(JsString::from("owned error"))
	}
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

#[js_sys]
fn import_result_string(value: String) -> Result<String, JsValue> {
	import_result_string_raw(value)
}
