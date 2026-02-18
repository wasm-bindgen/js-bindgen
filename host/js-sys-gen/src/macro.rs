#[cfg(not(test))]
use std::env;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parser;
use syn::{Error, ForeignItem, ItemForeignMod, LitStr, Path, meta};

use crate::{Function, FunctionJsOutput, Hygiene, Type};

pub fn js_sys(attr: TokenStream, item: TokenStream) -> Result<TokenStream, TokenStream> {
	let mut foreign_mod: ItemForeignMod = match syn::parse2(item) {
		Ok(foreign_mod) => foreign_mod,
		Err(e) => return Err(e.into_compile_error()),
	};
	let mut error = ErrorStack::new();

	let mut js_sys: Option<Path> = None;
	let mut namespace: Option<String> = None;

	if let Err(e) = meta::parser(|meta| {
		if meta.path.is_ident("js_sys") {
			if js_sys.is_some() {
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

	for attr in foreign_mod
		.attrs
		.extract_if(.., |attr| attr.path().is_ident("js_sys"))
	{
		error.push(Error::new_spanned(
			attr,
			"`js_sys` attribute not supported at that position",
		));
	}

	let mut output = TokenStream::new();

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

				#[cfg(not(test))]
				let crate_ = env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found");
				#[cfg(test)]
				let crate_ = String::from("test_crate");

				match Function::new(
					Hygiene::Hygiene {
						js_sys: js_sys.as_ref(),
					},
					&js_output,
					namespace.as_deref(),
					&crate_,
					&item,
				) {
					Ok(function) => function.to_tokens(&mut output),
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

				Type::new(
					Hygiene::Hygiene {
						js_sys: js_sys.as_ref(),
					},
					item,
				)
				.to_tokens(&mut output);
			}
			item => {
				error.push(Error::new_spanned(
					item,
					"expected foreign function or type ",
				));
			}
		}
	}

	if let Some(error) = error.into_token_stream() {
		output.extend(error);
		Err(output)
	} else {
		Ok(output)
	}
}

struct ErrorStack(Option<Error>);

impl ErrorStack {
	fn new() -> Self {
		Self(None)
	}

	fn push(&mut self, error: Error) {
		match &mut self.0 {
			Some(this) => this.combine(error),
			None => self.0 = Some(error),
		}
	}

	fn into_token_stream(self) -> Option<TokenStream> {
		self.0.map(Error::into_compile_error)
	}
}
