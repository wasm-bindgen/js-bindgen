mod fail;
mod success;

use proc_macro2::TokenStream;
use quote::quote;

#[track_caller]
fn test(output: TokenStream, expected: TokenStream) {
	let output = syn::parse2(output).unwrap();
	let output = prettyplease::unparse(&output);
	let expected = syn::parse2(expected).unwrap();
	let expected = prettyplease::unparse(&expected);

	similar_asserts::assert_eq!(expected, output);
}

#[track_caller]
fn error(output: TokenStream, message: &str) {
	let output = syn::parse2(output).unwrap();
	let output = prettyplease::unparse(&output);
	let expected = quote! {
		::core::compile_error!(#message);
	};
	let expected = syn::parse2(expected).unwrap();
	let expected = prettyplease::unparse(&expected);

	similar_asserts::assert_eq!(expected, output);
}
