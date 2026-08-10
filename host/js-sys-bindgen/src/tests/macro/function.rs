use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{GenericParam, Item};

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
fn constructor_generics() {
	let (output, _) = expand_function(
		None,
		None,
		syn::parse_quote! {
			#[js_sys(constructor = Owner<'a, Wrapper<T>, N>)]
			pub fn new<
				'a: 'b,
				'b,
				U,
				I,
				T: ?Sized + Dependency<U> = Fallback,
				const N: usize = 4,
			>(
				value: I,
			) -> Option<Owner<'a, Wrapper<T>, N>>;
		},
	);
	let item: syn::ItemImpl = syn::parse2(output).unwrap();
	let expected_owner: syn::Type = syn::parse_quote!(Owner<'a, Wrapper<T>, N>);
	assert_eq!(*item.self_ty, expected_owner);
	assert_eq!(generic_names(&item.generics), ["'a", "T", "N"]);
	let type_param = item
		.generics
		.params
		.iter()
		.find_map(|param| match param {
			GenericParam::Type(param) => Some(param),
			_ => None,
		})
		.expect("T is an impl generic");
	assert_eq!(type_param.bounds.to_token_stream().to_string(), "? Sized");
	assert!(type_param.default.is_none());
	let const_param = item
		.generics
		.params
		.iter()
		.find_map(|param| match param {
			GenericParam::Const(param) => Some(param),
			_ => None,
		})
		.expect("N is an impl generic");
	assert!(const_param.default.is_none());

	let method = item
		.items
		.iter()
		.find_map(|item| match item {
			syn::ImplItem::Fn(item) => Some(item),
			_ => None,
		})
		.expect("constructor impl contains its method");
	assert_eq!(generic_names(&method.sig.generics), ["'b", "U", "I"]);
	let where_clause = method
		.sig
		.generics
		.where_clause
		.as_ref()
		.expect("impl generic bounds move to the method");
	let predicates = where_clause.predicates.to_token_stream().to_string();
	assert!(predicates.contains("'a : 'b"));
	assert!(predicates.contains("T : Dependency < U >"));
}

#[test]
fn qualified_owner_generics() {
	let (output, _) = expand_function(
		None,
		None,
		syn::parse_quote! {
			#[js_sys(constructor = Owner<other::T, U>)]
			pub fn new<T, U>() -> Option<Owner<other::T, U>>;
		},
	);
	let item: syn::ItemImpl = syn::parse2(output).unwrap();
	assert_eq!(generic_names(&item.generics), ["U"]);
	let method = item
		.items
		.iter()
		.find_map(|item| match item {
			syn::ImplItem::Fn(item) => Some(item),
			_ => None,
		})
		.expect("constructor impl contains its method");
	assert_eq!(generic_names(&method.sig.generics), ["T"]);
}

#[test]
fn qualified_owner_js_name() {
	let mut js_names = HashMap::new();
	js_names.insert("Owner".to_owned(), "LocalOwner".to_owned());
	let mut hygiene = crate::hygiene::Hygiene::Qualified { js_sys: None };
	let (_, import) = crate::function::expand(
		&mut hygiene,
		None,
		"test_crate",
		&js_names,
		syn::parse_quote! {
			#[js_sys(constructor = other::Owner)]
			pub fn new() -> Option<other::Owner>;
		},
	)
	.unwrap();

	let binding: syn::Expr = syn::parse2(
		import
			.binding
			.as_ref()
			.expect("generated constructor has a binding")
			.wire(&import.macro_path, Span::call_site()),
	)
	.unwrap();
	let expected: syn::Expr = syn::parse_quote! {
		::js_sys::wire::WireImportBinding::Construct {
			target: ::js_sys::wire::WireGlobalPath::new(
				::core::option::Option::None,
				::core::option::Option::None,
				"Owner"
			),
			variadic: false,
		}
	};
	assert_eq!(binding, expected);
}

fn generic_names(generics: &syn::Generics) -> Vec<String> {
	generics
		.params
		.iter()
		.map(|param| match param {
			GenericParam::Lifetime(param) => param.lifetime.to_string(),
			GenericParam::Type(param) => param.ident.to_string(),
			GenericParam::Const(param) => param.ident.to_string(),
		})
		.collect()
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
	assert_error!(
		{
			extern "js-sys" {
				#[js_sys(constructor = JsTest)]
				pub fn new();
			}
		},
		"`constructor` requires a return type"
	);
}
