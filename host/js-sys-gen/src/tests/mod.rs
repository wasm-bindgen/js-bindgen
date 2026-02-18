#[cfg(feature = "macro")]
mod r#macro;
mod r#type;
#[cfg(feature = "web-idl")]
mod web_idl;

use proc_macro2::TokenStream;

#[track_caller]
fn test(output: TokenStream, expected: TokenStream) {
	let output = syn::parse2(output).unwrap();
	let output = prettyplease::unparse(&output);
	let expected = syn::parse2(expected).unwrap();
	let expected = prettyplease::unparse(&expected);

	similar_asserts::assert_eq!(expected, output);
}
