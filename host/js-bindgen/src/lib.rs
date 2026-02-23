#![cfg_attr(all(coverage_nightly, not(test)), feature(coverage_attribute))]

#[cfg(test)]
mod tests;
mod util;

#[cfg(not(test))]
use std::env;
use std::iter::Peekable;

use proc_macro::{Span, TokenStream, TokenTree, token_stream};
#[cfg(test)]
use proc_macro2 as proc_macro;
use util::*;

#[proc_macro]
pub fn unsafe_embed_asm(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
	#[cfg_attr(
		not(test),
		expect(clippy::useless_conversion, reason = "`proc-macro2` compatiblity")
	)]
	embed_asm_internal(input.into())
		.unwrap_or_else(|e| e)
		.into()
}

fn embed_asm_internal(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();
	let mut data = Vec::new();
	parse_string_arguments(&mut input, Span::mixed_site(), &mut data)?;
	let output = custom_section("js_bindgen.assembly", &data);

	Ok(output)
}

#[proc_macro]
pub fn embed_js(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
	#[cfg_attr(
		not(test),
		expect(clippy::useless_conversion, reason = "`proc-macro2` compatiblity")
	)]
	embed_js_internal(input.into()).unwrap_or_else(|e| e).into()
}

fn embed_js_internal(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();

	let package = package();
	let name = expect_meta_name_string(&mut input, "name")?;

	let mut data = parse_required_embeds(&mut input)?;

	parse_string_arguments(&mut input, Span::mixed_site(), &mut data)?;
	let output = custom_section(&format!("js_bindgen.embed.{package}.{name}"), &data);

	Ok(output)
}

#[proc_macro]
pub fn import_js(input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
	#[cfg_attr(
		not(test),
		expect(clippy::useless_conversion, reason = "`proc-macro2` compatiblity")
	)]
	import_js_internal(input.into())
		.unwrap_or_else(|e| e)
		.into()
}

fn import_js_internal(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();

	let package = package();
	let import_name = expect_meta_name_string(&mut input, "name")?;

	let mut data = parse_required_embeds(&mut input)?;

	parse_string_arguments(&mut input, Span::mixed_site(), &mut data)?;
	let output = custom_section(&format!("js_bindgen.import.{package}.{import_name}"), &data);

	Ok(output)
}

fn parse_required_embeds(
	input: &mut Peekable<token_stream::IntoIter>,
) -> Result<Vec<Argument>, TokenStream> {
	let mut data = Vec::new();

	if let Some(TokenTree::Ident(_)) = input.peek() {
		let required_embeds = expect_meta_name_array(input, "required_embeds")?;
		data.reserve(required_embeds.len() + 1);

		let len = u8::try_from(required_embeds.len()).expect("too many `required_embeds` elements");
		data.push(Argument {
			cfg: None,
			kind: ArgumentKind::Bytes(vec![len]),
		});

		for required_embed in required_embeds {
			data.extend([Argument {
				cfg: None,
				kind: ArgumentKind::InterpolateWithLength(required_embed),
			}]);
		}
	} else {
		data.push(Argument {
			cfg: None,
			kind: ArgumentKind::Bytes(vec![0]),
		});
	}

	Ok(data)
}

#[cfg(not(test))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn package() -> String {
	env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found")
}

#[cfg(test)]
fn package() -> String {
	String::from("test_crate")
}
