use proc_macro2::TokenStream;
use quote::quote;

#[test]
fn basic() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				pub fn log(data: &JsValue);
			}
		},
		quote! {
			pub fn log(data: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm! {
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
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
					interpolate ::js_sys::r#macro::asm_import!(&JsValue as Input),
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_conv!(&JsValue as Input),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					required_embeds = [::js_sys::r#macro::js_import!(&JsValue as Input)],
					"{}{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "(data) => {\n", [&JsValue]),
					interpolate ::js_sys::r#macro::js_parameter!("data", &JsValue),
					interpolate ::js_sys::r#macro::js_select!("globalThis.log", "globalThis.log(data)\n}", [&JsValue]),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(::js_sys::hazard::Input::into_raw(data)) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.log, test_crate
			.import_name test_crate.import.log, log
			.functype test_crate.import.log (externref) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.log
			test_crate.log:
				.functype test_crate.log (i32) -> ()
				local.get 0
				call js_sys.externref.get
				call test_crate.import.log
				end_function"
		),
		"globalThis.log",
	);
}

#[test]
fn namespace() {
	super::test(
		quote! { namespace = "console" },
		quote! {
			extern "js-sys" {
				pub fn log(data: &JsValue);
			}
		},
		quote! {
			pub fn log(data: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm! {
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
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
					interpolate ::js_sys::r#macro::asm_import!(&JsValue as Input),
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_conv!(&JsValue as Input),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "console.log",
					required_embeds = [::js_sys::r#macro::js_import!(&JsValue as Input)],
					"{}{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "(data) => {\n", [&JsValue]),
					interpolate ::js_sys::r#macro::js_parameter!("data", &JsValue),
					interpolate ::js_sys::r#macro::js_select!("globalThis.console.log", "globalThis.console.log(data)\n}", [&JsValue]),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.console.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(::js_sys::hazard::Input::into_raw(data)) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.console.log, test_crate
			.import_name test_crate.import.console.log, console.log
			.functype test_crate.import.console.log (externref) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.console.log
			test_crate.console.log:
				.functype test_crate.console.log (i32) -> ()
				local.get 0
				call js_sys.externref.get
				call test_crate.import.console.log
				end_function"
		),
		"globalThis.console.log",
	);
}

#[test]
fn js_sys() {
	super::test(
		quote! { js_sys = js_sys },
		quote! {
			extern "js-sys" {
				pub fn log(data: &JsValue);
			}
		},
		quote! {
			pub fn log(data: &JsValue) {
				js_sys::js_bindgen::unsafe_embed_asm! {
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
					interpolate <&JsValue as js_sys::hazard::Input>::ASM_IMPORT_TYPE,
					interpolate js_sys::r#macro::asm_import!(&JsValue as Input),
					interpolate <&JsValue as js_sys::hazard::Input>::ASM_TYPE,
					interpolate js_sys::r#macro::asm_conv!(&JsValue as Input),
				}

				js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					required_embeds = [js_sys::r#macro::js_import!(&JsValue as Input)],
					"{}{}{}",
					interpolate js_sys::r#macro::js_select!("", "(data) => {\n", [&JsValue]),
					interpolate js_sys::r#macro::js_parameter!("data", &JsValue),
					interpolate js_sys::r#macro::js_select!("globalThis.log", "globalThis.log(data)\n}", [&JsValue]),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as js_sys::hazard::Input>::Type);
				}

				unsafe { log(js_sys::hazard::Input::into_raw(data)) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.log, test_crate
			.import_name test_crate.import.log, log
			.functype test_crate.import.log (externref) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.log
			test_crate.log:
				.functype test_crate.log (i32) -> ()
				local.get 0
				call js_sys.externref.get
				call test_crate.import.log
				end_function"
		),
		"globalThis.log",
	);
}

#[test]
fn two_parameters() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				pub fn log(data1: &JsValue, data2: &JsValue);
			}
		},
		quote! {
			pub fn log(data1: &JsValue, data2: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm! {
					".import_module test_crate.import.log, test_crate",
					".import_name test_crate.import.log, log",
					".functype test_crate.import.log ({}, {}) -> ()",
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
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
					interpolate ::js_sys::r#macro::asm_import!(&JsValue as Input),
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_conv!(&JsValue as Input),
					interpolate ::js_sys::r#macro::asm_conv!(&JsValue as Input),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					required_embeds = [::js_sys::r#macro::js_import!(&JsValue as Input)],
					"{}{}{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "(data1, data2) => {\n", [&JsValue]),
					interpolate ::js_sys::r#macro::js_parameter!("data1", &JsValue),
					interpolate ::js_sys::r#macro::js_parameter!("data2", &JsValue),
					interpolate ::js_sys::r#macro::js_select!("globalThis.log", "globalThis.log(data1, data2)\n}", [&JsValue]),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(
						data1: <&JsValue as ::js_sys::hazard::Input>::Type,
						data2: <&JsValue as ::js_sys::hazard::Input>::Type,
					);
				}

				unsafe { log(
					::js_sys::hazard::Input::into_raw(data1),
					::js_sys::hazard::Input::into_raw(data2),
				) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.log, test_crate
			.import_name test_crate.import.log, log
			.functype test_crate.import.log (externref, externref) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.log
			test_crate.log:
				.functype test_crate.log (i32, i32) -> ()
				local.get 0
				call js_sys.externref.get
				local.get 1
				call js_sys.externref.get
				call test_crate.import.log
				end_function"
		),
		"globalThis.log",
	);
}

#[test]
fn empty() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				pub fn log();
			}
		},
		quote! {
			pub fn log() {
				::js_sys::js_bindgen::unsafe_embed_asm! {
					".import_module test_crate.import.log, test_crate",
					".import_name test_crate.import.log, log",
					".functype test_crate.import.log () -> ()",
					"",
					".globl test_crate.log",
					"test_crate.log:",
					"\t.functype test_crate.log () -> ()",
					"\tcall test_crate.import.log",
					"\tend_function",
				}

				::js_sys::js_bindgen::import_js!(module = "test_crate", name = "log", "globalThis.log");

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log();
				}

				unsafe { log() };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.log, test_crate
			.import_name test_crate.import.log, log
			.functype test_crate.import.log () -> ()

			.globl test_crate.log
			test_crate.log:
				.functype test_crate.log () -> ()
				call test_crate.import.log
				end_function"
		),
		"globalThis.log",
	);
}

#[test]
fn js_name() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				#[js_sys(js_name = "log")]
				pub fn logx(data: &JsValue);
			}
		},
		quote! {
			pub fn logx(data: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm! {
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
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
					interpolate ::js_sys::r#macro::asm_import!(&JsValue as Input),
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_conv!(&JsValue as Input),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "logx",
					required_embeds = [::js_sys::r#macro::js_import!(&JsValue as Input)],
					"{}{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "(data) => {\n", [&JsValue]),
					interpolate ::js_sys::r#macro::js_parameter!("data", &JsValue),
					interpolate ::js_sys::r#macro::js_select!("globalThis.log", "globalThis.log(data)\n}", [&JsValue]),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.logx"]
					fn logx(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { logx(::js_sys::hazard::Input::into_raw(data)) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.logx, test_crate
			.import_name test_crate.import.logx, logx
			.functype test_crate.import.logx (externref) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.logx
			test_crate.logx:
				.functype test_crate.logx (i32) -> ()
				local.get 0
				call js_sys.externref.get
				call test_crate.import.logx
				end_function"
		),
		"globalThis.log",
	);
}

#[test]
fn js_import() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				#[js_sys(js_import)]
				pub fn log(data: &JsValue);
			}
		},
		quote! {
			pub fn log(data: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm! {
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
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
					interpolate ::js_sys::r#macro::asm_import!(&JsValue as Input),
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_conv!(&JsValue as Input),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(::js_sys::hazard::Input::into_raw(data)) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.log, test_crate
			.import_name test_crate.import.log, log
			.functype test_crate.import.log (externref) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.log
			test_crate.log:
				.functype test_crate.log (i32) -> ()
				local.get 0
				call js_sys.externref.get
				call test_crate.import.log
				end_function"
		),
		None,
	);
}

#[test]
fn js_embed() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				#[js_sys(js_embed = "embed")]
				pub fn log(data: &JsValue);
			}
		},
		quote! {
			pub fn log(data: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm! {
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
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
					interpolate ::js_sys::r#macro::asm_import!(&JsValue as Input),
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_conv!(&JsValue as Input),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					required_embeds = [
						("test_crate", "embed"),
						::js_sys::r#macro::js_import!(&JsValue as Input)
					],
					"{}{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "(data) => {\n", [&JsValue]),
					interpolate ::js_sys::r#macro::js_parameter!("data", &JsValue),
					interpolate ::js_sys::r#macro::js_select!("this.#jsEmbed.test_crate['embed']", "this.#jsEmbed.test_crate['embed'](data)\n}", [&JsValue]),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(::js_sys::hazard::Input::into_raw(data)) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.log, test_crate
			.import_name test_crate.import.log, log
			.functype test_crate.import.log (externref) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.log
			test_crate.log:
				.functype test_crate.log (i32) -> ()
				local.get 0
				call js_sys.externref.get
				call test_crate.import.log
				end_function"
		),
		"this.#jsEmbed.test_crate['embed']",
	);
}

#[test]
fn r#return() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				pub fn is_nan() -> JsValue;
			}
		},
		quote! {
			pub fn is_nan() -> JsValue {
				::js_sys::js_bindgen::unsafe_embed_asm! {
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
					interpolate <JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_TYPE,
					interpolate ::js_sys::r#macro::asm_import!(JsValue as Output),
					interpolate <JsValue as ::js_sys::hazard::Output>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_conv!(JsValue as Output),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "is_nan",
					required_embeds = [::js_sys::r#macro::js_import!(JsValue as Output)],
					"{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "() => {\n\treturn ", [], JsValue),
					interpolate ::js_sys::r#macro::js_output!("", "globalThis.is_nan", "globalThis.is_nan()", JsValue,),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.is_nan"]
					fn is_nan() -> <JsValue as ::js_sys::hazard::Output>::Type;
				}

				::js_sys::hazard::Output::from_raw(unsafe { is_nan() })
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.is_nan, test_crate
			.import_name test_crate.import.is_nan, is_nan
			.functype test_crate.import.is_nan () -> (externref)

			.functype js_sys.externref.insert (externref) -> (i32)

			.globl test_crate.is_nan
			test_crate.is_nan:
				.functype test_crate.is_nan () -> (i32)
				call test_crate.import.is_nan
				call js_sys.externref.insert
				end_function"
		),
		"globalThis.is_nan",
	);
}

#[test]
fn pointer() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				fn array(ptr: *const u8) -> JsString;
			}
		},
		quote! {
			fn array(ptr: *const u8) -> JsString {
				::js_sys::js_bindgen::unsafe_embed_asm! {
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
					interpolate <*const u8 as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
					interpolate <JsString as ::js_sys::hazard::Output>::ASM_IMPORT_TYPE,
					interpolate ::js_sys::r#macro::asm_import!(*const u8 as Input),
					interpolate ::js_sys::r#macro::asm_import!(JsString as Output),
					interpolate <*const u8 as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate <JsString as ::js_sys::hazard::Output>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_conv!(*const u8 as Input),
					interpolate ::js_sys::r#macro::asm_conv!(JsString as Output),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "array",
					required_embeds = [::js_sys::r#macro::js_import!(*const u8 as Input), ::js_sys::r#macro::js_import!(JsString as Output)],
					"{}{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "(ptr) => {\n", [*const u8], JsString),
					interpolate ::js_sys::r#macro::js_parameter!("ptr", * const u8),
					interpolate ::js_sys::r#macro::js_output!("\treturn ", "globalThis.array", "globalThis.array(ptr)", JsString, *const u8),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.array"]
					fn array(
						ptr: <*const u8 as ::js_sys::hazard::Input>::Type,
					) -> <JsString as ::js_sys::hazard::Output>::Type;

				}

				::js_sys::hazard::Output::from_raw(unsafe {
					array(::js_sys::hazard::Input::into_raw(ptr))
				})
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.array, test_crate
			.import_name test_crate.import.array, array
			.functype test_crate.import.array (i32) -> (externref)



			.functype js_sys.externref.insert (externref) -> (i32)

			.globl test_crate.array
			test_crate.array:
				.functype test_crate.array (i32) -> (i32)
				local.get 0
				
				call test_crate.import.array
				call js_sys.externref.insert
				end_function"
		),
		indoc::indoc!(
			"(ptr) => {
				ptr >>>= 0
				return globalThis.array(ptr)
			}"
		),
	);
}

#[test]
fn cfg() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				#[cfg(all())]
				pub fn log();
			}
		},
		quote! {
			#[cfg(all())]
			pub fn log() {
				::js_sys::js_bindgen::unsafe_embed_asm! {
					".import_module test_crate.import.log, test_crate",
					".import_name test_crate.import.log, log",
					".functype test_crate.import.log () -> ()",
					"",
					".globl test_crate.log",
					"test_crate.log:",
					"\t.functype test_crate.log () -> ()",
					"\tcall test_crate.import.log",
					"\tend_function",
				}

				::js_sys::js_bindgen::import_js!(module = "test_crate", name = "log", "globalThis.log");

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log();
				}

				unsafe { log() };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.log, test_crate
			.import_name test_crate.import.log, log
			.functype test_crate.import.log () -> ()

			.globl test_crate.log
			test_crate.log:
				.functype test_crate.log () -> ()
				call test_crate.import.log
				end_function"
		),
		"globalThis.log",
	);
}
