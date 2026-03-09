use std::path::Path;

use proc_macro2::{LineColumn, TokenStream};
use quote::quote_spanned;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Brace;
use syn::{Expr, Token};

#[proc_macro]
pub fn inline_snap(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let Input {
		output,
		brace,
		expected,
	} = syn::parse_macro_input!(input as Input);

	let span = expected.span();
	let mut expected_iter = expected.clone().into_iter();
	let start = expected_iter.next().map(|tok| tok.span());
	let end = expected_iter.last().map(|tok| tok.span()).or(start);
	let LineColumn {
		line: start_line,
		column: start_col,
	} = start.map_or(brace.span.open().end(), |span| span.start());
	let LineColumn {
		line: end_line,
		column: end_col,
	} = end.map_or(brace.span.close().start(), |span| span.end());

	let file = span.file();
	let path = Path::new(&file).canonicalize();
	let path = path
		.as_ref()
		.ok()
		.and_then(|path| path.to_str())
		.unwrap_or("");

	quote_spanned! {span=>
		#[allow(warnings)]
		{
			let expected = ::inline_snap::syn::parse2(::quote::quote! { #expected }).unwrap();
			let expected = ::inline_snap::prettyplease::unparse(&expected);

			if expected != #output && ::std::env::var("BLESS").is_ok_and(|value| value == "1") {
				::inline_snap::TEST_UPDATES.add(#path, #output, #start_line, #start_col, #end_line, #end_col);
			} else {
				::inline_snap::similar_asserts::assert_eq!(expected, #output);
			}
		}
	}
	.into()
}

struct Input {
	output: Expr,
	brace: Brace,
	expected: TokenStream,
}

impl Parse for Input {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let output = input.parse()?;
		input.parse::<Token![,]>()?;

		let expected;
		let brace = syn::braced!(expected in input);
		let expected = expected.parse()?;

		Ok(Self {
			output,
			brace,
			expected,
		})
	}
}
