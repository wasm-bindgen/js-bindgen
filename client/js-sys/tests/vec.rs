use js_bindgen_test::test;
use js_sys::{JsString, JsValue, js_sys};

macro_rules! identity {
	($name:ident: $element:ty) => {
		#[js_sys]
		extern "js-sys" {
			#[js_sys(js_embed = "identity")]
			fn $name(value: Vec<$element>) -> Vec<$element>;
		}
	};
}

macro_rules! typed_identity {
	($name:ident: $element:ty => $constructor:literal, $embed:literal) => {
		#[js_sys]
		extern "js-sys" {
			#[js_sys(js_embed = $embed)]
			fn $name(value: Vec<$element>) -> Vec<$element>;
		}

		js_bindgen::embed_js!(
			module = "vec",
			name = $embed,
			"value => {{",
			"    if (!(value instanceof {constructor}))",
			"        throw new TypeError('expected a {constructor}')",
			"    return value",
			"}}",
			constructor = interpolate $constructor,
		);
	};
}

identity!(js_value_identity: JsString);
identity!(string_identity: String);
typed_identity!(i8_identity: i8 => "Int8Array", "i8_identity");
typed_identity!(u8_identity: u8 => "Uint8Array", "u8_identity");
typed_identity!(i16_identity: i16 => "Int16Array", "i16_identity");
typed_identity!(u16_identity: u16 => "Uint16Array", "u16_identity");
typed_identity!(i32_identity: i32 => "Int32Array", "i32_identity");
typed_identity!(u32_identity: u32 => "Uint32Array", "u32_identity");
typed_identity!(i64_identity: i64 => "BigInt64Array", "i64_identity");
typed_identity!(u64_identity: u64 => "BigUint64Array", "u64_identity");
typed_identity!(f32_identity: f32 => "Float32Array", "f32_identity");
typed_identity!(f64_identity: f64 => "Float64Array", "f64_identity");

#[cfg(target_arch = "wasm32")]
typed_identity!(isize_identity: isize => "Int32Array", "isize_identity");
#[cfg(target_arch = "wasm64")]
typed_identity!(isize_identity: isize => "BigInt64Array", "isize_identity");
#[cfg(target_arch = "wasm32")]
typed_identity!(usize_identity: usize => "Uint32Array", "usize_identity");
#[cfg(target_arch = "wasm64")]
typed_identity!(usize_identity: usize => "BigUint64Array", "usize_identity");

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "wrong_js_value_type")]
	fn wrong_js_value_type() -> Result<Vec<JsValue>, JsValue>;

	#[js_sys(js_embed = "wrong_u32_type")]
	fn wrong_u32_type() -> Result<Vec<u32>, JsValue>;

	#[js_sys(js_embed = "throwing_array")]
	fn throwing_array() -> Result<Vec<JsValue>, JsValue>;

	#[js_sys(js_embed = "growing_array")]
	fn growing_array() -> Vec<JsValue>;

	#[js_sys(js_embed = "externref_length")]
	fn externref_length() -> u32;
}

js_bindgen::embed_js!(module = "vec", name = "identity", "value => value",);
js_bindgen::embed_js!(
	module = "vec",
	name = "wrong_js_value_type",
	"() => new Uint32Array()",
);
js_bindgen::embed_js!(module = "vec", name = "wrong_u32_type", "() => []");
js_bindgen::embed_js!(
	module = "vec",
	name = "throwing_array",
	"() => new Proxy([null, null], {{",
	"    get(target, property, receiver) {{",
	"        if (property === '1') throw new Error('boom')",
	"        return Reflect.get(target, property, receiver)",
	"    }},",
	"}})",
);
js_bindgen::embed_js!(
	module = "vec",
	name = "growing_array",
	"(() => {{",
	"    const memory = this.#memory",
	"    return () => new Proxy([null, null], {{",
	"        get(target, property, receiver) {{",
	#[cfg(target_arch = "wasm32")]
	"            if (property === '0') memory.grow(1)",
	#[cfg(target_arch = "wasm64")]
	"            if (property === '0') memory.grow(1n)",
	"            return Reflect.get(target, property, receiver)",
	"        }},",
	"    }})",
	"}})()",
);
js_bindgen::embed_js!(
	module = "vec",
	name = "externref_length",
	required_embeds = [("js_sys", "externref.table")],
	"() => this.#jsEmbed.js_sys['externref.table'].length",
);

#[test]
fn js_value_roundtrip() {
	let values = vec![
		JsString::from("first"),
		JsString::from(""),
		JsString::from("第三个 🦀"),
	];
	let result = js_value_identity(values);

	assert_eq!(result.len(), 3);
	assert_eq!(result[0], "first");
	assert_eq!(result[1], "");
	assert_eq!(result[2], "第三个 🦀");
	assert_eq!(js_value_identity(Vec::new()), Vec::<JsString>::new());
}

#[test]
fn u32_roundtrip() {
	assert_eq!(u32_identity(vec![0, 1, u32::MAX]), [0, 1, u32::MAX]);
	assert_eq!(u32_identity(Vec::new()), Vec::<u32>::new());
}

#[test]
fn string_roundtrip() {
	let values = vec![
		String::from("first"),
		String::new(),
		String::from("第三个 🦀"),
	];
	assert_eq!(string_identity(values.clone()), values);
}

#[test]
fn numeric_roundtrips() {
	assert_eq!(
		i8_identity(vec![i8::MIN, -1, 0, i8::MAX]),
		[i8::MIN, -1, 0, i8::MAX]
	);
	assert_eq!(u8_identity(vec![0, 1, u8::MAX]), [0, 1, u8::MAX]);
	assert_eq!(
		i16_identity(vec![i16::MIN, -1, 0, i16::MAX]),
		[i16::MIN, -1, 0, i16::MAX]
	);
	assert_eq!(u16_identity(vec![0, 1, u16::MAX]), [0, 1, u16::MAX]);
	assert_eq!(
		i32_identity(vec![i32::MIN, -1, 0, i32::MAX]),
		[i32::MIN, -1, 0, i32::MAX]
	);
	assert_eq!(
		i64_identity(vec![i64::MIN, -1, 0, i64::MAX]),
		[i64::MIN, -1, 0, i64::MAX]
	);
	assert_eq!(u64_identity(vec![0, 1, u64::MAX]), [0, 1, u64::MAX]);
	assert_eq!(
		isize_identity(vec![isize::MIN, -1, 0, isize::MAX]),
		[isize::MIN, -1, 0, isize::MAX],
	);
	assert_eq!(usize_identity(vec![0, 1, usize::MAX]), [0, 1, usize::MAX]);
	assert_eq!(
		f32_identity(vec![-1.25, -0.0, 0.0, f32::INFINITY]),
		[-1.25, -0.0, 0.0, f32::INFINITY]
	);
	assert_eq!(
		f64_identity(vec![-1.25, -0.0, 0.0, f64::INFINITY]),
		[-1.25, -0.0, 0.0, f64::INFINITY]
	);
}

#[test]
fn invalid_representation() {
	assert!(wrong_js_value_type().is_err());
	assert!(wrong_u32_type().is_err());
}

#[test]
fn failed_conversion_recycles_slots() {
	assert!(throwing_array().is_err());
	let grown_length = externref_length();

	for _ in 0..256 {
		assert!(throwing_array().is_err());
	}

	assert_eq!(externref_length(), grown_length);
}

#[test]
fn conversion_survives_memory_growth() {
	let result = growing_array();
	assert_eq!(result, [JsValue::NULL, JsValue::NULL]);
}

#[test]
fn roundtrips_recycle_slots() {
	let value = JsString::from("value");
	let roundtrip = || js_value_identity((0..64).map(|_| value.clone()).collect());

	drop(roundtrip());
	let grown_length = externref_length();
	for _ in 0..16 {
		drop(roundtrip());
	}

	assert_eq!(externref_length(), grown_length);
}
