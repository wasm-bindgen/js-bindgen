#[cfg(test)]
mod tests;

use std::borrow::Cow;
#[cfg(not(test))]
use std::env;
use std::iter;
use std::iter::Peekable;

use js_bindgen_macro_shared::*;
use proc_macro2::{
	Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree, token_stream,
};

#[proc_macro_attribute]
pub fn js_sys(
	attr: proc_macro::TokenStream,
	original_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let original_item: TokenStream = original_item.into();

	js_sys_internal(attr.into(), original_item.clone())
		.unwrap_or_else(|mut e| {
			e.extend(original_item);
			e
		})
		.into()
}

enum JsFunction {
	Global(String),
	Embed(String),
	Import,
}

fn js_sys_internal(attr: TokenStream, item: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut attr = attr.into_iter().peekable();
	let mut item = item.into_iter().peekable();

	let mut js_sys_path = None;
	let mut namespace = None;

	while attr.peek().is_some() {
		let ident = parse_ident(&mut attr, Span::mixed_site(), "`<attribute> = ...`")?;
		let punct = expect_punct(&mut attr, '=', ident.span(), "`<attribute> = ...`", true)?;

		if attr.peek().is_none() {
			return Err(compile_error(punct.span(), "expected `<attribute> = ...`"));
		};

		match ident.to_string().as_str() {
			"js_sys" => {
				if js_sys_path.is_some() {
					return Err(compile_error(
						punct.span(),
						"`js_sys` attribute already set",
					));
				}

				js_sys_path =
					Some(parse_ty_or_value(&mut attr, punct.span(), "`js_sys = <path>`")?.1);
			}
			"namespace" => {
				if namespace.is_some() {
					return Err(compile_error(
						punct.span(),
						"`js_sys` attribute already set",
					));
				}

				let (_, string) =
					parse_string_literal(&mut attr, punct.span(), "`namespace = \"...\"`", true)?;
				namespace = Some(string);
			}
			_ => {
				return Err(compile_error(
					ident.span(),
					"expected `js_sys` or `namespace`",
				));
			}
		};

		if attr.peek().is_some() {
			expect_punct(&mut attr, ',', ident.span(), "`,` after attribute", false)?;
		}
	}

	let js_sys_path =
		js_sys_path.unwrap_or_else(|| path(iter::once("js_sys"), Span::mixed_site()).collect());

	let r#extern = expect_ident(
		&mut item,
		"extern",
		Span::mixed_site(),
		"`extern \"C\" { ... }` item",
		false,
	)?;

	let abi_span = match item.next() {
		Some(TokenTree::Literal(l)) if l.to_string() == "\"C\"" => l.span(),
		Some(tok) => return Err(compile_error(tok.span(), "expected `\"C\"` after `extern`")),
		None => {
			return Err(compile_error(
				r#extern.span(),
				"expected `\"C\"` after `extern`",
			));
		}
	};

	let fns_group = expect_group(&mut item, Delimiter::Brace, abi_span, "braces after ABI")?;
	let mut fns = fns_group.stream().into_iter().peekable();
	let mut output = TokenStream::new();

	while fns.peek().is_some() {
		let mut cfg = None;
		let mut js_sys = false;
		let mut js_function_attr = None;

		while let Some(TokenTree::Punct(p)) = fns.peek() {
			if p.as_char() == '#' {
				let hash = expect_punct(
					&mut fns,
					'#',
					Span::mixed_site(),
					"`js_sys` attribute",
					false,
				)?;
				let meta = expect_group(&mut fns, Delimiter::Bracket, hash.span(), "`[...]`")?;
				let mut inner = meta.stream().into_iter();
				let attribute = parse_ident(&mut inner, meta.span(), "`attribute(...)`")?;

				match attribute.to_string().as_str() {
					"cfg" => {
						if cfg.is_some() {
							return Err(compile_error(
								attribute.span(),
								"multiple `js_sys` attributes are not supported",
							));
						}

						cfg = Some([TokenTree::from(hash), meta.into()]);
					}
					"js_sys" => {
						if js_sys {
							return Err(compile_error(
								attribute.span(),
								"multiple `js_sys` attributes are not supported",
							));
						} else {
							js_sys = true;
						}

						let meta = expect_group(
							&mut inner,
							Delimiter::Parenthesis,
							attribute.span(),
							"`(...)`",
						)?;
						let mut inner = meta.stream().into_iter().peekable();

						while let Some(token) = inner.peek() {
							let TokenTree::Ident(ident) = token else {
								return Err(compile_error(
									token.span(),
									"`js_name`, `js_embed` or `js_import`",
								));
							};

							match ident.to_string().as_str() {
								"js_name" => {
									let (ident, string) = parse_meta_name_value(&mut inner)?;

									if let Some(js_function) = js_function_attr {
										match js_function {
											JsFunction::Global(_) => {
												return Err(compile_error(
													ident.span(),
													"found duplicate `js_name` attributes",
												));
											}
											JsFunction::Embed(_) => {
												return Err(compile_error(
													ident.span(),
													"can't set `js_name` and `js_embed` at the \
													 same time",
												));
											}
											JsFunction::Import => {
												return Err(compile_error(
													ident.span(),
													"can't set `js_name` and `js_import` at the \
													 same time",
												));
											}
										}
									}

									js_function_attr = Some(JsFunction::Global(string));
								}
								"js_embed" => {
									let (ident, string) = parse_meta_name_value(&mut inner)?;

									if let Some(js_function) = js_function_attr {
										match js_function {
											JsFunction::Embed(_) => {
												return Err(compile_error(
													ident.span(),
													"found duplicate `js_embed` attributes",
												));
											}
											JsFunction::Global(_) => {
												return Err(compile_error(
													ident.span(),
													"can't set `js_embed` and `js_name` at the \
													 same time",
												));
											}
											JsFunction::Import => {
												return Err(compile_error(
													ident.span(),
													"can't set `js_embed` and `js_import` at the \
													 same time",
												));
											}
										}
									}

									js_function_attr = Some(JsFunction::Embed(string));
								}
								"js_import" => {
									let span = ident.span();
									let _ = inner.next();

									if let Some(js_function) = js_function_attr {
										match js_function {
											JsFunction::Import => {
												return Err(compile_error(
													span,
													"found duplicate `js_import` attributes",
												));
											}
											JsFunction::Embed(_) => {
												return Err(compile_error(
													span,
													"can't set `js_import` and `js_embed` at the \
													 same time",
												));
											}
											JsFunction::Global(_) => {
												return Err(compile_error(
													span,
													"can't set `js_import` and `js_name` at the \
													 same time",
												));
											}
										}
									}

									if inner.peek().is_some() {
										expect_punct(
											&mut inner,
											',',
											span,
											"a `,` after an attribute",
											false,
										)?;
									}

									js_function_attr = Some(JsFunction::Import);
								}
								_ => {
									return Err(compile_error(
										ident.span(),
										"expected `js_name` or `js_embed`",
									));
								}
							}
						}
					}
					_ => {
						return Err(compile_error(
							attribute.span(),
							"unsupported attribute found",
						));
					}
				}
			}
		}

		let ExternFn {
			visibility,
			r#fn,
			name,
			parms,
			ret_ty,
		} = parse_extern_fn(&mut fns, Span::mixed_site())?;

		let import_name = name.to_string();
		let namespace_import_name = if let Some(namespace) = &namespace {
			Cow::Owned(format!("{namespace}.{import_name}"))
		} else {
			Cow::Borrowed(import_name.as_str())
		};

		#[cfg(not(test))]
		let package = env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found");
		#[cfg(test)]
		let package = String::from("test_crate");
		let import_parms: String = parms.iter().map(|_| "{},").collect();
		let import_ret = ret_ty.as_ref().map(|_| "{}").unwrap_or_default();

		let asm_import_name = format!("{package}.import.{namespace_import_name}");
		let extern_name = format!("{package}.{namespace_import_name}");

		let parms_input = parms.iter().enumerate().flat_map(|(index, _)| {
			[
				Cow::Owned(format!("\tlocal.get {index}")),
				Cow::Borrowed("\t{}"),
			]
		});

		let strings = [
			Cow::Owned(format!(".import_module {asm_import_name}, {package}")),
			Cow::Owned(format!(
				".import_name {asm_import_name}, {namespace_import_name}"
			)),
			Cow::Owned(format!(
				".functype {asm_import_name} ({import_parms}) -> ({import_ret})"
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
			Cow::Owned(format!(".globl {extern_name}")),
			Cow::Owned(format!("{extern_name}:")),
			Cow::Owned(format!(
				"\t.functype {extern_name} ({import_parms}) -> ({import_ret})",
			)),
		])
		.chain(parms_input)
		.chain(iter::once(Cow::Owned(format!("\tcall {asm_import_name}"))))
		.chain(ret_ty.as_ref().into_iter().map(|_| Cow::Borrowed("\t{}")))
		.chain(iter::once(Cow::Borrowed("\tend_function")));

		let interpolate = iter::once(TokenTree::from(Ident::new("interpolate", name.span())));
		let in_import_ty_fmt = parms.iter().flat_map(|Parameter { name, ty, .. }| {
			interpolate
				.clone()
				.chain(js_sys_hazard(
					ty,
					&js_sys_path,
					"Input",
					"IMPORT_TYPE",
					name.span(),
				))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let out_import_ty_fmt = ret_ty.as_ref().into_iter().flat_map(|(_, span, ty)| {
			interpolate
				.clone()
				.chain(js_sys_hazard(
					ty,
					&js_sys_path,
					"Output",
					"IMPORT_TYPE",
					*span,
				))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let in_import_func_fmt = parms.iter().flat_map(|Parameter { name, ty, .. }| {
			interpolate
				.clone()
				.chain(js_sys_hazard(
					ty,
					&js_sys_path,
					"Input",
					"IMPORT_FUNC",
					name.span(),
				))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let out_import_func_fmt = ret_ty.as_ref().into_iter().flat_map(|(_, span, ty)| {
			interpolate
				.clone()
				.chain(js_sys_hazard(
					ty,
					&js_sys_path,
					"Output",
					"IMPORT_FUNC",
					*span,
				))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let in_type_fmt = parms.iter().flat_map(|Parameter { name, ty, .. }| {
			interpolate
				.clone()
				.chain(js_sys_hazard(
					ty,
					&js_sys_path,
					"Input",
					"TYPE",
					name.span(),
				))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let out_type_fmt = ret_ty.as_ref().into_iter().flat_map(|(_, span, ty)| {
			interpolate
				.clone()
				.chain(js_sys_hazard(ty, &js_sys_path, "Output", "TYPE", *span))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let in_conv_fmt = parms.iter().flat_map(|Parameter { ty_span, ty, .. }| {
			interpolate
				.clone()
				.chain(js_sys_hazard(ty, &js_sys_path, "Input", "CONV", *ty_span))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let out_conv_fmt = ret_ty.as_ref().into_iter().flat_map(|(_, span, ty)| {
			interpolate
				.clone()
				.chain(js_sys_hazard(ty, &js_sys_path, "Output", "CONV", *span))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});

		let assembly = path_with_js_sys(
			&js_sys_path,
			["js_bindgen", "unsafe_embed_asm"],
			name.span(),
		)
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

		let js_function_name = match &js_function_attr {
			Some(JsFunction::Global(js_name)) => Cow::Owned(if let Some(namespace) = &namespace {
				format!("globalThis.{namespace}.{js_name}")
			} else {
				format!("globalThis.{js_name}")
			}),
			Some(JsFunction::Embed(js_name)) => {
				Cow::Owned(format!("this.#jsEmbed.{package}['{js_name}']"))
			}
			Some(JsFunction::Import) => Cow::Borrowed(namespace_import_name.as_ref()),
			None => Cow::Owned(format!("globalThis.{namespace_import_name}")),
		};

		let js_function = match &js_function_attr {
			Some(JsFunction::Global(_) | JsFunction::Embed(_)) | None => {
				if parms.is_empty() {
					vec![Literal::string(&js_function_name).into()]
				} else {
					let mut js_parms = String::new();

					for Parameter { name_string, .. } in &parms {
						if js_parms.is_empty() {
							js_parms.push_str(name_string);
						} else {
							js_parms.extend([", ", name_string]);
						}
					}

					let js_select_list = js_select_parms(&js_sys_path, parms.iter());

					let parms_fmt: String = parms.iter().map(|_| "{}{}{}").collect();

					[
						Literal::string(&format!("{{}}{parms_fmt}{{}}")).into(),
						Punct::new(',', Spacing::Alone).into(),
					]
					.into_iter()
					.chain(select(
						&js_sys_path,
						&js_function_name,
						iter::once(Literal::string(&format!("({js_parms}) => {{\n")).into()),
						js_select_list.clone(),
						name.span(),
					))
					.chain(parms.iter().flat_map(|p| {
						select(
							&js_sys_path,
							"",
							iter::once(Literal::string(&format!("\t{}", p.name_string)).into()),
							js_select_parms(&js_sys_path, iter::once(p)),
							p.ty_span,
						)
						.chain(select(
							&js_sys_path,
							"",
							js_sys_hazard(&p.ty, &js_sys_path, "Input", "JS_CONV", p.ty_span),
							js_select_parms(&js_sys_path, iter::once(p)),
							p.ty_span,
						))
						.chain(select(
							&js_sys_path,
							"",
							iter::once(Literal::string("\n").into()),
							js_select_parms(&js_sys_path, iter::once(p)),
							p.ty_span,
						))
					}))
					.chain(select(
						&js_sys_path,
						"",
						iter::once(
							Literal::string(&format!(
								"\t{}{js_function_name}({js_parms})\n}}",
								if ret_ty.is_some() { "return " } else { "" }
							))
							.into(),
						),
						js_select_list,
						name.span(),
					))
					.collect()
				}
			}
			Some(JsFunction::Import) => Vec::new(),
		};

		let import_js = path_with_js_sys(&js_sys_path, ["js_bindgen", "import_js"], name.span())
			.chain([
				Punct::new('!', Spacing::Alone).into(),
				Group::new(
					Delimiter::Parenthesis,
					TokenStream::from_iter(
						[
							TokenTree::from(Ident::new("name", name.span())),
							Punct::new('=', Spacing::Alone).into(),
							Literal::string(&namespace_import_name).into(),
							Punct::new(',', Spacing::Alone).into(),
						]
						.into_iter()
						.chain(
							js_function_attr
								.as_ref()
								.into_iter()
								.filter_map(|f| match f {
									JsFunction::Embed(js_name) => Some(vec![
										Ident::new("required_embed", name.span()).into(),
										Punct::new('=', Spacing::Alone).into(),
										Literal::string(js_name).into(),
										Punct::new(',', Spacing::Alone).into(),
									]),
									JsFunction::Import => {
										Some(vec![Ident::new("no_import", name.span()).into()])
									}
									JsFunction::Global(_) => None,
								})
								.flatten(),
						)
						.chain(js_function),
					),
				)
				.into(),
				Punct::new(';', Spacing::Alone).into(),
			]);

		let rust_parms = parms.iter().flat_map(
			|Parameter {
			     name, ty_span, ty, ..
			 }| {
				[
					TokenTree::from(name.clone()),
					Punct::new(':', Spacing::Alone).into(),
				]
				.into_iter()
				.chain(js_sys_hazard(ty, &js_sys_path, "Input", "Type", *ty_span))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
			},
		);

		let rust_ty = ret_ty.as_ref().into_iter().flat_map(|(arrow, span, ty)| {
			arrow
				.iter()
				.cloned()
				.chain(js_sys_hazard(ty, &js_sys_path, "Output", "Type", *span))
		});

		let import = [
			Ident::new("unsafe", name.span()).into(),
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
								Literal::string(&extern_name).into(),
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
			js_sys_hazard(ty, &js_sys_path, "Input", "into_raw", name.span()).chain([
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
					TokenTree::from(Ident::new(&import_name, name.span())),
					Group::new(Delimiter::Parenthesis, call_parms.collect()).into(),
				]),
			)
			.into(),
		];

		if let Some((_, span, ty)) = &ret_ty {
			call = js_sys_hazard(ty, &js_sys_path, "Output", "from_raw", *span)
				.chain(iter::once(
					Group::new(Delimiter::Parenthesis, call.into_iter().collect()).into(),
				))
				.collect();
		} else {
			call.push(Punct::new(';', Spacing::Alone).into());
		}

		output.extend(cfg.into_iter().flatten());
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
				.flat_map(|(arrow, _, ty)| arrow.into_iter().chain(ty)),
		);
		output.extend(iter::once(TokenTree::from(Group::new(
			Delimiter::Brace,
			TokenStream::from_iter(assembly.chain(import_js).chain(import).chain(call)),
		))));
	}

	Ok(output)
}

struct ExternFn {
	visibility: Option<Ident>,
	r#fn: Ident,
	name: Ident,
	parms: Vec<Parameter>,
	ret_ty: Option<([TokenTree; 2], SpanRange, Vec<TokenTree>)>,
}

struct Parameter {
	name: Ident,
	name_string: String,
	colon: Punct,
	ty_span: SpanRange,
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
			expect_ident(&mut stream, "fn", ident_span, "function item", false)?,
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
		let name_string = name.to_string();
		let colon = expect_punct(
			&mut parms_stream,
			':',
			name.span(),
			"colon after parameter name",
			false,
		)?;
		let (ty_span, ty) = parse_ty_or_value(&mut parms_stream, colon.span(), "a type")?;
		let comma = if parms_stream.peek().is_some() {
			Some(expect_punct(
				&mut parms_stream,
				',',
				name.span(),
				"`,` after parameter type",
				false,
			)?)
		} else {
			None
		};

		parms.push(Parameter {
			name,
			name_string,
			colon,
			ty_span,
			ty,
			comma,
		});
	}

	let punct = parse_punct(&mut stream, parms_span, "`;` or a return type")?;

	let ret_ty = match punct.as_char() {
		';' => None,
		'-' => {
			let closing = expect_punct(
				&mut stream,
				'>',
				parms_span,
				"`->` for the return type",
				true,
			)?;
			let (span, ret_ty) = parse_ty_or_value(stream, closing.span(), "a type")?;
			expect_punct(
				stream,
				';',
				closing.span(),
				"`;` after function definition",
				false,
			)?;

			Some(([punct.into(), closing.into()], span, ret_ty))
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

fn select<'a>(
	js_sys: &'a [TokenTree],
	a: &str,
	b: impl Iterator<Item = TokenTree>,
	check_list: TokenStream,
	span: impl Into<SpanRange>,
) -> impl 'a + Iterator<Item = TokenTree> {
	let span = span.into();

	iter::once(Ident::new("interpolate", span.start).into())
		.chain(path_with_js_sys(js_sys, ["r#macro", "select"], span))
		.chain([
			Group::new(
				Delimiter::Parenthesis,
				[
					TokenTree::from(Literal::string(a)),
					Punct::new(',', Spacing::Alone).into(),
				]
				.into_iter()
				.chain(b)
				.chain([
					Punct::new(',', Spacing::Alone).into(),
					Group::new(Delimiter::Bracket, check_list).into(),
				])
				.collect(),
			)
			.into(),
			Punct::new(',', Spacing::Alone).into(),
		])
}

fn js_select_parms<'a>(
	js_sys: &'a [TokenTree],
	parms: impl Iterator<Item = &'a Parameter>,
) -> TokenStream {
	parms
		.flat_map(|Parameter { ty_span, ty, .. }| {
			js_sys_hazard(ty, js_sys, "Input", "JS_CONV", *ty_span)
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		})
		.collect()
}

fn js_sys_hazard<'a>(
	ty: &'a [TokenTree],
	js_sys: &'a [TokenTree],
	r#trait: &'static str,
	field: &'static str,
	span: impl Into<SpanRange>,
) -> impl 'a + Iterator<Item = TokenTree> {
	let span = span.into();

	iter::once(Punct::new('<', Spacing::Alone).into())
		.chain(ty.iter().cloned())
		.chain(iter::once(Ident::new("as", span.start).into()))
		.chain(path_with_js_sys(js_sys, ["hazard", r#trait], span))
		.chain(iter::once(Punct::new('>', Spacing::Alone).into()))
		.chain(path(iter::once(field), span))
}

fn path_with_js_sys<'js_sys>(
	js_sys: &'js_sys [TokenTree],
	parts: impl 'js_sys + IntoIterator<Item = &'static str>,
	span: impl 'js_sys + Into<SpanRange>,
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
