use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use syn::Item;

fn expand_function(
	namespace: Option<&str>,
	js_sys: Option<&syn::Path>,
	function: syn::ForeignItemFn,
) -> (TokenStream, crate::function::FunctionImport) {
	let mut hygiene = crate::hygiene::Hygiene::Qualified { js_sys };
	crate::function::expand(
		&mut hygiene,
		namespace,
		"test_crate",
		&HashMap::new(),
		function,
	)
	.unwrap()
}

#[test]
fn function_descriptor_preserves_binding_options() {
	let js_sys: syn::Path = syn::parse_quote!(renamed);
	let (_, import) = expand_function(
		Some("console"),
		Some(&js_sys),
		syn::parse_quote! {
			#[cfg(all())]
			#[js_sys(js_name = "warn", return_abi = JsValue)]
			pub fn log(value: &JsValue) -> JsTest;
		},
	);

	assert_eq!(import.module.value(), "test_crate");
	assert_eq!(import.name.value(), "console.log");
	assert_eq!(import.macro_path, syn::parse_quote!(renamed::wire));
	assert_eq!(import.inputs[0].ty, syn::parse_quote!(&JsValue));
	assert_eq!(import.output_type, Some(syn::parse_quote!(JsValue)));
	assert_eq!(import.cfg_attrs.len(), 1);
	assert!(import.cfg_attrs[0].path().is_ident("cfg"));

	let binding: syn::Expr = syn::parse2(
		import
			.binding
			.as_ref()
			.expect("generated import has a binding")
			.wire(&import.macro_path, Span::call_site()),
	)
	.unwrap();
	let expected: syn::Expr = syn::parse_quote! {
		renamed::wire::WireImportBinding::CallGlobal {
			target: renamed::wire::WireGlobalPath::new(
				::core::option::Option::Some("console"),
				::core::option::Option::None,
				"warn"
			),
			variadic: false,
		}
	};
	assert_eq!(binding, expected);

	let (_, imported) = expand_function(
		None,
		None,
		syn::parse_quote! {
			#[js_sys(js_import)]
			pub fn imported();
		},
	);
	assert!(imported.binding.is_none());
}

#[test]
fn preserves_successful_functions_after_an_error() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			pub fn good(value: i32) -> i32;
			pub async fn bad();
		}
	};
	let (Some(output), error) =
		crate::r#macro::expand_for_test(TokenStream::new(), input, "test_crate").unwrap_err()
	else {
		panic!("expected the successful function to be preserved");
	};

	let items = output.into_items().unwrap();
	let functions: Vec<_> = items
		.iter()
		.filter_map(|item| match item {
			Item::Fn(function) => Some(function.sig.ident.to_string()),
			_ => None,
		})
		.collect();
	assert_eq!(functions, ["good"]);
	assert_eq!(error.to_string(), "`async` functions are not supported");
}

#[test]
fn invalid_options() {
	macro_rules! assert_error {
		($input:tt, $expected:literal) => {
			assert_eq!(super::macro_error(syn::parse_quote! $input), $expected);
		};
	}

	assert_error!(
		{
			extern "C" {
				pub fn log();
			}
		},
		"expected `js-sys` ABI"
	);
	assert_error!(
		{
			extern "js-sys" {
				#[js_sys(js_name = "renamed", js_import)]
				pub fn log();
			}
		},
		"`js_import` and `js_embed` cannot be combined with JavaScript binding options"
	);
	assert_error!(
		{
			extern "js-sys" {
				#[js_sys(js_import, suspending)]
				pub fn wait();
			}
		},
		"`suspending` cannot be combined with `js_import`; provide a `WebAssembly.Suspending` \
		 import directly"
	);
	assert_error!(
		{
			extern "js-sys" {
				#[js_sys(suspending, suspending)]
				pub fn wait();
			}
		},
		"duplicate attribute"
	);
	assert_error!(
		{
			extern "js-sys" {
				pub fn log(
					#[js_sys(type = i32)]
					#[js_sys(type = u32)]
					value: i32,
				);
			}
		},
		"duplicate attribute"
	);
	assert_error!(
		{
			extern "js-sys" {
				#[js_sys(return_abi = JsValue)]
				pub fn value();
			}
		},
		"`return_abi` requires a return value"
	);
	assert_error!(
		{
			extern "js-sys" {
				#[js_sys(return_abi = JsValue, return_abi = JsTest)]
				pub fn value() -> JsValue;
			}
		},
		"duplicate attribute"
	);
}
