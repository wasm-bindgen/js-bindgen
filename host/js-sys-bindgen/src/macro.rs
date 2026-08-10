use std::collections::{HashMap, VecDeque};
use std::env;

use proc_macro2::TokenStream;
use quote::ToTokens;
#[cfg(any(feature = "file", test))]
use syn::File;
use syn::parse::Parser;
use syn::{Attribute, Error, ForeignItem, Item, ItemForeignMod, LitStr, Path, meta};

use crate::function::{FunctionImport, expand};
use crate::hygiene::Hygiene;
#[cfg(feature = "file")]
use crate::hygiene::ImportManager;
use crate::r#type::{Type, TypeOptions};

pub fn r#macro(attr: TokenStream, item: TokenStream) -> Result<TokenStream, TokenStream> {
	match syn::parse2(item).map_err(Error::into_compile_error)? {
		Item::ForeignMod(foreign_mod) => {
			let crate_name = env::var("CARGO_CRATE_NAME").expect("`CARGO_CRATE_NAME` not found");

			expand_proc_macro(attr, foreign_mod, &crate_name)
				.map(GeneratedItems::into_token_stream)
				.map_err(|(output, error)| {
					let error = error.into_compile_error();

					if let Some(output) = output {
						let mut output = output.into_token_stream();
						output.extend(error);
						output
					} else {
						error
					}
				})
		}
		Item::Fn(function) => {
			crate::export::r#macro(attr, &function, None).map_err(Error::into_compile_error)
		}
		item => Err(
			Error::new_spanned(item, "expected an extern block or function").into_compile_error(),
		),
	}
}

#[cfg(feature = "file")]
pub(crate) fn expand_file(
	attr: TokenStream,
	foreign_mod: ItemForeignMod,
	crate_name: &str,
	imports: &mut ImportManager,
) -> Result<GeneratedItems, (Option<GeneratedItems>, Error)> {
	let (_, namespace, error) = parse_block_options(attr, false);

	expand_foreign_mod(
		foreign_mod,
		crate_name,
		namespace.as_deref(),
		Hygiene::Imports(imports),
		error,
	)
}

#[cfg(test)]
pub(crate) fn expand_for_test(
	attr: TokenStream,
	foreign_mod: ItemForeignMod,
	crate_name: &str,
) -> Result<GeneratedItems, (Option<GeneratedItems>, Error)> {
	expand_proc_macro(attr, foreign_mod, crate_name)
}

fn expand_proc_macro(
	attr: TokenStream,
	foreign_mod: ItemForeignMod,
	crate_name: &str,
) -> Result<GeneratedItems, (Option<GeneratedItems>, Error)> {
	let (js_sys, namespace, error) = parse_block_options(attr, true);

	expand_foreign_mod(
		foreign_mod,
		crate_name,
		namespace.as_deref(),
		Hygiene::Qualified {
			js_sys: js_sys.as_ref(),
		},
		error,
	)
}

fn parse_block_options(
	attr: TokenStream,
	allow_js_sys_path: bool,
) -> (Option<Path>, Option<String>, ErrorStack) {
	let mut error = ErrorStack::new();
	let mut js_sys: Option<Path> = None;
	let mut namespace: Option<String> = None;

	if let Err(e) = meta::parser(|meta| {
		if meta.path.is_ident("js_sys") {
			// The block-level `js_sys` option selects the crate path used by
			// every generated item in this foreign module.
			if !allow_js_sys_path {
				Err(meta.error("`js_sys` attribute only allowed with proc-macro hygiene"))
			} else if js_sys.is_some() {
				Err(meta.error("duplicate attribute"))
			} else {
				js_sys = Some(meta.value()?.parse()?);
				Ok(())
			}
		} else if meta.path.is_ident("namespace") {
			// The block-level `namespace` prefixes every generated JavaScript
			// global path and import symbol in this foreign module.
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

	(js_sys, namespace, error)
}

fn expand_foreign_mod(
	mut foreign_mod: ItemForeignMod,
	crate_name: &str,
	namespace: Option<&str>,
	mut hygiene: Hygiene<'_>,
	mut error: ErrorStack,
) -> Result<GeneratedItems, (Option<GeneratedItems>, Error)> {
	for attr in foreign_mod
		.attrs
		.extract_if(.., |attr| attr.path().is_ident("js_sys"))
	{
		error.push(Error::new_spanned(
			attr,
			"`js_sys` attribute not supported at that position",
		));
	}

	let mut output = GeneratedItems::default();
	let mut function_imports = Vec::new();
	let mut type_options = VecDeque::new();
	let mut js_names = HashMap::new();

	if foreign_mod
		.abi
		.name
		.as_ref()
		.is_none_or(|value| value.value() != "js-sys")
	{
		error.push(Error::new_spanned(
			&foreign_mod.abi.name,
			"expected `js-sys` ABI",
		));
	}

	for item in &mut foreign_mod.items {
		if let ForeignItem::Type(item) = item {
			let options = TypeOptions::parse(item, |e| error.push(e));
			let rust_name = item.ident.to_string();
			let js_name = options.js_name.clone().unwrap_or_else(|| rust_name.clone());

			js_names.insert(rust_name.clone(), js_name);
			type_options.push_back(options);
		}
	}

	for item in foreign_mod.items {
		match item {
			ForeignItem::Fn(item) => {
				match expand(&mut hygiene, namespace, crate_name, &js_names, item) {
					Ok((function, import)) => {
						output.push(&function);
						function_imports.push(import);
					}
					Err(e) => error.push(e),
				}
			}
			ForeignItem::Type(item) => {
				let options = type_options
					.pop_front()
					.expect("all foreign types were parsed in the first pass");

				for item in Type::with_extends(&mut hygiene, item, &options.extends) {
					output.push(&item);
				}
			}
			item => {
				error.push(Error::new_spanned(
					item,
					"expected foreign function or type ",
				));
			}
		}
	}

	output.extend(render_import_groups(function_imports));

	if let Some(error) = error.resolve() {
		Err((Some(output), error))
	} else {
		Ok(output)
	}
}

#[derive(Debug, Default)]
pub(crate) struct GeneratedItems(TokenStream);

impl GeneratedItems {
	fn push(&mut self, item: &impl ToTokens) {
		item.to_tokens(&mut self.0);
	}

	fn extend(&mut self, items: TokenStream) {
		self.0.extend(items);
	}

	pub(crate) fn into_token_stream(self) -> TokenStream {
		self.0
	}

	#[cfg(any(feature = "file", test))]
	pub(crate) fn into_items(self) -> Result<Vec<Item>, Error> {
		Ok(syn::parse2::<File>(self.0)?.items)
	}
}

struct ImportGroup {
	cfg_attrs: Vec<Attribute>,
	descriptors: Vec<TokenStream>,
	needs_js_section: bool,
	macro_path: Path,
}

fn render_import_groups(imports: Vec<FunctionImport>) -> TokenStream {
	let mut groups: Vec<ImportGroup> = Vec::new();

	for import in imports {
		// One section static may cover every import with the same conditional
		// compilation boundary. Keeping the original attributes on a containing
		// item also preserves arbitrary `cfg_attr` expansions.
		if let Some(group) = groups
			.iter_mut()
			.find(|group| group.cfg_attrs == import.cfg_attrs)
		{
			group.descriptors.push(import.descriptor);
			group.needs_js_section |= import.needs_js_section;
		} else {
			groups.push(ImportGroup {
				cfg_attrs: import.cfg_attrs,
				descriptors: vec![import.descriptor],
				needs_js_section: import.needs_js_section,
				macro_path: import.macro_path,
			});
		}
	}

	groups
		.into_iter()
		.fold(TokenStream::new(), |mut output, group| {
			let ImportGroup {
				cfg_attrs,
				descriptors,
				needs_js_section,
				macro_path,
			} = group;
			let js = needs_js_section.then(|| {
				quote::quote! {
					const JS_CAPACITY: ::core::primitive::usize =
						#macro_path::import_js_capacity(IMPORTS);

					#[used]
					#[unsafe(link_section = "js_bindgen.import")]
					static JS_SECTION: #macro_path::ImportSection<JS_CAPACITY> =
						#macro_path::import_js::<JS_CAPACITY>(IMPORTS);
				}
			});

			output.extend(quote::quote! {
				#(#cfg_attrs)*
				const _: () = {
					const IMPORTS: &[#macro_path::ImportDescriptor] = &[#(#descriptors),*];
					const WAT_CAPACITY: ::core::primitive::usize =
						#macro_path::import_wat_capacity(IMPORTS);

					#[used]
					#[unsafe(link_section = "js_bindgen.wat")]
					static WAT_SECTION: #macro_path::ImportSection<WAT_CAPACITY> =
						#macro_path::import_wat::<WAT_CAPACITY>(IMPORTS);

					#js
				};
			});
			output
		})
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
