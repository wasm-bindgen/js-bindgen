use js_bindgen_test::test;
use js_sys::{JsValue, js_sys};

js_bindgen::embed_js!(
	module = "builtins",
	name = "binding.install",
	"() => {{",
	"    globalThis.JsSysBindingTest = class {{",
	"        constructor(...values) {{ this.length = values.length }}",
	"    }}",
	"    globalThis.JsSysBindingTest.value = 0",
	"}}",
);

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "binding.install")]
	fn install();

	#[js_sys(js_name = "JsSysBindingTest")]
	type TestBinding;

	#[js_sys(constructor, variadic)]
	fn new(values: &[JsValue]) -> TestBinding;

	#[js_sys(static_of = TestBinding, getter = "value")]
	fn static_value() -> i32;

	#[js_sys(static_of = TestBinding, setter = "value")]
	fn set_static_value(value: i32);

	#[js_sys(getter)]
	fn length(self: &TestBinding) -> u32;
}

#[test]
fn binding_shapes() {
	install();

	TestBinding::set_static_value(42);
	assert_eq!(TestBinding::static_value(), 42);

	let value = TestBinding::new(&[JsValue::NULL, JsValue::UNDEFINED]);
	assert_eq!(value.length(), 2);
}
