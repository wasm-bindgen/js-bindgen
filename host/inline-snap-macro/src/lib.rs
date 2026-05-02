use std::ops::Deref;
use std::path::Path;

use js_bindgen_shared::ReadFile;
use proc_macro2::{LineColumn, Span, TokenStream};
use quote::quote_spanned;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Brace;
use syn::{Expr, LitStr, Result, Token};
use xxhash_rust::xxh3;

#[proc_macro]
pub fn inline_snap(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let Input { output, expected } = syn::parse_macro_input!(input as Input);

	match expected {
		Expected::Tokens { brace, tokens } => {
			let span = tokens.span();
			let mut expected_iter = tokens.clone().into_iter();
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

			let File { path, size, hash } = File::new(span);

			quote_spanned! {span=>
				#[allow(warnings)]
				{
					let expected = ::inline_snap::syn::parse2(::quote::quote! { #tokens }).unwrap();
					let expected = ::inline_snap::prettyplease::unparse(&expected);

					if expected != #output && ::std::env::var("BLESS").is_ok_and(|value| value == "1") {
						::inline_snap::TEST_UPDATES.add_tokens(#path, #size, #hash, #output, (#start_line, #start_col), (#end_line, #end_col));
					} else {
						::inline_snap::similar_asserts::assert_eq!(expected, #output);
					}
				}
			}
			.into()
		}
		Expected::String(string) => {
			let span = string.span();
			let LineColumn {
				line: start_line,
				column: start_col,
			} = span.start();
			let LineColumn {
				line: end_line,
				column: end_col,
			} = span.end();

			let File { path, size, hash } = File::new(span);

			quote_spanned! {span=>
				#[allow(warnings)]
				{
					let expected = ::inline_snap::normalize_wat_input(#string);

					if #string != #output && ::std::env::var("BLESS").is_ok_and(|value| value == "1") {
						::inline_snap::TEST_UPDATES.add_string(#path, #size, #hash, #output, (#start_line, #start_col), (#end_line, #end_col));
					} else {
						::inline_snap::similar_asserts::assert_eq!(expected, #output);
					}
				}
			}
			.into()
		}
	}
}

struct File {
	path: String,
	size: usize,
	hash: u64,
}

impl File {
	fn new(span: Span) -> Self {
		let file = span.file();
		let path = Path::new(&file).canonicalize();
		let (path, size, hash) = path
			.ok()
			.and_then(|path| {
				path.into_os_string()
					.into_string()
					.map(|path_str| {
						let file = ReadFile::new(Path::new(&path_str)).unwrap();
						let size = file.len();
						let hash = xxh3::xxh3_64(file.deref());

						(path_str, size, hash)
					})
					.ok()
			})
			.unwrap_or_default();

		Self { path, size, hash }
	}
}

struct Input {
	output: Expr,
	expected: Expected,
}

impl Parse for Input {
	fn parse(input: ParseStream) -> Result<Self> {
		let output = input.parse()?;
		input.parse::<Token![,]>()?;
		let lookahead = input.lookahead1();

		let expected = if lookahead.peek(Brace) {
			let expected;
			let brace = syn::braced!(expected in input);
			let tokens = expected.parse()?;

			Expected::Tokens { brace, tokens }
		} else if lookahead.peek(LitStr) {
			Expected::String(input.parse()?)
		} else {
			return Err(lookahead.error());
		};

		Ok(Self { output, expected })
	}
}

enum Expected {
	Tokens { brace: Brace, tokens: TokenStream },
	String(LitStr),
}
