#[cfg(test)]
mod tests;

use std::borrow::Cow;
#[cfg(not(test))]
use std::env;
use std::iter;
use std::iter::Peekable;
use std::str::FromStr;

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
		}

		match ident.to_string().as_str() {
			"js_sys" => {
				if js_sys_path.is_some() {
					return Err(compile_error(
						punct.span(),
						"`js_sys` attribute already set",
					));
				}

				let mut out = Vec::new();
				parse_ty_or_value(&mut attr, punct.span(), "`js_sys = <path>`", &mut out)?;
				js_sys_path = Some(out);
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
		}

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

	let items_group = expect_group(&mut item, Delimiter::Brace, abi_span, "braces after ABI")?;
	let mut items = items_group.stream().into_iter().peekable();
	let mut output = TokenStream::new();

	while items.peek().is_some() {
		let mut cfg = None;
		let mut js_sys = false;
		let mut js_function_attr = None;

		while let Some(TokenTree::Punct(p)) = items.peek() {
			if p.as_char() == '#' {
				let hash = expect_punct(
					&mut items,
					'#',
					Span::mixed_site(),
					"`js_sys` attribute",
					false,
				)?;
				let meta = expect_group(&mut items, Delimiter::Bracket, hash.span(), "`[...]`")?;
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
						}

						js_sys = true;

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

		let item = ExternItem::parse(&mut items)?;

		match item {
			ExternItem::Fn(extern_fn) => extern_fn.emit(
				&mut output,
				&js_sys_path,
				namespace.as_deref(),
				cfg,
				js_function_attr.as_ref(),
			),
			ExternItem::Type(extern_type) => {
				if js_function_attr.is_some() {
					return Err(compile_error(
						extern_type.name.span(),
						"types don't support any attributes",
					));
				}

				extern_type.emit(&mut output, &js_sys_path, cfg);
			}
		}
	}

	Ok(output)
}

enum ExternItem {
	Fn(ExternFn),
	Type(ExternType),
}

impl ExternItem {
	fn parse(stream: &mut Peekable<token_stream::IntoIter>) -> Result<Self, TokenStream> {
		let mut visibility = None;
		let mut peek_next = stream.peek();

		if let Some(TokenTree::Ident(ident)) = peek_next
			&& ident == "pub"
		{
			let Some(TokenTree::Ident(ident)) = stream.next() else {
				unreachable!()
			};
			visibility = Some(ident);
			peek_next = stream.peek();
		}

		let Some(TokenTree::Ident(ident)) = peek_next else {
			return Err(compile_error(
				peek_next.map_or(Span::mixed_site(), TokenTree::span),
				"expected item",
			));
		};

		match ident.to_string().as_str() {
			"fn" => {
				let mut extern_fn = ExternFn::parse(stream)?;
				extern_fn.visibility = visibility;
				Ok(Self::Fn(extern_fn))
			}
			"type" => {
				let mut extern_type = ExternType::parse(stream)?;
				extern_type.visibility = visibility;
				Ok(Self::Type(extern_type))
			}
			_ => Err(compile_error(
				ident.span(),
				"expected extern function or type",
			)),
		}
	}
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

impl ExternFn {
	fn parse(mut stream: &mut Peekable<token_stream::IntoIter>) -> Result<Self, TokenStream> {
		let ident = parse_ident(&mut stream, Span::mixed_site(), "function item")?;
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
			let mut ty = Vec::new();
			let ty_span = parse_ty_or_value(&mut parms_stream, colon.span(), "a type", &mut ty)?;
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
				let mut ret_ty = Vec::new();
				let span = parse_ty_or_value(stream, closing.span(), "a type", &mut ret_ty)?;
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

		Ok(Self {
			visibility,
			r#fn,
			name,
			parms,
			ret_ty,
		})
	}

	fn emit(
		self,
		output: &mut TokenStream,
		js_sys: &[TokenTree],
		namespace: Option<&str>,
		cfg: Option<[TokenTree; 2]>,
		js_function_attr: Option<&JsFunction>,
	) {
		let import_name = self.name.to_string();
		let namespace_import_name = if let Some(namespace) = &namespace {
			Cow::Owned(format!("{namespace}.{import_name}"))
		} else {
			Cow::Borrowed(import_name.as_str())
		};

		#[cfg(not(test))]
		let package = env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found");
		#[cfg(test)]
		let package = String::from("test_crate");
		let import_parms: String = self.parms.iter().map(|_| "{},").collect();
		let import_ret = self.ret_ty.as_ref().map(|_| "{}").unwrap_or_default();

		let asm_import_name = format!("{package}.import.{namespace_import_name}");
		let extern_name = format!("{package}.{namespace_import_name}");

		let parms_input = self.parms.iter().enumerate().flat_map(|(index, _)| {
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
			self.parms
				.iter()
				.flat_map(|_| [Cow::Borrowed("{}"), Cow::Borrowed("")]),
		)
		.chain(
			self.ret_ty
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
		.chain(
			self.ret_ty
				.as_ref()
				.into_iter()
				.map(|_| Cow::Borrowed("\t{}")),
		)
		.chain(iter::once(Cow::Borrowed("\tend_function")));

		let interpolate = iter::once(TokenTree::from(Ident::new("interpolate", self.name.span())));
		let in_import_ty_fmt = self.parms.iter().flat_map(|Parameter { name, ty, .. }| {
			interpolate
				.clone()
				.chain(js_sys_hazard(
					ty,
					js_sys,
					"Input",
					"IMPORT_TYPE",
					name.span(),
				))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let out_import_ty_fmt = self.ret_ty.as_ref().into_iter().flat_map(|(_, span, ty)| {
			interpolate
				.clone()
				.chain(js_sys_hazard(ty, js_sys, "Output", "IMPORT_TYPE", *span))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let in_import_func_fmt = self.parms.iter().flat_map(|Parameter { name, ty, .. }| {
			interpolate
				.clone()
				.chain(js_sys_hazard(
					ty,
					js_sys,
					"Input",
					"IMPORT_FUNC",
					name.span(),
				))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let out_import_func_fmt = self.ret_ty.as_ref().into_iter().flat_map(|(_, span, ty)| {
			interpolate
				.clone()
				.chain(js_sys_hazard(ty, js_sys, "Output", "IMPORT_FUNC", *span))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let in_type_fmt = self.parms.iter().flat_map(|Parameter { name, ty, .. }| {
			interpolate
				.clone()
				.chain(js_sys_hazard(ty, js_sys, "Input", "TYPE", name.span()))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let out_type_fmt = self.ret_ty.as_ref().into_iter().flat_map(|(_, span, ty)| {
			interpolate
				.clone()
				.chain(js_sys_hazard(ty, js_sys, "Output", "TYPE", *span))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let in_conv_fmt = self.parms.iter().flat_map(|Parameter { ty_span, ty, .. }| {
			interpolate
				.clone()
				.chain(js_sys_hazard(ty, js_sys, "Input", "CONV", *ty_span))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});
		let out_conv_fmt = self.ret_ty.as_ref().into_iter().flat_map(|(_, span, ty)| {
			interpolate
				.clone()
				.chain(js_sys_hazard(ty, js_sys, "Output", "CONV", *span))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
		});

		let assembly =
			path_with_js_sys(js_sys, ["js_bindgen", "unsafe_embed_asm"], self.name.span()).chain([
				Punct::new('!', Spacing::Alone).into(),
				Group::new(
					Delimiter::Parenthesis,
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
						.chain(out_conv_fmt)
						.collect::<TokenStream>(),
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
				if self.parms.is_empty() {
					vec![Literal::string(&js_function_name).into()]
				} else {
					let mut js_parms = String::new();

					for Parameter { name_string, .. } in &self.parms {
						if js_parms.is_empty() {
							js_parms.push_str(name_string);
						} else {
							js_parms.extend([", ", name_string]);
						}
					}

					let js_select_list = js_select_parms(js_sys, self.parms.iter());

					let parms_fmt: String = self.parms.iter().map(|_| "{}{}{}").collect();

					[
						Literal::string(&format!("{{}}{parms_fmt}{{}}")).into(),
						Punct::new(',', Spacing::Alone).into(),
					]
					.into_iter()
					.chain(select(
						js_sys,
						&js_function_name,
						iter::once(Literal::string(&format!("({js_parms}) => {{\n")).into()),
						js_select_list.clone(),
						self.name.span(),
					))
					.chain(self.parms.iter().flat_map(|p| {
						select(
							js_sys,
							"",
							iter::once(Literal::string(&format!("\t{}", p.name_string)).into()),
							js_select_parms(js_sys, iter::once(p)),
							p.ty_span,
						)
						.chain(select(
							js_sys,
							"",
							js_sys_hazard(&p.ty, js_sys, "Input", "JS_CONV", p.ty_span),
							js_select_parms(js_sys, iter::once(p)),
							p.ty_span,
						))
						.chain(select(
							js_sys,
							"",
							iter::once(Literal::string("\n").into()),
							js_select_parms(js_sys, iter::once(p)),
							p.ty_span,
						))
					}))
					.chain(select(
						js_sys,
						"",
						iter::once(
							Literal::string(&format!(
								"\t{}{js_function_name}({js_parms})\n}}",
								if self.ret_ty.is_some() { "return " } else { "" }
							))
							.into(),
						),
						js_select_list,
						self.name.span(),
					))
					.collect()
				}
			}
			Some(JsFunction::Import) => Vec::new(),
		};

		let import_js = path_with_js_sys(js_sys, ["js_bindgen", "import_js"], self.name.span())
			.chain([
				Punct::new('!', Spacing::Alone).into(),
				Group::new(
					Delimiter::Parenthesis,
					[
						TokenTree::from(Ident::new("name", self.name.span())),
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
									Ident::new("required_embed", self.name.span()).into(),
									Punct::new('=', Spacing::Alone).into(),
									Literal::string(js_name).into(),
									Punct::new(',', Spacing::Alone).into(),
								]),
								JsFunction::Import => {
									Some(vec![Ident::new("no_import", self.name.span()).into()])
								}
								JsFunction::Global(_) => None,
							})
							.flatten(),
					)
					.chain(js_function)
					.collect(),
				)
				.into(),
				Punct::new(';', Spacing::Alone).into(),
			]);

		let rust_parms = self.parms.iter().flat_map(
			|Parameter {
			     name, ty_span, ty, ..
			 }| {
				[
					TokenTree::from(name.clone()),
					Punct::new(':', Spacing::Alone).into(),
				]
				.into_iter()
				.chain(js_sys_hazard(ty, js_sys, "Input", "Type", *ty_span))
				.chain(iter::once(Punct::new(',', Spacing::Alone).into()))
			},
		);

		let rust_ty = self
			.ret_ty
			.as_ref()
			.into_iter()
			.flat_map(|(arrow, span, ty)| {
				arrow
					.iter()
					.cloned()
					.chain(js_sys_hazard(ty, js_sys, "Output", "Type", *span))
			});

		let import = [
			Ident::new("unsafe", self.name.span()).into(),
			TokenTree::from(Ident::new("extern", self.name.span())),
			Literal::string("C").into(),
			Group::new(
				Delimiter::Brace,
				[
					TokenTree::from(Punct::new('#', Spacing::Alone)),
					Group::new(
						Delimiter::Bracket,
						TokenStream::from_iter([
							TokenTree::from(Ident::new("link_name", self.name.span())),
							Punct::new('=', Spacing::Alone).into(),
							Literal::string(&extern_name).into(),
						]),
					)
					.into(),
					self.r#fn.clone().into(),
					self.name.clone().into(),
					Group::new(Delimiter::Parenthesis, rust_parms.collect()).into(),
				]
				.into_iter()
				.chain(rust_ty)
				.chain(iter::once(Punct::new(';', Spacing::Alone).into()))
				.collect(),
			)
			.into(),
		];

		let call_parms = self.parms.iter().flat_map(|Parameter { name, ty, .. }| {
			js_sys_hazard(ty, js_sys, "Input", "into_raw", name.span()).chain([
				Group::new(
					Delimiter::Parenthesis,
					iter::once(TokenTree::from(name.clone())).collect(),
				)
				.into(),
				Punct::new(',', Spacing::Alone).into(),
			])
		});

		let mut call = vec![
			TokenTree::from(Ident::new("unsafe", self.name.span())),
			Group::new(
				Delimiter::Brace,
				TokenStream::from_iter([
					TokenTree::from(Ident::new(&import_name, self.name.span())),
					Group::new(Delimiter::Parenthesis, call_parms.collect()).into(),
				]),
			)
			.into(),
		];

		if let Some((_, span, ty)) = &self.ret_ty {
			call = js_sys_hazard(ty, js_sys, "Output", "from_raw", *span)
				.chain(iter::once(
					Group::new(Delimiter::Parenthesis, call.into_iter().collect()).into(),
				))
				.collect();
		} else {
			call.push(Punct::new(';', Spacing::Alone).into());
		}

		output.extend(cfg.into_iter().flatten());
		output.extend(self.visibility.map(TokenTree::from));
		output.extend([
			TokenTree::from(self.r#fn),
			self.name.into(),
			Group::new(
				Delimiter::Parenthesis,
				self.parms
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
			self.ret_ty
				.into_iter()
				.flat_map(|(arrow, _, ty)| arrow.into_iter().chain(ty)),
		);
		output.extend(iter::once(TokenTree::from(Group::new(
			Delimiter::Brace,
			assembly
				.chain(import_js)
				.chain(import)
				.chain(call)
				.collect(),
		))));
	}
}

struct ExternType {
	visibility: Option<Ident>,
	name: Ident,
	generic_group: Option<GenericGroup>,
}

struct GenericGroup {
	open: Punct,
	generics: Vec<Generic>,
	close: Punct,
}

struct Generic {
	name: Ident,
	traits: Option<(Punct, Vec<TokenTree>)>,
	default: Option<(Punct, Vec<TokenTree>)>,
	comma: Option<Punct>,
}

impl ExternType {
	fn parse(mut stream: &mut Peekable<token_stream::IntoIter>) -> Result<Self, TokenStream> {
		let ident = parse_ident(&mut stream, Span::mixed_site(), "type item")?;
		let ident_span = ident.span();
		let ident_string = ident.to_string();

		let (visibility, r#type) = match ident_string.as_str() {
			"pub" => (
				Some(ident),
				expect_ident(&mut stream, "type", ident_span, "type item", false)?,
			),
			"type" => (None, ident),
			_ => return Err(compile_error(ident_span, "expected type item")),
		};

		let name = parse_ident(&mut stream, r#type.span(), "identifier after `type`")?;

		match stream.next() {
			Some(TokenTree::Punct(p)) if p.as_char() == ';' => Ok(Self {
				visibility,
				name,
				generic_group: None,
			}),
			Some(TokenTree::Punct(open)) if open.as_char() == '<' => {
				let mut generics = Vec::new();

				let generic_group = 'outer: loop {
					let Some(token) = stream.next() else {
						return Err(compile_error(
							name.span(),
							"expected generic identifier or `>`",
						));
					};

					match token {
						TokenTree::Punct(close) if close.as_char() == '>' => {
							break GenericGroup {
								open,
								generics,
								close,
							};
						}
						_ => (),
					}

					let TokenTree::Ident(name) = token else {
						return Err(compile_error(token.span(), "expected generic identifier"));
					};

					let mut generic = Generic {
						name,
						traits: None,
						default: None,
						comma: None,
					};

					'inner: while let Some(token) = stream.next() {
						match token {
							TokenTree::Punct(colon) if colon.as_char() == ':' => {
								let mut traits = Vec::new();
								parse_ty_or_value(
									stream,
									colon.span(),
									"generic trait",
									&mut traits,
								)?;

								while let Some(token) = stream.peek() {
									match token {
										TokenTree::Punct(close) if close.as_char() == '>' => {
											generic.traits = Some((colon, traits));
											break 'inner;
										}
										TokenTree::Punct(p) if p.as_char() == '+' => {
											traits.push(stream.next().unwrap());
										}
										TokenTree::Punct(p) if p.as_char() == '=' => {
											generic.traits = Some((colon, traits));
											continue 'inner;
										}
										_ => {
											parse_ty_or_value(
												stream,
												colon.span(),
												"generic trait",
												&mut traits,
											)?;
										}
									}
								}
							}
							TokenTree::Punct(equal) if equal.as_char() == '=' => {
								let mut default = Vec::new();
								parse_ty_or_value(
									stream,
									equal.span(),
									"generic default type",
									&mut default,
								)?;
								generic.default = Some((equal, default));

								if let Some(TokenTree::Punct(p)) = stream.peek()
									&& p.as_char() == ','
								{
									let Some(TokenTree::Punct(comma)) = stream.next() else {
										unreachable!()
									};
									generic.comma = Some(comma);
								}

								break;
							}
							TokenTree::Punct(p) if p.as_char() == ',' => break,
							TokenTree::Punct(close) if close.as_char() == '>' => {
								generics.push(generic);

								break 'outer GenericGroup {
									open,
									generics,
									close,
								};
							}
							token => {
								return Err(compile_error(token.span(), "expected `:` or `=`"));
							}
						}
					}

					generics.push(generic);
				};

				expect_punct(stream, ';', name.span(), "`;`", false)?;

				Ok(Self {
					visibility,
					name,
					generic_group: Some(generic_group),
				})
			}
			token => Err(compile_error(
				token.map_or_else(|| name.span(), |token| token.span()),
				"expected `;`",
			)),
		}
	}

	fn emit(self, output: &mut TokenStream, js_sys: &[TokenTree], cfg: Option<[TokenTree; 2]>) {
		let cfg: String = cfg.into_iter().flatten().map(|t| t.to_string()).collect();
		let visibility = self.visibility.map(|i| i.to_string()).unwrap_or_default();
		let name = self.name.to_string();
		let fields;
		let value;
		let field_values;
		let js_sys: String = js_sys.iter().map(TokenTree::to_string).collect();
		let mut generics_all = String::new();
		let mut generics_names = String::new();
		let mut generics_with_traits = String::new();

		if let Some(generic_group) = self.generic_group {
			generics_all.push(generic_group.open.as_char());
			generics_names.push(generic_group.open.as_char());
			generics_with_traits.push(generic_group.open.as_char());

			for generic in &generic_group.generics {
				let name = generic.name.to_string();
				generics_all.push_str(&name);
				generics_names.push_str(&name);
				generics_with_traits.push_str(&name);

				if let Some((colon, traits)) = &generic.traits {
					let traits: String = traits.iter().map(TokenTree::to_string).collect();

					generics_all.push(colon.as_char());
					generics_with_traits.push(colon.as_char());
					generics_all.push_str(&traits);
					generics_with_traits.push_str(&traits);
				}

				if let Some((equal, r#type)) = &generic.default {
					generics_all.push(equal.as_char());
					generics_all.extend(r#type.iter().map(TokenTree::to_string));
				}

				if let Some(comma) = &generic.comma {
					generics_all.push(comma.as_char());
					generics_names.push(comma.as_char());
					generics_with_traits.push(comma.as_char());
				}
			}

			generics_all.push(generic_group.close.as_char());
			generics_names.push(generic_group.close.as_char());
			generics_with_traits.push(generic_group.close.as_char());

			fields = format!(
				"{{ value: {js_sys}::JsValue, _type: ::core::marker::PhantomData{generics_names} \
				 }}"
			);
			value = "value";
			field_values = format!(
				"{{ value: {js_sys}::hazard::Output::from_raw(raw), _type: \
				 ::core::marker::PhantomData }}"
			);
		} else {
			fields = format!("({js_sys}::JsValue);");
			value = "0";
			field_values = format!("({js_sys}::hazard::Output::from_raw(raw))");
		}

		let output_str = format!(
			r#"{cfg}
			#[repr(transparent)]
			{visibility} struct {name}{generics_all}{fields}

			{cfg}
			impl{generics_with_traits} ::core::ops::Deref for {name}{generics_names} {{
				type Target = {js_sys}::JsValue;

				fn deref(&self) -> &Self::Target {{
					&self.{value}
				}}
			}}
			
			{cfg}
			unsafe impl{generics_with_traits} {js_sys}::hazard::Input for &{name}{generics_names} {{
				const IMPORT_FUNC: &'static ::core::primitive::str = ".functype js_sys.externref.get (i32) -> (externref)";
				const IMPORT_TYPE: &'static ::core::primitive::str = "externref";
				const TYPE: &'static ::core::primitive::str = "i32";
				const CONV: &'static ::core::primitive::str = "call js_sys.externref.get";

				type Type = ::core::primitive::i32;

				fn into_raw(self) -> Self::Type {{
					{js_sys}::hazard::Input::into_raw(&self.{value})
				}}
			}}

			{cfg}
			unsafe impl{generics_with_traits} {js_sys}::hazard::Output for {name}{generics_names} {{
				const IMPORT_FUNC: &::core::primitive::str = ".functype js_sys.externref.insert (externref) -> (i32)";
				const IMPORT_TYPE: &::core::primitive::str = "externref";
				const TYPE: &::core::primitive::str = "i32";
				const CONV: &::core::primitive::str = "call js_sys.externref.insert";

				type Type = ::core::primitive::i32;

				fn from_raw(raw: Self::Type) -> Self {{
					Self{field_values}
				}}
			}}"#
		);

		output.extend(TokenStream::from_str(&output_str).unwrap());
	}
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
	previous_span: impl Into<SpanRange>,
	expected: &str,
) -> Result<Punct, TokenStream> {
	match stream.next() {
		Some(TokenTree::Punct(p)) => Ok(p),
		Some(tok) => Err(compile_error(tok.span(), format!("expected {expected}"))),
		None => Err(compile_error(previous_span, format!("expected {expected}"))),
	}
}
