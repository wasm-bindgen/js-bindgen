use std::borrow::Cow;

use foldhash::fast::FixedState;
use hashbrown::{HashMap, HashSet};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Attribute, Ident, ItemUse, Path, parse_quote, parse_quote_spanned};

pub enum Hygiene<'a> {
	Imports(&'a mut ImportManager),
	Hygiene { js_sys: Option<&'a Path> },
}

impl Hygiene<'_> {
	pub(crate) fn js_value(&mut self, attrs: &[Attribute], span: Span) -> Path {
		match self {
			Hygiene::Imports(imports) => {
				imports.js_sys_push(attrs, parse_quote_spanned!(span=>JsValue));
				parse_quote_spanned!(span=>JsValue)
			}
			Hygiene::Hygiene { js_sys } => Self::with_js_sys(*js_sys, &quote!(JsValue), span),
		}
	}

	pub(crate) fn js_bindgen(&mut self, attrs: &[Attribute], span: Span) -> Path {
		match self {
			Hygiene::Imports(imports) => {
				imports.js_sys_push(attrs, parse_quote_spanned! {span=>js_bindgen});
				parse_quote_spanned!(span=> js_bindgen)
			}
			Hygiene::Hygiene { js_sys } => Self::with_js_sys(*js_sys, &quote!(js_bindgen), span),
		}
	}

	pub(crate) fn input(&mut self, attrs: &[Attribute], span: Span) -> Path {
		match self {
			Hygiene::Imports(imports) => {
				imports.hazard_push(attrs, parse_quote_spanned! {span=>Input});
				parse_quote_spanned!(span=> Input)
			}
			Hygiene::Hygiene { js_sys } => Self::with_js_sys(*js_sys, &quote!(hazard::Input), span),
		}
	}

	pub(crate) fn input_asm_conv(&mut self, attrs: &[Attribute], span: Span) -> Path {
		match self {
			Hygiene::Imports(imports) => {
				imports.hazard_push(attrs, parse_quote_spanned! {span=>InputAsmConv});
				parse_quote_spanned!(span=> InputAsmConv)
			}
			Hygiene::Hygiene { js_sys } => {
				Self::with_js_sys(*js_sys, &quote!(hazard::InputAsmConv), span)
			}
		}
	}

	pub(crate) fn input_js_conv(&mut self, attrs: &[Attribute], span: Span) -> Path {
		match self {
			Hygiene::Imports(imports) => {
				imports.hazard_push(attrs, parse_quote_spanned! {span=>InputJsConv});
				parse_quote_spanned!(span=> InputJsConv)
			}
			Hygiene::Hygiene { js_sys } => {
				Self::with_js_sys(*js_sys, &quote!(hazard::InputJsConv), span)
			}
		}
	}

	pub(crate) fn output(&mut self, attrs: &[Attribute], span: Span) -> Path {
		match self {
			Hygiene::Imports(imports) => {
				imports.hazard_push(attrs, parse_quote_spanned! {span=>Output});
				parse_quote_spanned!(span=> Output)
			}
			Hygiene::Hygiene { js_sys } => {
				Self::with_js_sys(*js_sys, &quote!(hazard::Output), span)
			}
		}
	}

	pub(crate) fn output_asm_conv(&mut self, attrs: &[Attribute], span: Span) -> Path {
		match self {
			Hygiene::Imports(imports) => {
				imports.hazard_push(attrs, parse_quote_spanned! {span=>OutputAsmConv});
				parse_quote_spanned!(span=> OutputAsmConv)
			}
			Hygiene::Hygiene { js_sys } => {
				Self::with_js_sys(*js_sys, &quote!(hazard::OutputAsmConv), span)
			}
		}
	}

	pub(crate) fn output_js_conv(&mut self, attrs: &[Attribute], span: Span) -> Path {
		match self {
			Hygiene::Imports(imports) => {
				imports.hazard_push(attrs, parse_quote_spanned! {span=>OutputJsConv});
				parse_quote_spanned!(span=> OutputJsConv)
			}
			Hygiene::Hygiene { js_sys } => {
				Self::with_js_sys(*js_sys, &quote!(hazard::OutputJsConv), span)
			}
		}
	}

	pub(crate) fn r#macro(&mut self, attrs: &[Attribute], span: Span) -> Path {
		match self {
			Hygiene::Imports(imports) => {
				imports.js_sys_push(attrs, parse_quote_spanned! {span=>r#macro});
				parse_quote_spanned!(span=> r#macro)
			}
			Hygiene::Hygiene { js_sys } => Self::with_js_sys(*js_sys, &quote!(r#macro), span),
		}
	}

	pub(crate) fn as_ref(&mut self, span: Span) -> Path {
		match self {
			Hygiene::Imports(_) => {
				parse_quote_spanned!(span=> AsRef)
			}
			Hygiene::Hygiene { .. } => {
				parse_quote_spanned!(span=> ::core::convert::AsRef)
			}
		}
	}

	pub(crate) fn phantom_data(&mut self, attrs: &[Attribute], span: Span) -> Path {
		match self {
			Hygiene::Imports(imports) => {
				imports
					.phantom_data
					.get_or_insert_with(attrs, <[_]>::to_vec);
				parse_quote_spanned!(span=> PhantomData)
			}
			Hygiene::Hygiene { .. } => {
				parse_quote_spanned!(span=> ::core::marker::PhantomData)
			}
		}
	}

	pub(crate) fn str(&mut self, span: Span) -> Path {
		match self {
			Hygiene::Imports(_) => {
				parse_quote_spanned!(span=> str)
			}
			Hygiene::Hygiene { .. } => {
				parse_quote_spanned!(span=> ::core::primitive::str)
			}
		}
	}

	pub(crate) fn from(&mut self, span: Span) -> Path {
		match self {
			Hygiene::Imports(_) => {
				parse_quote_spanned!(span=> From)
			}
			Hygiene::Hygiene { .. } => {
				parse_quote_spanned!(span=> ::core::convert::From)
			}
		}
	}

	pub(crate) fn option(&mut self, span: Span) -> Path {
		match self {
			Hygiene::Imports(_) => {
				parse_quote_spanned!(span=> Option)
			}
			Hygiene::Hygiene { .. } => {
				parse_quote_spanned!(span=> ::core::option::Option)
			}
		}
	}

	fn with_js_sys(js_sys: Option<&Path>, path: &TokenStream, span: Span) -> Path {
		let js_sys = js_sys.map_or_else(
			|| Cow::Owned(parse_quote_spanned!(span=> ::js_sys)),
			Cow::Borrowed,
		);

		parse_quote_spanned!(span=> #js_sys::#path)
	}
}

type FixedHashMap<K, V> = HashMap<K, V, FixedState>;
type FixedHashSet<T> = HashSet<T, FixedState>;

pub struct ImportManager {
	js_sys: Path,
	deref: FixedHashSet<Vec<Attribute>>,
	phantom_data: FixedHashSet<Vec<Attribute>>,
	js_sys_imports: FixedHashMap<Vec<Attribute>, FixedHashSet<Ident>>,
	hazard_imports: FixedHashMap<Vec<Attribute>, FixedHashSet<Ident>>,
}

impl ImportManager {
	#[must_use]
	pub fn new(js_sys: Option<Path>) -> Self {
		Self {
			js_sys: js_sys.unwrap_or_else(|| parse_quote! { js_sys }),
			deref: FixedHashSet::default(),
			phantom_data: FixedHashSet::default(),
			js_sys_imports: FixedHashMap::default(),
			hazard_imports: FixedHashMap::default(),
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = ItemUse> {
		self.phantom_data
			.iter()
			.map(|attr| {
				parse_quote! {
					#(#attr)*
					use core::marker::PhantomData;
				}
			})
			.chain(self.deref.iter().map(|attr| {
				parse_quote! {
					#(#attr)*
					use core::ops::Deref;
				}
			}))
			.chain(self.js_sys_imports.iter().filter_map(|(attrs, types)| {
				let js_sys = &self.js_sys;
				let types = types.iter();

				match types.len() {
					0 => None,
					1 => Some(parse_quote! { #(#attrs)* use #js_sys::#(#types)*; }),
					_ => Some(parse_quote! { #(#attrs)* use #js_sys::{#(#types),*}; }),
				}
			}))
			.chain(self.hazard_imports.iter().filter_map(|(attrs, types)| {
				let js_sys = &self.js_sys;
				let types = types.iter();

				match types.len() {
					0 => None,
					1 => Some(parse_quote! { #(#attrs)* use #js_sys::hazard::#(#types)*; }),
					_ => Some(parse_quote! { #(#attrs)* use #js_sys::hazard::{#(#types),*}; }),
				}
			}))
	}

	pub(crate) fn js_sys_push(&mut self, attrs: &[Attribute], path: Ident) {
		self.js_sys_imports
			.entry_ref(attrs)
			.or_default()
			.insert(path);
	}

	pub(crate) fn hazard_push(&mut self, attrs: &[Attribute], path: Ident) {
		self.hazard_imports
			.entry_ref(attrs)
			.or_default()
			.insert(path);
	}
}

impl ToTokens for ImportManager {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		for item_use in self.iter() {
			item_use.to_tokens(tokens);
		}
	}
}
