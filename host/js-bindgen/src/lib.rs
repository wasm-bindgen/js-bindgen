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
use std::{iter, mem};

use js_bindgen_macro_shared::*;
use proc_macro::{
	Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree, token_stream,
};

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

	Ok(custom_section(
		&format!("js_bindgen.import.{package}.{import_name}"),
		Some(required_embed.as_deref().unwrap_or("")),
		&parse_string_arguments(&mut input, Span::mixed_site())?,
	))
}

struct Argument {
	cfg: Option<[TokenTree; 2]>,
	kind: ArgumentKind,
}

enum ArgumentKind {
	String(String),
	Interpolate(Vec<TokenTree>),
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

/// ```"not rust"
/// const _: () = {
/// 	const LEN: u32 = {
/// 		let mut len: usize = 0;
/// 		#(len += LEN_<index>;)*
/// 		len as u32
/// 	};
///
/// 	const _: () = {
/// 		#[repr(C)]
/// 		struct Layout([u8; 4], #([u8; LEN_<index>]),*);
///
/// 		#[link_section = name]
/// 		static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), #(ARR_<index>),*);
/// 	};
/// };
/// ```
fn custom_section(name: &str, prefix: Option<&str>, data: &[Argument]) -> TokenStream {
	let span = Span::mixed_site();

	// If a prefix is present:
	// `const ARR_PREFIX: [u8; <prefix>.len()] = *<prefix>;`
	let const_prefix = prefix
		.into_iter()
		.filter(|prefix| !prefix.is_empty())
		.flat_map(|prefix| {
			r#const(
				"ARR_PREFIX",
				iter::once(group(
					Delimiter::Bracket,
					[
						ident("u8"),
						Punct::new(';', Spacing::Alone).into(),
						Literal::usize_unsuffixed(prefix.len()).into(),
					],
				)),
				[
					Punct::new('*', Spacing::Alone).into(),
					Literal::byte_string(prefix.as_bytes()).into(),
				],
			)
		});

	// For every string we insert:
	// ```
	// const ARR_<index>: [u8; <argument>.len()] = *<argument>;
	// ```
	//
	// For every formatting argument we insert:
	// ```
	// const VAL_<index>: &str = <argument>;
	// const LEN_<index>: usize = ::core::primitive::str::len(VAL_<index>);
	// const PTR_<index>: *const u8 = ::core::primitive::str::as_ptr(VAL_<index>);
	// const ARR_<index>: [u8; LEN_<index>] = unsafe { *(PTR_<index> as *const _) };
	// ```
	let consts = data
		.iter()
		.enumerate()
		.flat_map(|(index, arg)| match &arg.kind {
			ArgumentKind::String(string) => {
				// `const ARR_<index>: [u8; <argument>.len()] = *<argument>;`
				arg.cfg
					.clone()
					.into_iter()
					.flatten()
					.chain(r#const(
						&format!("ARR_{index}"),
						iter::once(group(
							Delimiter::Bracket,
							[
								ident("u8"),
								Punct::new(';', Spacing::Alone).into(),
								Literal::usize_unsuffixed(string.len()).into(),
							],
						)),
						[
							Punct::new('*', Spacing::Alone).into(),
							Literal::byte_string(string.as_bytes()).into(),
						],
					))
					.collect::<Vec<_>>()
			}
			ArgumentKind::Interpolate(interpolate) => {
				let value_name = format!("VAL_{index}");
				let value = ident(&value_name);
				let len_name = format!("LEN_{index}");
				let ptr_name = format!("PTR_{index}");

				// `const VAL_<index>: &str = <argument>;`
				arg.cfg
					.clone()
					.into_iter()
					.flatten()
					.chain(r#const(
						&value_name,
						[Punct::new('&', Spacing::Alone).into(), ident("str")],
						interpolate.iter().cloned(),
					))
					// `const LEN_<index>: usize = ::core::primitive::str::len(VAL_<index>);`
					.chain(arg.cfg.clone().into_iter().flatten())
					.chain(r#const(
						&len_name,
						iter::once(ident("usize")),
						path(["core", "primitive", "str", "len"], span).chain(iter::once(group(
							Delimiter::Parenthesis,
							iter::once(value.clone()),
						))),
					))
					// `const PTR_<index>: *const u8 = ::core::primitive::str::as_ptr(VAL_<index>);`
					.chain(arg.cfg.clone().into_iter().flatten())
					.chain(r#const(
						&ptr_name,
						[
							Punct::new('*', Spacing::Alone).into(),
							ident("const"),
							ident("u8"),
						],
						path(["core", "primitive", "str", "as_ptr"], span)
							.chain(iter::once(group(Delimiter::Parenthesis, iter::once(value)))),
					))
					// `const ARR_<index>: [u8; LEN_<index>] = unsafe { *(PTR_<index> as *const _)
					// };`
					.chain(arg.cfg.clone().into_iter().flatten())
					.chain(r#const(
						&format!("ARR_{index}"),
						iter::once(group(
							Delimiter::Bracket,
							[
								ident("u8"),
								Punct::new(';', Spacing::Alone).into(),
								ident(&len_name),
							],
						)),
						[
							ident("unsafe"),
							group(
								Delimiter::Brace,
								[
									Punct::new('*', Spacing::Alone).into(),
									group(
										Delimiter::Parenthesis,
										[
											ident(&ptr_name),
											ident("as"),
											Punct::new('*', Spacing::Alone).into(),
											ident("const"),
											ident("_"),
										],
									),
								],
							),
						],
					))
					.collect::<Vec<_>>()
			}
		});

	// ```
	// const LEN: u32 = {
	//     let mut len: usize = 0;
	//     #(len += LEN_<index>;)*
	//     len as u32
	// };
	// ```
	let len = r#const(
		"LEN",
		iter::once(ident("u32")),
		[group(
			Delimiter::Brace,
			[
				ident("let"),
				ident("mut"),
				ident("len"),
				Punct::new(':', Spacing::Alone).into(),
				ident("usize"),
				Punct::new('=', Spacing::Alone).into(),
				Literal::usize_unsuffixed(0).into(),
				Punct::new(';', Spacing::Alone).into(),
			]
			.into_iter()
			.chain(data.iter().enumerate().flat_map(|(index, par)| {
				par.cfg
					.clone()
					.into_iter()
					.flatten()
					.chain(iter::once(group(
						Delimiter::Brace,
						[
							ident("len"),
							Punct::new('+', Spacing::Joint).into(),
							Punct::new('=', Spacing::Alone).into(),
							match &par.kind {
								ArgumentKind::String(string) => {
									Literal::usize_unsuffixed(string.len()).into()
								}
								ArgumentKind::Interpolate(_) => ident(&format!("LEN_{index}")),
							},
							Punct::new(';', Spacing::Alone).into(),
						],
					)))
			}))
			.chain([ident("len"), ident("as"), ident("u32")]),
		)],
	);

	// `[u8; 4], #([u8; LEN_<index>]),*`
	let tys = [
		group(
			Delimiter::Bracket,
			[
				ident("u8"),
				Punct::new(';', Spacing::Alone).into(),
				Literal::usize_unsuffixed(4).into(),
			],
		),
		Punct::new(',', Spacing::Alone).into(),
	]
	.into_iter()
	// Optional prefix length.
	.chain(prefix.into_iter().flat_map(|prefix| {
		[
			group(
				Delimiter::Bracket,
				[
					ident("u8"),
					Punct::new(';', Spacing::Alone).into(),
					Literal::usize_unsuffixed(2).into(),
				],
			),
			Punct::new(',', Spacing::Alone).into(),
		]
		.into_iter()
		.chain(
			(!prefix.is_empty())
				.then(|| {
					[
						group(
							Delimiter::Bracket,
							[
								ident("u8"),
								Punct::new(';', Spacing::Alone).into(),
								Literal::usize_unsuffixed(prefix.len()).into(),
							],
						),
						Punct::new(',', Spacing::Alone).into(),
					]
				})
				.into_iter()
				.flatten(),
		)
	}))
	.chain(data.iter().enumerate().flat_map(move |(index, arg)| {
		arg.cfg.clone().into_iter().flatten().chain([
			group(
				Delimiter::Bracket,
				[
					ident("u8"),
					Punct::new(';', Spacing::Alone).into(),
					match &arg.kind {
						ArgumentKind::String(string) => {
							Literal::usize_unsuffixed(string.len()).into()
						}
						ArgumentKind::Interpolate(_) => ident(&format!("LEN_{index}")),
					},
				],
			),
			Punct::new(',', Spacing::Alone).into(),
		])
	}));

	// ```
	// #[repr(C)]
	// struct Layout(...);
	// ```
	let layout = [
		Punct::new('#', Spacing::Alone).into(),
		group(
			Delimiter::Bracket,
			[
				ident("repr"),
				group(Delimiter::Parenthesis, iter::once(ident("C"))),
			],
		),
		ident("struct"),
		ident("Layout"),
		group(Delimiter::Parenthesis, tys),
		Punct::new(';', Spacing::Alone).into(),
	];

	// `#[link_section = name]`
	let link_section = [
		Punct::new('#', Spacing::Alone).into(),
		group(
			Delimiter::Bracket,
			[
				ident("unsafe"),
				group(
					Delimiter::Parenthesis,
					[
						ident("link_section"),
						Punct::new('=', Spacing::Alone).into(),
						Literal::string(name).into(),
					],
				),
			],
		),
	];

	// (::core::primitive::u32::to_le_bytes(LEN), #(ARR_<index>),*)
	let values = group(
		Delimiter::Parenthesis,
		path(["core", "primitive", "u32", "to_le_bytes"], span)
			.chain([
				group(Delimiter::Parenthesis, iter::once(ident("LEN"))),
				Punct::new(',', Spacing::Alone).into(),
			])
			// Optional prefix value.
			.chain(prefix.into_iter().flat_map(|prefix| {
				let len = u16::try_from(prefix.len()).unwrap().to_le_bytes();

				[
					group(
						Delimiter::Bracket,
						[
							Literal::u8_unsuffixed(len[0]).into(),
							Punct::new(',', Spacing::Alone).into(),
							Literal::u8_unsuffixed(len[1]).into(),
							Punct::new(',', Spacing::Alone).into(),
						],
					),
					Punct::new(',', Spacing::Alone).into(),
				]
				.into_iter()
				.chain(
					(!prefix.is_empty())
						.then(|| [ident("ARR_PREFIX"), Punct::new(',', Spacing::Alone).into()])
						.into_iter()
						.flatten(),
				)
			}))
			.chain(data.iter().enumerate().flat_map(move |(index, arg)| {
				arg.cfg.clone().into_iter().flatten().chain([
					ident(&format!("ARR_{index}")),
					Punct::new(',', Spacing::Alone).into(),
				])
			})),
	);

	// `static CUSTOM_SECTION: Layout = Layout(...);`
	let custom_section = [
		ident("static"),
		ident("CUSTOM_SECTION"),
		Punct::new(':', Spacing::Alone).into(),
		ident("Layout"),
		Punct::new('=', Spacing::Alone).into(),
		ident("Layout"),
		values,
		Punct::new(';', Spacing::Alone).into(),
	];

	// `const _: () = { ... }`
	r#const(
		"_",
		iter::once(group(Delimiter::Parenthesis, iter::empty())),
		iter::once(group(
			Delimiter::Brace,
			const_prefix.chain(consts).chain(len).chain(r#const(
				"_",
				iter::once(group(Delimiter::Parenthesis, iter::empty())),
				iter::once(group(
					Delimiter::Brace,
					layout.into_iter().chain(link_section).chain(custom_section),
				)),
			)),
		)),
	)
	.collect()
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

fn r#const(
	name: &str,
	ty: impl IntoIterator<Item = TokenTree>,
	value: impl IntoIterator<Item = TokenTree>,
) -> impl Iterator<Item = TokenTree> {
	[
		ident("const"),
		ident(name),
		Punct::new(':', Spacing::Alone).into(),
	]
	.into_iter()
	.chain(ty)
	.chain(iter::once(Punct::new('=', Spacing::Alone).into()))
	.chain(value)
	.chain(iter::once(Punct::new(';', Spacing::Alone).into()))
}

fn group(delimiter: Delimiter, inner: impl IntoIterator<Item = TokenTree>) -> TokenTree {
	Group::new(delimiter, inner.into_iter().collect()).into()
}

fn ident(string: &str) -> TokenTree {
	Ident::new(string, Span::mixed_site()).into()
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
