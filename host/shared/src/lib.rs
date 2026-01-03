#[cfg(not(test))]
extern crate proc_macro;
#[cfg(test)]
extern crate proc_macro2 as proc_macro;

use std::fmt::Display;
use std::iter::{self, Peekable};

use proc_macro::{
	Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree, token_stream,
};

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
) -> Result<Vec<TokenTree>, TokenStream> {
	let mut ty = Vec::new();
	let mut span = SpanRange::from(previous_span);

	match stream.peek() {
		Some(TokenTree::Punct(p)) => {
			if p.as_char() == '&' {
				ty.push(stream.next().unwrap());
			} else if p.as_char() == '*' {
				let star = stream.next().unwrap();
				let r#const = expect_ident(&mut stream, "const", star.span(), "`*const`", true)?;
				ty.extend_from_slice(&[star, r#const.into()]);
			}
		}
		Some(TokenTree::Literal(_)) => return Ok(vec![stream.next().unwrap()]),
		_ => (),
	}

	while let Some(tok) = stream.peek() {
		match tok {
			TokenTree::Ident(_) | TokenTree::Group(_) => ty.push(stream.next().unwrap()),
			TokenTree::Punct(p) if p.as_char() == '<' => {
				ty.extend(parse_angular(&mut stream, previous_span)?)
			}
			TokenTree::Punct(p) if [':', '.', '!'].contains(&p.as_char()) => {
				ty.push(stream.next().unwrap())
			}
			tok => {
				span = tok.span().into();
				break;
			}
		}
	}

	if ty.is_empty() {
		Err(compile_error(span, format!("expected {expected}")))
	} else {
		Ok(ty)
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
	span: Span,
) -> impl Iterator<Item = TokenTree> {
	parts.into_iter().flat_map(move |p| {
		[
			TokenTree::from(punct(':', Spacing::Joint, span)),
			punct(':', Spacing::Alone, span).into(),
			Ident::new(p, span).into(),
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
