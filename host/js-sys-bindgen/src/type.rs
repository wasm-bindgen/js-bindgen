use proc_macro2::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
	Error, Fields, ForeignItemType, Item, ItemImpl, ItemStruct, LitStr, Path, Token,
	parse_quote_spanned,
};

use crate::hygiene::Hygiene;

pub(crate) struct Type {
	r#struct: ItemStruct,
	impls: Vec<ItemImpl>,
}

#[derive(Default)]
pub(crate) struct TypeOptions {
	/// JavaScript type name used by constructors and static members in this
	/// block.
	pub(crate) js_name: Option<String>,
	/// JavaScript parent types. The first parent is also the `Deref` target.
	pub(crate) extends: Vec<Path>,
}

impl TypeOptions {
	pub(crate) fn parse(item: &mut ForeignItemType, mut on_error: impl FnMut(Error)) -> Self {
		let mut options = Self::default();

		// Type-level `#[js_sys(...)]` attributes describe the foreign type itself;
		// function binding options are parsed separately.
		for attr in item
			.attrs
			.extract_if(.., |attr| attr.path().is_ident("js_sys"))
		{
			if let Err(error) = attr.parse_nested_meta(|meta| {
				if meta.path.is_ident("js_name") {
					let js_name = meta.value()?.parse::<LitStr>()?.value();

					if options.js_name.replace(js_name).is_some() {
						Err(meta.error("duplicate attribute"))
					} else {
						Ok(())
					}
				} else if meta.path.is_ident("extends") {
					options.extends.push(meta.value()?.parse()?);
					Ok(())
				} else {
					Err(meta.error("unsupported attribute"))
				}
			}) {
				on_error(error);
			}
		}

		options
	}
}

impl Type {
	#[cfg(any(feature = "web-idl", test))]
	#[must_use]
	pub(crate) fn new(hygiene: &mut Hygiene<'_>, item: ForeignItemType) -> Self {
		Self::with_extends(hygiene, item, &[])
	}

	#[must_use]
	pub(crate) fn with_extends(
		hygiene: &mut Hygiene<'_>,
		item: ForeignItemType,
		extends: &[Path],
	) -> Self {
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
			let marker_types: Vec<_> = generics
				.params
				.iter()
				.filter_map(|param| match param {
					syn::GenericParam::Lifetime(param) => {
						let lifetime = &param.lifetime;
						Some(quote_spanned!(span=> &#lifetime ()))
					}
					syn::GenericParam::Type(param) => {
						let ident = &param.ident;
						Some(quote_spanned!(span=> #ident))
					}
					syn::GenericParam::Const(_) => None,
				})
				.collect();
			let marker_type = match marker_types.as_slice() {
				[] => quote!(()),
				[ty] => quote!(#ty),
				types => quote!((#(#types,)*)),
			};

			(
				Fields::Named(parse_quote_spanned! {span=>
					{
						value: #js_value,
						_type: #phantom_data<#marker_type>,
					}
				}),
				None,
				quote_spanned! {span=>value},
			)
		};

		let mut impls = vec![
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
		];

		if let Some(parent) = extends.first() {
			let deref = hygiene.deref(&cfgs, span);

			impls.push(parse_quote_spanned! {span=>
				#(#cfgs)*
				impl #gen_impl #deref for #ident #gen_type #gen_where {
					type Target = #parent;

					#[inline]
					fn deref(&self) -> &Self::Target {
						<#ident #gen_type as #as_ref<#parent>>::as_ref(self)
					}
				}
			});
		}

		for parent in extends {
			impls.push(parse_quote_spanned! {span=>
				#(#cfgs)*
				impl #gen_impl #as_ref<#parent> for #ident #gen_type #gen_where {
					#[inline]
					fn as_ref(&self) -> &#parent {
						<#parent as #js_cast>::unchecked_from_ref(
							<#ident #gen_type as #as_ref<#js_value>>::as_ref(self),
						)
					}
				}
			});
			impls.push(parse_quote_spanned! {span=>
				#(#cfgs)*
				impl #gen_impl #from<#ident #gen_type> for #parent #gen_where {
					#[inline]
					fn from(value: #ident #gen_type) -> Self {
						<#parent as #js_cast>::unchecked_from(#js_value::from(value))
					}
				}
			});
		}

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
	type IntoIter = std::vec::IntoIter<Item>;

	fn into_iter(self) -> Self::IntoIter {
		let mut items = Vec::with_capacity(self.impls.len() + 1);
		items.push(Item::from(self.r#struct));
		items.extend(self.impls.into_iter().map(Item::from));
		items.into_iter()
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
