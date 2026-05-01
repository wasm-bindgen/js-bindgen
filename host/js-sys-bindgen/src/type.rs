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
		let input = hygiene.input(&cfgs, span);
		let input_asm_conv = hygiene.input_asm_conv(&cfgs, span);
		let input_js_conv = hygiene.input_js_conv(&cfgs, span);
		let js_cast = hygiene.js_cast(&cfgs, span);
		let output = hygiene.output(&cfgs, span);
		let output_asm_conv = hygiene.output_asm_conv(&cfgs, span);
		let output_js_conv = hygiene.output_js_conv(&cfgs, span);
		let as_ref = hygiene.as_ref(span);
		let str = hygiene.str(span);
		let from = hygiene.from(span);
		let option = hygiene.option(span);

		let (gen_impl, gen_type, gen_where) = generics.split_for_impl();

		let (fields, semi_token, value, from_raw) = if generics.params.is_empty() {
			(
				Fields::Unnamed(parse_quote_spanned! {span=>(#js_value)}),
				Some(Token![;](span)),
				quote_spanned! {span=>0},
				quote_spanned! {span=>Self(#output::from_raw(raw))},
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
				quote_spanned! {span=>
					Self {
						value: #output::from_raw(raw),
						_type: #phantom_data,
					}
				},
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
				unsafe impl #gen_impl #input for &#ident #gen_type #gen_where {
					const ASM_TYPE: &'static #str = <&#js_value as #input>::ASM_TYPE;
					const ASM_CONV: #option<#input_asm_conv> = <&#js_value as #input>::ASM_CONV;
					const JS_CONV: #option<#input_js_conv> = <&#js_value as #input>::JS_CONV;

					type Type = <&'static #js_value as #input>::Type;

					fn into_raw(self) -> Self::Type {
						#input::into_raw(&self.#value)
					}
				}
			},
			parse_quote_spanned! {span=>
				#(#cfgs)*
				unsafe impl #gen_impl #js_cast for #ident #gen_type #gen_where {}
			},
			parse_quote_spanned! {span=>
				#(#cfgs)*
				unsafe impl #gen_impl #output for #ident #gen_type #gen_where {
					const ASM_TYPE: &#str = <#js_value as #output>::ASM_TYPE;
					const ASM_CONV: #option<#output_asm_conv> = <#js_value as #output>::ASM_CONV;
					const JS_CONV: #option<#output_js_conv> = <#js_value as #output>::JS_CONV;

					type Type = <#js_value as #output>::Type;

					fn from_raw(raw: Self::Type) -> Self {
						#from_raw
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
