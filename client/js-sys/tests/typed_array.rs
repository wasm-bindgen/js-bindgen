#![allow(
	clippy::float_cmp,
	reason = "typed-array copies must preserve exact values"
)]

use js_bindgen_test::test;
use js_sys::hazard::JsCast;
use js_sys::{
	ArrayBuffer, BigInt64Array, BigUint64Array, Float16Array, Float64Array, Int8Array, TypedArray,
	TypedArrayCopyError, Uint8Array, Uint8ClampedArray, Uint32Array, js_sys,
};

js_bindgen::embed_js!(
	module = "typed_array",
	name = "has_float16_array",
	"() => typeof Float16Array === 'function'",
);
js_bindgen::embed_js!(
	module = "typed_array",
	name = "shadow_length",
	"(array, length) => Object.defineProperty(array, 'length', {{ value: length }})",
);
js_bindgen::embed_js!(
	module = "typed_array",
	name = "has_resizable_array_buffer",
	"() => typeof ArrayBuffer.prototype.resize === 'function'",
);
js_bindgen::embed_js!(
	module = "typed_array",
	name = "resizable",
	"() => {{",
	"    const buffer = new ArrayBuffer(16, {{ maxByteLength: 16 }})",
	"    const array = new Uint32Array(buffer)",
	"    array.set([1, 2, 3, 4])",
	"    return array",
	"}}",
);
js_bindgen::embed_js!(
	module = "typed_array",
	name = "resize",
	"(array, byteLength) => array.buffer.resize(byteLength)",
);

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "has_float16_array")]
	fn has_float16_array() -> bool;

	#[js_sys(js_embed = "shadow_length")]
	fn shadow_length(array: &Uint32Array, length: u32);

	#[js_sys(js_embed = "has_resizable_array_buffer")]
	fn has_resizable_array_buffer() -> bool;

	#[js_sys(js_embed = "resizable")]
	fn resizable() -> Uint32Array;

	#[js_sys(js_embed = "resize")]
	fn resize(array: &Uint32Array, byte_length: u32);
}

macro_rules! copy_tests {
	($name:ident, $array:ty, $values:expr, $replacement:expr) => {
		#[test]
		fn $name() {
			let values = $values;
			let array = <$array>::from(&values);
			assert_eq!(
				array.length(),
				f64::from(u32::try_from(values.len()).unwrap())
			);
			assert_eq!(array.to_vec().unwrap(), values);

			let replacement = $replacement;
			array.copy_from(&replacement).unwrap();
			let mut output = replacement;
			output.fill(replacement[0]);
			array.copy_to(&mut output).unwrap();
			assert_eq!(output, replacement);

			let mut wrong = [replacement[0]; 1];
			assert!(matches!(
				array.copy_to(&mut wrong),
				Err(TypedArrayCopyError::LengthMismatch)
			));
		}
	};
}

copy_tests!(int8, Int8Array, [-128_i8, 0, 127], [1_i8, 2, 3]);
copy_tests!(uint8, Uint8Array, [0_u8, 128, 255], [3_u8, 2, 1]);
copy_tests!(
	uint8_clamped,
	Uint8ClampedArray,
	[0_u8, 128, 255],
	[3_u8, 2, 1]
);
copy_tests!(
	uint32,
	Uint32Array,
	[0_u32, 0x8000_0000, u32::MAX],
	[1_u32, 2, 3]
);
copy_tests!(
	float64,
	Float64Array,
	[f64::NEG_INFINITY, -0.0, f64::INFINITY],
	[1.0_f64, 2.0, 3.0]
);
copy_tests!(
	big_int64,
	BigInt64Array,
	[i64::MIN, 0, i64::MAX],
	[1_i64, 2, 3]
);
copy_tests!(
	big_uint64,
	BigUint64Array,
	[0_u64, 1 << 63, u64::MAX],
	[1_u64, 2, 3]
);

#[test]
fn float16() {
	if !has_float16_array() {
		return;
	}

	let initial = [0x3c00_u16, 0xc000, 0x3555];
	let array = Float16Array::new_from_u16_slice(&initial).unwrap();
	assert_eq!(array.to_u16_vec().unwrap(), initial);

	let replacement = [0x0001, 0x7bff, 0xfc00];
	array.copy_from_u16_slice(&replacement).unwrap();
	let mut copied = [0; 3];
	array.copy_to_u16_slice(&mut copied).unwrap();
	assert_eq!(copied, replacement);
	assert!(matches!(
		array.copy_to_u16_slice(&mut [0]),
		Err(TypedArrayCopyError::LengthMismatch)
	));
}

#[test]
fn float16_errors() {
	if !has_float16_array() {
		return;
	}

	let array = Float16Array::new_from_u16_slice(&[0x3c00]).unwrap();
	let buffer = ArrayBuffer::unchecked_from(array.buffer());
	buffer.transfer().unwrap();

	assert!(matches!(
		array.copy_to_u16_slice(&mut [0]),
		Err(TypedArrayCopyError::JavaScript(_))
	));
}

#[test]
fn rust_iteration() {
	let array = Uint32Array::from(&[5, 8, 13]);
	let mut iter = array.iter();
	assert_eq!(iter.size_hint(), (0, Some(3)));
	assert_eq!(iter.next(), Some(5));
	assert_eq!(iter.next_back(), Some(13));
	assert_eq!(iter.next(), Some(8));
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next_back(), None);

	assert_eq!(array.clone().into_iter().collect::<Vec<_>>(), [5, 8, 13]);
	assert_eq!((&array).into_iter().rev().collect::<Vec<_>>(), [13, 8, 5]);
}

#[test]
fn copy_ignores_shadowed_length() {
	let array = Uint32Array::from(&[3, 5]);
	shadow_length(&array, 1);

	let mut destination = [u32::MAX];
	assert!(matches!(
		array.copy_to(&mut destination),
		Err(TypedArrayCopyError::LengthMismatch)
	));
	assert_eq!(destination, [u32::MAX]);

	assert!(matches!(
		array.copy_from(&[8]),
		Err(TypedArrayCopyError::LengthMismatch)
	));
	assert_eq!(array.get(0.0), Some(3));
	assert_eq!(array.get(1.0), Some(5));
	assert_eq!(array.typed_array_length(), 2);
	assert_eq!(array.to_vec().unwrap(), [3, 5]);
	assert_eq!(array.iter().collect::<Vec<_>>(), [3, 5]);

	let array = Uint32Array::from(&[3, 5]);
	shadow_length(&array, 3);
	let mut destination = [u32::MAX; 3];
	assert!(matches!(
		array.copy_to(&mut destination),
		Err(TypedArrayCopyError::LengthMismatch)
	));
	assert_eq!(destination, [u32::MAX; 3]);
	assert_eq!(array.clone().into_iter().collect::<Vec<_>>(), [3, 5]);
}

#[test]
fn shrinking_during_iteration() {
	if !has_resizable_array_buffer() {
		return;
	}

	let array = resizable();
	let mut iter = array.iter();
	assert_eq!(iter.size_hint(), (0, Some(4)));
	resize(&array, 8);
	assert_eq!(iter.next_back(), Some(2));
	assert_eq!(iter.next_back(), Some(1));
	assert_eq!(iter.next_back(), None);
}
