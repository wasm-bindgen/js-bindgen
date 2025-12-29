#[cfg(test)]
extern crate proc_macro2 as proc_macro;
#[cfg(test)]
use shared as js_bindgen_shared;

// There is currently no way to execute proc-macros in non-proc-macro crates.
// However, we need it for testing. So we somehow have to enable `proc-macro2`,
// even in dependencies. It turns out that this is quite difficult to accomplish
// in dependencies, e.g. via crate features. Including the crate via a module is
// what worked for now. `rust-analyzer` doesn't seem to like `path`s outside the
// crate though, so we added a symlink.
#[cfg(test)]
#[path = "shared/lib.rs"]
mod shared;
#[cfg(test)]
mod test;

use std::iter::Peekable;
use std::{env, iter, mem};

use js_bindgen_shared::*;
use proc_macro::{
	token_stream, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

#[cfg_attr(not(test), proc_macro)]
pub fn unsafe_embed_asm(input: TokenStream) -> TokenStream {
	embed_asm_internal(input).unwrap_or_else(|e| e)
}

fn embed_asm_internal(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();
	let assembly = parse_string_arguments(&mut input, Span::mixed_site())?;
	let output = custom_section("js_bindgen.assembly", &assembly);

	if let Some(tok) = input.next() {
		Err(compile_error(
			tok.span(),
			"expected no tokens after string literals and formatting parameters",
		))
	} else {
		Ok(output)
	}
}

#[cfg_attr(not(test), proc_macro)]
pub fn js_import(input: TokenStream) -> TokenStream {
	js_import_internal(input).unwrap_or_else(|e| e)
}

fn js_import_internal(input: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut input = input.into_iter().peekable();

	let package = env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found");
	let name = expect_meta_name_value(&mut input, "name")?;

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

struct Argument {
	cfg: Option<[TokenTree; 2]>,
	kind: ArgumentKind,
}

enum ArgumentKind {
	String(String),
	Type(Vec<TokenTree>),
}

fn parse_string_arguments(
	mut stream: &mut Peekable<token_stream::IntoIter>,
	mut previous_span: Span,
) -> Result<Vec<Argument>, TokenStream> {
	let mut current_cfg = None;
	let mut strings: Vec<(Option<[TokenTree; 2]>, String)> = Vec::new();

	while let Some(tok) = stream.peek() {
		match tok {
			TokenTree::Literal(_) => {
				let (lit, string) = parse_string_literal(&mut stream, previous_span)?;
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
					)?
					.span();
				}

				strings.push((current_cfg.take(), string));
			}
			TokenTree::Punct(p) if p.as_char() == '#' => {
				if current_cfg.is_some() {
					return Err(compile_error(
						previous_span,
						"multiple `cfg`s in a row not supported",
					));
				}

				let punct = expect_punct(&mut stream, '#', previous_span, "`#`")?;
				previous_span = punct.span();
				let group =
					expect_group(&mut stream, Delimiter::Bracket, previous_span, "`[...]`")?;
				previous_span = group.span();
				previous_span =
					expect_ident(group.stream().into_iter(), "cfg", previous_span, "`cfg`")?.span();
				// We don't want to parse the rest.

				current_cfg = Some([punct.into(), group.into()]);
			}
			_ => break,
		}
	}

	if strings.is_empty() {
		return Err(compile_error(
			previous_span,
			"requires at least a string argument",
		));
	};

	let mut arguments = Vec::new();
	let mut current_string = String::new();

	// Apply argument formatting.
	for (cfg, string) in strings {
		// Something is leftover from the previous iteration but we now have a new `cfg`
		// to contend with!
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
					Some('{') => current_string.push('{'),
					Some('}') => match chars.peek() {
						Some('}') => {
							return Err(compile_error(
								previous_span,
								"no corresponding closing bracers found",
							))
						}
						_ => {
							arguments.push(Argument {
								cfg: cfg.clone(),
								kind: ArgumentKind::String(mem::take(&mut current_string)),
							});
							previous_span = expect_ident(
								&mut stream,
								"interpolate",
								previous_span,
								"`interpolate`, the only supported operand type",
							)?
							.span();
							arguments.push(Argument {
								cfg: cfg.clone(),
								kind: ArgumentKind::Type(parse_ty_or_value(stream, previous_span)?),
							});

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

	Ok(arguments)
}

/// ```"not rust"
/// const _: () = {
/// 	const LEN: u32 = {
/// 		let mut len = 0;
/// 		len += LEN<index>;
/// 		len += LEN<index>;
/// 		...
/// 		len as u32
/// 	};
///
///     #[repr(C)]
/// 	struct Layout([u8; 4], #([u8; <argument>.len()]),*);
///
/// 	#[link_section = name]
/// 	static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), #(data),*);
/// };
/// ```
fn custom_section(name: &str, data: &[Argument]) -> TokenStream {
	let span = Span::mixed_site();

	// For every string we insert:
	// ```
	// const LEN<index>: usize = <argument>.len();
	// const ARR<index>: [u8; LEN<index>] = *<argument>;
	// ```
	//
	// For every formatting argument we insert:
	// ```
	// const VAL<index>: &str = <argument>;
	// const LEN<index>: usize = ::core::primitive::str::len(VAL<index>);
	// const PTR<index>: *const u8 = ::core::primitive::str::as_ptr(VAL<index>);
	// const ARR<index>: [u8; LEN<index>] = unsafe { *(PTR<index> as *const _) };
	// ```
	let consts = data
		.iter()
		.enumerate()
		.flat_map(|(index, arg)| match &arg.kind {
			ArgumentKind::String(string) => {
				let len = format!("LEN{index}");

				arg.cfg
					.clone()
					.into_iter()
					.flatten()
					// `const LEN<index>: usize = <argument>.len();`
					.chain(r#const(
						&len,
						iter::once(Ident::new("usize", span).into()),
						iter::once(Literal::usize_unsuffixed(string.len()).into()),
						span,
					))
					.chain(arg.cfg.clone().into_iter().flatten())
					// `const ARR<index>: [u8; LEN<index>] = *<argument>;`
					.chain(r#const(
						&format!("ARR{index}"),
						iter::once(
							Group::new(
								Delimiter::Bracket,
								TokenStream::from_iter([
									TokenTree::from(Ident::new("u8", span)),
									Punct::new(';', Spacing::Alone).into(),
									Ident::new(&len, span).into(),
								]),
							)
							.into(),
						),
						[
							TokenTree::from(Punct::new('*', Spacing::Alone)),
							Literal::byte_string(string.as_bytes()).into(),
						],
						span,
					))
					.collect::<Vec<_>>()
			}
			ArgumentKind::Type(ty) => {
				let value_name = format!("VAL{index}");
				let value = TokenTree::from(Ident::new(&value_name, span));
				let len_name = format!("LEN{index}");
				let ptr_name = format!("PTR{index}");

				arg.cfg
					.clone()
					.into_iter()
					.flatten()
					// `const VAL<index>: &str = <argument>;`
					.chain(r#const(
						&value_name,
						[
							Punct::new('&', Spacing::Alone).into(),
							Ident::new("str", span).into(),
						],
						ty.iter().cloned(),
						span,
					))
					.chain(arg.cfg.clone().into_iter().flatten())
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
					.chain(arg.cfg.clone().into_iter().flatten())
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
					.chain(arg.cfg.clone().into_iter().flatten())
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

	// ```
	// const LEN: u32 = {
	//     let mut len = 0;
	//     len += LEN<index>;
	//     len += LEN<index>;
	//     ...
	//     len as u32
	// };
	// ```
	let len = r#const(
		"LEN",
		iter::once(Ident::new("u32", span).into()),
		[Group::new(
			Delimiter::Brace,
			[
				Ident::new("let", span).into(),
				Ident::new("mut", span).into(),
				Ident::new("len", span).into(),
				Punct::new('=', Spacing::Alone).into(),
				Literal::usize_unsuffixed(0).into(),
				Punct::new(';', Spacing::Alone).into(),
			]
			.into_iter()
			.chain(data.iter().enumerate().flat_map(|(index, par)| {
				par.cfg.clone().into_iter().flatten().chain(iter::once(
					Group::new(
						Delimiter::Brace,
						[
							TokenTree::from(Ident::new("len", span)),
							Punct::new('+', Spacing::Joint).into(),
							Punct::new('=', Spacing::Alone).into(),
							Ident::new(&format!("LEN{index}"), span).into(),
							Punct::new(';', Spacing::Alone).into(),
						]
						.into_iter()
						.collect(),
					)
					.into(),
				))
			}))
			.chain([
				Ident::new("len", span).into(),
				Ident::new("as", span).into(),
				Ident::new("u32", span).into(),
			])
			.collect(),
		)
		.into()],
		span,
	);

	// `[u8; 4], #([u8; <argument>.len()]),*`
	let tys = [
		Group::new(
			Delimiter::Bracket,
			[
				TokenTree::from(Ident::new("u8", span)),
				Punct::new(';', Spacing::Alone).into(),
				Literal::usize_unsuffixed(4).into(),
			]
			.into_iter()
			.collect(),
		)
		.into(),
		Punct::new(',', Spacing::Alone).into(),
	]
	.into_iter()
	.chain(data.iter().enumerate().flat_map(move |(index, arg)| {
		arg.cfg.clone().into_iter().flatten().chain([
			TokenTree::Group(Group::new(
				Delimiter::Bracket,
				TokenStream::from_iter([
					TokenTree::from(Ident::new("u8", span)),
					Punct::new(';', Spacing::Alone).into(),
					match &arg.kind {
						ArgumentKind::String(string) => {
							Literal::usize_unsuffixed(string.len()).into()
						}
						ArgumentKind::Type(_) => Ident::new(&format!("LEN{index}"), span).into(),
					},
				]),
			)),
			Punct::new(',', Spacing::Alone).into(),
		])
	}));

	// ```
	// #[repr(C)]
	// struct Layout(...);
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
		Group::new(Delimiter::Parenthesis, tys.collect()).into(),
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

	// (::core::primitive::u32::to_le_bytes(LEN), #(data),*)
	let values = Group::new(
		Delimiter::Parenthesis,
		path(["core", "primitive", "u32", "to_le_bytes"], span)
			.chain([
				Group::new(
					Delimiter::Parenthesis,
					iter::once(TokenTree::from(Ident::new("LEN", span))).collect(),
				)
				.into(),
				Punct::new(',', Spacing::Alone).into(),
			])
			.chain(data.iter().enumerate().flat_map(move |(index, arg)| {
				arg.cfg.clone().into_iter().flatten().chain([
					TokenTree::from(Ident::new(&format!("ARR{index}"), span)),
					Punct::new(',', Spacing::Alone).into(),
				])
			}))
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
					.chain(len)
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

fn expect_meta_name_value(
	mut stream: impl Iterator<Item = TokenTree>,
	ident: &str,
) -> Result<String, TokenStream> {
	let expected = format!("`{ident} = \"...\"`");

	let span = expect_ident(&mut stream, ident, Span::mixed_site(), &expected)?.span();
	let span = expect_punct(&mut stream, '=', span, &expected)?.span();
	let (_, string) = parse_string_literal(stream, span)?;

	Ok(string)
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
