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

	let mut custom_section = CustomSection::new();

	expect_js_path(&mut input, "module", &mut custom_section)?;
	expect_js_path(&mut input, "name", &mut custom_section)?;

	parse_required_embeds(&mut input, &mut custom_section)?;
	parse_string_arguments(&mut input, Span::mixed_site(), &mut custom_section)?;

	Ok(custom_section.output(section))
}

fn parse_required_embeds(
	input: &mut Peekable<token_stream::IntoIter>,
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
			custom_section.byte_value(None, count);
		} else {
			custom_section.tuple_count();
		}

		for embed in embeds {
			custom_section.tuple_value(embed.cfg, embed.expr);
		}
	} else {
		custom_section.byte_value(None, 0);
	}

	Ok(())
}

fn expect_js_path(
	mut stream: &mut Peekable<token_stream::IntoIter>,
	attribute: &str,
	custom_section: &mut CustomSection,
) -> Result<(), TokenStream> {
	let error = format!("`{attribute} = ...`");
	let ident = parse_ident(&mut stream, Span::mixed_site(), &error)?;
	let mut span = SpanRange::from(ident.span());

	#[cfg_attr(test, expect(clippy::cmp_owned, reason = "`proc-macro2` compatiblity"))]
	if ident.to_string() != attribute {
		return Err(compile_error(span, format!("expected `{attribute}`")));
	}

	span.end = expect_punct(&mut stream, '=', span, &error, true)?.span();

	if let Some(TokenTree::Literal(l)) = stream.peek()
		&& let lit = l.to_string()
		&& let Some(stripped) = { lit.strip_prefix('"').and_then(|lit| lit.strip_suffix('"')) }
	{
		let lit_span = l.span();
		span.end = lit_span;
		stream.next();
		let mut string = parse_inner_string(stripped, span)?;

		let Ok(length) = u16::try_from(string.len()) else {
			return Err(compile_error(
				lit_span,
				"expected string length to be less than or equal to `u16::MAX`",
			));
		};

		custom_section.bytes_value(None, length.to_le_bytes());
		custom_section.string_value(None, &mut string);
	} else {
		let (value, value_span) = parse_ty_or_value(stream, span, &error)?;
		span.end = value_span.end;
		custom_section.interpolate_with_length_value(value);
	}

	if stream.peek().is_some() {
		expect_punct(&mut stream, ',', span, "a `,` after an attribute", false)?;
	}

	Ok(())
}
