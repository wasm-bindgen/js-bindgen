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

signed!(JsNumber, i8, i16, i32);
signed!(JsBigInt, i64, i128);
