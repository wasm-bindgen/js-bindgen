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

use std::borrow::Cow;
#[cfg(not(test))]
use std::env;
use std::iter;
use std::iter::Peekable;

use js_bindgen_shared::*;
use proc_macro::{
	token_stream, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

#[cfg_attr(not(test), proc_macro_attribute)]
pub fn js_sys(attr: TokenStream, original_item: TokenStream) -> TokenStream {
	js_sys_internal(attr, original_item.clone()).unwrap_or_else(|mut e| {
		e.extend(original_item);
		e
	})
}

fn js_sys_internal(attr: TokenStream, item: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut attr = attr.into_iter().peekable();
	let mut item = item.into_iter().peekable();

	let mut js_sys = None;
	let mut namespace = None;

	while attr.peek().is_some() {
		let ident = parse_ident(&mut attr, Span::mixed_site(), "identifier")?;
		let punct = expect_punct(&mut attr, '=', ident.span(), "`=`")?;

		if attr.peek().is_none() {
			return Err(compile_error(
				punct.span(),
				"expected value after `attribute = `",
			));
		};

		match ident.to_string().as_str() {
			"js_sys" => {
				if js_sys.is_some() {
					return Err(compile_error(
						punct.span(),
						"`js_sys` attribute already set",
					));
				}

				js_sys = Some(parse_ty_or_value(&mut attr, punct.span())?);
			}
			"namespace" => {
				if namespace.is_some() {
					return Err(compile_error(
						punct.span(),
						"`js_sys` attribute already set",
					));
				}

				let namespace = namespace.get_or_insert_with(String::new);
				parse_string_literal(&mut attr, punct.span(), namespace)?;
			}
			_ => {
				return Err(compile_error(
					ident.span(),
					"expected `js_sys` or `namespace`",
				))
			}
		};

		if attr.peek().is_some() {
			expect_punct(&mut attr, ',', ident.span(), "`,` after attribute")?;
		}
	}

	let js_sys = js_sys.unwrap_or_else(|| path(iter::once("js_sys"), Span::mixed_site()).collect());

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

	while let Some(tok) = fns.peek() {
		let mut real_name = None;

		if let TokenTree::Punct(p) = tok {
			if p.as_char() == '#' {
				let hash = expect_punct(&mut fns, '#', Span::mixed_site(), "`js_sys` attribute")?;
				let meta = expect_group(&mut fns, Delimiter::Bracket, hash.span(), "`[...]`")?;
				let mut inner = meta.stream().into_iter();
				let js_sys = expect_ident(&mut inner, "js_sys", meta.span(), "`js_sys(...)")?;
				let meta =
					expect_group(&mut inner, Delimiter::Parenthesis, js_sys.span(), "`(...)`")?;
				let mut inner = meta.stream().into_iter();
				let name = expect_ident(&mut inner, "name", meta.span(), "`name = \"...\"`")?;
				let equal = expect_punct(&mut inner, '=', name.span(), "`= \"...\"`")?;
				let mut name = String::new();
				parse_string_literal(&mut inner, equal.span(), &mut name)?;
				real_name = Some(name);
			}
		}

		let ExternFn {
			visibility,
			r#fn,
			name,
			parms,
			ret_ty,
		} = parse_extern_fn(&mut fns, Span::mixed_site())?;

		let name_string = name.to_string();
		let comp_name = if let Some(namespace) = &namespace {
			format!("{namespace}.{name_string}")
		} else {
			name_string.clone()
		};

		#[cfg(not(test))]
		let package = env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found");
		#[cfg(test)]
		let package = String::from("test_crate");
		let mut comp_parms = String::new();
		let comp_ret = ret_ty.as_ref().map(|_| "{}").unwrap_or_default();

		for _ in &parms {
			if comp_parms.is_empty() {
				comp_parms.push_str("{}");
			} else {
				comp_parms.push_str(", {}");
			}
		}

		let parms_input = parms.iter().enumerate().flat_map(|(index, _)| {
			[
				Cow::Owned(format!("\tlocal.get {index}")),
				Cow::Borrowed("\t{}"),
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
		.chain(
			parms
				.iter()
				.flat_map(|_| [Cow::Borrowed("{}"), Cow::Borrowed("")]),
		)
		.chain(
			ret_ty
				.as_ref()
				.into_iter()
				.flat_map(|_| [Cow::Borrowed("{}"), Cow::Borrowed("")]),
		)
		.chain([
			Cow::Owned(format!(".globl {package}.{comp_name}")),
			Cow::Owned(format!("{package}.{comp_name}:")),
			Cow::Owned(format!(
				"\t.functype {package}.{comp_name} ({comp_parms}) -> ({comp_ret})",
			)),
		])
		.chain(parms_input)
		.chain(iter::once(Cow::Owned(format!(
			"\tcall {package}.import.{comp_name}"
		))))
		.chain(ret_ty.as_ref().into_iter().map(|_| Cow::Borrowed("\t{}")))
		.chain(iter::once(Cow::Borrowed("\tend_function")));

		let in_import_ty_fmt = parms.iter().flat_map(|Parameter { name, ty, .. }| {
			js_sys_hazard(ty, &js_sys, "Input", "IMPORT_TYPE", name.span())
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let out_import_ty_fmt = ret_ty.as_ref().into_iter().flat_map(|(_, ty)| {
			js_sys_hazard(ty, &js_sys, "Output", "IMPORT_TYPE", name.span())
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let in_import_func_fmt = parms.iter().flat_map(|Parameter { name, ty, .. }| {
			js_sys_hazard(ty, &js_sys, "Input", "IMPORT_FUNC", name.span())
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let out_import_func_fmt = ret_ty.as_ref().into_iter().flat_map(|(_, ty)| {
			js_sys_hazard(ty, &js_sys, "Output", "IMPORT_FUNC", name.span())
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let in_type_fmt = parms.iter().flat_map(|Parameter { name, ty, .. }| {
			js_sys_hazard(ty, &js_sys, "Input", "TYPE", name.span())
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let out_type_fmt = ret_ty.as_ref().into_iter().flat_map(|(_, ty)| {
			js_sys_hazard(ty, &js_sys, "Output", "TYPE", name.span())
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let in_conv_fmt = parms.iter().flat_map(|Parameter { name, ty, .. }| {
			js_sys_hazard(ty, &js_sys, "Input", "CONV", name.span())
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let out_conv_fmt = ret_ty.as_ref().into_iter().flat_map(|(_, ty)| {
			js_sys_hazard(ty, &js_sys, "Output", "CONV", name.span())
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});

		let assembly = path_with_js_sys(&js_sys, ["js_bindgen", "unsafe_embed_asm"], name.span())
			.chain([
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
							.chain(in_import_ty_fmt)
							.chain(out_import_ty_fmt)
							.chain(in_import_func_fmt)
							.chain(out_import_func_fmt)
							.chain(in_type_fmt)
							.chain(out_type_fmt)
							.chain(in_conv_fmt)
							.chain(out_conv_fmt),
					),
				)
				.into(),
				Punct::new(';', Spacing::Alone).into(),
			]);

		let comp_real_name = if let Some(real_name) = real_name {
			if let Some(namespace) = &namespace {
				Cow::Owned(format!("{namespace}.{real_name}"))
			} else {
				Cow::Owned(real_name)
			}
		} else {
			Cow::Borrowed(&comp_name)
		};

		let js_glue = path_with_js_sys(&js_sys, ["js_bindgen", "js_import"], name.span()).chain([
			Punct::new('!', Spacing::Alone).into(),
			Group::new(
				Delimiter::Parenthesis,
				TokenStream::from_iter([
					TokenTree::from(Ident::new("name", name.span())),
					Punct::new('=', Spacing::Alone).into(),
					Literal::string(&comp_name).into(),
					Punct::new(',', Spacing::Alone).into(),
					Literal::string(&comp_real_name).into(),
				]),
			)
			.into(),
			Punct::new(';', Spacing::Alone).into(),
		]);

		let rust_parms = parms.iter().flat_map(|Parameter { name, ty, .. }| {
			[
				TokenTree::from(name.clone()),
				Punct::new(':', Spacing::Alone).into(),
			]
			.into_iter()
			.chain(js_sys_hazard(ty, &js_sys, "Input", "Type", name.span()))
			.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});

		let rust_ty = ret_ty.as_ref().into_iter().flat_map(|(arrow, ty)| {
			arrow
				.iter()
				.cloned()
				.chain(js_sys_hazard(ty, &js_sys, "Output", "Type", name.span()))
		});

		let import = [
			TokenTree::from(Ident::new("extern", name.span())),
			Literal::string("C").into(),
			Group::new(
				Delimiter::Brace,
				TokenStream::from_iter(
					[
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
						Group::new(Delimiter::Parenthesis, rust_parms.collect()).into(),
					]
					.into_iter()
					.chain(rust_ty)
					.chain(iter::once(Punct::new(';', Spacing::Alone).into())),
				),
			)
			.into(),
		];

		let call_parms = parms.iter().flat_map(|Parameter { name, ty, .. }| {
			js_sys_hazard(ty, &js_sys, "Input", "as_raw", name.span()).chain([
				Group::new(
					Delimiter::Parenthesis,
					TokenStream::from_iter(iter::once(TokenTree::from(name.clone()))),
				)
				.into(),
				Punct::new(',', Spacing::Alone).into(),
			])
		});

		let mut call = vec![
			TokenTree::from(Ident::new("unsafe", name.span())),
			Group::new(
				Delimiter::Brace,
				TokenStream::from_iter([
					TokenTree::from(Ident::new(&name_string, name.span())),
					Group::new(Delimiter::Parenthesis, call_parms.collect()).into(),
				]),
			)
			.into(),
		];

		if let Some((_, ty)) = &ret_ty {
			call = js_sys_hazard(ty, &js_sys, "Output", "from_raw", name.span())
				.chain(iter::once(
					Group::new(Delimiter::Parenthesis, call.into_iter().collect()).into(),
				))
				.collect();
		} else {
			call.push(Punct::new(';', Spacing::Alone).into());
		}

		output.extend(visibility.map(TokenTree::from));
		output.extend([
			TokenTree::from(r#fn),
			name.into(),
			Group::new(
				Delimiter::Parenthesis,
				parms
					.into_iter()
					.flat_map(|p| {
						[p.name.into(), p.colon.into()]
							.into_iter()
							.chain(p.r#ref.into_iter().map(TokenTree::from))
							.chain(p.ty)
							.chain(p.comma.into_iter().map(TokenTree::from))
					})
					.collect(),
			)
			.into(),
		]);
		output.extend(
			ret_ty
				.into_iter()
				.flat_map(|(arrow, ty)| arrow.into_iter().chain(ty)),
		);
		output.extend(iter::once(TokenTree::from(Group::new(
			Delimiter::Brace,
			TokenStream::from_iter(assembly.chain(js_glue).chain(import).chain(call)),
		))));
	}

	Ok(output)
}

struct ExternFn {
	visibility: Option<Ident>,
	r#fn: Ident,
	name: Ident,
	parms: Vec<Parameter>,
	ret_ty: Option<([TokenTree; 2], Vec<TokenTree>)>,
}

struct Parameter {
	name: Ident,
	colon: Punct,
	r#ref: Option<Punct>,
	ty: Vec<TokenTree>,
	comma: Option<Punct>,
}

fn parse_extern_fn(
	mut stream: &mut Peekable<token_stream::IntoIter>,
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

	let parms_group = expect_group(
		&mut stream,
		Delimiter::Parenthesis,
		name.span(),
		"paranthesis after function identifier",
	)?;
	let parms_span = parms_group.span();

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

		let r#ref = match parms_stream.peek() {
			Some(TokenTree::Punct(p)) if p.as_char() == '&' => {
				Some(expect_punct(&mut parms_stream, '&', colon.span(), "`&`")?)
			}
			_ => None,
		};

		let ty = parse_ty_or_value(&mut parms_stream, colon.span())?;

		let comma = if parms_stream.peek().is_some() {
			Some(expect_punct(
				&mut parms_stream,
				',',
				name.span(),
				"`,` after parameter type",
			)?)
		} else {
			None
		};

		parms.push(Parameter {
			name,
			colon,
			r#ref,
			ty,
			comma,
		});
	}

	let punct = parse_punct(&mut stream, parms_span, "`;` or a return type")?;

	let ret_ty = match punct.as_char() {
		';' => None,
		'-' => {
			let closing = expect_punct(&mut stream, '>', parms_span, "`->` for the return type")?;
			let ret_ty = parse_ty_or_value(stream, closing.span())?;
			expect_punct(stream, ';', closing.span(), "`;` after function definition")?;

			Some(([punct.into(), closing.into()], ret_ty))
		}
		_ => return Err(compile_error(parms_span, "expected `;` or `->`")),
	};

	Ok(ExternFn {
		visibility,
		r#fn,
		name,
		parms,
		ret_ty,
	})
}

fn js_sys_hazard<'a>(
	ty: &'a [TokenTree],
	js_sys: &'a [TokenTree],
	r#trait: &'static str,
	field: &'static str,
	span: Span,
) -> impl 'a + Iterator<Item = TokenTree> {
	iter::once(Punct::new('<', Spacing::Alone).into())
		.chain(ty.iter().cloned())
		.chain(iter::once(Ident::new("as", span).into()))
		.chain(path_with_js_sys(js_sys, ["hazard", r#trait], span))
		.chain(iter::once(Punct::new('>', Spacing::Alone).into()))
		.chain(path(iter::once(field), span))
}

fn path_with_js_sys<'js_sys>(
	js_sys: &'js_sys [TokenTree],
	parts: impl 'js_sys + IntoIterator<Item = &'static str>,
	span: Span,
) -> impl 'js_sys + Iterator<Item = TokenTree> {
	js_sys.iter().cloned().chain(path(parts, span))
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
