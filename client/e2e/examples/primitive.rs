#[rustfmt::skip]
fn main() {
	// ;; exports["not_bool"](false) === true
	// ;; exports["not_bool"](true) === false
	// ;; exports["add_i32"](-2_147_483_648, 1) === -2_147_483_647
	// ;; exports["add_u32"](0xffff_fffe, 1) === 0xffff_ffff
	// ;; exports["add_f32"](Math.fround(1 / 3), 0) === Math.fround(1 / 3)
	// ;; exports["add_f64"](Number.MAX_SAFE_INTEGER, 1) === 9_007_199_254_740_992
	// ;; exports["add_i64"](-(1n << 63n), 1n) === -(1n << 63n) + 1n
	// ;; exports["add_u64"](0xffff_ffff_ffff_fffen, 1n) === 0xffff_ffff_ffff_ffffn
	// ;; (() => { const value = exports["isize_min"](); const one = typeof value === "bigint" ? 1n : 1; return exports["add_isize"](value, one) === value + one })()
	// ;; exports["add_u128"](1n << 96n, 3n) === (1n << 96n) + 3n
	// ;; exports["add_i128"](-(1n << 96n), -3n) === -(1n << 96n) - 3n
	// ;; exports["add_i32_ref"](-2_147_483_648, 1) === -2_147_483_647
	// ;; exports["add_u128_ref"](1n << 96n, 3n) === (1n << 96n) + 3n
	// ;; exports["not_bool_ref"](false) === true
	// ;; (() => { const value = exports["usize_max"](); return value === (typeof value === "bigint" ? 0xffff_ffff_ffff_ffffn : 0xffff_ffff) })()
	// ;; exports["option_bool"](undefined) === undefined
	// ;; exports["option_bool"](false) === true
	// ;; exports["option_i16"](undefined) === undefined
	// ;; exports["option_i16"](-32_768) === -32_767
	// ;; exports["option_u32"](undefined) === undefined
	// ;; exports["option_u32"](0xffff_fffe) === 0xffff_ffff
	// ;; exports["option_f32"](undefined) === undefined
	// ;; Number.isNaN(exports["option_f32"](NaN))
	// ;; exports["option_f64"](undefined) === undefined
	// ;; exports["option_f64"](Number.MAX_VALUE) === Number.MAX_VALUE
	// ;; exports["option_i64"](undefined) === undefined
	// ;; exports["option_i64"](-(1n << 63n)) === -(1n << 63n) + 1n
	// ;; exports["option_u64"](undefined) === undefined
	// ;; exports["option_u64"](0xffff_ffff_ffff_fffen) === 0xffff_ffff_ffff_ffffn
	// ;; (() => { const value = exports["isize_min"](); const one = typeof value === "bigint" ? 1n : 1; return exports["option_isize"](value) === value + one })()
	// ;; exports["option_isize"](undefined) === undefined
	// ;; (() => { const value = exports["usize_max"](); return exports["option_usize"](value) === value })()
	// ;; exports["option_usize"](undefined) === undefined
	// ;; exports["option_u128"](undefined) === undefined
	// ;; exports["option_u128"]((1n << 128n) - 2n) === (1n << 128n) - 1n
	// ;; exports["option_i128"](undefined) === undefined
	// ;; exports["option_i128"](-(1n << 127n)) === -(1n << 127n) + 1n
	// ;; exports["checked_add_u128"](1n << 96n, 3n) === (1n << 96n) + 3n
	// ;; (() => { try { exports["checked_add_u128"]((1n << 128n) - 1n, 1n); return false } catch (error) { return error === "overflow" } })()
	// ;; exports["import_result_i64"](41n) === 42n
	// ;; (() => { try { exports["import_result_i64"](-1n); return false } catch (error) { return error === "i64 error" } })()
	// ;; (() => { try { exports["import_result_i64"](-2n); return false } catch (error) { return error === undefined } })()
	// ;; (() => { try { exports["import_result_i64"](-3n); return false } catch (error) { return error === null } })()
	// ;; exports["import_result_u128"](1n << 96n) === (1n << 96n) + 1n
	// ;; (() => { try { exports["import_result_u128"]((1n << 128n) - 1n); return false } catch (error) { return error === "u128 error" } })()
}

use js_sys::{JsString, JsValue, js_sys};

type JsResult<T> = Result<T, JsString>;

js_sys::js_bindgen::embed_js!(
	module = "primitive",
	name = "result.i64",
	"(value) => {{",
	"	if (value === -2n) throw undefined",
	"	if (value === -3n) throw null",
	"	if (value < 0n) throw 'i64 error'",
	"	return value + 1n",
	"}}",
);

js_sys::js_bindgen::embed_js!(
	module = "primitive",
	name = "result.u128",
	"(value) => {{",
	"	if (value === (1n << 128n) - 1n) throw 'u128 error'",
	"	return value + 1n",
	"}}",
);

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "result.i64")]
	fn import_result_i64_raw(value: i64) -> Result<i64, JsValue>;

	#[js_sys(js_embed = "result.u128")]
	fn import_result_u128_raw(value: u128) -> Result<u128, JsValue>;
}

#[js_sys]
fn not_bool(value: bool) -> bool {
	!value
}

#[js_sys]
fn add_i32(value: i32, delta: i32) -> i32 {
	value + delta
}

#[js_sys]
fn add_u32(value: u32, delta: u32) -> u32 {
	value + delta
}

#[js_sys]
fn add_f32(value: f32, delta: f32) -> f32 {
	value + delta
}

#[js_sys]
fn add_f64(value: f64, delta: f64) -> f64 {
	value + delta
}

#[js_sys]
fn add_i64(value: i64, delta: i64) -> i64 {
	value + delta
}

#[js_sys]
fn add_u64(value: u64, delta: u64) -> u64 {
	value + delta
}

#[js_sys]
fn add_isize(value: isize, delta: isize) -> isize {
	value + delta
}

#[js_sys]
fn isize_min() -> isize {
	isize::MIN
}

#[js_sys]
fn add_u128(value: u128, delta: u128) -> u128 {
	value + delta
}

#[js_sys]
fn add_i128(value: i128, delta: i128) -> i128 {
	value + delta
}

#[expect(
	clippy::trivially_copy_pass_by_ref,
	reason = "tests reference ABI conversion"
)]
#[js_sys]
fn add_i32_ref(value: &i32, delta: &i32) -> i32 {
	*value + *delta
}

#[js_sys]
fn add_u128_ref(value: &u128, delta: &u128) -> u128 {
	*value + *delta
}

#[expect(
	clippy::trivially_copy_pass_by_ref,
	reason = "tests reference ABI conversion"
)]
#[js_sys]
fn not_bool_ref(value: &bool) -> bool {
	!*value
}

#[js_sys]
fn usize_max() -> usize {
	usize::MAX
}

#[js_sys]
fn option_bool(value: Option<bool>) -> Option<bool> {
	value.map(|value| !value)
}

#[js_sys]
fn option_i16(value: Option<i16>) -> Option<i16> {
	value.map(|value| value + 1)
}

#[js_sys]
fn option_u32(value: Option<u32>) -> Option<u32> {
	value.map(|value| value + 1)
}

#[js_sys]
fn option_f32(value: Option<f32>) -> Option<f32> {
	value
}

#[js_sys]
fn option_f64(value: Option<f64>) -> Option<f64> {
	value
}

#[js_sys]
fn option_i64(value: Option<i64>) -> Option<i64> {
	value.map(|value| value + 1)
}

#[js_sys]
fn option_u64(value: Option<u64>) -> Option<u64> {
	value.map(|value| value + 1)
}

#[js_sys]
fn option_isize(value: Option<isize>) -> Option<isize> {
	value.map(|value| value + 1)
}

#[js_sys]
fn option_usize(value: Option<usize>) -> Option<usize> {
	value
}

#[js_sys]
fn option_u128(value: Option<u128>) -> Option<u128> {
	value.map(|value| value + 1)
}

#[js_sys]
fn option_i128(value: Option<i128>) -> Option<i128> {
	value.map(|value| value + 1)
}

#[js_sys]
fn checked_add_u128(value: u128, delta: u128) -> JsResult<u128> {
	value
		.checked_add(delta)
		.ok_or_else(|| JsString::from("overflow"))
}

#[js_sys]
fn import_result_i64(value: i64) -> Result<i64, JsValue> {
	import_result_i64_raw(value)
}

#[js_sys]
fn import_result_u128(value: u128) -> Result<u128, JsValue> {
	import_result_u128_raw(value)
}
