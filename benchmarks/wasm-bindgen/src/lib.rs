use core::array;
use core::future::Future;
use core::hint::black_box;
use core::pin::Pin;
use core::task::{Context, Poll};

use js_sys::{Promise, Uint32Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, future_to_promise};

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

	#[wasm_bindgen(js_name = identity)]
	fn import_vec_js_value_raw(value: Vec<JsValue>) -> Vec<JsValue>;

	#[wasm_bindgen(js_name = identity)]
	fn import_vec_u32_raw(value: Vec<u32>) -> Vec<u32>;

	#[wasm_bindgen(js_name = identity)]
	fn import_vec_u8_raw(value: Vec<u8>) -> Vec<u8>;

	#[wasm_bindgen(js_name = identity)]
	fn import_vec_u64_raw(value: Vec<u64>) -> Vec<u64>;

	#[wasm_bindgen(js_name = identity)]
	fn import_vec_f64_raw(value: Vec<f64>) -> Vec<f64>;

	#[wasm_bindgen(js_name = identity)]
	fn import_vec_string_raw(value: Vec<String>) -> Vec<String>;
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
	fn import_string_length_raw(value: String) -> u32;

	#[wasm_bindgen(js_name = length)]
	fn import_u32_slice_raw(value: &[u32]) -> u32;

	#[wasm_bindgen(js_name = length)]
	fn import_u64_slice_raw(value: &[u64]) -> u32;

	#[wasm_bindgen(js_name = length)]
	fn import_js_value_slice_raw(value: &[JsValue]) -> u32;
}

#[wasm_bindgen(inline_js = "export function string() { return 'js-bindgen benchmark'; }")]
extern "C" {
	#[wasm_bindgen(js_name = string)]
	fn import_string_raw() -> String;
}

#[wasm_bindgen(inline_js = "export function string_identity(value) { return value; }")]
extern "C" {
	#[wasm_bindgen(js_name = string_identity)]
	fn import_string_roundtrip_raw(value: String) -> String;
}

#[wasm_bindgen(
	inline_js = "export function invoke_closure(callback, value) { return callback(value); }"
)]
extern "C" {
	#[wasm_bindgen(js_name = invoke_closure)]
	fn invoke_closure_raw(callback: &Closure<dyn FnMut(i32) -> i32>, value: i32) -> i32;

	#[wasm_bindgen(js_name = invoke_closure)]
	fn invoke_closure_u128_raw(callback: &Closure<dyn FnMut(u128) -> u128>, value: u128) -> u128;
}

#[wasm_bindgen(inline_js = "export function pending_promise() {
    const { promise, resolve } = Promise.withResolvers();
    globalThis.queueMicrotask(resolve);
    return promise;
}")]
extern "C" {
	fn pending_promise() -> Promise;
}

std::thread_local! {
	static CALLBACK: Closure<dyn FnMut(i32) -> i32> =
		Closure::new(|value| value);
	static CALLBACK_U128: Closure<dyn FnMut(u128) -> u128> =
		Closure::new(|value| value);
	static UINT32_ARRAY: Uint32Array = Uint32Array::from(UINT32_VALUES.as_slice());
}

const UINT32_VALUES: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
const UINT8_VALUES: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
const UINT64_VALUES: [u64; 8] = [1, 2, 3, 4, 5, 6, 7, u64::MAX];
const FLOAT64_VALUES: [f64; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

#[wasm_bindgen]
pub fn bench_closure_call(value: i32) -> i32 {
	CALLBACK.with(|callback| invoke_closure_raw(callback, value))
}

#[wasm_bindgen]
pub fn bench_closure_call_u128(value: u128) -> u128 {
	CALLBACK_U128.with(|callback| invoke_closure_u128_raw(callback, value))
}

#[wasm_bindgen]
pub fn bench_future_to_promise_ready() -> Promise {
	future_to_promise(async { Ok(JsValue::UNDEFINED) })
}

struct YieldOnce(bool);

impl Future for YieldOnce {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
		if self.0 {
			Poll::Ready(())
		} else {
			self.0 = true;
			context.waker().wake_by_ref();
			Poll::Pending
		}
	}
}

#[wasm_bindgen]
pub fn bench_future_to_promise_pending() -> Promise {
	future_to_promise(async {
		YieldOnce(false).await;
		Ok(JsValue::UNDEFINED)
	})
}

#[wasm_bindgen]
pub fn bench_future_to_promise_err() -> Promise {
	future_to_promise(async { Err(JsValue::UNDEFINED) })
}

#[wasm_bindgen]
pub fn bench_promise_future_roundtrip_ready() -> Promise {
	let promise = Promise::resolve(&JsValue::UNDEFINED);
	future_to_promise(JsFuture::from(promise))
}

#[wasm_bindgen]
pub fn bench_promise_future_roundtrip_pending() -> Promise {
	future_to_promise(JsFuture::from(pending_promise()))
}

#[wasm_bindgen]
pub fn bench_promise_future_roundtrip_err() -> Promise {
	let promise = Promise::reject(&JsValue::UNDEFINED);
	future_to_promise(JsFuture::from(promise))
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
pub fn bench_export_vec_js_value(value: JsValue) -> Vec<JsValue> {
	vec![value]
}

#[wasm_bindgen]
pub fn bench_export_vec_u32() -> Vec<u32> {
	black_box(UINT32_VALUES).to_vec()
}

#[wasm_bindgen]
pub fn bench_export_vec_u8() -> Vec<u8> {
	black_box(UINT8_VALUES).to_vec()
}

#[wasm_bindgen]
pub fn bench_export_vec_u64() -> Vec<u64> {
	black_box(UINT64_VALUES).to_vec()
}

#[wasm_bindgen]
pub fn bench_export_vec_f64() -> Vec<f64> {
	black_box(FLOAT64_VALUES).to_vec()
}

#[wasm_bindgen]
pub fn bench_export_vec_string() -> Vec<String> {
	black_box(["js", "bindgen", "benchmark", "🦀"])
		.into_iter()
		.map(String::from)
		.collect()
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
pub fn bench_import_vec_js_value(value: JsValue) -> usize {
	import_vec_js_value_raw(vec![value]).len()
}

#[wasm_bindgen]
pub fn bench_import_vec_u32() -> usize {
	import_vec_u32_raw(black_box(UINT32_VALUES).to_vec()).len()
}

#[wasm_bindgen]
pub fn bench_import_vec_u8() -> usize {
	import_vec_u8_raw(black_box(UINT8_VALUES).to_vec()).len()
}

#[wasm_bindgen]
pub fn bench_import_vec_u64() -> usize {
	import_vec_u64_raw(black_box(UINT64_VALUES).to_vec()).len()
}

#[wasm_bindgen]
pub fn bench_import_vec_f64() -> usize {
	import_vec_f64_raw(black_box(FLOAT64_VALUES).to_vec()).len()
}

#[wasm_bindgen]
pub fn bench_import_vec_string() -> usize {
	let values = black_box(["js", "bindgen", "benchmark", "🦀"])
		.into_iter()
		.map(String::from)
		.collect();
	import_vec_string_raw(values).len()
}

#[wasm_bindgen]
pub fn bench_import_str() -> u32 {
	import_str_raw(black_box("js-bindgen benchmark"))
}

#[wasm_bindgen]
pub fn bench_import_string_to_js() -> u32 {
	import_string_length_raw(String::from(black_box("js-bindgen benchmark")))
}

#[wasm_bindgen]
pub fn bench_import_string_from_js() -> usize {
	import_string_raw().len()
}

#[wasm_bindgen]
pub fn bench_import_string_roundtrip() -> usize {
	import_string_roundtrip_raw(String::from(black_box("js-bindgen benchmark"))).len()
}

#[wasm_bindgen]
pub fn bench_import_u32_slice() -> u32 {
	import_u32_slice_raw(black_box(&UINT32_VALUES))
}

#[wasm_bindgen]
pub fn bench_import_u64_slice() -> u32 {
	import_u64_slice_raw(black_box(&UINT64_VALUES))
}

#[wasm_bindgen]
pub fn bench_import_js_value_slice(value: JsValue) -> u32 {
	import_js_value_slice_raw(core::slice::from_ref(&value))
}

#[wasm_bindgen]
pub fn bench_typed_array_copy_to_u32() -> u32 {
	let mut output = [0; UINT32_VALUES.len()];
	UINT32_ARRAY.with(|array| array.copy_to(&mut output));
	black_box(output)[output.len() - 1]
}

#[wasm_bindgen]
pub fn bench_typed_array_copy_from_u32() -> usize {
	UINT32_ARRAY.with(|array| {
		array.copy_from(black_box(&UINT32_VALUES));
		array.length() as usize
	})
}

#[wasm_bindgen]
pub fn bench_typed_array_from_u32() -> usize {
	Uint32Array::from(black_box(UINT32_VALUES.as_slice())).length() as usize
}
