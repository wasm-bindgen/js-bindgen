use proc_macro2::TokenStream;
use quote::quote;

fn test_js_bindgen(attr: TokenStream, item: TokenStream, expected: TokenStream) {
	let output = crate::js_bingen_internal(attr, item).unwrap();

	let output = syn::parse2(output).unwrap();
	let output = prettyplease::unparse(&output);
	let expected = syn::parse2(expected).unwrap();
	let expected = prettyplease::unparse(&expected);

	similar_asserts::assert_eq!(expected, output);
}

#[test]
fn basic() {
	test_js_bindgen(
		quote! { namespace = "console"},
		quote! {
			extern "C" {
				pub fn log(data: &JsValue);
			}
		},
		quote! {
			pub fn log(data: &JsValue) {
				::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.console.log, test_crate",
					".import_name test_crate.import.console.log, console.log",
					".functype test_crate.import.console.log ({}) -> ()",
					"",
					"{}",
					"",
					".globl test_crate.console.log",
					"test_crate.console.log:",
					"\t.functype test_crate.console.log ({}) -> ()",
					"\tlocal.get 0",
					"\t{}",
					"\tcall test_crate.import.console.log",
					"\tend_function",
					<::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::TYPE,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_bindgen::js_import!(name = "console.log", "console.log");

				extern "C" {
					#[link_name = "test_crate.console.log"]
					fn log(data: <::js_sys::JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(<::js_sys::JsValue as ::js_sys::hazard::Input>::as_raw(data)) };
			}
		},
	);
}

#[test]
fn two_parameters() {
	test_js_bindgen(
		quote! { namespace = "console"},
		quote! {
			extern "C" {
				pub fn log(data1: &JsValue, data2: &JsValue);
			}
		},
		quote! {
			pub fn log(data1: &JsValue, data2: &JsValue) {
				::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.console.log, test_crate",
					".import_name test_crate.import.console.log, console.log",
					".functype test_crate.import.console.log ({}, {}) -> ()",
					"",
					"{}",
					"{}",
					"",
					".globl test_crate.console.log",
					"test_crate.console.log:",
					"\t.functype test_crate.console.log ({}, {}) -> ()",
					"\tlocal.get 0",
					"\t{}",
					"\tlocal.get 1",
					"\t{}",
					"\tcall test_crate.import.console.log",
					"\tend_function",
					<::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::TYPE,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::TYPE,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::CONV,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_bindgen::js_import!(name = "console.log", "console.log");

				extern "C" {
					#[link_name = "test_crate.console.log"]
					fn log(
						data1: <::js_sys::JsValue as ::js_sys::hazard::Input>::Type,
						data2: <::js_sys::JsValue as ::js_sys::hazard::Input>::Type,
					);
				}

				unsafe {
					log(
						<::js_sys::JsValue as ::js_sys::hazard::Input>::as_raw(data1),
						<::js_sys::JsValue as ::js_sys::hazard::Input>::as_raw(data2),
					)
				};
			}
		},
	);
}

#[test]
fn real_name() {
	test_js_bindgen(
		quote! { namespace = "console"},
		quote! {
			extern "C" {
				#[js_bindgen(name = "log")]
				pub fn logx(data: &JsValue);
			}
		},
		quote! {
			pub fn logx(data: &JsValue) {
				::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.console.logx, test_crate",
					".import_name test_crate.import.console.logx, console.logx",
					".functype test_crate.import.console.logx ({}) -> ()",
					"",
					"{}",
					"",
					".globl test_crate.console.logx",
					"test_crate.console.logx:",
					"\t.functype test_crate.console.logx ({}) -> ()",
					"\tlocal.get 0",
					"\t{}",
					"\tcall test_crate.import.console.logx",
					"\tend_function",
					<::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::TYPE,
					<::js_sys::JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_bindgen::js_import!(name = "console.logx", "console.log");

				extern "C" {
					#[link_name = "test_crate.console.logx"]
					fn logx(data: <::js_sys::JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { logx(<::js_sys::JsValue as ::js_sys::hazard::Input>::as_raw(data)) };
			}
		},
	);
}
