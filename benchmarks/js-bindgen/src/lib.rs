use js_sys::{JsValue, js_sys};

js_sys::js_bindgen::embed_js!(
	module = "js_bindgen_benchmark",
	name = "identity",
	"(value) => value",
);

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "identity")]
	fn import_i32_identity_raw(value: i32) -> i32;

	#[js_sys(js_embed = "identity")]
	fn import_u128_identity_raw(value: u128) -> u128;

	#[js_sys(js_embed = "identity")]
	fn import_option_i32_identity_raw(value: Option<i32>) -> Option<i32>;

	#[js_sys(js_embed = "identity")]
	fn import_result_i32_identity_raw(value: i32) -> Result<i32, JsValue>;
}

#[js_sys]
fn i32_identity(value: i32) -> i32 {
	value
}

#[js_sys]
fn u128_identity(value: u128) -> u128 {
	value
}

#[js_sys]
fn option_i32_identity(value: i32) -> Option<i32> {
	Some(value)
}

#[js_sys]
fn result_i32_identity(value: i32) -> Result<i32, JsValue> {
	Ok(value)
}

#[js_sys]
fn import_i32_identity(value: i32) -> i32 {
	import_i32_identity_raw(value)
}

#[js_sys]
fn import_u128_identity(value: u128) -> u128 {
	import_u128_identity_raw(value)
}

#[js_sys]
fn import_option_i32_identity(value: Option<i32>) -> Option<i32> {
	import_option_i32_identity_raw(value)
}

#[js_sys]
fn import_result_i32_identity(value: i32) -> Result<i32, JsValue> {
	import_result_i32_identity_raw(value)
}
