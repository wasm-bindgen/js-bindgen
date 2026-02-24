#![cfg_attr(all(coverage_nightly, not(test)), feature(coverage_attribute))]

#[cfg(test)]
mod tests;
mod util;

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
	js_internal(input, "js_bindgen.embed")
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
	js_internal(input, "js_bindgen.import")
}

fn js_internal(input: TokenStream, section: &str) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();

	let names = parse_names(&mut input)?;
	let mut data = parse_required_embeds(&mut input, names)?;
	parse_string_arguments(&mut input, Span::mixed_site(), &mut data)?;

	let output = custom_section(section, &data);

	Ok(output)
}

fn parse_names(input: &mut Peekable<token_stream::IntoIter>) -> Result<Vec<u8>, TokenStream> {
	let module = expect_meta_name_string(input, "module")?;
	let module_len: u16 = module
		.len()
		.try_into()
		.expect("import module length is too large");
	let name = expect_meta_name_string(input, "name")?;
	let name_len: u16 = name
		.len()
		.try_into()
		.expect("import name length is too large");

	Ok([
		module_len.to_le_bytes().as_slice(),
		module.as_bytes(),
		&name_len.to_le_bytes(),
		name.as_bytes(),
	]
	.concat())
}

fn parse_required_embeds(
	input: &mut Peekable<token_stream::IntoIter>,
	mut names: Vec<u8>,
) -> Result<Vec<Argument>, TokenStream> {
	let mut data = Vec::new();

	if let Some(TokenTree::Ident(_)) = input.peek() {
		let embeds = expect_meta_name_required_embeds(input, "required_embeds")?;
		data.reserve(embeds.len() + 1);

		let len = u8::try_from(embeds.len()).expect("too many `required_embeds` elements");
		names.push(len);
		data.push(Argument::bytes(names));

		for embed in embeds {
			data.extend([
				Argument::interpolate_with_length(embed.module),
				Argument::interpolate_with_length(embed.name),
			]);
		}
	} else {
		names.push(0);
		data.push(Argument::bytes(names));
	}

	Ok(data)
}
