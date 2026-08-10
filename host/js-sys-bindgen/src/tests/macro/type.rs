use proc_macro2::TokenStream;

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
	let output = prettyplease::unparse(&syn::File {
		shebang: None,
		attrs: Vec::new(),
		items: output,
	});

	assert!(output.contains("pub struct Child<T: Sized = JsValue>"));
	assert!(output.contains("impl<T: Sized> ::core::convert::AsRef<JsTest> for Child<T>"));
	assert!(output.contains("impl<T: Sized> ::core::convert::From<Child<T>> for JsTest"));
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

	assert_eq!(output.len(), 15);
	for (index, item) in output.into_iter().enumerate() {
		let attrs = match item {
			syn::Item::Struct(item) => item.attrs,
			syn::Item::Impl(item) => item.attrs,
			item => panic!("unexpected generated item: {item:?}"),
		};
		let has_cfg_attr = attrs.iter().any(|attr| attr.path().is_ident("cfg_attr"));

		assert_eq!(has_cfg_attr, index == 0);
	}
}
