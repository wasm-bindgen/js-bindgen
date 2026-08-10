use proc_macro2::TokenStream;

fn generated_rust(input: syn::ItemForeignMod) -> String {
	let items = crate::r#macro::expand_for_test(TokenStream::new(), input, "test_crate")
		.unwrap()
		.into_items()
		.unwrap();

	prettyplease::unparse(&syn::File {
		shebang: None,
		attrs: Vec::new(),
		items,
	})
}

#[test]
fn member_operations() {
	let output = generated_rust(syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(js_name = "JavaScriptType")]
			pub type RustType;

			#[js_sys(constructor)]
			pub fn new() -> RustType;

			#[js_sys(static_of = RustType, getter = "value")]
			pub fn static_value() -> i32;

			pub fn call(self: &RustType);

			#[js_sys(getter = "value")]
			pub fn value(self: &RustType) -> i32;

			#[js_sys(setter)]
			pub fn set_value(self: &RustType, value: i32);

			#[js_sys(indexing_getter)]
			pub fn get(self: &RustType, index: u32) -> JsValue;

			#[js_sys(indexing_setter)]
			pub fn set(self: &RustType, index: u32, value: &JsValue);

			#[js_sys(indexing_deleter)]
			pub fn delete(self: &RustType, index: u32);

			#[js_sys(variadic)]
			pub fn push(self: &RustType, first: &JsValue, rest: &[JsValue]);
		}
	});

	let operations: Vec<_> = output
		.lines()
		.filter_map(|line| line.trim().strip_prefix("direct_call: "))
		.map(|line| line.trim_end_matches(','))
		.collect();
	assert_eq!(
		operations,
		[
			r#""new globalThis.JavaScriptType()""#,
			r#""globalThis.JavaScriptType.value""#,
			r#""arg0_0.call()""#,
			r#""arg0_0.value""#,
			r#""arg0_0.value = arg1_0""#,
			r#""arg0_0[arg1_0]""#,
			r#""arg0_0[arg1_0] = arg2_0""#,
			r#""delete arg0_0[arg1_0]""#,
			r#""arg0_0.push(arg1_0, ...arg2_0)""#,
		]
	);
}

#[test]
fn invalid_member_options() {
	let variadic = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(variadic)]
			pub fn call();
		}
	};
	assert_eq!(
		super::macro_error(variadic),
		"`variadic` requires at least one argument"
	);

	let setter = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(setter)]
			pub fn update(self: &JsTest, value: i32);
		}
	};
	assert_eq!(
		super::macro_error(setter),
		"`setter` cannot infer a field name; use `setter = \"field\"`"
	);
}
