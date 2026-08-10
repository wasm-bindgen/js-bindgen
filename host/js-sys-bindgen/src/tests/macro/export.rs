use proc_macro2::TokenStream;
use quote::quote;

#[test]
fn invalid_export_options() {
	let function: syn::ItemFn = syn::parse_quote! {
		fn answer() -> i32 {
			42
		}
	};
	let error = crate::export::r#macro(quote!(promising = true), &function, Some("test_crate"))
		.unwrap_err();
	assert_eq!(error.to_string(), "`promising` supports no values");

	let error = crate::export::r#macro(quote!(promising, promising), &function, Some("test_crate"))
		.unwrap_err();
	assert_eq!(error.to_string(), "duplicate `promising` argument");

	let borrowed: syn::ItemFn = syn::parse_quote! {
		fn echo(value: &JsString) -> &JsString {
			value
		}
	};
	let error =
		crate::export::r#macro(TokenStream::new(), &borrowed, Some("test_crate")).unwrap_err();
	assert_eq!(error.to_string(), "cannot return a borrowed reference");
}
