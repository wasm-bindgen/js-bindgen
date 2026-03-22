use js_bindgen_test::test;
use js_sys::{JsString, js_sys};
use paste::paste;
use quickcheck::quickcheck;

js_bindgen::embed_js!(module = "interop", name = "test", "(value) => value");

macro_rules! test_interop {
	($($ty:ty),+) => {$(paste! {
        #[js_sys]
        extern "js-sys" {
	        #[js_sys(js_embed = "test")]
	        fn [<interop_ $ty>](value: $ty) -> $ty;
        }
        #[test]
        fn [<test_interop_ $ty>]() {
	        fn prop(val: $ty) -> bool {
		        val == [<interop_ $ty>](val)
	        }
	        quickcheck(prop as fn($ty) -> bool);
        }
    })+};
}

macro_rules! test_interop_float {
	($($ty:ty),+) => {$(paste! {
        #[js_sys]
        extern "js-sys" {
	        #[js_sys(js_embed = "test")]
	        fn [<interop_ $ty>](value: $ty) -> $ty;
        }
        #[test]
        fn [<test_interop_ $ty>]() {
	        fn prop(val: $ty) -> bool {
		        val.total_cmp(&[<interop_ $ty>](val)) == std::cmp::Ordering::Equal
	        }
	        quickcheck(prop as fn($ty) -> bool);
        }
    })+};
}

test_interop!(i8, u8, i32, u32, i64, u64, isize, usize, i128, u128);

test_interop_float!(f32, f64);

#[test]
fn test_interop_str() {
	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_embed = "test")]
		fn interop_str(value: &str) -> JsString;
	}
	#[expect(clippy::needless_pass_by_value, reason = "checked")]
	fn prop(val: String) -> bool {
		interop_str(val.as_str()) == val.as_str()
	}
	quickcheck(prop as fn(String) -> bool);
}
