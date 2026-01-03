use proc_macro2::TokenStream;
use quote::quote;

#[track_caller]
fn test(attr: TokenStream, item: TokenStream, expected: TokenStream) {
	let output = crate::js_sys(attr, item);

	let output = syn::parse2(output).unwrap();
	let output = prettyplease::unparse(&output);
	let expected = syn::parse2(expected).unwrap();
	let expected = prettyplease::unparse(&expected);

	similar_asserts::assert_eq!(expected, output);
}

#[test]
fn basic() {
	test(
		TokenStream::new(),
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
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					interpolate <&JsValue as ::js_sys::hazard::Input>::TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_sys::js_bindgen::import_js!(name = "log", "(data) => globalThis.log(data)");

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(<&JsValue as ::js_sys::hazard::Input>::into_raw(data)) };
			}
		},
	);
}

#[test]
fn namespace() {
	test(
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
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					interpolate <&JsValue as ::js_sys::hazard::Input>::TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_sys::js_bindgen::import_js!(name = "console.log", "(data) => globalThis.console.log(data)");

				unsafe extern "C" {
					#[link_name = "test_crate.console.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(<&JsValue as ::js_sys::hazard::Input>::into_raw(data)) };
			}
		},
	);
}

#[test]
fn js_sys() {
	test(
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
					interpolate <&JsValue as crate::hazard::Input>::IMPORT_TYPE,
					interpolate <&JsValue as crate::hazard::Input>::IMPORT_FUNC,
					interpolate <&JsValue as crate::hazard::Input>::TYPE,
					interpolate <&JsValue as crate::hazard::Input>::CONV,
				);

				crate::js_bindgen::import_js!(name = "log", "(data) => globalThis.log(data)");

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as crate::hazard::Input>::Type);
				}

				unsafe { log(<&JsValue as crate::hazard::Input>::into_raw(data)) };
			}
		},
	);
}

#[test]
fn two_parameters() {
	test(
		TokenStream::new(),
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
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					interpolate <&JsValue as ::js_sys::hazard::Input>::TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::CONV,
					interpolate <&JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_sys::js_bindgen::import_js!(name = "log", "(data1, data2) => globalThis.log(data1, data2)");

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(
						data1: <&JsValue as ::js_sys::hazard::Input>::Type,
						data2: <&JsValue as ::js_sys::hazard::Input>::Type,
					);
				}

				unsafe {
					log(
						<&JsValue as ::js_sys::hazard::Input>::into_raw(data1),
						<&JsValue as ::js_sys::hazard::Input>::into_raw(data2),
					)
				};
			}
		},
	);
}

#[test]
fn empty() {
	test(
		TokenStream::new(),
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

				::js_sys::js_bindgen::import_js!(name = "log", "() => globalThis.log()");

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log();
				}

				unsafe { log() };
			}
		},
	);
}

#[test]
fn js_name() {
	test(
		TokenStream::new(),
		quote! {
			extern "C" {
				#[js_sys(js_name = "log")]
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
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					interpolate <&JsValue as ::js_sys::hazard::Input>::TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_sys::js_bindgen::import_js!(name = "logx", "(data) => globalThis.log(data)");

				unsafe extern "C" {
					#[link_name = "test_crate.logx"]
					fn logx(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { logx(<&JsValue as ::js_sys::hazard::Input>::into_raw(data)) };
			}
		},
	);
}

#[test]
fn js_embed() {
	test(
		TokenStream::new(),
		quote! {
			extern "C" {
				#[js_sys(js_embed = "custom")]
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
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					interpolate <&JsValue as ::js_sys::hazard::Input>::TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_sys::js_bindgen::import_js!(
					name = "logx",
					required_embed = "custom",
					"(data) => jsEmbed.test_crate[\"custom\"](data)"
				);

				unsafe extern "C" {
					#[link_name = "test_crate.logx"]
					fn logx(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { logx(<&JsValue as ::js_sys::hazard::Input>::into_raw(data)) };
			}
		},
	);
}

#[test]
fn r#return() {
	test(
		TokenStream::new(),
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
					"{}",
					"",
					".globl test_crate.is_nan",
					"test_crate.is_nan:",
					"\t.functype test_crate.is_nan () -> ({})",
					"\tcall test_crate.import.is_nan",
					"\t{}",
					"\tend_function",
					interpolate <JsValue as ::js_sys::hazard::Output>::IMPORT_TYPE,
					interpolate <JsValue as ::js_sys::hazard::Output>::IMPORT_FUNC,
					interpolate <JsValue as ::js_sys::hazard::Output>::TYPE,
					interpolate <JsValue as ::js_sys::hazard::Output>::CONV,
				);

				::js_sys::js_bindgen::import_js!(name = "is_nan", "() => globalThis.is_nan()");

				unsafe extern "C" {
					#[link_name = "test_crate.is_nan"]
					fn is_nan() -> <JsValue as ::js_sys::hazard::Output>::Type;
				}

				<JsValue as ::js_sys::hazard::Output>::from_raw(unsafe { is_nan() })
			}
		},
	);
}

#[test]
fn pointer() {
	test(
		TokenStream::new(),
		quote! {
			extern "C" {
				fn array(array: *const u8) -> JsString;
			}
		},
		quote! {
			fn array(array: *const u8) -> JsString {
				::js_sys::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.array, test_crate",
					".import_name test_crate.import.array, array",
					".functype test_crate.import.array ({}) -> ({})",
					"",
					"{}",
					"",
					"{}",
					"",
					".globl test_crate.array", "test_crate.array:",
					"\t.functype test_crate.array ({}) -> ({})",
					"\tlocal.get 0",
					"\t{}",
					"\tcall test_crate.import.array",
					"\t{}",
					"\tend_function",
					interpolate <*const u8 as ::js_sys::hazard::Input>::IMPORT_TYPE,
					interpolate <JsString as ::js_sys::hazard::Output>::IMPORT_TYPE,
					interpolate <*const u8 as ::js_sys::hazard::Input>::IMPORT_FUNC,
					interpolate <JsString as ::js_sys::hazard::Output>::IMPORT_FUNC,
					interpolate <*const u8 as ::js_sys::hazard::Input>::TYPE,
					interpolate <JsString as ::js_sys::hazard::Output>::TYPE,
					interpolate <*const u8 as ::js_sys::hazard::Input>::CONV,
					interpolate <JsString as ::js_sys::hazard::Output>::CONV,
				);

				::js_sys::js_bindgen::import_js!(name = "array", "(array) => globalThis.array(array)");

				unsafe extern "C" {
					#[link_name = "test_crate.array"]
					fn array(
						array: <*const u8 as ::js_sys::hazard::Input>::Type,
					) -> <JsString as ::js_sys::hazard::Output>::Type;

				}

				<JsString as ::js_sys::hazard::Output>::from_raw(unsafe {
					array(<*const u8 as ::js_sys::hazard::Input>::into_raw(array))
				})
			}
		},
	);
}

#[test]
fn cfg() {
	test(
		TokenStream::new(),
		quote! {
			extern "C" {
				#[cfg(test)]
				pub fn log();
			}
		},
		quote! {
			#[cfg(test)]
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

				::js_sys::js_bindgen::import_js!(name = "log", "() => globalThis.log()");

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log();
				}

				unsafe { log() };
			}
		},
	);
}
