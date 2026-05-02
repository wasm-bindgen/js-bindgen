use std::collections::VecDeque;
use std::fmt::Display;
use std::iter::{self, Peekable};
use std::str::FromStr;
use std::{mem, panic};

use proc_macro::{
	Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree, token_stream,
};
#[cfg(test)]
use proc_macro2 as proc_macro;

use crate::custom_section::CustomSection;

pub fn parse_string_arguments(
	mut stream: &mut Peekable<token_stream::IntoIter>,
	mut previous_span: Span,
	custom_section: &mut CustomSection,
) -> Result<(), TokenStream> {
	struct NamedArgument {
		used: bool,
		cfg: Option<[TokenTree; 2]>,
		name: String,
		kind: ArgKind,
		span: SpanRange,
	}

	enum ArgKind {
		Const(Vec<TokenTree>),
		Interpolate(Vec<TokenTree>),
	}

	let mut cfg = None;
	let mut strings: Vec<(Option<[TokenTree; 2]>, String, Span)> = Vec::new();

	while let Some(mut tok) = stream.peek() {
		if let TokenTree::Punct(p) = tok
			&& p.as_char() == '#'
		{
			let [punct, group] = parse_cfg(&mut stream, previous_span)?;

			previous_span = punct.span();

			if let Some(TokenTree::Punct(p)) = stream.peek()
				&& p.as_char() == '#'
			{
				let [_, group] = parse_cfg(&mut stream, previous_span)?;

				return Err(compile_error(
					(previous_span, group.span()),
					"multiple `cfg`s in a row not supported",
				));
			}

			tok = stream.peek().ok_or_else(|| {
				compile_error((punct.span(), group.span()), "leftover `cfg` attribute")
			})?;
			cfg = Some([punct, group]);
		}

		if let TokenTree::Literal(l) = tok {
			let lit = l.to_string();

			let Some(stripped) = lit.strip_prefix('"').and_then(|lit| lit.strip_suffix('"')) else {
				break;
			};
			previous_span = l.span();
			stream.next();

			let string = parse_inner_string(stripped, previous_span)?;

			// Only insert newline when there are multiple strings.
			if let Some((_, string, _)) = strings.last_mut() {
				string.push('\n');
			}

			strings.push((cfg.take(), string, previous_span));

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
		} else {
			break;
		}
	}

	if strings.is_empty() {
		return Err(compile_error(
			stream.peek().map_or_else(Span::mixed_site, TokenTree::span),
			"requires at least a string template",
		));
	}

	let mut named_arguments: Vec<NamedArgument> = Vec::new();
	let mut unnamed_arguments: VecDeque<(ArgKind, SpanRange)> = VecDeque::new();

	while let Some(tok) = stream.peek() {
		if let TokenTree::Punct(p) = tok
			&& p.as_char() == '#'
		{
			let [punct, group] = parse_cfg(&mut stream, previous_span)?;
			previous_span = punct.span();

			if let Some(TokenTree::Punct(p)) = stream.peek()
				&& p.as_char() == '#'
			{
				let [_, group] = parse_cfg(&mut stream, previous_span)?;

				return Err(compile_error(
					(previous_span, group.span()),
					"multiple `cfg`s in a row not supported",
				));
			}

			if stream.peek().is_none() {
				return Err(compile_error(
					(punct.span(), group.span()),
					"leftover `cfg` attribute",
				));
			}

			cfg = Some([punct, group]);
		}

		let mut operator = parse_ident(
			&mut stream,
			previous_span,
			"named argument, `const` or `interpolate`",
		)?;
		previous_span = operator.span();

		// TODO: `a == b` is an expression, not a named argument.
		let named = if let Some(TokenTree::Punct(p)) = stream.peek()
			&& p.as_char() == '='
		{
			if !unnamed_arguments.is_empty() {
				return Err(compile_error(
					previous_span,
					"named argument must come first",
				));
			}

			previous_span = expect_punct(&mut stream, '=', previous_span, "", false)
				.unwrap()
				.span();
			let operation = parse_ident(&mut stream, previous_span, "`const` or `interpolate`")?;
			previous_span = operation.span();
			let named = mem::replace(&mut operator, operation);

			Some(named)
		} else {
			None
		};

		let (expr, expr_span) = parse_ty_or_value(stream, previous_span, "a value")?;

		let kind = match operator.to_string().as_str() {
			"const" => ArgKind::Const(expr),
			"interpolate" => ArgKind::Interpolate(expr),
			_ => {
				return Err(compile_error(
					operator.span(),
					"expected `const` or `interpolate`",
				));
			}
		};

		if let Some(named) = named {
			let name = named.to_string();

			if named_arguments.iter().any(|arg| {
				arg.name == name && {
					let a = arg
						.cfg
						.clone()
						.map(|[_, group]| TokenStream::from(group).to_string());
					let b = cfg
						.clone()
						.map(|[_, group]| TokenStream::from(group).to_string());

					a == b
				}
			}) {
				return Err(compile_error(
					named.span(),
					"found duplicate named argument",
				));
			}

			named_arguments.push(NamedArgument {
				used: false,
				cfg: cfg.take(),
				name,
				kind,
				span: (named.span(), expr_span.end).into(),
			});
		} else {
			if let Some([punct, group]) = cfg {
				return Err(compile_error(
					(punct.span(), group.span()),
					"`cfg` attributes are only supported on named arguments",
				));
			}

			unnamed_arguments.push_back((kind, (operator.span(), expr_span.end).into()));
		}

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

	let mut current_string = String::new();

	// Apply argument formatting.
	for (cfg, string, span) in strings {
		// Don't merge strings when dealing with a `cfg`.
		if cfg.is_some() && !current_string.is_empty() {
			custom_section.string_value(None, &mut current_string);
		}

		let mut chars = string.chars().peekable();

		while let Some(char) = chars.next() {
			match char {
				'{' => match chars.next() {
					// Escaped `{`.
					Some('{') => current_string.push('{'),
					Some(char) => {
						let mut name = String::new();

						for char in iter::once(char).chain(chars.by_ref()) {
							if char == '}' {
								break;
							}

							name.push(char);
						}

						if let Some('}') = chars.peek() {
							return Err(compile_error(
								span,
								"no corresponding closing bracers found",
							));
						}

						let name = if name.is_empty() {
							None
						} else {
							let mut stream = token_stream_from_str(
								&name,
								"invalid template string named argument identifier",
								span,
							)?
							.into_iter();

							if let Some(TokenTree::Ident(_)) = stream.next()
								&& stream.next().is_none()
							{
								Some(name)
							} else {
								return Err(compile_error(
									span,
									"invalid template string named argument identifier",
								));
							}
						};

						if !current_string.is_empty() {
							custom_section.string_value(cfg.clone(), &mut current_string);
						}

						if let Some(name) = name {
							let mut found_any = false;

							for arg in named_arguments.iter_mut().filter(|arg| arg.name == name) {
								found_any = true;
								arg.used = true;
							}

							if !found_any {
								return Err(compile_error(
									span,
									format!("expected a named argument for `{name}`"),
								));
							}

							custom_section.named_value(cfg.clone(), name.clone());
						} else if let Some((kind, _)) = unnamed_arguments.pop_front() {
							match kind {
								ArgKind::Const(expr) => {
									custom_section.const_value(cfg.clone(), expr);
								}
								ArgKind::Interpolate(expr) => {
									custom_section.interpolate_value(cfg.clone(), expr);
								}
							}
						} else {
							return Err(compile_error(span, "expected an argument for `{}`"));
						}
					}
					_ => {
						return Err(compile_error(
							span,
							"no corresponding closing bracers found",
						));
					}
				},
				'}' => match chars.next() {
					// Escaped `}`.
					Some('}') => current_string.push('}'),
					_ => {
						return Err(compile_error(
							span,
							"no corresponding opening bracers found",
						));
					}
				},
				c => current_string.push(c),
			}
		}

		// Don't merge strings when dealing with a `cfg`.
		if cfg.is_some() && !current_string.is_empty() {
			custom_section.string_value(cfg, &mut current_string);
		}
	}

	if !current_string.is_empty() {
		custom_section.string_value(None, &mut current_string);
	}

	if let Some(span) = named_arguments
		.iter()
		.find_map(|arg| (!arg.used).then_some(arg.span))
		.or_else(|| unnamed_arguments.front().map(|(_, span)| *span))
	{
		return Err(compile_error(span, "expected no leftover arguments"));
	}

	for arg in named_arguments {
		match arg.kind {
			ArgKind::Const(expr) => custom_section.named_const(arg.name, arg.cfg, expr),
			ArgKind::Interpolate(expr) => {
				custom_section.named_interpolate(arg.name, arg.cfg, expr);
			}
		}
	}

	Ok(())
}

pub fn parse_inner_string(
	stripped: &str,
	previous_span: impl Into<SpanRange>,
) -> Result<String, TokenStream> {
	let mut string = String::with_capacity(stripped.len());
	let mut chars = stripped.chars();

	while let Some(char) = chars.next() {
		match char {
			'\\' => match chars.next().unwrap() {
				'"' => string.push('"'),
				'\\' => string.push('\\'),
				'n' => string.push('\n'),
				't' => string.push('\t'),
				'\n' => (),
				c => {
					return Err(compile_error(
						previous_span,
						format!("escaping `{c}` is not supported"),
					));
				}
			},
			c => string.push(c),
		}
	}

	Ok(string)
}

pub struct RequiredEmbed {
	pub cfg: Option<[TokenTree; 2]>,
	pub expr: Vec<TokenTree>,
}

pub fn expect_meta_name_required_embeds(
	mut stream: &mut Peekable<token_stream::IntoIter>,
) -> Result<Vec<RequiredEmbed>, TokenStream> {
	let ident = expect_ident(
		&mut stream,
		"required_embeds",
		Span::mixed_site(),
		"`required_embeds`",
	)?;
	let mut span = SpanRange::from(ident.span());
	span.end = expect_punct(&mut stream, '=', span, "`required_embeds =`", true)?.span();

	let array = expect_group(
		&mut stream,
		Delimiter::Bracket,
		span,
		"array of string pairs",
		false,
	)?;
	let mut values = Vec::new();
	let mut array_stream = array.stream().into_iter().peekable();

	while let Some(mut tok) = array_stream.peek() {
		let mut cfg = None;

		if let TokenTree::Punct(p) = tok
			&& p.as_char() == '#'
		{
			let [punct, group] = parse_cfg(&mut array_stream, span)?;
			tok = array_stream.peek().ok_or_else(|| {
				compile_error((punct.span(), group.span()), "leftover `cfg` attribute")
			})?;
			cfg = Some([punct, group]);

			if let TokenTree::Punct(p) = tok
				&& p.as_char() == '#'
			{
				let [punct, group] = parse_cfg(&mut array_stream, span)?;

				return Err(compile_error(
					(punct.span(), group.span()),
					"multiple `cfg`s in a row not supported",
				));
			}
		}

		let (expr, expr_span) = parse_ty_or_value(&mut array_stream, span, "string value")?;

		values.push(RequiredEmbed { cfg, expr });

		if array_stream.peek().is_some() {
			span.end = expect_punct(
				&mut array_stream,
				',',
				expr_span,
				"a `,` after a tuple",
				false,
			)?
			.span();
		}
	}

	if stream.peek().is_some() {
		expect_punct(
			&mut stream,
			',',
			(span.start, array.span_close()),
			"a `,` after an attribute",
			false,
		)?;
	}

	Ok(values)
}

fn parse_cfg(
	mut stream: impl Iterator<Item = TokenTree>,
	previous_span: impl Into<SpanRange>,
) -> Result<[TokenTree; 2], TokenStream> {
	let punct = expect_punct(&mut stream, '#', previous_span, "`#`", false).unwrap();
	let group = expect_group(
		&mut stream,
		Delimiter::Bracket,
		punct.span(),
		"`#[...]`",
		true,
	)?;

	expect_ident(group.stream().into_iter(), "cfg", group.span(), "`cfg`")?;

	Ok([punct.into(), group.into()])
}

pub fn parse_ty_or_value(
	mut stream: &mut Peekable<token_stream::IntoIter>,
	previous_span: impl Into<SpanRange>,
	expected: &str,
) -> Result<(Vec<TokenTree>, SpanRange), TokenStream> {
	let mut out = Vec::new();
	let mut span = previous_span.into();
	let mut found = false;

	if let Some(tok) = stream.peek() {
		span.start = tok.span();
		span.end = tok.span();
	}

	while let Some(tok) = stream.peek() {
		match tok {
			TokenTree::Ident(_) | TokenTree::Literal(_) | TokenTree::Group(_) => {
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
			TokenTree::Punct(p) if [':', '!', '*', '&'].contains(&p.as_char()) => {
				span.end = p.span();
				out.push(stream.next().unwrap());
				found = true;
			}
			TokenTree::Punct(p)
				if found && ['+', '-', '/', '%', '=', '?'].contains(&p.as_char()) =>
			{
				span.end = p.span();
				out.push(stream.next().unwrap());
				found = true;
			}
			TokenTree::Punct(_) => break,
		}
	}

	if found {
		Ok((out, span))
	} else {
		Err(compile_error(span, format!("expected {expected}")))
	}
}

fn parse_angular(
	mut stream: impl Iterator<Item = TokenTree>,
	previous_span: impl Into<SpanRange>,
) -> Result<(SpanRange, TokenStream), TokenStream> {
	let opening = expect_punct(&mut stream, '<', previous_span.into(), "`<`", false).unwrap();
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
	with_previous: bool,
) -> Result<Group, TokenStream> {
	match stream.next() {
		Some(TokenTree::Group(g)) if g.delimiter() == delimiter => Ok(g),
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

pub fn expect_ident(
	stream: impl Iterator<Item = TokenTree>,
	ident: &str,
	previous_span: Span,
	expected: &str,
) -> Result<Ident, TokenStream> {
	let i = parse_ident(stream, previous_span, expected)?;

	#[cfg_attr(
		test,
		expect(clippy::cmp_owned, reason = "`proc-macro2` compatibility")
	)]
	if i.to_string() == ident {
		Ok(i)
	} else {
		Err(compile_error(i.span(), format!("expected {expected}")))
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

#[cfg_attr(coverage_nightly, coverage(off))]
fn token_stream_from_str(
	value: &str,
	message: &str,
	span: Span,
) -> Result<TokenStream, TokenStream> {
	let result = panic::catch_unwind(|| TokenStream::from_str(value));

	let Ok(result) = result else {
		return Err(compile_error(span, message));
	};

	result.map_err(|error| compile_error(span, format!("{message}: {error}")))
}
