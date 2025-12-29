#[cfg(not(test))]
extern crate proc_macro;
#[cfg(test)]
extern crate proc_macro2 as proc_macro;

use std::fmt::Display;
use std::iter::{self, Peekable};

use proc_macro::{
	token_stream, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

pub fn parse_ty_or_value(
	mut stream: &mut Peekable<token_stream::IntoIter>,
	previous_span: Span,
) -> Result<Vec<TokenTree>, TokenStream> {
	let mut ty = Vec::new();

	if let Some(TokenTree::Punct(p)) = stream.peek() {
		if p.as_char() == '&' {
			ty.push(stream.next().unwrap());
		} else if p.as_char() == '*' {
			ty.push(stream.next().unwrap());
			ty.push(expect_ident(&mut stream, "const", previous_span, "`*const`")?.into())
		}
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
			_ => break,
		}
	}

	if ty.is_empty() {
		Err(compile_error(previous_span, "expected type"))
	} else {
		Ok(ty)
	}
}

fn parse_angular(
	mut stream: impl Iterator<Item = TokenTree>,
	previous_span: Span,
) -> Result<TokenStream, TokenStream> {
	let opening = expect_punct(&mut stream, '<', previous_span, "`<`")?;
	let span = opening.span();
	let mut angular = TokenStream::from_iter(iter::once(TokenTree::from(opening)));

	let mut opened = 1;

	for tok in &mut stream {
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
	previous_span: Span,
	string: &mut String,
) -> Result<Literal, TokenStream> {
	match stream.next() {
		Some(TokenTree::Literal(l)) => {
			let span = l.span();
			let lit = l.to_string();

			// Strip starting and ending `"`.
			let Some(stripped) = lit.strip_prefix('"').and_then(|lit| lit.strip_suffix('"')) else {
				return Err(compile_error(span, "expected a string literal"));
			};

			string.reserve(stripped.len());
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
							))
						}
					},
					c => string.push(c),
				}
			}

			Ok(l)
		}
		Some(tok) => Err(compile_error(tok.span(), "expected a string literal")),
		None => Err(compile_error(previous_span, "expected a string literal`")),
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

pub fn expect_ident(
	stream: impl Iterator<Item = TokenTree>,
	ident: &str,
	previous_span: Span,
	expected: &str,
) -> Result<Ident, TokenStream> {
	let i = parse_ident(stream, previous_span, expected)?;

	#[cfg_attr(test, allow(clippy::cmp_owned))]
	if i.to_string() == ident {
		Ok(i)
	} else {
		Err(compile_error(previous_span, format!("expected {expected}")))
	}
}

pub fn expect_punct(
	mut stream: impl Iterator<Item = TokenTree>,
	char: char,
	previous_span: Span,
	expected: &str,
) -> Result<Punct, TokenStream> {
	match stream.next() {
		Some(TokenTree::Punct(p)) if p.as_char() == char => Ok(p),
		Some(tok) => Err(compile_error(tok.span(), format!("expected {expected}"))),
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

/// ```"not rust"
/// ::core::compile_error!(error);
/// ```
pub fn compile_error<E: Display>(span: Span, error: E) -> TokenStream {
	TokenStream::from_iter(
		path(["core", "compile_error"], span).chain([
			punct('!', Spacing::Alone, span).into(),
			group(
				Delimiter::Parenthesis,
				span,
				iter::once(Literal::string(&error.to_string()).into()),
			)
			.into(),
			punct(';', Spacing::Alone, span).into(),
		]),
	)
}

pub fn punct(ch: char, spacing: Spacing, span: Span) -> Punct {
	let mut p = Punct::new(ch, spacing);
	p.set_span(span);
	p
}

pub fn group(delimiter: Delimiter, span: Span, stream: impl Iterator<Item = TokenTree>) -> Group {
	let mut g = Group::new(delimiter, stream.collect());
	g.set_span(span);
	g
}
