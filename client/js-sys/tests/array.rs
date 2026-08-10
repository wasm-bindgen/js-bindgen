use core::array;
use core::mem::MaybeUninit;

use js_bindgen_test::test;
use js_sys::{Array, JsString, JsValue, TryFromArrayError, js_sys};

js_bindgen::embed_js!(module = "array", name = "test", "(value) => value");
js_bindgen::embed_js!(
	module = "array",
	name = "mutate_u32",
	"(value) => {{",
	"    const original = value[0]",
	"    value[0] = 0",
	"    return original",
	"}}",
);
js_bindgen::embed_js!(
	module = "array",
	name = "throwing",
	"(value, len) => new Proxy(new Array(len).fill(value), {{",
	"    get(target, property) {{",
	"        if (property === '1') throw new Error('boom')",
	"        return target[property]",
	"    }}",
	"}})",
);
js_bindgen::embed_js!(
	module = "array",
	name = "custom_js_iterator",
	"() => {{",
	"    const array = ['indexed 0', 'indexed 1']",
	"    array[Symbol.iterator] = function* () {{ yield 'protocol' }}",
	"    return array",
	"}}",
);
js_bindgen::embed_js!(
	module = "array",
	name = "invalid_length",
	"(value) => new Proxy([value, value], {{",
	"    get(target, property) {{",
	"        if (property === 'length') return 2.5",
	"        return target[property]",
	"    }}",
	"}})",
);
js_bindgen::embed_js!(
	module = "array",
	name = "externref_length",
	required_embeds = [("js_sys", "externref.table")],
	"() => this.#jsEmbed.js_sys['externref.table'].length",
);

macro_rules! typed_slice_import {
	($name:ident: $element:ty => $constructor:literal, $embed:literal) => {
		#[js_sys]
		extern "js-sys" {
			#[js_sys(js_embed = $embed)]
			fn $name(value: &[$element]) -> Array<$element>;
		}

		js_bindgen::embed_js!(
			module = "array",
			name = $embed,
			"(value) => {{",
			"    if (!(value instanceof {constructor}))",
			"        throw new TypeError('expected a {constructor}')",
			"    return Array.from(value)",
			"}}",
			constructor = interpolate $constructor,
		);
	};
}

typed_slice_import!(i8_slice: i8 => "Int8Array", "i8_slice");
typed_slice_import!(u64_slice: u64 => "BigUint64Array", "u64_slice");
typed_slice_import!(f64_slice: f64 => "Float64Array", "f64_slice");

macro_rules! integer_array_roundtrip {
	($element:ty, $length:expr, $values:expr) => {{
		let values: [$element; $length] = $values;
		let array: Array<$element> = Array::from(&values);
		assert!(Array::is_array(array.as_ref()));

		let copied: [$element; $length] = array.to_array().unwrap();
		assert_eq!(copied, values);

		let mut copied = [<$element>::default(); $length];
		array.to_slice(&mut copied).unwrap();
		assert_eq!(copied, values);
	}};
}

macro_rules! assert_float_values {
	($actual:expr, $expected:expr) => {
		for (&actual, &expected) in $actual.iter().zip($expected.iter()) {
			if expected.is_nan() {
				assert!(actual.is_nan());
			} else {
				assert_eq!(actual.to_bits(), expected.to_bits());
			}
		}
	};
}

macro_rules! float_array_roundtrip {
	($element:ty, $length:expr, $values:expr) => {{
		let values: [$element; $length] = $values;
		let array: Array<$element> = Array::from(&values);
		assert!(Array::is_array(array.as_ref()));

		let copied: [$element; $length] = array.to_array().unwrap();
		assert_float_values!(copied, values);

		let mut copied = [<$element>::default(); $length];
		array.to_slice(&mut copied).unwrap();
		assert_float_values!(copied, values);
	}};
}

#[test]
fn js_value() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn js(value: &[JsValue]) -> Array<JsValue>;

		#[js_sys(js_embed = "throwing")]
		fn throwing(value: &JsValue, len: u32) -> Array<JsValue>;

		#[js_sys(js_embed = "invalid_length")]
		fn invalid_length(value: &JsValue) -> Array<JsValue>;

		#[js_sys(js_embed = "externref_length")]
		fn externref_length() -> u32;
	}

	let rust_array = [JsValue::UNDEFINED; 42];
	let js_array = Array::from(&rust_array);
	assert_eq!(rust_array.len(), js_array.length().try_into().unwrap());
	let empty: [JsValue; 0] = [];
	assert_eq!(Array::from(&empty).to_array::<0>().unwrap(), empty);

	let ffi_array = js(&rust_array);
	assert_eq!(rust_array.len(), ffi_array.length().try_into().unwrap());

	let mut wrong_length = [JsValue::UNDEFINED; 41];
	assert!(matches!(
		js_array.to_slice(&mut wrong_length),
		Err(TryFromArrayError::LengthMismatch {
			actual: 42,
			expected: 41,
		})
	));

	let previous = JsString::from("previous");
	let previous: JsValue = previous.into();
	let mut destination: [JsValue; 42] = array::from_fn(|_| previous.clone());
	let throwing_array = throwing(&JsValue::NULL, 42);
	assert!(matches!(
		throwing_array.to_slice(&mut destination),
		Err(TryFromArrayError::JavaScript(_))
	));
	assert!(destination.iter().all(|value| value == &previous));
	assert!(matches!(
		invalid_length(&JsValue::NULL).to_array::<2>(),
		Err(TryFromArrayError::JavaScript(_))
	));

	js_array.to_slice(&mut destination).unwrap();
	assert_eq!(rust_array, destination);

	let returned_array: [JsValue; 42] = js_array.to_array().unwrap();
	assert_eq!(rust_array, returned_array);

	let returned_array: [JsValue; 42] = ffi_array.to_array().unwrap();
	assert_eq!(rust_array, returned_array);

	let mut uninit: [MaybeUninit<JsValue>; 42] = array::from_fn(|_| MaybeUninit::uninit());
	let initialized = js_array.to_uninit_slice(&mut uninit).unwrap();
	assert_eq!(rust_array, initialized);

	let large: [JsValue; 129] = array::from_fn(|_| JsValue::UNDEFINED);
	assert!(matches!(
		throwing(&JsValue::NULL, 129).to_array::<129>(),
		Err(TryFromArrayError::JavaScript(_))
	));
	assert_eq!(Array::from(&large).to_array::<129>().unwrap(), large);

	let table_length = externref_length();
	let mut oversized = [JsValue::UNDEFINED; 513];
	assert!(matches!(
		Array::new().to_slice(&mut oversized),
		Err(TryFromArrayError::LengthMismatch {
			actual: 0,
			expected: 513,
		})
	));
	assert_eq!(externref_length(), table_length);
}

#[test]
fn u32() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn u32(value: &[u32]) -> Array<u32>;

		#[js_sys(js_embed = "throwing")]
		fn throwing_u32(value: u32, len: u32) -> Array<u32>;

		#[js_sys(js_embed = "invalid_length")]
		fn invalid_length_u32(value: u32) -> Array<u32>;

		#[js_sys(js_embed = "mutate_u32")]
		fn mutate_u32(value: &[u32]) -> u32;
	}

	let mut rust_array: [u32; 42] = array::from_fn(|i| i.try_into().unwrap());
	rust_array[0] = u32::MAX;
	rust_array[1] = 0x8000_0000;
	let js_array = Array::from(&rust_array);
	assert_eq!(rust_array.len(), js_array.length().try_into().unwrap());
	let empty: [u32; 0] = [];
	assert_eq!(Array::from(&empty).to_array::<0>().unwrap(), empty);

	let ffi_array = u32(&rust_array);
	assert_eq!(rust_array.len(), ffi_array.length().try_into().unwrap());
	assert_eq!(mutate_u32(&rust_array), u32::MAX);
	assert_eq!(rust_array[0], u32::MAX);

	let returned_array: [u32; 42] = js_array.to_array().unwrap();
	assert_eq!(rust_array, returned_array);

	let returned_array: [u32; 42] = ffi_array.to_array().unwrap();
	assert_eq!(rust_array, returned_array);
	assert!(matches!(
		invalid_length_u32(7).to_array::<2>(),
		Err(TryFromArrayError::JavaScript(_))
	));

	let mut destination = [0; 42];
	js_array.to_slice(&mut destination).unwrap();
	assert_eq!(rust_array, destination);

	let mut wrong_length = [0; 41];
	assert!(matches!(
		js_array.to_slice(&mut wrong_length),
		Err(TryFromArrayError::LengthMismatch {
			actual: 42,
			expected: 41,
		})
	));

	let mut uninit = [MaybeUninit::uninit(); 42];
	let initialized = js_array.to_uninit_slice(&mut uninit).unwrap();
	assert_eq!(rust_array, initialized);

	let throwing = throwing_u32(7, 42);
	let mut destination = [u32::MAX; 42];
	assert!(matches!(
		throwing.to_slice(&mut destination),
		Err(TryFromArrayError::JavaScript(_))
	));
	assert_eq!(destination, [u32::MAX; 42]);
}

#[test]
fn primitive_roundtrips() {
	integer_array_roundtrip!(i8, 4, [i8::MIN, -1, 0, i8::MAX]);
	integer_array_roundtrip!(u8, 4, [0, 1, 1 << 7, u8::MAX]);
	integer_array_roundtrip!(i16, 4, [i16::MIN, -1, 0, i16::MAX]);
	integer_array_roundtrip!(u16, 4, [0, 1, 1 << 15, u16::MAX]);
	integer_array_roundtrip!(i32, 4, [i32::MIN, -1, 0, i32::MAX]);
	integer_array_roundtrip!(u32, 4, [0, 1, 1 << 31, u32::MAX]);
	integer_array_roundtrip!(i64, 4, [i64::MIN, -1, 0, i64::MAX]);
	integer_array_roundtrip!(u64, 4, [0, 1, 1 << 63, u64::MAX]);
	integer_array_roundtrip!(isize, 4, [isize::MIN, -1, 0, isize::MAX]);
	integer_array_roundtrip!(usize, 4, [0, 1, 1usize << (usize::BITS - 1), usize::MAX]);

	float_array_roundtrip!(
		f32,
		6,
		[f32::NEG_INFINITY, -1.25, -0.0, 0.0, f32::INFINITY, f32::NAN,]
	);
	float_array_roundtrip!(
		f64,
		6,
		[f64::NEG_INFINITY, -1.25, -0.0, 0.0, f64::INFINITY, f64::NAN,]
	);
}

#[test]
fn primitive_slices() {
	let values = [i8::MIN, -1, 0, i8::MAX];
	assert_eq!(i8_slice(&values).to_array::<4>().unwrap(), values);

	let values = [0, 1, 1 << 63, u64::MAX];
	assert_eq!(u64_slice(&values).to_array::<4>().unwrap(), values);

	let values = [f64::NEG_INFINITY, -0.0, f64::INFINITY, f64::NAN];
	let copied = f64_slice(&values).to_array::<4>().unwrap();
	assert_float_values!(copied, values);
}

#[test]
fn rust_iteration() {
	let values = [JsValue::NULL, JsValue::UNDEFINED, JsValue::NULL];
	let array = Array::of(&values);
	let mut iterator = array.iter();

	assert_eq!(iterator.size_hint(), (3, Some(3)));
	assert_eq!(iterator.len(), 3);
	assert_eq!(iterator.next(), Some(JsValue::NULL));
	assert_eq!(iterator.next_back(), Some(JsValue::NULL));
	assert_eq!(iterator.len(), 1);
	assert_eq!(iterator.next(), Some(JsValue::UNDEFINED));
	assert_eq!(iterator.next(), None);
	assert_eq!(iterator.next_back(), None);
	assert_eq!(iterator.next(), None);

	let array = Array::of(&values[..2]);
	let mut iterator = array.iter();
	array.set_length(4);
	array.set(2, &JsValue::NULL);
	array.set(3, &JsValue::NULL);
	assert_eq!(iterator.len(), 2);
	assert_eq!(iterator.next(), Some(JsValue::NULL));
	assert_eq!(iterator.next(), Some(JsValue::UNDEFINED));
	assert_eq!(iterator.next(), None);

	let array = Array::of(&values);
	let shared = array.clone();
	let mut iterator = array.into_iter();
	shared.set_length(1);
	assert_eq!(iterator.len(), 3);
	assert_eq!(iterator.next(), Some(JsValue::NULL));
	assert_eq!(iterator.next_back(), Some(JsValue::UNDEFINED));
	assert_eq!(iterator.next(), Some(JsValue::UNDEFINED));
	assert_eq!(iterator.next(), None);
}

#[test]
fn custom_symbol_iterator() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "custom_js_iterator")]
		fn custom_js_iterator() -> Array<JsString>;
	}

	let array = custom_js_iterator();
	assert_eq!(
		array.to_array::<2>().unwrap(),
		[JsString::from("indexed 0"), JsString::from("indexed 1")]
	);
	let indexed: Vec<String> = array.iter().map(|value| String::from(&value)).collect();
	assert_eq!(indexed, ["indexed 0", "indexed 1"]);

	let protocol: Vec<String> = array
		.symbol_iterator()
		.into_iter()
		.map(|value| String::from(&value.unwrap()))
		.collect();
	assert_eq!(protocol, ["protocol"]);
}
