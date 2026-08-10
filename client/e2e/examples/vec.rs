#[rustfmt::skip]
fn main() {
	// ;; (() => { const first = {}; const last = {}; const input = [first, null, last]; const result = exports["js_value_roundtrip"](input); return result !== input && Array.isArray(result) && result.length === 3 && result[0] === first && result[1] === null && result[2] === last })()
	// ;; (() => { const result = exports["js_value_roundtrip"]([]); return Array.isArray(result) && result.length === 0 })()
	// ;; (() => { try { exports["js_value_roundtrip"](new Uint32Array()); return false } catch (error) { return error instanceof TypeError } })()
	// ;; (() => { const input = new Uint32Array([0, 1, 0xffffffff]); const result = exports["u32_roundtrip"](input); return result !== input && result instanceof Uint32Array && result.length === 3 && result[0] === 0 && result[1] === 1 && result[2] === 0xffffffff })()
	// ;; (() => { const result = exports["u32_roundtrip"](new Uint32Array()); return result instanceof Uint32Array && result.length === 0 })()
	// ;; (() => { try { exports["u32_roundtrip"]([]); return false } catch (error) { return error instanceof TypeError } })()
	// ;; (() => { const result = exports["i8_roundtrip"](new Int8Array([-128, -1, 0, 127])); return result instanceof Int8Array && result.join() === '-128,-1,0,127' })()
	// ;; (() => { const result = exports["u16_roundtrip"](new Uint16Array([0, 1, 0x8000, 0xffff])); return result instanceof Uint16Array && result.join() === '0,1,32768,65535' })()
	// ;; (() => { const result = exports["i64_roundtrip"](new BigInt64Array([-(1n << 63n), -1n, 0n, (1n << 63n) - 1n])); return result instanceof BigInt64Array && result[0] === -(1n << 63n) && result[3] === (1n << 63n) - 1n })()
	// ;; (() => { const result = exports["u64_roundtrip"](new BigUint64Array([0n, 1n, 18446744073709551615n])); return result instanceof BigUint64Array && result[2] === 18446744073709551615n })()
	// ;; (() => { const result = exports["f32_roundtrip"](new Float32Array([Math.fround(1 / 3), -0, Infinity, NaN])); return result instanceof Float32Array && result[0] === Math.fround(1 / 3) && Object.is(result[1], -0) && result[2] === Infinity && Number.isNaN(result[3]) })()
	// ;; (() => { const result = exports["f64_roundtrip"](new Float64Array([-1.25, 0, 1.25])); return result instanceof Float64Array && result.join() === '-1.25,0,1.25' })()
	// ;; (() => { const bits = exports["pointer_width"](); const input = bits === 64 ? new BigUint64Array([0n, 0xffffffffffffffffn]) : new Uint32Array([0, 0xffffffff]); const result = exports["usize_roundtrip"](input); const Constructor = bits === 64 ? BigUint64Array : Uint32Array; return result !== input && result instanceof Constructor && result.length === 2 && result[0] === input[0] && result[1] === input[1] })()
	// ;; (() => { const input = ['first', '', '第三个 🦀']; const result = exports["string_roundtrip"](input); return result !== input && Array.isArray(result) && result.join('|') === 'first||第三个 🦀' })()
	// ;; (() => { try { exports["string_roundtrip"](['valid', 42]); return false } catch (error) { return error instanceof TypeError } })()
	// ;; exports["sum_u32_slice"](new Uint32Array([1, 2, 3, 0xffffffff])) === 5
	// ;; exports["join_string_slice"](['first', '', '第三个 🦀']) === 'first||第三个 🦀'
}

use js_sys::{JsValue, js_sys};

#[js_sys]
fn js_value_roundtrip(value: Vec<JsValue>) -> Vec<JsValue> {
	value
}

#[js_sys]
fn u32_roundtrip(value: Vec<u32>) -> Vec<u32> {
	value
}

#[js_sys]
fn i8_roundtrip(value: Vec<i8>) -> Vec<i8> {
	value
}

#[js_sys]
fn u16_roundtrip(value: Vec<u16>) -> Vec<u16> {
	value
}

#[js_sys]
fn i64_roundtrip(value: Vec<i64>) -> Vec<i64> {
	value
}

#[js_sys]
fn u64_roundtrip(value: Vec<u64>) -> Vec<u64> {
	value
}

#[js_sys]
fn f32_roundtrip(value: Vec<f32>) -> Vec<f32> {
	value
}

#[js_sys]
fn f64_roundtrip(value: Vec<f64>) -> Vec<f64> {
	value
}

#[js_sys]
fn usize_roundtrip(value: Vec<usize>) -> Vec<usize> {
	value
}

#[js_sys]
fn pointer_width() -> u32 {
	usize::BITS
}

#[js_sys]
fn string_roundtrip(value: Vec<String>) -> Vec<String> {
	value
}

#[js_sys]
fn sum_u32_slice(value: &[u32]) -> u32 {
	value.iter().fold(0, |sum, value| sum.wrapping_add(*value))
}

#[js_sys]
fn join_string_slice(value: &[String]) -> String {
	value.join("|")
}
