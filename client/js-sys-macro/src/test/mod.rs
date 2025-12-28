use proc_macro2::TokenStream;
use quote::quote;

fn test_js_bindgen(attr: TokenStream, item: TokenStream, expected: TokenStream) {
	let output = crate::js_sys(attr, item);

	let output = syn::parse2(output).unwrap();
	let output = prettyplease::unparse(&output);
	let expected = syn::parse2(expected).unwrap();
	let expected = prettyplease::unparse(&expected);

	similar_asserts::assert_eq!(expected, output);
}

#[test]
fn basic() {
	test_js_bindgen(
		quote! {},
		quote! {
			extern "C" {
				pub fn log(data: &JsValue);
			}
		},
		quote! {
			pub fn log(data: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.log, test_crate",
					".import_name test_crate.import.log, log",
					".functype test_crate.import.log ({}) -> ()",
					"",
					"{}",
					"",
					".globl test_crate.log",
					"test_crate.log:",
					"\t.functype test_crate.log ({}) -> ()",
					"\tlocal.get 0",
					"\t{}",
					"\tcall test_crate.import.log",
					"\tend_function",
					<JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					<JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					<JsValue as ::js_sys::hazard::Input>::TYPE,
					<JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_sys::js_bindgen::js_import!(name = "log", "log");

				extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(<JsValue as ::js_sys::hazard::Input>::as_raw(data)) };
			}
		},
	);
}

#[test]
fn namespace() {
	test_js_bindgen(
		quote! { namespace = "console" },
		quote! {
			extern "C" {
				pub fn log(data: &JsValue);
			}
		},
		quote! {
			pub fn log(data: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm!(
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
					<JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					<JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					<JsValue as ::js_sys::hazard::Input>::TYPE,
					<JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_sys::js_bindgen::js_import!(name = "console.log", "console.log");

				extern "C" {
					#[link_name = "test_crate.console.log"]
					fn log(data: <JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(<JsValue as ::js_sys::hazard::Input>::as_raw(data)) };
			}
		},
	);
}

#[test]
fn js_sys() {
	test_js_bindgen(
		quote! { js_sys = crate },
		quote! {
			extern "C" {
				pub fn log(data: &JsValue);
			}
		},
		quote! {
			pub fn log(data: &JsValue) {
				crate::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.log, test_crate",
					".import_name test_crate.import.log, log",
					".functype test_crate.import.log ({}) -> ()",
					"",
					"{}",
					"",
					".globl test_crate.log",
					"test_crate.log:",
					"\t.functype test_crate.log ({}) -> ()",
					"\tlocal.get 0",
					"\t{}",
					"\tcall test_crate.import.log",
					"\tend_function",
					<JsValue as crate::hazard::Input>::IMPORT_TYPE,
					<JsValue as crate::hazard::Input>::IMPORT_FUNC,
					<JsValue as crate::hazard::Input>::TYPE,
					<JsValue as crate::hazard::Input>::CONV,
				);

				crate::js_bindgen::js_import!(name = "log", "log");

				extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <JsValue as crate::hazard::Input>::Type);
				}

				unsafe { log(<JsValue as crate::hazard::Input>::as_raw(data)) };
			}
		},
	);
}

#[test]
fn two_parameters() {
	test_js_bindgen(
		quote! {},
		quote! {
			extern "C" {
				pub fn log(data1: &JsValue, data2: &JsValue);
			}
		},
		quote! {
			pub fn log(data1: &JsValue, data2: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.log, test_crate",
					".import_name test_crate.import.log, log",
					".functype test_crate.import.log ({}, {}) -> ()",
					"",
					"{}",
					"",
					"{}",
					"",
					".globl test_crate.log",
					"test_crate.log:",
					"\t.functype test_crate.log ({}, {}) -> ()",
					"\tlocal.get 0",
					"\t{}",
					"\tlocal.get 1",
					"\t{}",
					"\tcall test_crate.import.log",
					"\tend_function",
					<JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					<JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					<JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					<JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					<JsValue as ::js_sys::hazard::Input>::TYPE,
					<JsValue as ::js_sys::hazard::Input>::TYPE,
					<JsValue as ::js_sys::hazard::Input>::CONV,
					<JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_sys::js_bindgen::js_import!(name = "log", "log");

				extern "C" {
					#[link_name = "test_crate.log"]
					fn log(
						data1: <JsValue as ::js_sys::hazard::Input>::Type,
						data2: <JsValue as ::js_sys::hazard::Input>::Type,
					);
				}

				unsafe {
					log(
						<JsValue as ::js_sys::hazard::Input>::as_raw(data1),
						<JsValue as ::js_sys::hazard::Input>::as_raw(data2),
					)
				};
			}
		},
	);
}

#[test]
fn empty() {
	test_js_bindgen(
		quote! {},
		quote! {
			extern "C" {
				pub fn log();
			}
		},
		quote! {
			pub fn log() {
				::js_sys::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.log, test_crate",
					".import_name test_crate.import.log, log",
					".functype test_crate.import.log () -> ()",
					"",
					".globl test_crate.log",
					"test_crate.log:",
					"\t.functype test_crate.log () -> ()",
					"\tcall test_crate.import.log",
					"\tend_function",
				);

				::js_sys::js_bindgen::js_import!(name = "log", "log");

				extern "C" {
					#[link_name = "test_crate.log"]
					fn log();
				}

				unsafe { log() };
			}
		},
	);
}

#[test]
fn real_name() {
	test_js_bindgen(
		quote! {},
		quote! {
			extern "C" {
				#[js_sys(name = "log")]
				pub fn logx(data: &JsValue);
			}
		},
		quote! {
			pub fn logx(data: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.logx, test_crate",
					".import_name test_crate.import.logx, logx",
					".functype test_crate.import.logx ({}) -> ()",
					"",
					"{}",
					"",
					".globl test_crate.logx",
					"test_crate.logx:",
					"\t.functype test_crate.logx ({}) -> ()",
					"\tlocal.get 0",
					"\t{}",
					"\tcall test_crate.import.logx",
					"\tend_function",
					<JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					<JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					<JsValue as ::js_sys::hazard::Input>::TYPE,
					<JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_sys::js_bindgen::js_import!(name = "logx", "log");

				extern "C" {
					#[link_name = "test_crate.logx"]
					fn logx(data: <JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { logx(<JsValue as ::js_sys::hazard::Input>::as_raw(data)) };
			}
		},
	);
}

#[test]
fn r#return() {
	test_js_bindgen(
		quote! {},
		quote! {
			extern "C" {
				pub fn is_nan() -> JsValue;
			}
		},
		quote! {
			pub fn is_nan() -> JsValue {
				::js_sys::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.is_nan, test_crate",
					".import_name test_crate.import.is_nan, is_nan",
					".functype test_crate.import.is_nan () -> ({})",
					"",
					".globl test_crate.is_nan",
					"test_crate.is_nan:",
					"\t.functype test_crate.is_nan () -> ({})",
					"\tcall test_crate.import.is_nan",
					"\t{}",
					"\tend_function",
					<JsValue as ::js_sys::hazard::Output>::IMPORT_TYPE,
					<JsValue as ::js_sys::hazard::Output>::TYPE,
					<JsValue as ::js_sys::hazard::Output>::CONV,
				);

				::js_sys::js_bindgen::js_import!(name = "is_nan", "is_nan");

				extern "C" {
					#[link_name = "test_crate.is_nan"]
					fn is_nan() -> <JsValue as ::js_sys::hazard::Output>::Type;
				}

				<JsValue as ::js_sys::hazard::Output>::from_raw(unsafe { is_nan() })
			}
		},
	);
}
