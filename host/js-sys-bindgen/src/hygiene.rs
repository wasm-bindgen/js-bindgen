#[cfg(feature = "web-idl")]
use foldhash::fast::FixedState;
#[cfg(feature = "web-idl")]
use hashbrown::{HashMap, HashSet};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Attribute, Ident, Path, parse_quote_spanned};
#[cfg(feature = "web-idl")]
use syn::{ItemUse, parse_quote};

pub(crate) enum Hygiene<'a> {
	/// Source generation mode: emit short paths and record the required `use`
	/// items.
	#[cfg(feature = "web-idl")]
	Imports(&'a mut ImportManager),
	/// Procedural macro mode: emit paths qualified through the selected crate.
	Qualified { js_sys: Option<&'a Path> },
}

#[cfg_attr(
	not(feature = "web-idl"),
	expect(
		unused_variables,
		reason = "attributes are only consumed by source-generation hygiene"
	)
)]
impl Hygiene<'_> {
	pub(crate) fn js_value(&mut self, attrs: &[Attribute], span: Span) -> Path {
		self.js_sys_item(attrs, &parse_quote_spanned!(span=> JsValue), span)
	}

	pub(crate) fn js_cast(&mut self, attrs: &[Attribute], span: Span) -> Path {
		self.hazard_item(attrs, &parse_quote_spanned!(span=> JsCast), span)
	}

	pub(crate) fn js_into(&mut self, attrs: &[Attribute], span: Span) -> Path {
		self.hazard_item(attrs, &parse_quote_spanned!(span=> IntoJS), span)
	}

	pub(crate) fn r#macro(&mut self, attrs: &[Attribute], span: Span) -> Path {
		self.js_sys_item(attrs, &parse_quote_spanned!(span=> wire), span)
	}

	fn js_sys_item(&mut self, attrs: &[Attribute], ident: &Ident, span: Span) -> Path {
		match self {
			#[cfg(feature = "web-idl")]
			Hygiene::Imports(imports) => {
				imports.js_sys_push(attrs, ident.clone());
				parse_quote_spanned!(span=> #ident)
			}
			Hygiene::Qualified { js_sys } => {
				Self::with_js_sys(*js_sys, &ident.to_token_stream(), span)
			}
		}
	}

	fn hazard_item(&mut self, attrs: &[Attribute], ident: &Ident, span: Span) -> Path {
		match self {
			#[cfg(feature = "web-idl")]
			Hygiene::Imports(imports) => {
				imports.hazard_push(attrs, ident.clone());
				parse_quote_spanned!(span=> #ident)
			}
			Hygiene::Qualified { js_sys } => {
				Self::with_js_sys(*js_sys, &quote!(hazard::#ident), span)
			}
		}
	}

	pub(crate) fn as_ref(&mut self, span: Span) -> Path {
		match self {
			#[cfg(feature = "web-idl")]
			Hygiene::Imports(_) => {
				parse_quote_spanned!(span=> AsRef)
			}
			Hygiene::Qualified { .. } => {
				parse_quote_spanned!(span=> ::core::convert::AsRef)
			}
		}
	}

	pub(crate) fn deref(&mut self, attrs: &[Attribute], span: Span) -> Path {
		match self {
			#[cfg(feature = "web-idl")]
			Hygiene::Imports(imports) => {
				imports.deref.insert(attrs.to_vec());
				parse_quote_spanned!(span=> Deref)
			}
			Hygiene::Qualified { .. } => {
				parse_quote_spanned!(span=> ::core::ops::Deref)
			}
		}
	}

	pub(crate) fn phantom_data(&mut self, attrs: &[Attribute], span: Span) -> Path {
		match self {
			#[cfg(feature = "web-idl")]
			Hygiene::Imports(imports) => {
				imports
					.phantom_data
					.get_or_insert_with(attrs, <[_]>::to_vec);
				parse_quote_spanned!(span=> PhantomData)
			}
			Hygiene::Qualified { .. } => {
				parse_quote_spanned!(span=> ::core::marker::PhantomData)
			}
		}
	}

	pub(crate) fn from(&mut self, span: Span) -> Path {
		match self {
			#[cfg(feature = "web-idl")]
			Hygiene::Imports(_) => {
				parse_quote_spanned!(span=> From)
			}
			Hygiene::Qualified { .. } => {
				parse_quote_spanned!(span=> ::core::convert::From)
			}
		}
	}

	fn with_js_sys(js_sys: Option<&Path>, path: &TokenStream, span: Span) -> Path {
		if let Some(js_sys) = js_sys {
			parse_quote_spanned!(span=> #js_sys::#path)
		} else {
			parse_quote_spanned!(span=> ::js_sys::#path)
		}
	}
}

#[cfg(feature = "web-idl")]
type FixedHashMap<K, V> = HashMap<K, V, FixedState>;
#[cfg(feature = "web-idl")]
type FixedHashSet<T> = HashSet<T, FixedState>;

#[cfg(feature = "web-idl")]
pub(crate) struct ImportManager {
	js_sys: Path,
	deref: FixedHashSet<Vec<Attribute>>,
	phantom_data: FixedHashSet<Vec<Attribute>>,
	js_sys_imports: FixedHashMap<Vec<Attribute>, FixedHashSet<Ident>>,
	hazard_imports: FixedHashMap<Vec<Attribute>, FixedHashSet<Ident>>,
}

#[cfg(feature = "web-idl")]
impl ImportManager {
	#[must_use]
	pub(crate) fn new(js_sys: Option<Path>) -> Self {
		Self {
			js_sys: js_sys.unwrap_or_else(|| parse_quote! { js_sys }),
			deref: FixedHashSet::default(),
			phantom_data: FixedHashSet::default(),
			js_sys_imports: FixedHashMap::default(),
			hazard_imports: FixedHashMap::default(),
		}
	}

	pub(crate) fn iter(&self) -> impl Iterator<Item = ItemUse> {
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

#[cfg(feature = "web-idl")]
impl ToTokens for ImportManager {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		for item_use in self.iter() {
			item_use.to_tokens(tokens);
		}
	}
}
