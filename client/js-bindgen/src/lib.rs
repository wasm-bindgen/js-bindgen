use std::borrow::Cow;
use std::fmt::Display;
use std::iter::Peekable;
use std::{env, iter, mem};

use proc_macro::{
	token_stream, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

#[proc_macro]
pub fn embed_asm(input: TokenStream) -> TokenStream {
	embed_asm_internal(input).unwrap_or_else(|e| e)
}

fn embed_asm_internal(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();
	let assembly = parse_string_arguments(&mut input, Span::mixed_site())?;
	let output = custom_section("js_bindgen.assembly", &assembly);

	if input.next().is_some() {
		Err(compile_error(
			Span::mixed_site(),
			"expected no tokens after string literals and formatting parameters",
		))
	} else {
		Ok(output)
	}
}

#[proc_macro]
pub fn js_import(input: TokenStream) -> TokenStream {
	js_import_internal(input).unwrap_or_else(|e| e)
}

fn js_import_internal(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();

	let name = parse_meta_name_value(&mut input, "name")?;

	let package = env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found");

	let comma = expect_punct(
		&mut input,
		',',
		Span::mixed_site(),
		"`,` and a list of string literals",
	)?;

	let output = custom_section(
		&format!("js_bindgen.import.{package}.{name}"),
		&parse_string_arguments(&mut input, comma.span())?,
	);

	if input.next().is_some() {
		Err(compile_error(
			Span::mixed_site(),
			"expected no tokens after string literals and formatting parameters",
		))
	} else {
		Ok(output)
	}
}

#[proc_macro_attribute]
pub fn js_bindgen(attr: TokenStream, original_item: TokenStream) -> TokenStream {
	js_bingen_internal(attr, original_item.clone()).unwrap_or_else(|mut e| {
		e.extend(original_item);
		e
	})
}

fn js_bingen_internal(attr: TokenStream, item: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut attr = attr.into_iter().peekable();
	let mut item = item.into_iter().peekable();

	let namespace = if attr.peek().is_some() {
		Some(parse_meta_name_value(&mut attr, "namespace")?)
	} else {
		None
	};

	let r#extern = expect_ident(
		&mut item,
		"extern",
		Span::mixed_site(),
		"`extern \"C\" { ... }` item",
	)?;

	let abi_span = match item.next() {
		Some(TokenTree::Literal(l)) if l.to_string() == "\"C\"" => l.span(),
		Some(tok) => return Err(compile_error(tok.span(), "expected `\"C\"` after `extern`")),
		None => {
			return Err(compile_error(
				r#extern.span(),
				"expected `\"C\"` after `extern`",
			))
		}
	};

	let fns_group = expect_group(&mut item, Delimiter::Brace, abi_span, "braces after ABI")?;
	let mut fns = fns_group.stream().into_iter().peekable();
	let mut output = TokenStream::new();

	while fns.peek().is_some() {
		let ExternFn {
			visibility,
			r#fn,
			name,
			parms: parms_group,
			ret_ty,
		} = parse_extern_fn(&mut fns, Span::mixed_site())?;

		let mut parms = Vec::new();
		let mut parms_stream = parms_group.stream().into_iter().peekable();

		while parms_stream.peek().is_some() {
			let name = parse_ident(&mut parms_stream, parms_group.span(), "parameter name")?;
			let colon = expect_punct(
				&mut parms_stream,
				':',
				name.span(),
				"colon after parameter name",
			)?;
			let ty = parse_ty_or_value(&mut parms_stream, colon.span())?;

			parms.push((name, ty));
		}

		let name_string = name.to_string();
		let comp_name = if let Some(namespace) = &namespace {
			format!("{namespace}.{name_string}")
		} else {
			name_string.clone()
		};

		let package = env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found");
		let mut comp_parms = String::new();
		let comp_ret = ret_ty.as_ref().map(|_| "{}").unwrap_or_default();

		for (_, _) in &parms {
			if comp_parms.is_empty() {
				comp_parms.push_str("{}");
			} else {
				comp_parms.push_str(", {}");
			}
		}

		let parms_input = parms.iter().enumerate().flat_map(|(index, _)| {
			[
				Cow::Owned(format!("    local.get {index}")),
				Cow::Borrowed("    {}"),
			]
		});

		let strings = [
			Cow::Owned(format!(
				".import_module {package}.import.{comp_name}, {package}"
			)),
			Cow::Owned(format!(
				".import_name {package}.import.{comp_name}, {comp_name}"
			)),
			Cow::Owned(format!(
				".functype {package}.import.{comp_name} ({comp_parms}) -> ({comp_ret})"
			)),
			Cow::Borrowed(""),
		]
		.into_iter()
		.chain(parms.iter().map(|_| Cow::Borrowed("{}")))
		.chain([
			Cow::Borrowed(""),
			Cow::Owned(format!(".globl {package}.{comp_name}")),
			Cow::Owned(format!("{package}.{comp_name}:")),
			Cow::Owned(format!(
				"    .functype {package}.{comp_name} ({comp_parms}) -> ({comp_ret})",
			)),
		])
		.chain(parms_input)
		.chain([
			Cow::Owned(format!("    call {package}.import.{comp_name}")),
			Cow::Borrowed("    end_function"),
		]);

		let import_fmt = parms.iter().flat_map(|(name, _)| {
			js_sys_input("IMPORT_TYPE", name.span())
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let parms_fmt = parms.iter().flat_map(|(name, _)| {
			js_sys_input("IMPORT_FUNC", name.span())
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let input_fmt = parms.iter().flat_map(|(name, _)| {
			js_sys_input("TYPE", name.span())
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
				.chain(js_sys_input("CONV", name.span()))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});

		let assembly = path(["js_bindgen", "embed_asm"], name.span()).chain([
			Punct::new('!', Spacing::Alone).into(),
			Group::new(
				Delimiter::Parenthesis,
				TokenStream::from_iter(
					strings
						.flat_map(|string| {
							[
								TokenTree::from(Literal::string(&string)),
								Punct::new(',', Spacing::Alone).into(),
							]
						})
						.chain(import_fmt)
						.chain(parms_fmt)
						.chain(input_fmt),
				),
			)
			.into(),
			Punct::new(';', Spacing::Alone).into(),
		]);

		let mut comp_parms = String::new();

		for (name, _) in &parms {
			if comp_parms.is_empty() {
				comp_parms = name.to_string();
			} else {
				comp_parms.extend([", ", &name.to_string()]);
			}
		}

		let js_glue = path(["js_bindgen", "js_import"], name.span()).chain([
			Punct::new('!', Spacing::Alone).into(),
			Group::new(
				Delimiter::Parenthesis,
				TokenStream::from_iter([
					TokenTree::from(Ident::new("name", name.span())),
					Punct::new('=', Spacing::Alone).into(),
					Literal::string(&comp_name).into(),
					Punct::new(',', Spacing::Alone).into(),
					Literal::string(&format!("({comp_parms}) => {comp_name}({comp_parms})")).into(),
				]),
			)
			.into(),
			Punct::new(';', Spacing::Alone).into(),
		]);

		let import_parms = parms.iter().flat_map(|(name, _)| {
			[
				TokenTree::from(name.clone()),
				Punct::new(':', Spacing::Alone).into(),
			]
			.into_iter()
			.chain(js_sys_input("Type", name.span()))
			.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});

		let import = [
			TokenTree::from(Ident::new("extern", name.span())),
			Literal::string("C").into(),
			Group::new(
				Delimiter::Brace,
				TokenStream::from_iter([
					TokenTree::from(Punct::new('#', Spacing::Alone)),
					Group::new(
						Delimiter::Bracket,
						TokenStream::from_iter([
							TokenTree::from(Ident::new("link_name", name.span())),
							Punct::new('=', Spacing::Alone).into(),
							Literal::string(&format!("{package}.{comp_name}")).into(),
						]),
					)
					.into(),
					r#fn.clone().into(),
					name.clone().into(),
					Group::new(Delimiter::Parenthesis, import_parms.collect()).into(),
					Punct::new(';', Spacing::Alone).into(),
				]),
			)
			.into(),
		];

		let call_parms = parms.into_iter().flat_map(|(name, _)| {
			js_sys_input("as_raw", name.span()).chain([
				Group::new(
					Delimiter::Parenthesis,
					TokenStream::from_iter(iter::once(TokenTree::from(name))),
				)
				.into(),
				Punct::new(',', Spacing::Alone).into(),
			])
		});

		let call = [
			TokenTree::from(Ident::new("unsafe", name.span())),
			Group::new(
				Delimiter::Brace,
				TokenStream::from_iter([
					TokenTree::from(Ident::new(&name_string, name.span())),
					Group::new(Delimiter::Parenthesis, call_parms.collect()).into(),
				]),
			)
			.into(),
			Punct::new(';', Spacing::Alone).into(),
		];

		output.extend(visibility);
		output.extend([TokenTree::from(r#fn), name.into(), parms_group.into()]);
		output.extend(ret_ty);
		output.extend(iter::once(Group::new(
			Delimiter::Brace,
			TokenStream::from_iter(assembly.chain(js_glue).chain(import).chain(call)),
		)));
	}

	Ok(output)
}

struct ExternFn {
	visibility: Option<Ident>,
	r#fn: Ident,
	name: Ident,
	parms: Group,
	ret_ty: Option<TokenStream>,
}

fn parse_extern_fn(
	mut stream: impl Iterator<Item = TokenTree>,
	span: Span,
) -> Result<ExternFn, TokenStream> {
	let ident = parse_ident(&mut stream, span, "function item")?;
	let ident_span = ident.span();
	let ident_string = ident.to_string();

	let (visibility, r#fn) = match ident_string.as_str() {
		"pub" => (
			Some(ident),
			expect_ident(&mut stream, "fn", ident_span, "function item")?,
		),
		"fn" => (None, ident),
		_ => return Err(compile_error(ident_span, "expected function item")),
	};

	let name = parse_ident(&mut stream, r#fn.span(), "identifier after `fn`")?;

	let parms = expect_group(
		&mut stream,
		Delimiter::Parenthesis,
		name.span(),
		"paranthesis after function identifier",
	)?;

	let punct = parse_punct(&mut stream, parms.span(), "`;` or a return type")?;

	let ret_ty = match punct.as_char() {
		';' => None,
		'-' => {
			let closing = expect_punct(&mut stream, '>', parms.span(), "`->` for the return type")?;

			let mut ret_ty = TokenStream::new();

			loop {
				match stream.next() {
					Some(TokenTree::Punct(p)) if p.as_char() == ';' => break,
					Some(tok) => ret_ty.extend(iter::once(tok)),
					None => return Err(compile_error(closing.span(), "expected `;`")),
				}
			}

			Some(ret_ty)
		}
		_ => return Err(compile_error(parms.span(), "expected `;` or `->`")),
	};

	Ok(ExternFn {
		visibility,
		r#fn,
		name,
		parms,
		ret_ty,
	})
}

fn js_sys_input(field: &'static str, span: Span) -> impl Iterator<Item = TokenTree> {
	iter::once(Punct::new('<', Spacing::Alone).into())
		.chain(path(["js_sys", "JsValue"], span))
		.chain(iter::once(Ident::new("as", span).into()))
		.chain(path(["js_sys", "hazard", "Input"], span))
		.chain(iter::once(Punct::new('>', Spacing::Alone).into()))
		.chain(path(iter::once(field), span))
}

enum Argument {
	String(String),
	Type(Vec<TokenTree>),
}

fn parse_string_arguments(
	mut stream: &mut Peekable<token_stream::IntoIter>,
	mut previous_span: Span,
) -> Result<Vec<Argument>, TokenStream> {
	let mut string = String::new();

	while let Some(TokenTree::Literal(..)) = stream.peek() {
		// Only insert newline when there are multiple strings.
		if !string.is_empty() {
			string.push('\n');
		}

		let lit = parse_string_literal(&mut stream, previous_span, &mut string)?;
		previous_span = lit.span();

		if stream.peek().is_some() {
			expect_punct(
				&mut stream,
				',',
				previous_span,
				"a `,` after string literal",
			)?;
		}
	}

	string.push('\0');

	// Apply argument formatting.
	let mut chars = string.chars().peekable();
	let mut arguments = Vec::new();
	let mut current_string = String::new();

	while let Some(char) = chars.next() {
		match char {
			'{' => match chars.next() {
				Some('{') => current_string.push('{'),
				Some('}') => match chars.peek() {
					Some('}') => {
						return Err(compile_error(
							previous_span,
							"no corresponding closing bracers found",
						))
					}
					_ => {
						arguments.push(Argument::String(mem::take(&mut current_string)));
						arguments.push(Argument::Type(parse_ty_or_value(stream, previous_span)?));

						if stream.peek().is_some() {
							let punct = expect_punct(
								&mut stream,
								',',
								previous_span,
								"a `,` between formatting parameters",
							)?;
							previous_span = punct.span();
						}
					}
				},
				_ => {
					return Err(compile_error(
						previous_span,
						"no corresponding closing bracers found",
					))
				}
			},
			'}' => match chars.next() {
				Some('}') => current_string.push('}'),
				_ => {
					return Err(compile_error(
						previous_span,
						"no corresponding opening bracers found",
					))
				}
			},
			c => current_string.push(c),
		}
	}

	if !current_string.is_empty() {
		arguments.push(Argument::String(current_string));
	}

	if arguments.is_empty() {
		Err(compile_error(
			previous_span,
			"requires at least a string argument",
		))
	} else {
		Ok(arguments)
	}
}

fn parse_string_literal(
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
				return Err(compile_error(span, "expecting a string literal"));
			};

			string.reserve(stripped.len());
			let mut chars = stripped.chars();

			while let Some(char) = chars.next() {
				match char {
					'\\' => match chars.next().unwrap() {
						'"' => string.push('"'),
						'\\' => string.push('\\'),
						_ => return Err(compile_error(span, "only escaping `\"` is supported")),
					},
					'\0' => return Err(compile_error(span, "null characters are not supported")),
					c => string.push(c),
				}
			}

			Ok(l)
		}
		Some(tok) => Err(compile_error(tok.span(), "expected a string literal")),
		None => Err(compile_error(previous_span, "expected a string literal`")),
	}
}

fn parse_meta_name_value(
	mut stream: impl Iterator<Item = TokenTree>,
	ident: &str,
) -> Result<String, TokenStream> {
	let expected = format!("`{ident} = \"...\"`");

	let span = expect_ident(&mut stream, ident, Span::mixed_site(), &expected)?.span();
	let span = expect_punct(&mut stream, '=', span, &expected)?.span();
	let mut string = String::new();
	parse_string_literal(stream, span, &mut string)?;

	Ok(string)
}

fn parse_ty_or_value(
	mut stream: &mut Peekable<token_stream::IntoIter>,
	mut previous_span: Span,
) -> Result<Vec<TokenTree>, TokenStream> {
	let mut ty = Vec::new();

	if let Some(TokenTree::Punct(p)) = stream.peek() {
		if p.as_char() == '&' {
			ty.extend(iter::once(stream.next().unwrap()));
		}
	}

	if let Some(TokenTree::Punct(p)) = stream.peek() {
		if p.as_char() == '<' {
			ty.extend(parse_angular(&mut stream, previous_span)?);
			let colon1 = expect_punct(&mut stream, ':', previous_span, "`:` after qualified path")?;
			let colon2 = expect_punct(&mut stream, ':', colon1.span(), "`:` after `:`")?;
			previous_span = colon2.span();
			ty.extend([colon1.into(), colon2.into()]);
		}
	}

	let ident = parse_ident(&mut stream, previous_span, "identifier")?;
	previous_span = ident.span();
	ty.push(ident.into());

	if let Some(TokenTree::Punct(p)) = stream.peek() {
		if p.as_char() == '<' {
			ty.extend(parse_angular(&mut stream, previous_span)?);
		}
	}

	while let Some(tok) = stream.peek() {
		match tok {
			TokenTree::Punct(p) if p.as_char() == ':' => {
				let p = stream.next().unwrap();
				let colon = expect_punct(&mut stream, ':', p.span(), "`:` after `:`")?;
				previous_span = colon.span();
				ty.extend([p, colon.into()]);
			}
			_ => break,
		};

		let ident = parse_ident(&mut stream, previous_span, "identifier")?;
		previous_span = ident.span();
		ty.push(ident.into());

		if let Some(TokenTree::Punct(p)) = stream.peek() {
			if p.as_char() == '<' {
				ty.extend(parse_angular(&mut stream, previous_span)?);
			}
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

fn parse_ident(
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

fn parse_punct(
	mut stream: impl Iterator<Item = TokenTree>,
	previous_span: Span,
	expected: &str,
) -> Result<Punct, TokenStream> {
	match stream.next() {
		Some(TokenTree::Punct(p)) => Ok(p),
		Some(tok) => Err(compile_error(tok.span(), format!("expected {expected}"))),
		None => Err(compile_error(previous_span, format!("expected {expected}"))),
	}
}

fn expect_ident(
	stream: impl Iterator<Item = TokenTree>,
	ident: &str,
	previous_span: Span,
	expected: &str,
) -> Result<Ident, TokenStream> {
	let i = parse_ident(stream, previous_span, expected)?;

	if i.to_string() == ident {
		Ok(i)
	} else {
		Err(compile_error(previous_span, format!("expected {expected}")))
	}
}

fn expect_group(
	mut stream: impl Iterator<Item = TokenTree>,
	delimiter: Delimiter,
	previous_span: Span,
	expected: &str,
) -> Result<Group, TokenStream> {
	match stream.next() {
		Some(TokenTree::Group(g)) if g.delimiter() == delimiter => Ok(g),
		Some(tok) => Err(compile_error(tok.span(), format!("expected {expected}"))),
		None => Err(compile_error(previous_span, format!("expected {expected}"))),
	}
}

fn expect_punct(
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

/// ```
/// const _: () = {
///     #[repr(C)]
/// 	struct Layout(#([u8; data.len()]),*);
///
/// 	#[link_section = name]
/// 	static CUSTOM_SECTION: Layout = Layout(#(data),*);
/// };
/// ```
fn custom_section(name: &str, data: &[Argument]) -> TokenStream {
	let span = Span::mixed_site();

	// For every string we insert:
	// ```
	// const ARR<index>: [u8; data.len()] = *data;
	// ```
	//
	// For every formatting argument we insert:
	// ```
	// const VAL<index>: &str = <argument>;
	// const LEN<index>: usize = ::core::primitive::str::len(VAL<index>);
	// const PTR<index>: *const u8 = ::core::primitive::str::as_ptr(VAL<index>);
	// const ARR<index>: [u8; LEN<index>] = unsafe { *(PTR<index> as *const _) };
	// ```
	let consts = data.iter().enumerate().flat_map(|(index, arg)| match arg {
		Argument::String(string) => {
			// `const ARR<index>: [u8; data.len()] = *data;`
			r#const(
				&format!("ARR{index}"),
				iter::once(
					Group::new(
						Delimiter::Bracket,
						TokenStream::from_iter([
							TokenTree::from(Ident::new("u8", span)),
							Punct::new(';', Spacing::Alone).into(),
							Literal::usize_unsuffixed(string.len()).into(),
						]),
					)
					.into(),
				),
				[
					TokenTree::from(Punct::new('*', Spacing::Alone)),
					Literal::byte_string(string.as_bytes()).into(),
				],
				span,
			)
			.collect::<Vec<_>>()
		}
		Argument::Type(ty) => {
			let value_name = format!("VAL{index}");
			let value = TokenTree::from(Ident::new(&value_name, span));
			let len_name = format!("LEN{index}");
			let ptr_name = format!("PTR{index}");

			// `const VAL<index>: &str = <argument>;`
			r#const(
				&value_name,
				[
					Punct::new('&', Spacing::Alone).into(),
					Ident::new("str", span).into(),
				],
				ty.iter().cloned(),
				span,
			)
			// `const LEN<index>: usize = ::core::primitive::str::len(VAL<index>);`
			.chain(r#const(
				&len_name,
				iter::once(Ident::new("usize", span).into()),
				path(["core", "primitive", "str", "len"], span).chain(iter::once(
					Group::new(
						Delimiter::Parenthesis,
						TokenStream::from_iter(iter::once(value.clone())),
					)
					.into(),
				)),
				span,
			))
			// `const PTR<index>: *const u8 = ::core::primitive::str::as_ptr(VAL<index>);`
			.chain(r#const(
				&ptr_name,
				[
					Punct::new('*', Spacing::Alone).into(),
					Ident::new("const", span).into(),
					Ident::new("u8", span).into(),
				],
				path(["core", "primitive", "str", "as_ptr"], span).chain(iter::once(
					Group::new(
						Delimiter::Parenthesis,
						TokenStream::from_iter(iter::once(value)),
					)
					.into(),
				)),
				span,
			))
			// `const ARR<index>: [u8; LEN<index>] = unsafe { *(PTR<index> as *const _) };`
			.chain(r#const(
				&format!("ARR{index}"),
				iter::once(
					Group::new(
						Delimiter::Bracket,
						TokenStream::from_iter([
							TokenTree::from(Ident::new("u8", span)),
							Punct::new(';', Spacing::Alone).into(),
							TokenTree::from(Ident::new(&len_name, span)),
						]),
					)
					.into(),
				),
				[
					TokenTree::from(Ident::new("unsafe", span)),
					Group::new(
						Delimiter::Brace,
						TokenStream::from_iter([
							TokenTree::from(Punct::new('*', Spacing::Alone)),
							Group::new(
								Delimiter::Parenthesis,
								TokenStream::from_iter([
									TokenTree::from(Ident::new(&ptr_name, span)),
									Ident::new("as", span).into(),
									Punct::new('*', Spacing::Alone).into(),
									Ident::new("const", span).into(),
									Ident::new("_", span).into(),
								]),
							)
							.into(),
						]),
					)
					.into(),
				],
				span,
			))
			.collect::<Vec<_>>()
		}
	});

	// `#([u8; data.len()]),*`
	let tys = TokenStream::from_iter(data.iter().enumerate().flat_map(move |(index, data)| {
		[
			TokenTree::Group(Group::new(
				Delimiter::Bracket,
				TokenStream::from_iter([
					TokenTree::from(Ident::new("u8", span)),
					Punct::new(';', Spacing::Alone).into(),
					match data {
						Argument::String(string) => Literal::usize_unsuffixed(string.len()).into(),
						Argument::Type(_) => Ident::new(&format!("LEN{index}"), span).into(),
					},
				]),
			)),
			Punct::new(',', Spacing::Alone).into(),
		]
	}));

	// ```
	// #[repr(C)]
	// struct Layout(#([u8; data.len()]),*);
	// ```
	let layout = [
		TokenTree::from(Punct::new('#', Spacing::Alone)),
		Group::new(
			Delimiter::Bracket,
			TokenStream::from_iter([
				TokenTree::from(Ident::new("repr", span)),
				Group::new(
					Delimiter::Parenthesis,
					TokenStream::from_iter(iter::once(TokenTree::from(Ident::new("C", span)))),
				)
				.into(),
			]),
		)
		.into(),
		Ident::new("struct", span).into(),
		Ident::new("Layout", span).into(),
		Group::new(Delimiter::Parenthesis, tys.clone()).into(),
		Punct::new(';', Spacing::Alone).into(),
	];

	// `#[link_section = name]`
	let link_section = [
		TokenTree::from(Punct::new('#', Spacing::Alone)),
		Group::new(
			Delimiter::Bracket,
			TokenStream::from_iter([
				TokenTree::from(Ident::new("link_section", span)),
				Punct::new('=', Spacing::Alone).into(),
				Literal::string(name).into(),
			]),
		)
		.into(),
	];

	// (#(data),*)
	let values = Group::new(
		Delimiter::Parenthesis,
		data.iter()
			.enumerate()
			.flat_map(move |(index, _)| {
				[
					TokenTree::from(Ident::new(&format!("ARR{index}"), span)),
					Punct::new(',', Spacing::Alone).into(),
				]
			})
			.collect(),
	);

	// `static CUSTOM_SECTION: Layout = Layout(...);`
	let custom_section = [
		Ident::new("static", span).into(),
		Ident::new("CUSTOM_SECTION", span).into(),
		Punct::new(':', Spacing::Alone).into(),
		Ident::new("Layout", span).into(),
		Punct::new('=', Spacing::Alone).into(),
		Ident::new("Layout", span).into(),
		values.into(),
		Punct::new(';', Spacing::Alone).into(),
	];

	// `const _: () = { ... }`
	r#const(
		"_",
		iter::once(Group::new(Delimiter::Parenthesis, TokenStream::new()).into()),
		iter::once(
			Group::new(
				Delimiter::Brace,
				consts
					.chain(layout)
					.chain(link_section)
					.chain(custom_section)
					.collect(),
			)
			.into(),
		),
		span,
	)
	.collect()
}

fn r#const(
	name: &str,
	ty: impl IntoIterator<Item = TokenTree>,
	value: impl IntoIterator<Item = TokenTree>,
	span: Span,
) -> impl Iterator<Item = TokenTree> {
	[
		Ident::new("const", span).into(),
		Ident::new(name, span).into(),
		Punct::new(':', Spacing::Alone).into(),
	]
	.into_iter()
	.chain(ty)
	.chain(iter::once(Punct::new('=', Spacing::Alone).into()))
	.chain(value)
	.chain(iter::once(Punct::new(';', Spacing::Alone).into()))
}

fn path(
	parts: impl IntoIterator<Item = &'static str>,
	span: Span,
) -> impl Iterator<Item = TokenTree> {
	parts.into_iter().flat_map(move |p| {
		[
			TokenTree::from(Punct::new(':', Spacing::Joint)),
			Punct::new(':', Spacing::Alone).into(),
			Ident::new(p, span).into(),
		]
	})
}

/// ```
/// ::core::compile_error!(error);
/// ```
fn compile_error<E: Display>(span: Span, error: E) -> TokenStream {
	TokenStream::from_iter(
		path(["core", "compile_error"], span).chain([
			Punct::new('!', Spacing::Alone).into(),
			Group::new(
				Delimiter::Parenthesis,
				TokenTree::from(Literal::string(&error.to_string())).into(),
			)
			.into(),
			Punct::new(';', Spacing::Alone).into(),
		]),
	)
}
