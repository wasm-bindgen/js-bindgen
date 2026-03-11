#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod custom_section;
#[cfg(test)]
mod tests;
mod util;

use std::iter::Peekable;

use proc_macro::{Span, TokenStream, TokenTree, token_stream};
#[cfg(test)]
use proc_macro2 as proc_macro;
use util::*;

use crate::custom_section::CustomSection;

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
	let mut custom_section = CustomSection::new();
	parse_string_arguments(&mut input, Span::mixed_site(), &mut custom_section)?;

	Ok(custom_section.output("js_bindgen.assembly"))
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

	let init_bytes = parse_names(&mut input)?;
	let mut custom_section = CustomSection::new();
	parse_required_embeds(&mut input, init_bytes, &mut custom_section)?;
	parse_string_arguments(&mut input, Span::mixed_site(), &mut custom_section)?;

	Ok(custom_section.output(section))
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
	mut init_bytes: Vec<u8>,
	custom_section: &mut CustomSection,
) -> Result<(), TokenStream> {
	if let Some(TokenTree::Ident(_)) = input.peek()
		&& let embeds = expect_meta_name_required_embeds(input)?
		&& !embeds.is_empty()
	{
		let Ok(count) = u8::try_from(embeds.len()) else {
			return Err(crate::compile_error(
				Span::mixed_site(),
				"expected at most 255 `required_embeds`",
			));
		};

		if embeds.iter().all(|value| value.cfg.is_none()) {
			init_bytes.push(count);
			custom_section.bytes_value(None, init_bytes);
		} else {
			custom_section.bytes_value(None, init_bytes);
			custom_section.tuple_count();
		}

		for embed in embeds {
			custom_section.tuple_value(embed.cfg, embed.expr);
		}
	} else {
		init_bytes.push(0);
		custom_section.bytes_value(None, init_bytes);
	}

	Ok(())
}
