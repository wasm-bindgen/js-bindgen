use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export function identity(value) { return value; }")]
extern "C" {
	#[wasm_bindgen(js_name = identity)]
	fn import_i32_identity_raw(value: i32) -> i32;

	#[wasm_bindgen(js_name = identity)]
	fn import_u128_identity_raw(value: u128) -> u128;

	#[wasm_bindgen(js_name = identity)]
	fn import_option_i32_identity_raw(value: Option<i32>) -> Option<i32>;

	#[wasm_bindgen(catch, js_name = identity)]
	fn import_result_i32_identity_raw(value: i32) -> Result<i32, JsValue>;
}

#[wasm_bindgen]
pub fn i32_identity(value: i32) -> i32 {
	value
}

#[wasm_bindgen]
pub fn u128_identity(value: u128) -> u128 {
	value
}

#[wasm_bindgen]
pub fn option_i32_identity(value: i32) -> Option<i32> {
	Some(value)
}

#[wasm_bindgen]
pub fn result_i32_identity(value: i32) -> Result<i32, JsValue> {
	Ok(value)
}

#[wasm_bindgen]
pub fn import_i32_identity(value: i32) -> i32 {
	import_i32_identity_raw(value)
}

#[wasm_bindgen]
pub fn import_u128_identity(value: u128) -> u128 {
	import_u128_identity_raw(value)
}

#[wasm_bindgen]
pub fn import_option_i32_identity(value: Option<i32>) -> Option<i32> {
	import_option_i32_identity_raw(value)
}

#[wasm_bindgen]
pub fn import_result_i32_identity(value: i32) -> Result<i32, JsValue> {
	import_result_i32_identity_raw(value)
}
