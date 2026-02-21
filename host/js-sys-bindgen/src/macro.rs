use std::env;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parser;
use syn::{Error, ForeignItem, Item, ItemForeignMod, LitStr, Path, meta};

use crate::{Function, FunctionJsOutput, Hygiene, ImportManager, Type};

pub fn r#macro(
	attr: TokenStream,
	item: TokenStream,
	imports: Option<&mut ImportManager>,
) -> Result<TokenStream, TokenStream> {
	let foreign_mod: ItemForeignMod = syn::parse2(item).map_err(Error::into_compile_error)?;

	internal(attr, foreign_mod, None, imports)
		.map(|items| items.into_iter().map(Item::into_token_stream).collect())
		.map_err(|(output, error)| {
			let error = error.into_compile_error();

			if let Some(output) = output {
				let mut output: TokenStream =
					output.into_iter().map(Item::into_token_stream).collect();
				output.extend(error);
				output
			} else {
				error
			}
		})
}

pub(crate) fn internal(
	attr: TokenStream,
	mut foreign_mod: ItemForeignMod,
	crate_: Option<&str>,
	imports: Option<&mut ImportManager>,
) -> Result<Vec<Item>, (Option<Vec<Item>>, Error)> {
	let mut error = ErrorStack::new();

	let mut js_sys: Option<Path> = None;
	let mut namespace: Option<String> = None;

	if let Err(e) = meta::parser(|meta| {
		if meta.path.is_ident("js_sys") {
			if imports.is_some() {
				Err(meta.error("`js_sys` attribute only allowed with proc-macro hygiene"))
			} else if js_sys.is_some() {
				Err(meta.error("duplicate attribute"))
			} else {
				js_sys = Some(meta.value()?.parse()?);
				Ok(())
			}
		} else if meta.path.is_ident("namespace") {
			if namespace.is_some() {
				Err(meta.error("duplicate attribute"))
			} else {
				namespace = Some(meta.value()?.parse::<LitStr>()?.value());
				Ok(())
			}
		} else {
			Err(meta.error("unsupported attribute"))
		}
	})
	.parse2(attr)
	{
		error.push(e);
	}

	let mut hygiene = if let Some(imports) = imports {
		Hygiene::Imports(imports)
	} else {
		Hygiene::Hygiene {
			js_sys: js_sys.as_ref(),
		}
	};

	for attr in foreign_mod
		.attrs
		.extract_if(.., |attr| attr.path().is_ident("js_sys"))
	{
		error.push(Error::new_spanned(
			attr,
			"`js_sys` attribute not supported at that position",
		));
	}

	let mut output = Vec::new();

	if foreign_mod
		.abi
		.name
		.as_ref()
		.is_some_and(|value| value.value() != "js-sys")
	{
		error.push(Error::new_spanned(
			&foreign_mod.abi.name,
			"expected `js-sys` ABI",
		));
	}

	for item in foreign_mod.items {
		match item {
			ForeignItem::Fn(mut item) => {
				let mut js_output = FunctionJsOutput::default();

				for attr in item
					.attrs
					.extract_if(.., |attr| attr.path().is_ident("js_sys"))
				{
					if let Err(e) = attr.parse_nested_meta(|meta| {
						if js_output != FunctionJsOutput::default() {
							return Err(meta.error("found duplicate/incompatible attribute"));
						}

						if meta.path.is_ident("js_name") {
							js_output = FunctionJsOutput::Generate(Some(
								meta.value()?.parse::<LitStr>()?.value(),
							));
							Ok(())
						} else if meta.path.is_ident("js_import") {
							if meta.input.is_empty() {
								js_output = FunctionJsOutput::Import;
								Ok(())
							} else {
								Err(meta.error("`js_import` supports no values"))
							}
						} else if meta.path.is_ident("js_embed") {
							js_output =
								FunctionJsOutput::Embed(meta.value()?.parse::<LitStr>()?.value());
							Ok(())
						} else {
							Err(meta.error("unsupported attribute"))
						}
					}) {
						error.push(e);
					}
				}

				let crate_ = if let Some(crate_) = crate_ {
					crate_
				} else {
					&env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found")
				};

				match Function::new(
					&mut hygiene,
					&js_output,
					namespace.as_deref(),
					crate_,
					&item,
				) {
					Ok(function) => output.push(function.0.into()),
					Err(e) => error.push(e),
				}
			}
			ForeignItem::Type(mut item) => {
				if let Some(attr) = item
					.attrs
					.extract_if(.., |attr| attr.path().is_ident("js_sys"))
					.next()
				{
					error.push(Error::new_spanned(attr, "unsupported attribute"));
				}

				output.extend(Type::new(&mut hygiene, item).into_iter());
			}
			item => {
				error.push(Error::new_spanned(
					item,
					"expected foreign function or type ",
				));
			}
		}
	}

	if let Some(error) = error.resolve() {
		Err((Some(output), error))
	} else {
		Ok(output)
	}
}

pub(crate) struct ErrorStack(Option<Error>);

impl ErrorStack {
	pub(crate) fn new() -> Self {
		Self(None)
	}

	pub(crate) fn push(&mut self, error: Error) {
		match &mut self.0 {
			Some(this) => this.combine(error),
			None => self.0 = Some(error),
		}
	}

	pub(crate) fn resolve(self) -> Option<Error> {
		self.0
	}
}
