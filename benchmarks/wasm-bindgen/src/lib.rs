use core::{array, hint::black_box};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export function identity(value) { return value; }")]
extern "C" {
	#[wasm_bindgen(js_name = identity)]
	fn import_bool_raw(value: bool) -> bool;

	#[wasm_bindgen(js_name = identity)]
	fn import_i32_raw(value: i32) -> i32;

	#[wasm_bindgen(js_name = identity)]
	fn import_u32_raw(value: u32) -> u32;

	#[wasm_bindgen(js_name = identity)]
	fn import_u64_raw(value: u64) -> u64;

	#[wasm_bindgen(js_name = identity)]
	fn import_f64_raw(value: f64) -> f64;

	#[wasm_bindgen(js_name = identity)]
	fn import_usize_raw(value: usize) -> usize;

	#[wasm_bindgen(js_name = identity)]
	fn import_u128_raw(value: u128) -> u128;

	#[wasm_bindgen(js_name = identity)]
	fn import_option_i16_raw(value: Option<i16>) -> Option<i16>;

	#[wasm_bindgen(js_name = identity)]
	fn import_option_i32_raw(value: Option<i32>) -> Option<i32>;

	#[wasm_bindgen(js_name = identity)]
	fn import_option_i64_raw(value: Option<i64>) -> Option<i64>;

	#[wasm_bindgen(js_name = identity)]
	fn import_option_f64_raw(value: Option<f64>) -> Option<f64>;

	#[wasm_bindgen(js_name = identity)]
	fn import_option_u128_raw(value: Option<u128>) -> Option<u128>;

	#[wasm_bindgen(js_name = identity)]
	fn import_option_js_value_raw(value: Option<JsValue>) -> Option<JsValue>;

	#[wasm_bindgen(catch, js_name = identity)]
	fn import_result_unit_raw() -> Result<(), JsValue>;

	#[wasm_bindgen(catch, js_name = identity)]
	fn import_result_i32_raw(value: i32) -> Result<i32, JsValue>;

	#[wasm_bindgen(catch, js_name = identity)]
	fn import_result_u128_raw(value: u128) -> Result<u128, JsValue>;

	#[wasm_bindgen(catch, js_name = identity)]
	fn import_result_js_value_raw(value: JsValue) -> Result<JsValue, JsValue>;

	#[wasm_bindgen(js_name = identity)]
	fn import_js_value_raw(value: JsValue) -> JsValue;
}

#[wasm_bindgen(inline_js = "export function throw_value(value) { throw value; }")]
extern "C" {
	#[wasm_bindgen(catch, js_name = throw_value)]
	fn import_result_i32_err_raw(value: i32) -> Result<i32, JsValue>;

	#[wasm_bindgen(catch, js_name = throw_value)]
	fn import_result_u128_err_raw(value: u128) -> Result<u128, JsValue>;

	#[wasm_bindgen(catch, js_name = throw_value)]
	fn import_result_js_value_err_raw(value: JsValue) -> Result<JsValue, JsValue>;

	#[wasm_bindgen(catch, js_name = throw_value)]
	fn import_result_unit_err_raw() -> Result<(), JsValue>;
}

#[wasm_bindgen(inline_js = "export function length(value) { return value.length; }")]
extern "C" {
	#[wasm_bindgen(js_name = length)]
	fn import_str_raw(value: &str) -> u32;

	#[wasm_bindgen(js_name = length)]
	fn import_u32_slice_raw(value: &[u32]) -> u32;

	#[wasm_bindgen(js_name = length)]
	fn import_js_value_slice_raw(value: &[JsValue]) -> u32;
}

#[wasm_bindgen]
pub fn bench_export_bool() -> bool {
	true
}

#[wasm_bindgen]
pub fn bench_export_i32(value: i32) -> i32 {
	value
}

#[wasm_bindgen]
pub fn bench_export_u32(value: u32) -> u32 {
	value
}

#[wasm_bindgen]
pub fn bench_export_u64(value: u64) -> u64 {
	value
}

#[wasm_bindgen]
pub fn bench_export_f64(value: f64) -> f64 {
	value
}

#[wasm_bindgen]
pub fn bench_export_usize(value: usize) -> usize {
	value
}

#[wasm_bindgen]
pub fn bench_export_u128(value: u128) -> u128 {
	value
}

#[wasm_bindgen]
pub fn bench_export_option_i16_some(value: i16) -> Option<i16> {
	Some(value)
}

#[wasm_bindgen]
pub fn bench_export_option_i16_none() -> Option<i16> {
	None
}

#[wasm_bindgen]
pub fn bench_export_option_i32_some(value: i32) -> Option<i32> {
	Some(value)
}

#[wasm_bindgen]
pub fn bench_export_option_i32_none() -> Option<i32> {
	None
}

#[wasm_bindgen]
pub fn bench_export_option_i64_some(value: i64) -> Option<i64> {
	Some(value)
}

#[wasm_bindgen]
pub fn bench_export_option_i64_none() -> Option<i64> {
	None
}

#[wasm_bindgen]
pub fn bench_export_option_f64_some(value: f64) -> Option<f64> {
	Some(value)
}

#[wasm_bindgen]
pub fn bench_export_option_f64_none() -> Option<f64> {
	None
}

#[wasm_bindgen]
pub fn bench_export_option_u128_some(value: u128) -> Option<u128> {
	Some(value)
}

#[wasm_bindgen]
pub fn bench_export_option_u128_none() -> Option<u128> {
	None
}

#[wasm_bindgen]
pub fn bench_export_option_js_value_some(value: JsValue) -> Option<JsValue> {
	Some(value)
}

#[wasm_bindgen]
pub fn bench_export_option_js_value_none() -> Option<JsValue> {
	None
}

#[wasm_bindgen]
pub fn bench_export_result_unit_ok() -> Result<(), JsValue> {
	Ok(())
}

#[wasm_bindgen]
pub fn bench_export_result_unit_err() -> Result<(), JsValue> {
	Err(JsValue::UNDEFINED)
}

#[wasm_bindgen]
pub fn bench_export_result_i32_ok(value: i32) -> Result<i32, JsValue> {
	Ok(value)
}

#[wasm_bindgen]
pub fn bench_export_result_i32_err(_value: i32) -> Result<i32, JsValue> {
	Err(JsValue::UNDEFINED)
}

#[wasm_bindgen]
pub fn bench_export_result_u128_ok(value: u128) -> Result<u128, JsValue> {
	Ok(value)
}

#[wasm_bindgen]
pub fn bench_export_result_u128_err(_value: u128) -> Result<u128, JsValue> {
	Err(JsValue::UNDEFINED)
}

#[wasm_bindgen]
pub fn bench_export_result_js_value_ok(value: JsValue) -> Result<JsValue, JsValue> {
	Ok(value)
}

#[wasm_bindgen]
pub fn bench_export_result_js_value_err(value: JsValue) -> Result<JsValue, JsValue> {
	Err(value)
}

#[wasm_bindgen]
pub fn bench_export_js_value(value: JsValue) -> JsValue {
	value
}

#[wasm_bindgen]
pub fn bench_export_js_value_ref(value: &JsValue) -> i32 {
	black_box(value);
	1
}

#[wasm_bindgen]
pub fn bench_export_js_value_alloc(value: JsValue) -> i32 {
	let values: [JsValue; 512] = array::from_fn(|_| value.clone());
	black_box(&values);
	512
}

#[wasm_bindgen]
pub fn bench_import_i32(value: i32) -> i32 {
	import_i32_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_bool(value: i32) -> bool {
	import_bool_raw(value != 0)
}

#[wasm_bindgen]
pub fn bench_import_u32(value: u32) -> u32 {
	import_u32_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_u64(value: u64) -> u64 {
	import_u64_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_f64(value: f64) -> f64 {
	import_f64_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_usize(value: usize) -> usize {
	import_usize_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_u128(value: u128) -> u128 {
	import_u128_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_option_i16_some(value: Option<i16>) -> Option<i16> {
	import_option_i16_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_option_i16_none() -> i32 {
	i32::from(import_option_i16_raw(None).is_none())
}

#[wasm_bindgen]
pub fn bench_import_option_i32_some(value: Option<i32>) -> Option<i32> {
	import_option_i32_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_option_i32_none() -> i32 {
	i32::from(import_option_i32_raw(None).is_none())
}

#[wasm_bindgen]
pub fn bench_import_option_i64_some(value: Option<i64>) -> Option<i64> {
	import_option_i64_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_option_i64_none() -> i32 {
	i32::from(import_option_i64_raw(None).is_none())
}

#[wasm_bindgen]
pub fn bench_import_option_f64_some(value: Option<f64>) -> Option<f64> {
	import_option_f64_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_option_f64_none() -> i32 {
	i32::from(import_option_f64_raw(None).is_none())
}

#[wasm_bindgen]
pub fn bench_import_option_u128_some(value: Option<u128>) -> Option<u128> {
	import_option_u128_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_option_u128_none() -> i32 {
	i32::from(import_option_u128_raw(None).is_none())
}

#[wasm_bindgen]
pub fn bench_import_option_js_value_some(value: Option<JsValue>) -> Option<JsValue> {
	import_option_js_value_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_option_js_value_none() -> i32 {
	i32::from(import_option_js_value_raw(None).is_none())
}

#[wasm_bindgen]
pub fn bench_import_result_unit_ok() -> Result<(), JsValue> {
	import_result_unit_raw()
}

#[wasm_bindgen]
pub fn bench_import_result_unit_err() -> i32 {
	i32::from(import_result_unit_err_raw().is_err())
}

#[wasm_bindgen]
pub fn bench_import_result_i32_ok(value: i32) -> Result<i32, JsValue> {
	import_result_i32_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_result_i32_err(value: i32) -> i32 {
	i32::from(import_result_i32_err_raw(value).is_err())
}

#[wasm_bindgen]
pub fn bench_import_result_u128_ok(value: u128) -> Result<u128, JsValue> {
	import_result_u128_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_result_u128_err(value: u128) -> i32 {
	i32::from(import_result_u128_err_raw(value).is_err())
}

#[wasm_bindgen]
pub fn bench_import_result_js_value_ok(value: JsValue) -> Result<JsValue, JsValue> {
	import_result_js_value_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_result_js_value_err(value: JsValue) -> i32 {
	i32::from(import_result_js_value_err_raw(value).is_err())
}

#[wasm_bindgen]
pub fn bench_import_js_value(value: JsValue) -> JsValue {
	import_js_value_raw(value)
}

#[wasm_bindgen]
pub fn bench_import_str() -> u32 {
	import_str_raw(black_box("js-bindgen benchmark"))
}

#[wasm_bindgen]
pub fn bench_import_u32_slice() -> u32 {
	import_u32_slice_raw(black_box(&[1, 2, 3, 4, 5, 6, 7, 8]))
}

#[wasm_bindgen]
pub fn bench_import_js_value_slice(value: JsValue) -> u32 {
	import_js_value_slice_raw(core::slice::from_ref(&value))
}
