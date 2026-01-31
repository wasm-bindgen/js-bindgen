#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

#[cfg(test)]
extern crate proc_macro2 as proc_macro;
#[cfg(test)]
use shared as js_bindgen_macro_shared;

// There is currently no way to execute proc-macros in non-proc-macro crates.
// However, we need it for testing. So we somehow have to enable `proc-macro2`,
// even in dependencies. It turns out that this is quite difficult to accomplish
// in dependencies, e.g. via crate features. Including the crate via a module is
// what worked for now. `rust-analyzer` doesn't seem to like `path`s outside the
// crate though, so we added a symlink.
//
// See https://github.com/rust-lang/rust-analyzer/issues/3898.
#[cfg(test)]
#[path = "shared/lib.rs"]
mod shared;
#[cfg(test)]
mod tests;

#[cfg(not(test))]
use std::env;
use std::iter::Peekable;
use std::mem;

use js_bindgen_macro_shared::*;
use proc_macro::{Delimiter, Span, TokenStream, TokenTree, token_stream};

#[cfg_attr(not(test), proc_macro)]
pub fn unsafe_embed_asm(input: TokenStream) -> TokenStream {
	embed_asm_internal(input).unwrap_or_else(|e| e)
}

fn embed_asm_internal(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();
	let assembly = parse_string_arguments(&mut input, Span::mixed_site())?;
	Ok(custom_section("js_bindgen.assembly", None, &assembly))
}

#[cfg_attr(not(test), proc_macro)]
pub fn embed_js(input: TokenStream) -> TokenStream {
	embed_js_internal(input).unwrap_or_else(|e| e)
}

fn embed_js_internal(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();

	let package = package();
	let name = expect_meta_name_value(&mut input, "name")?;

	Ok(custom_section(
		&format!("js_bindgen.js.{package}.{name}"),
		None,
		&parse_string_arguments(&mut input, Span::mixed_site())?,
	))
}

#[cfg_attr(not(test), proc_macro)]
pub fn import_js(input: TokenStream) -> TokenStream {
	import_js_internal(input).unwrap_or_else(|e| e)
}

fn import_js_internal(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();

	let package = package();
	let import_name = expect_meta_name_value(&mut input, "name")?;

	let required_embed = if let Some(TokenTree::Ident(_)) = input.peek() {
		Some(expect_meta_name_value(&mut input, "required_embed")?)
	} else {
		None
	};

	let len = u16::try_from(required_embed.as_deref().map(str::len).unwrap_or(0))
		.expect("`required_embed` name too long")
		.to_le_bytes();
	let unstructured = [
		&len,
		required_embed.as_deref().map(str::as_bytes).unwrap_or(&[]),
	]
	.concat();

	Ok(custom_section(
		&format!("js_bindgen.import.{package}.{import_name}"),
		Some(&unstructured),
		&parse_string_arguments(&mut input, Span::mixed_site())?,
	))
}

fn parse_string_arguments(
	mut stream: &mut Peekable<token_stream::IntoIter>,
	mut previous_span: Span,
) -> Result<Vec<Argument>, TokenStream> {
	let mut current_cfg = None;
	let mut strings: Vec<(Option<[TokenTree; 2]>, String)> = Vec::new();

	while let Some(tok) = stream.peek() {
		match tok {
			TokenTree::Literal(l) => {
				let lit = l.to_string();

				if lit
					.strip_prefix('"')
					.and_then(|lit| lit.strip_suffix('"'))
					.is_none()
				{
					break;
				}

				let (lit, string) =
					parse_string_literal(&mut stream, previous_span, "string literal", false)?;
				previous_span = lit.span();

				// Only insert newline when there are multiple strings.
				if let Some((_, string)) = strings.last_mut() {
					string.push('\n');
				}

				if stream.peek().is_some() {
					previous_span = expect_punct(
						&mut stream,
						',',
						previous_span,
						"a `,` after string literal",
						false,
					)?
					.span();
				}

				strings.push((current_cfg.take(), string));
			}
			TokenTree::Punct(p) if p.as_char() == '#' => {
				let punct = expect_punct(&mut stream, '#', previous_span, "`#`", false).unwrap();
				let group =
					expect_group(&mut stream, Delimiter::Bracket, punct.span(), "`#[...]`")?;

				if current_cfg.is_some() {
					return Err(compile_error(
						(previous_span, group.span()),
						"multiple `cfg`s in a row not supported",
					));
				}

				expect_ident(
					group.stream().into_iter(),
					"cfg",
					group.span(),
					"`cfg`",
					false,
				)?;
				previous_span = punct.span();
				// We don't need to parse the rest.

				current_cfg = Some([punct.into(), group.into()]);
			}
			_ => break,
		}
	}

	if strings.is_empty() {
		return Err(compile_error(
			stream
				.peek()
				.map(TokenTree::span)
				.unwrap_or_else(Span::mixed_site),
			"requires at least a string argument",
		));
	};

	let mut arguments = Vec::new();
	let mut current_string = String::new();

	// Apply argument formatting.
	for (cfg, string) in strings {
		// Don't merge strings when dealing with a `cfg`.
		if cfg.is_some() && !current_string.is_empty() {
			arguments.push(Argument {
				cfg: None,
				kind: ArgumentKind::String(mem::take(&mut current_string)),
			});
		}

		let mut chars = string.chars().peekable();

		while let Some(char) = chars.next() {
			match char {
				'{' => match chars.next() {
					// Escaped `{`.
					Some('{') => current_string.push('{'),
					Some('}') => match chars.peek() {
						Some('}') => {
							return Err(compile_error(
								previous_span,
								"no corresponding closing bracers found",
							));
						}
						_ => {
							if !current_string.is_empty() {
								arguments.push(Argument {
									cfg: cfg.clone(),
									kind: ArgumentKind::String(mem::take(&mut current_string)),
								});
							}

							match stream.peek() {
								Some(_) => {
									previous_span = expect_ident(
										&mut stream,
										"interpolate",
										previous_span,
										"`interpolate`",
										false,
									)?
									.span();
									arguments.push(Argument {
										cfg: cfg.clone(),
										kind: ArgumentKind::Interpolate(
											parse_ty_or_value(stream, previous_span, "a value")?.1,
										),
									});

									if stream.peek().is_some() {
										let punct = expect_punct(
											&mut stream,
											',',
											previous_span,
											"a `,` between formatting parameters",
											false,
										)?;
										previous_span = punct.span();
									}
								}
								None => {
									return Err(compile_error(
										previous_span,
										"expected an argument for `{}`",
									));
								}
							}
						}
					},
					_ => {
						return Err(compile_error(
							previous_span,
							"no corresponding closing bracers found",
						));
					}
				},
				'}' => match chars.next() {
					// Escaped `}`.
					Some('}') => current_string.push('}'),
					_ => {
						return Err(compile_error(
							previous_span,
							"no corresponding opening bracers found",
						));
					}
				},
				c => current_string.push(c),
			}
		}

		// Don't merge strings when dealing with a `cfg`.
		if cfg.is_some() && !current_string.is_empty() {
			arguments.push(Argument {
				cfg: cfg.clone(),
				kind: ArgumentKind::String(mem::take(&mut current_string)),
			});
		}
	}

	if !current_string.is_empty() {
		arguments.push(Argument {
			cfg: None,
			kind: ArgumentKind::String(current_string),
		});
	}

	if let Some(tok) = stream.next() {
		Err(compile_error(
			tok.span(),
			"expected no tokens after string literals and formatting parameters",
		))
	} else {
		Ok(arguments)
	}
}

fn expect_meta_name_value(
	stream: &mut Peekable<token_stream::IntoIter>,
	attribute: &str,
) -> Result<String, TokenStream> {
	let (ident, string) = parse_meta_name_value(stream)?;

	#[cfg_attr(test, allow(clippy::cmp_owned))]
	if ident.to_string() != attribute {
		return Err(compile_error(
			ident.span(),
			format!("expected `{attribute}`"),
		));
	}

	Ok(string)
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
