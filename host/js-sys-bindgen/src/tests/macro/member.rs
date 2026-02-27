use proc_macro2::TokenStream;
use quote::quote;

#[test]
fn method() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				pub fn test(self: &JsTest);
			}
		},
		quote! {
			impl JsTest {
				pub fn test(self: &JsTest) {
					::js_sys::js_bindgen::unsafe_embed_asm! {
						".import_module test_crate.import.test, test_crate",
						".import_name test_crate.import.test, test",
						".functype test_crate.import.test ({}) -> ()",
						"",
						"{}",
						"",
						".globl test_crate.test",
						"test_crate.test:",
						"\t.functype test_crate.test ({}) -> ()",
						"\tlocal.get 0",
						"\t{}",
						"\tcall test_crate.import.test",
						"\tend_function",
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
						interpolate ::js_sys::r#macro::asm_import!(&::js_sys::JsValue as Input),
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate ::js_sys::r#macro::asm_conv!(&::js_sys::JsValue as Input),
					}

					::js_sys::js_bindgen::import_js! {
						module = "test_crate",
						name = "test",
						required_embeds = [::js_sys::r#macro::js_import!(&::js_sys::JsValue as Input)],
						"{}{}{}",
						interpolate ::js_sys::r#macro::js_select!("(self) => ", "(self) => {\n", [&::js_sys::JsValue]),
						interpolate ::js_sys::r#macro::js_parameter!("self", &::js_sys::JsValue),
						interpolate ::js_sys::r#macro::js_select!("self.test()", "self.test()\n}", [&::js_sys::JsValue]),
					}

					unsafe extern "C" {
						#[link_name = "test_crate.test"]
						fn test(this: <&::js_sys::JsValue as ::js_sys::hazard::Input>::Type);
					}

					unsafe { test(::js_sys::hazard::Input::into_raw(self)) };
				}
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.test, test_crate
			.import_name test_crate.import.test, test
			.functype test_crate.import.test (externref) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.test
			test_crate.test:
				.functype test_crate.test (i32) -> ()
				local.get 0
				call js_sys.externref.get
				call test_crate.import.test
				end_function"
		),
		"(self) => self.test()",
	);
}

#[test]
fn method_par() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				pub fn test(self: &JsTest, par1: &JsValue, par2: &JsValue);
			}
		},
		quote! {
			impl JsTest {
				pub fn test(self: &JsTest, par1: &JsValue, par2: &JsValue) {
					::js_sys::js_bindgen::unsafe_embed_asm! {
						".import_module test_crate.import.test, test_crate",
						".import_name test_crate.import.test, test",
						".functype test_crate.import.test ({}, {}, {}) -> ()",
						"",
						"{}",
						"",
						"{}",
						"",
						".globl test_crate.test",
						"test_crate.test:",
						"\t.functype test_crate.test ({}, {}, {}) -> ()",
						"\tlocal.get 0",
						"\t{}",
						"\tlocal.get 1",
						"\t{}",
						"\tlocal.get 2",
						"\t{}",
						"\tcall test_crate.import.test",
						"\tend_function",
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
						interpolate ::js_sys::r#macro::asm_import!(&::js_sys::JsValue as Input),
						interpolate ::js_sys::r#macro::asm_import!(&JsValue as Input),
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate ::js_sys::r#macro::asm_conv!(&::js_sys::JsValue as Input),
						interpolate ::js_sys::r#macro::asm_conv!(&JsValue as Input),
						interpolate ::js_sys::r#macro::asm_conv!(&JsValue as Input),
					}

					::js_sys::js_bindgen::import_js! {
						module = "test_crate",
						name = "test",
						required_embeds = [::js_sys::r#macro::js_import!(&::js_sys::JsValue as Input), ::js_sys::r#macro::js_import!(&JsValue as Input)],
						"{}{}{}{}{}",
						interpolate ::js_sys::r#macro::js_select!("(self, par1, par2) => ", "(self, par1, par2) => {\n", [&::js_sys::JsValue, &JsValue]),
						interpolate ::js_sys::r#macro::js_parameter!("self", &::js_sys::JsValue),
						interpolate ::js_sys::r#macro::js_parameter!("par1", &JsValue),
						interpolate ::js_sys::r#macro::js_parameter!("par2", &JsValue),
						interpolate ::js_sys::r#macro::js_select!("self.test(par1, par2)", "self.test(par1, par2)\n}", [&::js_sys::JsValue, &JsValue]),
					}

					unsafe extern "C" {
						#[link_name = "test_crate.test"]
						fn test(
							this: <&::js_sys::JsValue as ::js_sys::hazard::Input>::Type,
							par1: <&JsValue as ::js_sys::hazard::Input>::Type,
							par2: <&JsValue as ::js_sys::hazard::Input>::Type,
						);
					}

					unsafe { test(
						::js_sys::hazard::Input::into_raw(self),
						::js_sys::hazard::Input::into_raw(par1),
						::js_sys::hazard::Input::into_raw(par2),
					) };
				}
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.test, test_crate
			.import_name test_crate.import.test, test
			.functype test_crate.import.test (externref, externref, externref) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.test
			test_crate.test:
				.functype test_crate.test (i32, i32, i32) -> ()
				local.get 0
				call js_sys.externref.get
				local.get 1
				call js_sys.externref.get
				local.get 2
				call js_sys.externref.get
				call test_crate.import.test
				end_function"
		),
		"(self, par1, par2) => self.test(par1, par2)",
	);
}

#[test]
fn getter() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				#[js_sys(property)]
				pub fn test(self: &JsTest) -> JsValue;
			}
		},
		quote! {
			impl JsTest {
				pub fn test(self: &JsTest) -> JsValue {
					::js_sys::js_bindgen::unsafe_embed_asm! {
						".import_module test_crate.import.test, test_crate",
						".import_name test_crate.import.test, test",
						".functype test_crate.import.test ({}) -> ({})",
						"",
						"{}",
						"",
						"{}",
						"",
						".globl test_crate.test",
						"test_crate.test:",
						"\t.functype test_crate.test ({}) -> ({})",
						"\tlocal.get 0",
						"\t{}",
						"\tcall test_crate.import.test",
						"\t{}",
						"\tend_function",
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
						interpolate <JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_TYPE,
						interpolate ::js_sys::r#macro::asm_import!(&::js_sys::JsValue as Input),
						interpolate ::js_sys::r#macro::asm_import!(JsValue as Output),
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate <JsValue as ::js_sys::hazard::Output>::ASM_TYPE,
						interpolate ::js_sys::r#macro::asm_conv!(&::js_sys::JsValue as Input),
						interpolate ::js_sys::r#macro::asm_conv!(JsValue as Output),
					}

					::js_sys::js_bindgen::import_js! {
						module = "test_crate",
						name = "test",
						required_embeds = [::js_sys::r#macro::js_import!(&::js_sys::JsValue as Input), ::js_sys::r#macro::js_import!(JsValue as Output)],
						"{}{}{}",
						interpolate ::js_sys::r#macro::js_select!("(self) => ", "(self) => {\n", [&::js_sys::JsValue], JsValue),
						interpolate ::js_sys::r#macro::js_parameter!("self", &::js_sys::JsValue),
						interpolate ::js_sys::r#macro::js_output!("\treturn ", "self.test", "self.test", JsValue, &::js_sys::JsValue),
					}

					unsafe extern "C" {
						#[link_name = "test_crate.test"]
						fn test(
							this: <&::js_sys::JsValue as ::js_sys::hazard::Input>::Type
						) -> <JsValue as ::js_sys::hazard::Output>::Type;
					}

					::js_sys::hazard::Output::from_raw(unsafe {
						test(::js_sys::hazard::Input::into_raw(self))
					})
				}
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.test, test_crate
			.import_name test_crate.import.test, test
			.functype test_crate.import.test (externref) -> (externref)

			.functype js_sys.externref.get (i32) -> (externref)

			.functype js_sys.externref.insert (externref) -> (i32)

			.globl test_crate.test
			test_crate.test:
				.functype test_crate.test (i32) -> (i32)
				local.get 0
				call js_sys.externref.get
				call test_crate.import.test
				call js_sys.externref.insert
				end_function"
		),
		"(self) => self.test",
	);
}

#[test]
fn setter() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				#[js_sys(property)]
				pub fn test(self: &JsTest, value: &JsValue);
			}
		},
		quote! {
			impl JsTest {
				pub fn test(self: &JsTest, value: &JsValue) {
					::js_sys::js_bindgen::unsafe_embed_asm! {
						".import_module test_crate.import.test, test_crate",
						".import_name test_crate.import.test, test",
						".functype test_crate.import.test ({}, {}) -> ()",
						"",
						"{}",
						"",
						"{}",
						"",
						".globl test_crate.test",
						"test_crate.test:",
						"\t.functype test_crate.test ({}, {}) -> ()",
						"\tlocal.get 0",
						"\t{}",
						"\tlocal.get 1",
						"\t{}",
						"\tcall test_crate.import.test",
						"\tend_function",
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE,
						interpolate ::js_sys::r#macro::asm_import!(&::js_sys::JsValue as Input),
						interpolate ::js_sys::r#macro::asm_import!(&JsValue as Input),
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate ::js_sys::r#macro::asm_conv!(&::js_sys::JsValue as Input),
						interpolate ::js_sys::r#macro::asm_conv!(&JsValue as Input),
					}

					::js_sys::js_bindgen::import_js! {
						module = "test_crate",
						name = "test",
						required_embeds = [::js_sys::r#macro::js_import!(&::js_sys::JsValue as Input), ::js_sys::r#macro::js_import!(&JsValue as Input)],
						"{}{}{}self.test = {}",
						interpolate ::js_sys::r#macro::js_select!("(self, value) => ", "(self, value) => {\n", [&::js_sys::JsValue, &JsValue]),
						interpolate ::js_sys::r#macro::js_parameter!("self", &::js_sys::JsValue),
						interpolate ::js_sys::r#macro::js_parameter!("value", &JsValue),
						interpolate ::js_sys::r#macro::js_select!("value", "value\n}", [&::js_sys::JsValue, &JsValue]),
					}

					unsafe extern "C" {
						#[link_name = "test_crate.test"]
						fn test(
							this: <&::js_sys::JsValue as ::js_sys::hazard::Input>::Type,
							value: <&JsValue as ::js_sys::hazard::Input>::Type,
						);
					}

					unsafe { test(
						::js_sys::hazard::Input::into_raw(self),
						::js_sys::hazard::Input::into_raw(value),
					) };
				}
			}
		},
		indoc::indoc!(
			".import_module test_crate.import.test, test_crate
			.import_name test_crate.import.test, test
			.functype test_crate.import.test (externref, externref) -> ()

			.functype js_sys.externref.get (i32) -> (externref)

			.functype js_sys.externref.get (i32) -> (externref)

			.globl test_crate.test
			test_crate.test:
				.functype test_crate.test (i32, i32) -> ()
				local.get 0
				call js_sys.externref.get
				local.get 1
				call js_sys.externref.get
				call test_crate.import.test
				end_function"
		),
		"(self, value) => self.test = value",
	);
}
