use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

fn expand(attr: TokenStream, input: syn::ItemForeignMod) -> String {
	let items = crate::r#macro::expand_for_test(attr, input, "test_crate")
		.unwrap()
		.into_items()
		.unwrap();

	prettyplease::unparse(&syn::File {
		shebang: None,
		attrs: Vec::new(),
		items,
	})
}

fn link(input: syn::ItemForeignMod) -> (Option<String>, Option<String>) {
	let output = expand(TokenStream::new(), input);
	let dir = tempfile::tempdir().unwrap();
	let (wat, js, _) = super::inner(dir.path(), &output).unwrap();
	(wat, js)
}

#[test]
fn imports_are_batched_end_to_end() {
	let (wat, js) = link(syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(js_import)]
			pub fn first(value: &JsValue);

			#[js_sys(js_import)]
			pub fn second(value: &JsValue);
		}
	});
	let wat = wat.unwrap();

	assert!(wat.contains(r#"(import "test_crate" "first""#));
	assert!(wat.contains(r#"(import "test_crate" "second""#));
	assert_eq!(
		wat.matches(r#"(import "js_sys" "externref.table""#).count(),
		1
	);
	assert_eq!(
		wat.matches("table.get $js_sys.import.externref.table")
			.count(),
		2
	);
	assert_eq!(js, None);
}

#[test]
fn binding_options() {
	let output = expand(
		quote!(js_sys = renamed, namespace = "console"),
		syn::parse_quote! {
			extern "js-sys" {
				pub fn log(value: &JsValue);

				#[js_sys(js_name = "warn")]
				pub fn renamed_log(value: &JsValue);

				#[js_sys(return_abi = JsValue)]
				pub fn value() -> JsTest;

				#[cfg(all())]
				pub fn configured();
			}
		},
	);

	assert!(output.contains("renamed::r#macro::InputSlot1"));
	assert!(output.contains(r#"direct_call: "globalThis.console.log(arg0_0)""#));
	assert!(output.contains(r#"direct_call: "globalThis.console.warn(arg0_0)""#));
	assert!(output.contains("join_output_as::<JsTest, JsValue>"));
	assert!(output.contains("#[cfg(all())]"));

	let output = expand(
		TokenStream::new(),
		syn::parse_quote! {
			extern "js-sys" {
				#[js_sys(js_import)]
				pub fn imported();

				#[js_sys(js_embed = "embed")]
				pub fn embedded();
			}
		},
	);
	assert!(output.contains("::core::option::Option::None"));
	assert!(output.contains(r#"direct_call: "this.#jsEmbed.test_crate['embed']""#));
}

#[test]
fn suspending_end_to_end() {
	let (_, js) = link(syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(suspending)]
			pub fn wait() -> u128;
		}
	});

	assert_eq!(
		js.unwrap(),
		"new WebAssembly.Suspending(async ($retptr) => {\n    $retptr = $retptr >>> 0\n    const \
		 $ret = await (globalThis.wait())\n    this.#jsEmbed.js_sys['numeric.128.encode']($ret, \
		 $ret >> 64n, $retptr)\n})",
	);
}

#[test]
fn preserves_successful_functions_after_an_error() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			pub fn good(value: i32) -> i32;
			pub async fn bad();
		}
	};
	let (Some(output), error) =
		crate::r#macro::expand_for_test(TokenStream::new(), input, "test_crate").unwrap_err()
	else {
		panic!("expected the successful function to be preserved");
	};

	let items = output.into_items().unwrap();
	let functions: Vec<_> = items
		.iter()
		.filter_map(|item| match item {
			Item::Fn(function) => Some(function.sig.ident.to_string()),
			_ => None,
		})
		.collect();
	assert_eq!(functions, ["good"]);
	assert_eq!(error.to_string(), "`async` functions are not supported");
}

#[test]
fn invalid_options() {
	macro_rules! assert_error {
		($input:tt, $expected:literal) => {
			assert_eq!(super::macro_error(syn::parse_quote! $input), $expected);
		};
	}

	assert_error!(
		{
			extern "C" {
				pub fn log();
			}
		},
		"expected `js-sys` ABI"
	);
	assert_error!(
		{
			extern "js-sys" {
				#[js_sys(js_name = "renamed", js_import)]
				pub fn log();
			}
		},
		"`js_import` and `js_embed` cannot be combined with JavaScript binding options"
	);
	assert_error!(
		{
			extern "js-sys" {
				#[js_sys(js_import, suspending)]
				pub fn wait();
			}
		},
		"`suspending` cannot be combined with `js_import`; provide a `WebAssembly.Suspending` \
		 import directly"
	);
	assert_error!(
		{
			extern "js-sys" {
				#[js_sys(suspending, suspending)]
				pub fn wait();
			}
		},
		"duplicate attribute"
	);
	assert_error!(
		{
			extern "js-sys" {
				pub fn log(
					#[js_sys(type = i32)]
					#[js_sys(type = u32)]
					value: i32,
				);
			}
		},
		"duplicate attribute"
	);
	assert_error!(
		{
			extern "js-sys" {
				#[js_sys(return_abi = JsValue)]
				pub fn value();
			}
		},
		"`return_abi` requires a return value"
	);
	assert_error!(
		{
			extern "js-sys" {
				#[js_sys(return_abi = JsValue, return_abi = JsTest)]
				pub fn value() -> JsValue;
			}
		},
		"duplicate attribute"
	);
}
