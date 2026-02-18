use std::array;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote_spanned};
use syn::spanned::Spanned;
use syn::{Fields, ForeignItemType, Item, ItemImpl, ItemStruct, Token, parse_quote_spanned};

use crate::Hygiene;

pub struct Type {
	pub r#struct: ItemStruct,
	pub impls: [ItemImpl; 3],
}

impl Type {
	#[must_use]
	pub fn new(mut hygiene: Hygiene<'_>, item: ForeignItemType) -> Self {
		let span = item.span();
		let ForeignItemType {
			attrs,
			vis,
			ident,
			generics,
			..
		} = item;

		let js_value = hygiene.js_value(&attrs, span);
		let input = hygiene.input(&attrs, span);
		let output = hygiene.output(&attrs, span);
		let deref = hygiene.deref(&attrs, span);
		let str = hygiene.str(span);

		let mut item_attrs = attrs;
		let attrs: Vec<_> = item_attrs
			.iter()
			.filter(|attr| !attr.path().is_ident("doc"))
			.collect();

		let (gen_impl, gen_type, gen_where) = generics.split_for_impl();

		let (fields, semi_token, value, from_raw) = if generics.params.is_empty() {
			(
				Fields::Unnamed(parse_quote_spanned! {span=>(#js_value)}),
				Some(Token![;](span)),
				quote_spanned! {span=>0},
				quote_spanned! {span=>Self(#output::from_raw(raw))},
			)
		} else {
			let phantom_data = hygiene.phantom_data(&item_attrs, span);

			(
				Fields::Named(parse_quote_spanned! {span=>
					{
						value: #js_value,
						_type: #phantom_data #gen_type,
					}
				}),
				None,
				quote_spanned! {span=>value},
				quote_spanned! {span=>
					Self {
						value: #output::from_raw(raw),
						_type: #phantom_data,
					}
				},
			)
		};

		let impls: [ItemImpl; 3] = [
			parse_quote_spanned! {span=>
				#(#attrs)*
				impl #gen_impl #deref for #ident #gen_type #gen_where {
					type Target = #js_value;

					fn deref(&self) -> &Self::Target {
						&self.#value
					}
				}
			},
			parse_quote_spanned! {span=>
				#(#attrs)*
				unsafe impl #gen_impl #input for &#ident #gen_type #gen_where {
					const IMPORT_FUNC: &'static #str = <&#js_value as #input>::IMPORT_FUNC;
					const IMPORT_TYPE: &'static #str = <&#js_value as #input>::IMPORT_TYPE;
					const TYPE: &'static #str = <&#js_value as #input>::TYPE;
					const CONV: &'static #str = <&#js_value as #input>::CONV;

					type Type = <&'static #js_value as #input>::Type;

					fn into_raw(self) -> Self::Type {
						#input::into_raw(&self.#value)
					}
				}
			},
			parse_quote_spanned! {span=>
				#(#attrs)*
				unsafe impl #gen_impl #output for #ident #gen_type #gen_where {
					const IMPORT_FUNC: &#str = <#js_value as #output>::IMPORT_FUNC;
					const IMPORT_TYPE: &#str = <#js_value as #output>::IMPORT_TYPE;
					const TYPE: &#str = <#js_value as #output>::TYPE;
					const CONV: &#str = <#js_value as #output>::CONV;

					type Type = <#js_value as #output>::Type;

					fn from_raw(raw: Self::Type) -> Self {
						#from_raw
					}
				}
			},
		];

		item_attrs.push(parse_quote_spanned! {span=>#[repr(transparent)]});

		let r#struct = ItemStruct {
			attrs: item_attrs,
			vis,
			struct_token: Token![struct](span),
			ident,
			generics,
			fields,
			semi_token,
		};

		Self { r#struct, impls }
	}
}

impl IntoIterator for Type {
	type Item = Item;
	type IntoIter = array::IntoIter<Item, 4>;

	fn into_iter(self) -> Self::IntoIter {
		let [impl_1, impl_2, impl_3] = self.impls;
		[
			Item::from(self.r#struct),
			impl_1.into(),
			impl_2.into(),
			impl_3.into(),
		]
		.into_iter()
	}
}

impl ToTokens for Type {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.r#struct.to_tokens(tokens);

		for r#impl in &self.impls {
			r#impl.to_tokens(tokens);
		}
	}
}
