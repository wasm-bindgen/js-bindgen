#[cfg(not(test))]
extern crate proc_macro;
#[cfg(test)]
extern crate proc_macro2 as proc_macro;

use std::fmt::Display;
use std::iter::{self, Peekable};

use proc_macro::{
	Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree, token_stream,
};

pub struct Argument {
	pub cfg: Option<[TokenTree; 2]>,
	pub kind: ArgumentKind,
}

pub enum ArgumentKind {
	Bytes(Vec<u8>),
	String(String),
	Interpolate(Vec<TokenTree>),
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
pub fn custom_section(name: &str, prefix: Option<&str>, data: &[Argument]) -> TokenStream {
	fn group(delimiter: Delimiter, inner: impl IntoIterator<Item = TokenTree>) -> TokenTree {
		Group::new(delimiter, inner.into_iter().collect()).into()
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

	fn ident(string: &str) -> TokenTree {
		Ident::new(string, Span::mixed_site()).into()
	}

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
			ArgumentKind::Bytes(bytes) => {
				// `const ARR_<index>: [u8; <argument>.len()] = <argument>;`
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
								ArgumentKind::Bytes(bytes) => {
									Literal::usize_unsuffixed(bytes.len()).into()
								}
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
						ArgumentKind::Bytes(bytes) => Literal::usize_unsuffixed(bytes.len()).into(),
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

pub fn parse_meta_name_value(
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
	previous_span: Span,
	expected: &str,
) -> Result<(SpanRange, Vec<TokenTree>), TokenStream> {
	let mut ty = Vec::new();
	let mut span = SpanRange::from(previous_span);

	if let Some(tok) = stream.peek() {
		span.start = tok.span();

		match tok {
			TokenTree::Punct(p) => {
				if p.as_char() == '&' {
					ty.push(stream.next().unwrap());
				} else if p.as_char() == '*' {
					let star = stream.next().unwrap();
					let r#const =
						expect_ident(&mut stream, "const", star.span(), "`*const`", true)?;
					span.end = r#const.span();
					ty.extend_from_slice(&[star, r#const.into()]);
				}
			}
			TokenTree::Literal(_) => return Ok((span, vec![stream.next().unwrap()])),
			_ => (),
		}
	}

	while let Some(tok) = stream.peek() {
		match tok {
			TokenTree::Ident(_) | TokenTree::Group(_) => {
				span.end = tok.span();
				ty.push(stream.next().unwrap())
			}
			TokenTree::Punct(p) if p.as_char() == '<' => {
				ty.extend(parse_angular(&mut stream, previous_span)?);
				span.end = ty.last().unwrap().span();
			}
			TokenTree::Punct(p) if [':', '.', '!'].contains(&p.as_char()) => {
				span.end = p.span();
				ty.push(stream.next().unwrap())
			}
			_ => break,
		}
	}

	if ty.is_empty() {
		Err(compile_error(span, format!("expected {expected}")))
	} else {
		Ok((span, ty))
	}
}

fn parse_angular(
	mut stream: impl Iterator<Item = TokenTree>,
	previous_span: impl Into<SpanRange>,
) -> Result<TokenStream, TokenStream> {
	let opening = expect_punct(&mut stream, '<', previous_span.into(), "`<`", false)?;
	let mut span = SpanRange::from(opening.span());
	let mut angular = TokenStream::from_iter(iter::once(TokenTree::from(opening)));

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
		Ok(angular)
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
	previous_span: Span,
	expected: &str,
) -> Result<Group, TokenStream> {
	match stream.next() {
		Some(TokenTree::Group(g)) if g.delimiter() == delimiter => Ok(g),
		Some(tok) => Err(compile_error(
			(previous_span, tok.span()),
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

	#[cfg_attr(test, allow(clippy::cmp_owned))]
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

	TokenStream::from_iter(
		path(["core", "compile_error"], span.start).chain([
			punct('!', Spacing::Alone, span.start).into(),
			group(
				Delimiter::Parenthesis,
				span.end,
				iter::once(Literal::string(&error.to_string()).into()),
			)
			.into(),
			punct(';', Spacing::Alone, span.end).into(),
		]),
	)
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
