use proc_macro::TokenStream;
use quote::quote;

#[test]
fn basic() {
	super::test(
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
					".functype test_crate.import.log ({},) -> ()",
					"",
					"{}",
					"",
					".globl test_crate.log",
					"test_crate.log:",
					"\t.functype test_crate.log ({},) -> ()",
					"\tlocal.get 0",
					"\t{}",
					"\tcall test_crate.import.log",
					"\tend_function",
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					interpolate <&JsValue as ::js_sys::hazard::Input>::TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_sys::js_bindgen::import_js!(
					name = "log",
					"{}{}{}{}{}",
					interpolate ::js_sys::r#macro::select("globalThis.log", "(data) => {\n", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\tdata", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", <&JsValue as ::js_sys::hazard::Input>::JS_CONV, [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\n", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\tglobalThis.log(data)\n}", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
				);

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(<&JsValue as ::js_sys::hazard::Input>::into_raw(data)) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.log, test_crate
			.import_name test_crate.import.log, log
			.functype test_crate.import.log (externref,) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.log
			test_crate.log:
				.functype test_crate.log (i32,) -> ()
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
			extern "C" {
				pub fn log(data: &JsValue);
			}
		},
		quote! {
			pub fn log(data: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.console.log, test_crate",
					".import_name test_crate.import.console.log, console.log",
					".functype test_crate.import.console.log ({},) -> ()",
					"",
					"{}",
					"",
					".globl test_crate.console.log",
					"test_crate.console.log:",
					"\t.functype test_crate.console.log ({},) -> ()",
					"\tlocal.get 0",
					"\t{}",
					"\tcall test_crate.import.console.log",
					"\tend_function",
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					interpolate <&JsValue as ::js_sys::hazard::Input>::TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_sys::js_bindgen::import_js!(
					name = "console.log",
					"{}{}{}{}{}",
					interpolate ::js_sys::r#macro::select(
						"globalThis.console.log",
						"(data) => {\n",
						[<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]
					),
					interpolate ::js_sys::r#macro::select("", "\tdata", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", <&JsValue as ::js_sys::hazard::Input>::JS_CONV, [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\n", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\tglobalThis.console.log(data)\n}", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
				);

				unsafe extern "C" {
					#[link_name = "test_crate.console.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(<&JsValue as ::js_sys::hazard::Input>::into_raw(data)) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.console.log, test_crate
			.import_name test_crate.import.console.log, console.log
			.functype test_crate.import.console.log (externref,) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.console.log
			test_crate.console.log:
				.functype test_crate.console.log (i32,) -> ()
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
			extern "C" {
				pub fn log(data: &JsValue);
			}
		},
		quote! {
			pub fn log(data: &JsValue) {
				js_sys::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.log, test_crate",
					".import_name test_crate.import.log, log",
					".functype test_crate.import.log ({},) -> ()",
					"",
					"{}",
					"",
					".globl test_crate.log",
					"test_crate.log:",
					"\t.functype test_crate.log ({},) -> ()",
					"\tlocal.get 0",
					"\t{}",
					"\tcall test_crate.import.log",
					"\tend_function",
					interpolate <&JsValue as js_sys::hazard::Input>::IMPORT_TYPE,
					interpolate <&JsValue as js_sys::hazard::Input>::IMPORT_FUNC,
					interpolate <&JsValue as js_sys::hazard::Input>::TYPE,
					interpolate <&JsValue as js_sys::hazard::Input>::CONV,
				);

				js_sys::js_bindgen::import_js!(
					name = "log",
					"{}{}{}{}{}",
					interpolate js_sys::r#macro::select("globalThis.log", "(data) => {\n", [<&JsValue as js_sys::hazard::Input>::JS_CONV,]),
					interpolate js_sys::r#macro::select("", "\tdata", [<&JsValue as js_sys::hazard::Input>::JS_CONV,]),
					interpolate js_sys::r#macro::select("", <&JsValue as js_sys::hazard::Input>::JS_CONV, [<&JsValue as js_sys::hazard::Input>::JS_CONV,]),
					interpolate js_sys::r#macro::select("", "\n", [<&JsValue as js_sys::hazard::Input>::JS_CONV,]),
					interpolate js_sys::r#macro::select("", "\tglobalThis.log(data)\n}", [<&JsValue as js_sys::hazard::Input>::JS_CONV,]),
				);

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as js_sys::hazard::Input>::Type);
				}

				unsafe { log(<&JsValue as js_sys::hazard::Input>::into_raw(data)) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.log, test_crate
			.import_name test_crate.import.log, log
			.functype test_crate.import.log (externref,) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.log
			test_crate.log:
				.functype test_crate.log (i32,) -> ()
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
			extern "C" {
				pub fn log(data1: &JsValue, data2: &JsValue);
			}
		},
		quote! {
			pub fn log(data1: &JsValue, data2: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.log, test_crate",
					".import_name test_crate.import.log, log",
					".functype test_crate.import.log ({},{},) -> ()",
					"",
					"{}",
					"",
					"{}",
					"",
					".globl test_crate.log",
					"test_crate.log:",
					"\t.functype test_crate.log ({},{},) -> ()",
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

				::js_sys::js_bindgen::import_js!(
					name = "log",
					"{}{}{}{}{}{}{}{}",
					interpolate ::js_sys::r#macro::select(
						"globalThis.log",
						"(data1, data2) => {\n",
						[<&JsValue as ::js_sys::hazard::Input>::JS_CONV,<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]
					),
					interpolate ::js_sys::r#macro::select("", "\tdata1", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", <&JsValue as ::js_sys::hazard::Input>::JS_CONV, [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\n", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\tdata2", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", <&JsValue as ::js_sys::hazard::Input>::JS_CONV, [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\n", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select(
						"",
						"\tglobalThis.log(data1, data2)\n}",
						[<&JsValue as ::js_sys::hazard::Input>::JS_CONV,<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]
					),
				);

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(
						data1: <&JsValue as ::js_sys::hazard::Input>::Type,
						data2: <&JsValue as ::js_sys::hazard::Input>::Type,
					);
				}

				unsafe { log(
					<&JsValue as ::js_sys::hazard::Input>::into_raw(data1),
					<&JsValue as ::js_sys::hazard::Input>::into_raw(data2),
				) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.log, test_crate
			.import_name test_crate.import.log, log
			.functype test_crate.import.log (externref,externref,) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.log
			test_crate.log:
				.functype test_crate.log (i32,i32,) -> ()
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

				::js_sys::js_bindgen::import_js!(
					name = "log",
					"globalThis.log"
				);

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
					".functype test_crate.import.logx ({},) -> ()",
					"",
					"{}",
					"",
					".globl test_crate.logx",
					"test_crate.logx:",
					"\t.functype test_crate.logx ({},) -> ()",
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
					"{}{}{}{}{}",
					interpolate ::js_sys::r#macro::select("globalThis.log", "(data) => {\n", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\tdata", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", <&JsValue as ::js_sys::hazard::Input>::JS_CONV, [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\n", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\tglobalThis.log(data)\n}", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
				);

				unsafe extern "C" {
					#[link_name = "test_crate.logx"]
					fn logx(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { logx(<&JsValue as ::js_sys::hazard::Input>::into_raw(data)) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.logx, test_crate
			.import_name test_crate.import.logx, logx
			.functype test_crate.import.logx (externref,) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.logx
			test_crate.logx:
				.functype test_crate.logx (i32,) -> ()
				local.get 0
				call js_sys.externref.get
				call test_crate.import.logx
				end_function"
		),
		"globalThis.log",
	);
}

#[test]
fn js_embed() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "C" {
				#[js_sys(js_embed = "custom")]
				pub fn log(data: &JsValue);
			}
		},
		quote! {
			pub fn log(data: &JsValue) {
				::js_sys::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.log, test_crate",
					".import_name test_crate.import.log, log",
					".functype test_crate.import.log ({},) -> ()",
					"",
					"{}",
					"",
					".globl test_crate.log",
					"test_crate.log:",
					"\t.functype test_crate.log ({},) -> ()",
					"\tlocal.get 0",
					"\t{}",
					"\tcall test_crate.import.log",
					"\tend_function",
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
					interpolate <&JsValue as ::js_sys::hazard::Input>::TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::CONV,
				);

				::js_sys::js_bindgen::import_js!(
					name = "log",
					required_embed = "custom",
					"{}{}{}{}{}",
					interpolate ::js_sys::r#macro::select("jsEmbed.test_crate[\"custom\"]", "(data) => {\n", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\tdata", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", <&JsValue as ::js_sys::hazard::Input>::JS_CONV, [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\n", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\tjsEmbed.test_crate[\"custom\"](data)\n}", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV,]),
				);

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(<&JsValue as ::js_sys::hazard::Input>::into_raw(data)) };
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.log, test_crate
			.import_name test_crate.import.log, log
			.functype test_crate.import.log (externref,) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.log
			test_crate.log:
				.functype test_crate.log (i32,) -> ()
				local.get 0
				call js_sys.externref.get
				call test_crate.import.log
				end_function"
		),
		"jsEmbed.test_crate[\"custom\"]",
	);
}

#[test]
fn r#return() {
	super::test(
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

				::js_sys::js_bindgen::import_js!(
					name = "is_nan",
					"globalThis.is_nan"
				);

				unsafe extern "C" {
					#[link_name = "test_crate.is_nan"]
					fn is_nan() -> <JsValue as ::js_sys::hazard::Output>::Type;
				}

				<JsValue as ::js_sys::hazard::Output>::from_raw(unsafe { is_nan() })
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
			extern "C" {
				fn array(ptr: *const u8) -> JsString;
			}
		},
		quote! {
			fn array(ptr: *const u8) -> JsString {
				::js_sys::js_bindgen::unsafe_embed_asm!(
					".import_module test_crate.import.array, test_crate",
					".import_name test_crate.import.array, array",
					".functype test_crate.import.array ({},) -> ({})",
					"",
					"{}",
					"",
					"{}",
					"",
					".globl test_crate.array", "test_crate.array:",
					"\t.functype test_crate.array ({},) -> ({})",
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

				::js_sys::js_bindgen::import_js!(
					name = "array",
					"{}{}{}{}{}",
					interpolate ::js_sys::r#macro::select("globalThis.array", "(ptr) => {\n", [<*const u8 as ::js_sys::hazard::Input>::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\tptr", [<*const u8 as ::js_sys::hazard::Input> ::JS_CONV,]),
					interpolate ::js_sys::r#macro::select(
						"",
						<*const u8 as ::js_sys::hazard::Input>::JS_CONV,
						[<*const u8 as ::js_sys::hazard::Input>::JS_CONV,]
					),
					interpolate ::js_sys::r#macro::select("", "\n", [<*const u8 as ::js_sys::hazard::Input> ::JS_CONV,]),
					interpolate ::js_sys::r#macro::select("", "\treturn globalThis.array(ptr)\n}", [<*const u8 as ::js_sys::hazard::Input> ::JS_CONV,]),
				);

				unsafe extern "C" {
					#[link_name = "test_crate.array"]
					fn array(
						ptr: <*const u8 as ::js_sys::hazard::Input>::Type,
					) -> <JsString as ::js_sys::hazard::Output>::Type;

				}

				<JsString as ::js_sys::hazard::Output>::from_raw(unsafe {
					array(<*const u8 as ::js_sys::hazard::Input>::into_raw(ptr))
				})
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.array, test_crate
			.import_name test_crate.import.array, array
			.functype test_crate.import.array (i32,) -> (externref)



			.functype js_sys.externref.insert (externref) -> (i32)

			.globl test_crate.array
			test_crate.array:
				.functype test_crate.array (i32,) -> (i32)
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
			extern "C" {
				#[cfg(all())]
				pub fn log();
			}
		},
		quote! {
			#[cfg(all())]
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

				::js_sys::js_bindgen::import_js!(
					name = "log",
					"globalThis.log"
				);

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
