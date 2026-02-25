use std::ptr::{self, NonNull};

use js_bindgen_test::test;
use js_sys::{JsBigInt, JsNumber, JsString, js_sys};
use paste::paste;

js_bindgen::embed_js!(module = "numeric", name = "test", "(value) => value");

#[test]
fn bool() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn bool_input(value: bool) -> JsNumber;

		#[js_sys(js_embed = "test")]
		fn bool_output(value: &JsNumber) -> bool;
	}

	let r#false = bool_input(false);
	assert_eq!(JsString::new(&r#false), "false");
	assert!(!bool_output(&r#false));

	let r#true = bool_input(true);
	assert_eq!(JsString::new(&r#true), "true");
	assert!(bool_output(&r#true));
}

#[cfg(target_arch = "wasm32")]
type JsUsize = JsNumber;
#[cfg(target_arch = "wasm64")]
type JsUsize = JsBigInt;

macro_rules! unsigned {
    ($js:ty, $($ty:ty),*) => {$(paste! {
        #[test]
        fn $ty() {
            internal!($js, $ty);
        }
    })*};
}

macro_rules! signed {
    ($js:ty, $($ty:ty),*) => {$(paste! {
        #[test]
        fn $ty() {
            internal!($js, $ty);

            let null = [<$ty _input>](0);
			assert_eq!(JsString::new(&null), 0.to_string());
			assert_eq!([<$ty _output>](&null), 0);
        }
    })*};
}

macro_rules! internal {
	($js:ty, $ty:ty) => {
		paste! {
			#[js_sys]
			extern "js-sys" {
				#[js_sys(js_embed = "test")]
				fn [<$ty _input>](value: $ty) -> $js;

				#[js_sys(js_embed = "test")]
				fn [<$ty _output>](value: &$js) -> $ty;
			}

			let min = [<$ty _input>]($ty::MIN);
			assert_eq!(JsString::new(&min), $ty::MIN.to_string());
			assert_eq!([<$ty _output>](&min), $ty::MIN);

			let max = [<$ty _input>]($ty::MAX);
			assert_eq!(JsString::new(&max), $ty::MAX.to_string());
			assert_eq!([<$ty _output>](&max), $ty::MAX);
		}
	};
}

unsigned!(JsNumber, u8, u16, u32);
unsigned!(JsBigInt, u64, u128);
unsigned!(JsUsize, usize);

signed!(JsNumber, i8, i16, i32);
signed!(JsBigInt, i64, i128);
unsigned!(JsUsize, isize);

#[cfg(target_arch = "wasm32")]
const MAX_SAFE_PTR: usize = usize::MAX;
#[cfg(target_arch = "wasm64")]
const MAX_SAFE_PTR: usize = 0x0020_0000_0000_0000 - 1;

#[test]
fn ptr() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn ptr_input(value: *const ()) -> JsUsize;

		#[js_sys(js_embed = "test")]
		fn ptr_output(value: &JsUsize) -> *const ();
	}

	let null = ptr_input(ptr::null());
	assert_eq!(JsString::new(&null), "0");
	assert_eq!(ptr_output(&null), ptr::null());

	let max_ptr = ptr::without_provenance(MAX_SAFE_PTR);
	let max = ptr_input(max_ptr);
	assert_eq!(JsString::new(&max), MAX_SAFE_PTR.to_string());
	assert_eq!(ptr_output(&max), max_ptr);
}

#[test]
fn ptr_mut() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn ptr_mut_input(value: *mut ()) -> JsUsize;

		#[js_sys(js_embed = "test")]
		fn ptr_mut_output(value: &JsUsize) -> *mut ();
	}

	let null = ptr_mut_input(ptr::null_mut());
	assert_eq!(JsString::new(&null), "0");
	assert_eq!(ptr_mut_output(&null), ptr::null_mut());

	let max_ptr = ptr::without_provenance_mut(MAX_SAFE_PTR);
	let max = ptr_mut_input(max_ptr);
	assert_eq!(JsString::new(&max), MAX_SAFE_PTR.to_string());
	assert_eq!(ptr_mut_output(&max), max_ptr);
}

#[test]
fn non_null() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn non_null_input(value: NonNull<()>) -> JsUsize;

		#[js_sys(js_embed = "test")]
		fn non_null_output(value: &JsUsize) -> NonNull<()>;
	}

	let one_ptr = NonNull::new(ptr::without_provenance_mut(1)).unwrap();
	let one = non_null_input(one_ptr);
	assert_eq!(JsString::new(&one), "1");
	assert_eq!(non_null_output(&one), one_ptr);

	let max_ptr = NonNull::new(ptr::without_provenance_mut(MAX_SAFE_PTR)).unwrap();
	let max = non_null_input(max_ptr);
	assert_eq!(JsString::new(&max), MAX_SAFE_PTR.to_string());
	assert_eq!(non_null_output(&max), max_ptr);
}
