use proc_macro2::TokenStream;
use quote::quote;
use syn::File;

fn expand(attr: TokenStream, function: &TokenStream) -> (String, String) {
	let function = syn::parse2(quote! { #function }).unwrap();
	let output = crate::export::r#macro(attr, &function, Some("test_crate")).unwrap();
	let output = prettyplease::unparse(&syn::parse2::<File>(output).unwrap());
	let dir = tempfile::tempdir().unwrap();
	let (wat, js_import, js_export) = super::inner(dir.path(), &output).unwrap();

	assert_eq!(js_import, None);
	(wat.unwrap(), js_export.unwrap())
}

#[test]
fn named_indirect_export_end_to_end() {
	let (wat, js) = expand(
		quote!(js_name = concat!("module", "::add")),
		&quote! {
			pub fn add(value: u32, delta: u128) -> u128 {
				u128::from(value) + delta
			}
		},
	);

	assert!(wat.contains(r#"(@sym (name "__export_module::add"))"#));
	assert!(wat.contains("(param i32) (param i32) (param i64 i64)"));
	assert!(wat.contains("(result i64 i64)"));
	assert!(wat.contains("global.get $__stack_pointer"));
	assert!(wat.contains("i64.load offset=8"));
	let expected = [
		"(arg0, arg1) => {",
		"    const ret = wasmExports['module::add'](arg0, arg1, arg1 >> 64n)",
		"    return this.#jsEmbed.js_sys['numeric.u128.decode'](ret[0], ret[1])",
		"}",
	]
	.join("\n");
	assert_eq!(js, expected);
}

#[test]
fn promising_result_end_to_end() {
	let (wat, js) = expand(
		quote!(promising),
		&quote! {
			pub fn checked(value: u128) -> Result<u128, JsValue> {
				Ok(value)
			}
		},
	);

	assert!(wat.contains("(result i64 i64 i32 externref)"));
	assert!(wat.contains("global.get $__stack_pointer"));
	assert_eq!(
		js,
		"(() => {\n    const $promising = WebAssembly.promising(wasmExports['checked'])\n    \
		 return (arg0) => $promising(arg0, arg0 >> 64n).then(ret => {\n        if (ret[2] !== 0) \
		 throw ret[3]\n        return this.#jsEmbed.js_sys['numeric.u128.decode'](ret[0], \
		 ret[1])\n    })\n})()"
	);
}

#[test]
fn invalid_export_options() {
	let function: syn::ItemFn = syn::parse_quote! {
		fn answer() -> i32 {
			42
		}
	};
	let error = crate::export::r#macro(quote!(promising = true), &function, Some("test_crate"))
		.unwrap_err();
	assert_eq!(error.to_string(), "`promising` supports no values");

	let error = crate::export::r#macro(quote!(promising, promising), &function, Some("test_crate"))
		.unwrap_err();
	assert_eq!(error.to_string(), "duplicate `promising` argument");

	let borrowed: syn::ItemFn = syn::parse_quote! {
		fn echo(value: &JsString) -> &JsString {
			value
		}
	};
	let error =
		crate::export::r#macro(TokenStream::new(), &borrowed, Some("test_crate")).unwrap_err();
	assert_eq!(error.to_string(), "cannot return a borrowed reference");
}
