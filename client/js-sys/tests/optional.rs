#![cfg_attr(target_arch = "wasm64", feature(simd_wasm64))]

#[cfg(target_arch = "wasm32")]
use core::arch::wasm32 as wasm;
#[cfg(target_arch = "wasm64")]
use core::arch::wasm64 as wasm;

use js_bindgen_test::test;
use js_sys::{JsArray, JsString, JsValue, js_sys};

js_bindgen::embed_js!(module = "optional", name = "test", "(value) => value");

macro_rules! assert_roundtrip {
	($($function:ident: $ty:ty => [$($value:expr),+ $(,)?]),+ $(,)?) => {
		$(
			#[js_sys]
			extern "js-sys" {
				#[js_sys(js_embed = "test")]
				fn $function(value: Option<$ty>) -> Option<$ty>;
			}

			assert_eq!($function(None), None);
			$(
				assert_eq!($function(Some($value)), Some($value));
			)+
		)+
	};
}

#[test]
fn numeric() {
	assert_roundtrip! {
		bool_option: bool => [false, true],
		i8_option: i8 => [i8::MIN, 0, i8::MAX],
		u8_option: u8 => [u8::MIN, u8::MAX],
		i16_option: i16 => [i16::MIN, 0, i16::MAX],
		u16_option: u16 => [u16::MIN, u16::MAX],
		i32_option: i32 => [i32::MIN, 0, i32::MAX],
		u32_option: u32 => [u32::MIN, u32::MAX],
		i64_option: i64 => [i64::MIN, 0, i64::MAX],
		u64_option: u64 => [u64::MIN, u64::MAX],
		isize_option: isize => [isize::MIN, 0, isize::MAX],
		usize_option: usize => [usize::MIN, usize::MAX],
		i128_option: i128 => [i128::MIN, -(1_i128 << 64), -1, 0, 1_i128 << 64, i128::MAX],
		u128_option: u128 => [u128::MIN, u128::from(u64::MAX), 1_u128 << 64, u128::MAX],
	}

	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn f32_option(value: Option<f32>) -> Option<f32>;
		#[js_sys(js_embed = "test")]
		fn f64_option(value: Option<f64>) -> Option<f64>;
	}

	assert!(f32_option(None).is_none());
	assert!(f64_option(None).is_none());
	for value in [
		f32::NEG_INFINITY,
		f32::MIN,
		-0.0,
		0.0,
		f32::MAX,
		f32::INFINITY,
	] {
		assert_eq!(f32_option(Some(value)).unwrap().to_bits(), value.to_bits());
	}
	assert!(f32_option(Some(f32::NAN)).unwrap().is_nan());

	for value in [
		f64::NEG_INFINITY,
		f64::MIN,
		-0.0,
		0.0,
		f64::MAX,
		f64::INFINITY,
	] {
		assert_eq!(f64_option(Some(value)).unwrap().to_bits(), value.to_bits());
	}
	assert!(f64_option(Some(f64::NAN)).unwrap().is_nan());

	assert_eq!(u128_option(Some(u128::MAX)), Some(u128::MAX));
	assert_eq!(u128_option(None), None);
	assert_ne!(wasm::memory_grow::<0>(1), usize::MAX);
	assert_eq!(i64_option(Some(i64::MIN)), Some(i64::MIN));
	assert_eq!(u64_option(Some(u64::MAX)), Some(u64::MAX));
	assert_eq!(isize_option(Some(isize::MIN)), Some(isize::MIN));
	assert_eq!(usize_option(Some(usize::MAX)), Some(usize::MAX));
	assert_eq!(u128_option(Some(1_u128 << 64)), Some(1_u128 << 64));
	assert_eq!(i128_option(Some(-1)), Some(-1));
	assert_eq!(i128_option(None), None);
}

#[test]
fn js_value() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn js_value_option(value: Option<&JsValue>) -> Option<JsValue>;

		#[js_sys(js_embed = "test")]
		fn js_array_option(value: Option<&JsArray>) -> Option<JsArray>;

		#[js_sys(js_embed = "test")]
		fn js_string_option(value: Option<&JsString>) -> Option<JsString>;
	}

	assert_eq!(js_value_option(None), None);
	assert_eq!(js_value_option(Some(&JsValue::UNDEFINED)), None);
	assert_eq!(js_value_option(Some(&JsValue::NULL)), None);

	let string = JsString::from("test");
	let string = js_value_option(Some(string.as_ref())).unwrap();
	assert_eq!(JsString::new(&string), "test");

	let array = JsArray::from(&[JsValue::UNDEFINED]);
	let array = js_array_option(Some(&array)).unwrap();
	assert_eq!(array.length(), 1);
	assert!(js_array_option(None).is_none());

	assert!(js_string_option(None).is_none());
	let string = JsString::from("test");
	assert_eq!(js_string_option(Some(&string)).unwrap(), "test");
}
