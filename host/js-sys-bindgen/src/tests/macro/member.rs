use proc_macro2::TokenStream;
use quote::quote;

#[test]
fn method() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				pub type JsTest;

				pub fn test(self: &JsTest);
			}
		},
		quote! {
			#[repr(transparent)]
			pub struct JsTest(::js_sys::JsValue);

			impl ::core::ops::Deref for JsTest {
				type Target = ::js_sys::JsValue;

				fn deref(&self) -> &Self::Target {
					&self.0
				}
			}

			impl ::core::convert::From<JsTest> for ::js_sys::JsValue {
				fn from(value: JsTest) -> Self {
					value.0
				}
			}

			unsafe impl ::js_sys::hazard::Input for &JsTest {
				const IMPORT_FUNC: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC;
				const IMPORT_TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE;
				const TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::TYPE;
				const CONV: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.0)
				}
			}

			unsafe impl ::js_sys::hazard::Output for JsTest {
				const IMPORT_FUNC: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::IMPORT_FUNC;
				const IMPORT_TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::IMPORT_TYPE;
				const TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::TYPE;
				const CONV: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::CONV;

				type Type = <::js_sys::JsValue as ::js_sys::hazard::Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self(::js_sys::hazard::Output::from_raw(raw))
				}
			}

			impl JsTest {
				#[must_use]
				pub fn unchecked_from(value: ::js_sys::JsValue) -> Self {
					Self(value)
				}
			}

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
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::TYPE,
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::CONV,
					}

					::js_sys::js_bindgen::import_js! {
						name = "test", "{}{}{}{}{}",
						interpolate::js_sys::r#macro::select(
							"(self) => self.test()",
							"(self) => {\n",
							[<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]
						),
						interpolate::js_sys::r#macro::select("", "\tself", [<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select(
							"",
							<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV,
							[<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]
						),
						interpolate::js_sys::r#macro::select("", "\n", [<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select("", "\tself.test()\n}", [<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]),
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
				pub type JsTest;

				pub fn test(self: &JsTest, par1: &JsValue, par2: &JsValue);
			}
		},
		quote! {
			#[repr(transparent)]
			pub struct JsTest(::js_sys::JsValue);

			impl ::core::ops::Deref for JsTest {
				type Target = ::js_sys::JsValue;

				fn deref(&self) -> &Self::Target {
					&self.0
				}
			}

			impl ::core::convert::From<JsTest> for ::js_sys::JsValue {
				fn from(value: JsTest) -> Self {
					value.0
				}
			}

			unsafe impl ::js_sys::hazard::Input for &JsTest {
				const IMPORT_FUNC: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC;
				const IMPORT_TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE;
				const TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::TYPE;
				const CONV: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.0)
				}
			}

			unsafe impl ::js_sys::hazard::Output for JsTest {
				const IMPORT_FUNC: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::IMPORT_FUNC;
				const IMPORT_TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::IMPORT_TYPE;
				const TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::TYPE;
				const CONV: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::CONV;

				type Type = <::js_sys::JsValue as ::js_sys::hazard::Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self(::js_sys::hazard::Output::from_raw(raw))
				}
			}

			impl JsTest {
				#[must_use]
				pub fn unchecked_from(value: ::js_sys::JsValue) -> Self {
					Self(value)
				}
			}

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
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
						interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
						interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::TYPE,
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::CONV,
						interpolate <&JsValue as ::js_sys::hazard::Input>::CONV,
						interpolate <&JsValue as ::js_sys::hazard::Input>::CONV,
					}

					::js_sys::js_bindgen::import_js! {
						name = "test", "{}{}{}{}{}{}{}{}{}{}{}",
						interpolate::js_sys::r#macro::select(
							"(self, par1, par2) => self.test(par1, par2)",
							"(self, par1, par2) => {\n",
							[
								<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV,
								<&JsValue as ::js_sys::hazard::Input>::JS_CONV,
								<&JsValue as ::js_sys::hazard::Input>::JS_CONV
							]
						),
						interpolate::js_sys::r#macro::select("", "\tself", [<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select(
							"",
							<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV,
							[<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]
						),
						interpolate::js_sys::r#macro::select("", "\n", [<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select("", "\tpar1", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select(
							"",
							<&JsValue as ::js_sys::hazard::Input>::JS_CONV,
							[<&JsValue as ::js_sys::hazard::Input>::JS_CONV]
						),
						interpolate::js_sys::r#macro::select("", "\n", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select("", "\tpar2", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select(
							"",
							<&JsValue as ::js_sys::hazard::Input>::JS_CONV,
							[<&JsValue as ::js_sys::hazard::Input>::JS_CONV]
						),
						interpolate::js_sys::r#macro::select("", "\n", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select(
							"",
							"\tself.test(par1, par2)\n}",
							[
								<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV,
								<&JsValue as ::js_sys::hazard::Input>::JS_CONV,
								<&JsValue as ::js_sys::hazard::Input>::JS_CONV
							]
						),
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
				pub type JsTest;

				#[js_sys(property)]
				pub fn test(self: &JsTest) -> JsValue;
			}
		},
		quote! {
			#[repr(transparent)]
			pub struct JsTest(::js_sys::JsValue);

			impl ::core::ops::Deref for JsTest {
				type Target = ::js_sys::JsValue;

				fn deref(&self) -> &Self::Target {
					&self.0
				}
			}

			impl ::core::convert::From<JsTest> for ::js_sys::JsValue {
				fn from(value: JsTest) -> Self {
					value.0
				}
			}

			unsafe impl ::js_sys::hazard::Input for &JsTest {
				const IMPORT_FUNC: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC;
				const IMPORT_TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE;
				const TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::TYPE;
				const CONV: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.0)
				}
			}

			unsafe impl ::js_sys::hazard::Output for JsTest {
				const IMPORT_FUNC: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::IMPORT_FUNC;
				const IMPORT_TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::IMPORT_TYPE;
				const TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::TYPE;
				const CONV: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::CONV;

				type Type = <::js_sys::JsValue as ::js_sys::hazard::Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self(::js_sys::hazard::Output::from_raw(raw))
				}
			}

			impl JsTest {
				#[must_use]
				pub fn unchecked_from(value: ::js_sys::JsValue) -> Self {
					Self(value)
				}
			}

			impl JsTest {
				pub fn test(self: &JsTest) -> JsValue{
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
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
						interpolate <JsValue as ::js_sys::hazard::Output>::IMPORT_TYPE,
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
						interpolate <JsValue as ::js_sys::hazard::Output>::IMPORT_FUNC,
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::TYPE,
						interpolate <JsValue as ::js_sys::hazard::Output>::TYPE,
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::CONV,
						interpolate <JsValue as ::js_sys::hazard::Output>::CONV,
					}

					::js_sys::js_bindgen::import_js! {
						name = "test", "{}{}{}{}{}",
						interpolate::js_sys::r#macro::select(
							"(self) => self.test",
							"(self) => {\n",
							[<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]
						),
						interpolate::js_sys::r#macro::select("", "\tself", [<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select(
							"",
							<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV,
							[<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]
						),
						interpolate::js_sys::r#macro::select("", "\n", [<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select("", "\treturn self.test\n}", [<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]),
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
				pub type JsTest;

				#[js_sys(property)]
				pub fn test(self: &JsTest, value: &JsValue);
			}
		},
		quote! {
			#[repr(transparent)]
			pub struct JsTest(::js_sys::JsValue);

			impl ::core::ops::Deref for JsTest {
				type Target = ::js_sys::JsValue;

				fn deref(&self) -> &Self::Target {
					&self.0
				}
			}

			impl ::core::convert::From<JsTest> for ::js_sys::JsValue {
				fn from(value: JsTest) -> Self {
					value.0
				}
			}

			unsafe impl ::js_sys::hazard::Input for &JsTest {
				const IMPORT_FUNC: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC;
				const IMPORT_TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE;
				const TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::TYPE;
				const CONV: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.0)
				}
			}

			unsafe impl ::js_sys::hazard::Output for JsTest {
				const IMPORT_FUNC: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::IMPORT_FUNC;
				const IMPORT_TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::IMPORT_TYPE;
				const TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::TYPE;
				const CONV: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::CONV;

				type Type = <::js_sys::JsValue as ::js_sys::hazard::Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self(::js_sys::hazard::Output::from_raw(raw))
				}
			}

			impl JsTest {
				#[must_use]
				pub fn unchecked_from(value: ::js_sys::JsValue) -> Self {
					Self(value)
				}
			}

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
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_TYPE,
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
						interpolate <&JsValue as ::js_sys::hazard::Input>::IMPORT_FUNC,
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::TYPE,
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::CONV,
						interpolate <&JsValue as ::js_sys::hazard::Input>::CONV,
					}

					::js_sys::js_bindgen::import_js! {
						name = "test", "{}{}{}{}{}{}{}{}",
						interpolate::js_sys::r#macro::select(
							"(self, value) => self.test = value",
							"(self, value) => {\n",
							[
								<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV,
								<&JsValue as ::js_sys::hazard::Input>::JS_CONV
							]
						),
						interpolate::js_sys::r#macro::select("", "\tself", [<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select(
							"",
							<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV,
							[<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]
						),
						interpolate::js_sys::r#macro::select("", "\n", [<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select("", "\tvalue", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select(
							"",
							<&JsValue as ::js_sys::hazard::Input>::JS_CONV,
							[<&JsValue as ::js_sys::hazard::Input>::JS_CONV]
						),
						interpolate::js_sys::r#macro::select("", "\n", [<&JsValue as ::js_sys::hazard::Input>::JS_CONV]),
						interpolate::js_sys::r#macro::select(
							"",
							"\tself.test = value\n}",
							[
								<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV,
								<&JsValue as ::js_sys::hazard::Input>::JS_CONV
							]
						),
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
