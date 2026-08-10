use proc_macro2::TokenStream;

use crate::r#macro;

mod export;
mod function;
mod member;
mod r#type;

fn macro_error(input: syn::ItemForeignMod) -> String {
	let (_, error) = r#macro::expand_for_test(TokenStream::new(), input, "test_crate").unwrap_err();

	error.to_string()
}
