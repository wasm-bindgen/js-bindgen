use std::fmt::Display;
use std::iter::{self, Peekable};
use std::mem;

use proc_macro::{
	Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree, token_stream,
};
#[cfg(test)]
use proc_macro2 as proc_macro;

pub struct Argument {
	pub cfg: Option<[TokenTree; 2]>,
	pub kind: ArgumentKind,
}

pub enum ArgumentKind {
	Bytes(Vec<u8>),
	Interpolate(Vec<TokenTree>),
	InterpolateWithLength(Vec<TokenTree>),
}

impl Argument {
	pub fn bytes(value: Vec<u8>) -> Self {
		Self {
			cfg: None,
			kind: ArgumentKind::Bytes(value),
		}
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
#[must_use]
pub fn custom_section(name: &str, data: &[Argument]) -> TokenStream {
	fn group(delimiter: Delimiter, inner: impl IntoIterator<Item = TokenTree>) -> TokenTree {
		Group::new(delimiter, inner.into_iter().collect()).into()
	}

	fn r#const<TY, VALUE>(
		name: &str,
		ty: TY,
		value: VALUE,
	) -> impl use<TY, VALUE> + Iterator<Item = TokenTree>
	where
		TY: IntoIterator<Item = TokenTree>,
		VALUE: IntoIterator<Item = TokenTree>,
	{
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

	fn ident(string: &str) -> TokenTree {
		Ident::new(string, Span::mixed_site()).into()
	}

	let span = Span::mixed_site();

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
	//
	// Formatting arguments with length prefix additionally get:
	// ```
	// const VAL_<index>_LEN: [u8; 2] = ::core::primitive::usize::to_le_bytes(LEN_<index>);
	// ```
	let consts = data
		.iter()
		.enumerate()
		.flat_map(|(index, arg)| match &arg.kind {
			ArgumentKind::Bytes(bytes) => {
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
								Literal::usize_unsuffixed(bytes.len()).into(),
							],
						)),
						[
							Punct::new('*', Spacing::Alone).into(),
							Literal::byte_string(bytes).into(),
						],
					))
					.collect::<Vec<_>>()
			}
			ArgumentKind::Interpolate(interpolate)
			| ArgumentKind::InterpolateWithLength(interpolate) => {
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
					.chain(
						matches!(arg.kind, ArgumentKind::InterpolateWithLength(_))
							.then(|| {
								// const VAL_<index>_LEN: [u8; 2] =
								// ::core::primitive::u16::to_le_bytes(LEN_<index> as u16);
								arg.cfg.clone().into_iter().flatten().chain(r#const(
									&format!("VAL_{index}_LEN"),
									iter::once(group(
										Delimiter::Bracket,
										[
											ident("u8"),
											Punct::new(';', Spacing::Alone).into(),
											Literal::usize_unsuffixed(2).into(),
										],
									)),
									path(["core", "primitive", "u16", "to_le_bytes"], span).chain(
										iter::once(group(
											Delimiter::Parenthesis,
											[ident(&len_name), ident("as"), ident("u16")],
										)),
									),
								))
							})
							.into_iter()
							.flatten(),
					)
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
								ArgumentKind::Bytes(bytes) => {
									Literal::usize_unsuffixed(bytes.len()).into()
								}
								ArgumentKind::Interpolate(_)
								| ArgumentKind::InterpolateWithLength(_) => ident(&format!("LEN_{index}")),
							},
							Punct::new(';', Spacing::Alone).into(),
						]
						.into_iter()
						.chain(
							matches!(par.kind, ArgumentKind::InterpolateWithLength(_))
								.then(|| {
									[
										ident("len"),
										Punct::new('+', Spacing::Joint).into(),
										Punct::new('=', Spacing::Alone).into(),
										Literal::usize_unsuffixed(2).into(),
										Punct::new(';', Spacing::Alone).into(),
									]
								})
								.into_iter()
								.flatten(),
						),
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
	.chain(data.iter().enumerate().flat_map(move |(index, arg)| {
		arg.cfg
			.clone()
			.into_iter()
			.flatten()
			.chain([
				group(
					Delimiter::Bracket,
					[
						ident("u8"),
						Punct::new(';', Spacing::Alone).into(),
						match &arg.kind {
							ArgumentKind::Bytes(bytes) => {
								Literal::usize_unsuffixed(bytes.len()).into()
							}
							ArgumentKind::Interpolate(_) => ident(&format!("LEN_{index}")),
							ArgumentKind::InterpolateWithLength(_) => {
								Literal::usize_unsuffixed(2).into()
							}
						},
					],
				),
				Punct::new(',', Spacing::Alone).into(),
			])
			.chain(
				matches!(arg.kind, ArgumentKind::InterpolateWithLength(_))
					.then(|| {
						arg.cfg.clone().into_iter().flatten().chain([
							group(
								Delimiter::Bracket,
								[
									ident("u8"),
									Punct::new(';', Spacing::Alone).into(),
									ident(&format!("LEN_{index}")),
								],
							),
							Punct::new(',', Spacing::Alone).into(),
						])
					})
					.into_iter()
					.flatten(),
			)
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
			.chain(data.iter().enumerate().flat_map(move |(index, arg)| {
				arg.cfg
					.clone()
					.into_iter()
					.flatten()
					.chain([
						if let ArgumentKind::InterpolateWithLength(_) = arg.kind {
							ident(&format!("VAL_{index}_LEN"))
						} else {
							ident(&format!("ARR_{index}"))
						},
						Punct::new(',', Spacing::Alone).into(),
					])
					.chain(
						matches!(arg.kind, ArgumentKind::InterpolateWithLength(_))
							.then(|| {
								arg.cfg.clone().into_iter().flatten().chain([
									ident(&format!("ARR_{index}")),
									Punct::new(',', Spacing::Alone).into(),
								])
							})
							.into_iter()
							.flatten(),
					)
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
			consts.chain(len).chain(r#const(
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

pub fn parse_string_arguments(
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

pub fn expect_meta_name_array(
	stream: &mut Peekable<token_stream::IntoIter>,
	attribute: &str,
) -> Result<Vec<Vec<TokenTree>>, TokenStream> {
	let (ident, string) = parse_meta_name_array(stream)?;

	#[cfg_attr(test, expect(clippy::cmp_owned, reason = "`proc-macro2` compatiblity"))]
	if ident.to_string() != attribute {
		return Err(compile_error(
			ident.span(),
			format!("expected `{attribute}`"),
		));
	}

	Ok(string)
}

fn parse_meta_name_array(
	mut stream: &mut Peekable<token_stream::IntoIter>,
) -> Result<(Ident, Vec<Vec<TokenTree>>), TokenStream> {
	let ident = parse_ident(&mut stream, Span::mixed_site(), "`<attribute> = \"...\"`")?;
	let mut span = SpanRange::from(ident.span());
	span.end = expect_punct(&mut stream, '=', span, "`<attribute> = \"...\"`", true)?.span();

	let group = expect_group(&mut stream, Delimiter::Bracket, span, "array of strings")?;
	let mut values = Vec::new();
	let mut group_stream = group.stream().into_iter().peekable();

	while group_stream.peek().is_some() {
		let mut value = Vec::new();
		span.end = parse_ty_or_value(&mut group_stream, span, "string value", &mut value)?.end;
		values.push(value);

		if group_stream.peek().is_some() {
			expect_punct(
				&mut group_stream,
				',',
				span,
				"a `,` after a string value",
				false,
			)?;
		}
	}

	if stream.peek().is_some() {
		expect_punct(
			&mut stream,
			',',
			(span.start, group.span_close()),
			"a `,` after an attribute",
			false,
		)?;
	}

	Ok((ident, values))
}

pub fn expect_meta_name_string(
	stream: &mut Peekable<token_stream::IntoIter>,
	attribute: &str,
) -> Result<String, TokenStream> {
	let (ident, string) = parse_meta_name_string(stream)?;

	#[cfg_attr(test, expect(clippy::cmp_owned, reason = "`proc-macro2` compatiblity"))]
	if ident.to_string() != attribute {
		return Err(compile_error(
			ident.span(),
			format!("expected `{attribute}`"),
		));
	}

	Ok(string)
}

fn parse_meta_name_string(
	mut stream: &mut Peekable<token_stream::IntoIter>,
) -> Result<(Ident, String), TokenStream> {
	let ident = parse_ident(&mut stream, Span::mixed_site(), "`<attribute> = \"...\"`")?;
	let mut span = SpanRange::from(ident.span());
	span.end = expect_punct(&mut stream, '=', span, "`<attribute> = \"...\"`", true)?.span();
	let (lit, string) = parse_string_literal(&mut stream, span, "`<attribute> = \"...\"`", true)?;

	if stream.peek().is_some() {
		expect_punct(
			&mut stream,
			',',
			(ident.span(), lit.span()),
			"a `,` after an attribute",
			false,
		)?;
	}

	Ok((ident, string))
}

pub fn parse_ty_or_value(
	mut stream: &mut Peekable<token_stream::IntoIter>,
	previous_span: impl Into<SpanRange>,
	expected: &str,
	out: &mut Vec<TokenTree>,
) -> Result<SpanRange, TokenStream> {
	let mut span = previous_span.into();
	let mut found = false;

	if let Some(tok) = stream.peek() {
		span.start = tok.span();

		match tok {
			TokenTree::Punct(p) => {
				if p.as_char() == '&' {
					out.push(stream.next().unwrap());
					found = true;
				} else if p.as_char() == '*' {
					let star = stream.next().unwrap();
					let r#const =
						expect_ident(&mut stream, "const", star.span(), "`*const`", true)?;
					span.end = r#const.span();
					out.extend_from_slice(&[star, r#const.into()]);
					found = true;
				}
			}
			TokenTree::Literal(_) => {
				out.push(stream.next().unwrap());
				return Ok(span);
			}
			_ => (),
		}
	}

	while let Some(tok) = stream.peek() {
		match tok {
			TokenTree::Ident(_) | TokenTree::Group(_) => {
				span.end = tok.span();
				out.push(stream.next().unwrap());
				found = true;
			}
			TokenTree::Punct(p) if p.as_char() == '<' => {
				let generic = parse_angular(&mut stream, span)?;
				out.extend(generic.1);
				found = true;
				span.end = generic.0.end;
			}
			TokenTree::Punct(p) if [':', '.', '!'].contains(&p.as_char()) => {
				span.end = p.span();
				out.push(stream.next().unwrap());
				found = true;
			}
			_ => break,
		}
	}

	if found {
		Ok(span)
	} else {
		Err(compile_error(span, format!("expected {expected}")))
	}
}

fn parse_angular(
	mut stream: impl Iterator<Item = TokenTree>,
	previous_span: impl Into<SpanRange>,
) -> Result<(SpanRange, TokenStream), TokenStream> {
	let opening = expect_punct(&mut stream, '<', previous_span.into(), "`<`", false)?;
	let mut span = SpanRange::from(opening.span());
	let mut angular: TokenStream = iter::once(TokenTree::from(opening)).collect();

	let mut opened = 1;

	for tok in &mut stream {
		span.end = tok.span();

		match &tok {
			TokenTree::Punct(p) if p.as_char() == '>' => opened -= 1,
			TokenTree::Punct(p) if p.as_char() == '<' => opened += 1,
			_ => (),
		}

		angular.extend(iter::once(tok));

		if opened == 0 {
			break;
		}
	}

	if opened == 0 {
		Ok((span, angular))
	} else {
		Err(compile_error(span, "type not completed, missing `>`"))
	}
}

pub fn parse_string_literal(
	mut stream: impl Iterator<Item = TokenTree>,
	previous_span: impl Into<SpanRange>,
	expected: &str,
	with_previous: bool,
) -> Result<(Literal, String), TokenStream> {
	if let Some(tok) = stream.next() {
		let span: SpanRange = if with_previous {
			(previous_span.into().start, tok.span()).into()
		} else {
			tok.span().into()
		};

		match tok {
			TokenTree::Literal(l) => {
				let lit = l.to_string();

				// Strip starting and ending `"`.
				let Some(stripped) = lit.strip_prefix('"').and_then(|lit| lit.strip_suffix('"'))
				else {
					return Err(compile_error(span, format!("expected {expected}")));
				};

				let mut string = String::with_capacity(stripped.len());
				let mut chars = stripped.chars();

				while let Some(char) = chars.next() {
					match char {
						'\\' => match chars.next().unwrap() {
							'"' => string.push('"'),
							'\\' => string.push('\\'),
							'n' => string.push('\n'),
							't' => string.push('\t'),
							c => {
								return Err(compile_error(
									span,
									format!("escaping `{c}` is not supported"),
								));
							}
						},
						c => string.push(c),
					}
				}

				Ok((l, string))
			}
			_ => Err(compile_error(span, format!("expected {expected}"))),
		}
	} else {
		Err(compile_error(previous_span, format!("expected {expected}")))
	}
}

pub fn parse_ident(
	mut stream: impl Iterator<Item = TokenTree>,
	previous_span: Span,
	expected: &str,
) -> Result<Ident, TokenStream> {
	match stream.next() {
		Some(TokenTree::Ident(i)) => Ok(i),
		Some(tok) => Err(compile_error(tok.span(), format!("expected {expected}"))),
		None => Err(compile_error(previous_span, format!("expected {expected}"))),
	}
}

pub fn expect_group(
	mut stream: impl Iterator<Item = TokenTree>,
	delimiter: Delimiter,
	previous_span: impl Into<SpanRange>,
	expected: &str,
) -> Result<Group, TokenStream> {
	match stream.next() {
		Some(TokenTree::Group(g)) if g.delimiter() == delimiter => Ok(g),
		Some(tok) => Err(compile_error(
			(previous_span.into().start, tok.span()),
			format!("expected {expected}"),
		)),
		None => Err(compile_error(previous_span, format!("expected {expected}"))),
	}
}

pub fn expect_ident(
	stream: impl Iterator<Item = TokenTree>,
	ident: &str,
	previous_span: Span,
	expected: &str,
	with_previous: bool,
) -> Result<Ident, TokenStream> {
	let i = parse_ident(stream, previous_span, expected)?;

	#[cfg_attr(test, expect(clippy::cmp_owned, reason = "`proc-macro2` compatiblity"))]
	if i.to_string() == ident {
		Ok(i)
	} else {
		let span: SpanRange = if with_previous {
			(previous_span, i.span()).into()
		} else {
			i.span().into()
		};

		Err(compile_error(span, format!("expected {expected}")))
	}
}

pub fn expect_punct(
	mut stream: impl Iterator<Item = TokenTree>,
	char: char,
	previous_span: impl Into<SpanRange>,
	expected: &str,
	with_previous: bool,
) -> Result<Punct, TokenStream> {
	match stream.next() {
		Some(TokenTree::Punct(p)) if p.as_char() == char => Ok(p),
		Some(tok) => {
			let span: SpanRange = if with_previous {
				(previous_span.into().start, tok.span()).into()
			} else {
				tok.span().into()
			};
			Err(compile_error(span, format!("expected {expected}")))
		}
		None => Err(compile_error(previous_span, format!("expected {expected}"))),
	}
}

pub fn path(
	parts: impl IntoIterator<Item = &'static str>,
	span: impl Into<SpanRange>,
) -> impl Iterator<Item = TokenTree> {
	let span = span.into();

	parts.into_iter().flat_map(move |p| {
		[
			TokenTree::from(punct(':', Spacing::Joint, span.start)),
			punct(':', Spacing::Alone, span.start).into(),
			if let Some(p) = p.strip_prefix("r#") {
				Ident::new_raw(p, span.end).into()
			} else {
				Ident::new(p, span.end).into()
			},
		]
	})
}

#[derive(Clone, Copy)]
pub struct SpanRange {
	pub start: Span,
	pub end: Span,
}

impl From<(Span, Span)> for SpanRange {
	fn from((start, end): (Span, Span)) -> Self {
		Self { start, end }
	}
}

impl From<Span> for SpanRange {
	fn from(span: Span) -> Self {
		Self {
			start: span,
			end: span,
		}
	}
}

/// ```"not rust"
/// ::core::compile_error!(error);
/// ```
pub fn compile_error(span: impl Into<SpanRange>, error: impl Display) -> TokenStream {
	let span = span.into();

	path(["core", "compile_error"], span.start)
		.chain([
			punct('!', Spacing::Alone, span.start).into(),
			group(
				Delimiter::Parenthesis,
				span.end,
				iter::once(Literal::string(&error.to_string()).into()),
			)
			.into(),
			punct(';', Spacing::Alone, span.end).into(),
		])
		.collect::<TokenStream>()
}

fn punct(ch: char, spacing: Spacing, span: Span) -> Punct {
	let mut p = Punct::new(ch, spacing);
	p.set_span(span);
	p
}

fn group(delimiter: Delimiter, span: Span, stream: impl Iterator<Item = TokenTree>) -> Group {
	let mut g = Group::new(delimiter, stream.collect());
	g.set_span(span);
	g
}
