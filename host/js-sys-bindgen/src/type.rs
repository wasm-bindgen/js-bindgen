use std::array;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote_spanned};
use syn::spanned::Spanned;
use syn::{Fields, ForeignItemType, Item, ItemImpl, ItemStruct, Token, parse_quote_spanned};

use crate::Hygiene;

pub struct Type {
	pub r#struct: ItemStruct,
	pub impls: [ItemImpl; 5],
}

impl Type {
	#[must_use]
	pub fn new(hygiene: &mut Hygiene<'_>, item: ForeignItemType) -> Self {
		let span = item.span();
		let ForeignItemType {
			attrs,
			vis,
			ident,
			generics,
			..
		} = item;

		let mut item_attrs = attrs;
		let mut cfgs: Vec<_> = item_attrs
			.extract_if(.., |attr| attr.path().is_ident("cfg"))
			.collect();

		let js_value = hygiene.js_value(&cfgs, span);
		let js_cast = hygiene.js_cast(&cfgs, span);
		let into_js = hygiene.js_into(&cfgs, span);
		let option_into_js = hygiene.js_option_into(&cfgs, span);
		let as_ref = hygiene.as_ref(span);
		let from = hygiene.from(span);

		let (gen_impl, gen_type, gen_where) = generics.split_for_impl();

		let (fields, semi_token, value) = if generics.params.is_empty() {
			(
				Fields::Unnamed(parse_quote_spanned! {span=>(#js_value)}),
				Some(Token![;](span)),
				quote_spanned! {span=>0},
			)
		} else {
			let phantom_data = hygiene.phantom_data(&cfgs, span);

			(
				Fields::Named(parse_quote_spanned! {span=>
					{
						value: #js_value,
						_type: #phantom_data #gen_type,
					}
				}),
				None,
				quote_spanned! {span=>value},
			)
		};

		let impls = [
			parse_quote_spanned! {span=>
				#(#cfgs)*
				impl #gen_impl #as_ref<#js_value> for #ident #gen_type #gen_where {
					fn as_ref(&self) -> &#js_value {
						&self.#value
					}
				}
			},
			parse_quote_spanned! {span=>
				#(#cfgs)*
				impl #gen_impl #from<#ident #gen_type> for #js_value #gen_where {
					fn from(value: #ident #gen_type) -> Self {
						value.#value
					}
				}
			},
			parse_quote_spanned! {span=>
				#(#cfgs)*
				unsafe impl #gen_impl #js_cast for #ident #gen_type #gen_where {}
			},
			parse_quote_spanned! {span=>
				#(#cfgs)*
				unsafe impl #gen_impl #into_js for #ident #gen_type #gen_where {
					type Abi = <#js_value as #into_js>::Abi;

					fn into_abi(self) -> Self::Abi {
						#into_js::into_abi(#js_value::from(self))
					}
				}
			},
			parse_quote_spanned! {span=>
				#(#cfgs)*
				unsafe impl #gen_impl #option_into_js for #ident #gen_type #gen_where {
					type OptionAbi = <#js_value as #option_into_js>::OptionAbi;

					fn option_into_abi(value: ::core::option::Option<Self>) -> Self::OptionAbi {
						#option_into_js::option_into_abi(value.map(#js_value::from))
					}
				}
			},
		];

		item_attrs.append(&mut cfgs);
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
	type IntoIter = array::IntoIter<Item, 6>;

	fn into_iter(self) -> Self::IntoIter {
		let [impl_1, impl_2, impl_3, impl_4, impl_5] = self.impls;
		[
			Item::from(self.r#struct),
			impl_1.into(),
			impl_2.into(),
			impl_3.into(),
			impl_4.into(),
			impl_5.into(),
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
