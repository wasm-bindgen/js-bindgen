#![cfg_attr(all(coverage_nightly, not(test)), feature(coverage_attribute))]

#[cfg(test)]
mod tests;
mod util;

#[cfg(not(test))]
use std::env;

use proc_macro::{Span, TokenStream, TokenTree};
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
	let name = expect_meta_name_value(&mut input, "name")?;

	let mut data = Vec::new();

	let embed = if let Some(TokenTree::Ident(_)) = input.peek() {
		expect_meta_name_value(&mut input, "required_embed")?
	} else {
		String::new()
	};

	let mut embed_data = Vec::new();
	embed_data.extend_from_slice(
		&u16::try_from(embed.len())
			.expect("`required_embed` name too long")
			.to_le_bytes(),
	);
	embed_data.append(&mut embed.into_bytes());

	data.push(Argument {
		cfg: None,
		kind: ArgumentKind::Bytes(embed_data),
	});

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
	let import_name = expect_meta_name_value(&mut input, "name")?;

	let data = if let Some(TokenTree::Ident(_)) = input.peek() {
		let required_embed = expect_meta_name_value(&mut input, "required_embed")?;
		let len = u16::try_from(required_embed.len())
			.expect("`required_embed` name too long")
			.to_le_bytes();

		[&len, required_embed.as_bytes()].concat()
	} else {
		vec![0, 0]
	};

	let mut data = vec![Argument {
		cfg: None,
		kind: ArgumentKind::Bytes(data),
	}];
	parse_string_arguments(&mut input, Span::mixed_site(), &mut data)?;
	let output = custom_section(&format!("js_bindgen.import.{package}.{import_name}"), &data);

	Ok(output)
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
