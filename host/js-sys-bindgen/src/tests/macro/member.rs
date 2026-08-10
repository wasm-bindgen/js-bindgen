use std::collections::HashMap;

use proc_macro2::Span;

#[test]
fn static_getter_uses_the_javascript_type_name() {
	let mut js_names = HashMap::new();
	js_names.insert("RustType".to_owned(), "JavaScriptType".to_owned());
	let mut hygiene = crate::hygiene::Hygiene::Qualified { js_sys: None };
	let (_, import) = crate::function::expand(
		&mut hygiene,
		None,
		"test_crate",
		&js_names,
		syn::parse_quote! {
			#[js_sys(static_of = RustType, getter = "value")]
			pub fn static_value() -> i32;
		},
	)
	.unwrap();

	let binding: syn::Expr = syn::parse2(
		import
			.binding
			.as_ref()
			.expect("generated member has a binding")
			.wire(&import.macro_path, Span::call_site()),
	)
	.unwrap();
	let expected: syn::Expr = syn::parse_quote! {
		::js_sys::wire::WireImportBinding::GetGlobal(
			::js_sys::wire::WireGlobalPath::new(
				::core::option::Option::None,
				::core::option::Option::Some("JavaScriptType"),
				"value"
			)
		)
	};
	assert_eq!(binding, expected);
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
