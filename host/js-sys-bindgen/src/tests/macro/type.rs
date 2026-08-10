use proc_macro2::TokenStream;
use syn::{GenericArgument, PathArguments, Type};

fn expand(input: syn::ItemForeignMod) -> Vec<syn::Item> {
	crate::r#macro::expand_for_test(TokenStream::new(), input, "test_crate")
		.unwrap()
		.into_items()
		.unwrap()
}

#[test]
fn generic_options_and_extends() {
	let output = expand(syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(extends = JsTest)]
			pub type Child<T: Sized = JsValue>;
		}
	});
	let child = output
		.iter()
		.find_map(|item| match item {
			syn::Item::Struct(item) if item.ident == "Child" => Some(item),
			_ => None,
		})
		.expect("Child struct was generated");
	assert_eq!(child.generics, syn::parse_quote!(<T: Sized = JsValue>));

	let impls: Vec<_> = output
		.iter()
		.filter_map(|item| match item {
			syn::Item::Impl(item) => Some(item),
			_ => None,
		})
		.collect();
	assert!(impls.iter().any(|item| {
		trait_argument(item, "AsRef").is_some_and(|argument| type_is(argument, "JsTest"))
			&& type_is(&item.self_ty, "Child")
	}));
	assert!(impls.iter().any(|item| {
		trait_argument(item, "From").is_some_and(|argument| type_is(argument, "Child"))
			&& type_is(&item.self_ty, "JsTest")
	}));
}

fn trait_argument<'a>(item: &'a syn::ItemImpl, name: &str) -> Option<&'a Type> {
	let (_, path, _) = item.trait_.as_ref()?;
	let segment = path.segments.last()?;
	if segment.ident != name {
		return None;
	}
	let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
		return None;
	};
	match arguments.args.first()? {
		GenericArgument::Type(ty) => Some(ty),
		_ => None,
	}
}

fn type_is(ty: &Type, name: &str) -> bool {
	let Type::Path(ty) = ty else {
		return false;
	};
	ty.path
		.segments
		.last()
		.is_some_and(|segment| segment.ident == name)
}

#[test]
fn attributes_are_scoped_and_duplicate_names_do_not_panic() {
	let output = expand(syn::parse_quote! {
		extern "js-sys" {
			#[cfg_attr(all(), derive(Clone))]
			pub type First;
			pub type Duplicate;
			pub type Duplicate;
		}
	});

	let structs: Vec<_> = output
		.iter()
		.filter_map(|item| match item {
			syn::Item::Struct(item) => Some(item),
			_ => None,
		})
		.collect();
	assert_eq!(structs.len(), 3);
	assert_eq!(structs[0].ident, "First");
	assert_eq!(structs[1].ident, "Duplicate");
	assert_eq!(structs[2].ident, "Duplicate");

	let configured: Vec<_> = output
		.iter()
		.filter(|item| {
			match item {
				syn::Item::Struct(item) => &item.attrs,
				syn::Item::Impl(item) => &item.attrs,
				item => panic!("unexpected generated item: {item:?}"),
			}
			.iter()
			.any(|attr| attr.path().is_ident("cfg_attr"))
		})
		.collect();
	assert_eq!(configured.len(), 1);
	assert!(matches!(configured[0], syn::Item::Struct(item) if item.ident == "First"));
}
