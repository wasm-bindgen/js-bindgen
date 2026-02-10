#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

#[cfg(test)]
mod tests;

#[cfg(not(test))]
use std::env;
use std::iter::Peekable;
use std::mem;

use js_bindgen_macro_shared::*;
use proc_macro2::{Delimiter, Span, TokenStream, TokenTree, token_stream};

#[proc_macro]
pub fn unsafe_embed_asm(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
pub fn embed_js(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	embed_js_internal(input.into()).unwrap_or_else(|e| e).into()
}

fn embed_js_internal(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();

	let package = package();
	let name = expect_meta_name_value(&mut input, "name")?;

	let mut data = Vec::new();

	let embed = if let Some(TokenTree::Ident(_)) = input.peek() {
		expect_meta_name_value(&mut input, "js_embed")?
	} else {
		String::new()
	};

	let mut embed_data = Vec::new();
	embed_data.extend_from_slice(
		&u16::try_from(embed.len())
			.expect("`js_embed` name too long")
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
pub fn import_js(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	import_js_internal(input.into())
		.unwrap_or_else(|e| e)
		.into()
}

fn import_js_internal(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();

	let package = package();
	let import_name = expect_meta_name_value(&mut input, "name")?;

	let attr = if let Some(TokenTree::Ident(ident)) = input.peek() {
		let ident = ident.to_string();

		if ident == "required_embed" {
			let required_embed = expect_meta_name_value(&mut input, "required_embed")?;
			let len = u16::try_from(required_embed.len())
				.expect("`required_embed` name too long")
				.to_le_bytes();
			[[2].as_slice(), &len, required_embed.as_bytes()].concat()
		} else {
			let ident = expect_ident(
				&mut input,
				"no_import",
				Span::mixed_site(),
				"`required_embed` or `no_import`",
				false,
			)?;

			if input.peek().is_some() {
				expect_punct(
					&mut input,
					',',
					ident.span(),
					"a `,` after an attribute",
					false,
				)?;
			}

			if let Some(token) = input.next() {
				return Err(compile_error(
					token.span(),
					"`no_import` requires no string tokens",
				));
			}

			return Ok(custom_section(
				&format!("js_bindgen.import.{package}.{import_name}"),
				&[Argument {
					cfg: None,
					kind: ArgumentKind::Bytes(vec![1]),
				}],
			));
		}
	} else {
		vec![0]
	};

	let mut data = vec![Argument {
		cfg: None,
		kind: ArgumentKind::Bytes(attr),
	}];
	parse_string_arguments(&mut input, Span::mixed_site(), &mut data)?;
	let output = custom_section(&format!("js_bindgen.import.{package}.{import_name}"), &data);

	Ok(output)
}

fn parse_string_arguments(
	mut stream: &mut Peekable<token_stream::IntoIter>,
	mut previous_span: Span,
	arguments: &mut Vec<Argument>,
) -> Result<(), TokenStream> {
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
			stream.peek().map_or_else(Span::mixed_site, TokenTree::span),
			"requires at least a string argument",
		));
	}

	let mut current_string = String::new();

	// Apply argument formatting.
	for (cfg, string) in strings {
		// Don't merge strings when dealing with a `cfg`.
		if cfg.is_some() && !current_string.is_empty() {
			arguments.push(Argument {
				cfg: None,
				kind: ArgumentKind::Bytes(mem::take(&mut current_string).into_bytes()),
			});
		}

		let mut chars = string.chars().peekable();

		while let Some(char) = chars.next() {
			match char {
				'{' => match chars.next() {
					// Escaped `{`.
					Some('{') => current_string.push('{'),
					Some('}') => {
						if let Some('}') = chars.peek() {
							return Err(compile_error(
								previous_span,
								"no corresponding closing bracers found",
							));
						}
						if !current_string.is_empty() {
							arguments.push(Argument {
								cfg: cfg.clone(),
								kind: ArgumentKind::Bytes(
									mem::take(&mut current_string).into_bytes(),
								),
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
								let mut interpolate = Vec::new();
								parse_ty_or_value(
									stream,
									previous_span,
									"a value",
									&mut interpolate,
								)?;
								arguments.push(Argument {
									cfg: cfg.clone(),
									kind: ArgumentKind::Interpolate(interpolate),
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
				kind: ArgumentKind::Bytes(mem::take(&mut current_string).into_bytes()),
			});
		}
	}

	if !current_string.is_empty() {
		arguments.push(Argument {
			cfg: None,
			kind: ArgumentKind::Bytes(current_string.into_bytes()),
		});
	}

	if let Some(tok) = stream.next() {
		Err(compile_error(
			tok.span(),
			"expected no tokens after string literals and formatting parameters",
		))
	} else {
		Ok(())
	}
}

fn expect_meta_name_value(
	stream: &mut Peekable<token_stream::IntoIter>,
	attribute: &str,
) -> Result<String, TokenStream> {
	let (ident, string) = parse_meta_name_value(stream)?;

	if ident != attribute {
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
