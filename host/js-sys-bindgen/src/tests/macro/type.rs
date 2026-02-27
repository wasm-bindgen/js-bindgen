use proc_macro2::TokenStream;
use quote::quote;

#[test]
fn basic() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				pub type JsString;
			}
		},
		quote! {
			#[repr(transparent)]
			pub struct JsString(::js_sys::JsValue);

			impl ::core::ops::Deref for JsString {
				type Target = ::js_sys::JsValue;

				fn deref(&self) -> &Self::Target {
					&self.0
				}
			}

			impl ::core::convert::From<JsString> for ::js_sys::JsValue {
				fn from(value: JsString) -> Self {
					value.0
				}
			}

			unsafe impl ::js_sys::hazard::Input for &JsString {
				const ASM_IMPORT_FUNC: ::core::option::Option<&'static ::core::primitive::str> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&'static ::core::primitive::str> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.0)
				}
			}

			unsafe impl ::js_sys::hazard::Output for JsString {
				const ASM_IMPORT_FUNC: ::core::option::Option<&::core::primitive::str> = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&::core::primitive::str> = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_CONV;

				type Type = <::js_sys::JsValue as ::js_sys::hazard::Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self(::js_sys::hazard::Output::from_raw(raw))
				}
			}

			impl JsString {
				#[must_use]
				pub fn unchecked_from(value: ::js_sys::JsValue) -> Self {
					Self(value)
				}
			}
		},
		None,
		None,
	);
}

#[test]
fn generic() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				pub type JsString<T>;
			}
		},
		quote! {
			#[repr(transparent)]
			pub struct JsString<T> {
				value: ::js_sys::JsValue,
				_type: ::core::marker::PhantomData<T>
			}

			impl<T> ::core::ops::Deref for JsString<T> {
				type Target = ::js_sys::JsValue;

				fn deref(&self) -> &Self::Target {
					&self.value
				}
			}

			impl<T> ::core::convert::From<JsString<T>> for ::js_sys::JsValue {
				fn from(value: JsString<T>) -> Self {
					value.value
				}
			}

			unsafe impl<T> ::js_sys::hazard::Input for &JsString<T> {
				const ASM_IMPORT_FUNC: ::core::option::Option<&'static ::core::primitive::str> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&'static ::core::primitive::str> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.value)
				}
			}

			unsafe impl<T> ::js_sys::hazard::Output for JsString<T> {
				const ASM_IMPORT_FUNC: ::core::option::Option<&::core::primitive::str> = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&::core::primitive::str> = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_CONV;

				type Type = <::js_sys::JsValue as ::js_sys::hazard::Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self {
						value: ::js_sys::hazard::Output::from_raw(raw),
						_type: ::core::marker::PhantomData,
					}
				}
			}

			impl<T> JsString<T> {
				#[must_use]
				pub fn unchecked_from(value: ::js_sys::JsValue) -> Self {
					Self {
						value,
						_type: ::core::marker::PhantomData,
					}
				}
			}
		},
		None,
		None,
	);
}

#[test]
fn default() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				pub type JsString<T = JsValue>;
			}
		},
		quote! {
			#[repr(transparent)]
			pub struct JsString<T = JsValue> {
				value: ::js_sys::JsValue,
				_type: ::core::marker::PhantomData<T>
			}

			impl<T> ::core::ops::Deref for JsString<T> {
				type Target = ::js_sys::JsValue;

				fn deref(&self) -> &Self::Target {
					&self.value
				}
			}

			impl<T> ::core::convert::From<JsString<T>> for ::js_sys::JsValue {
				fn from(value: JsString<T>) -> Self {
					value.value
				}
			}

			unsafe impl<T> ::js_sys::hazard::Input for &JsString<T> {
				const ASM_IMPORT_FUNC: ::core::option::Option<&'static ::core::primitive::str> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&'static ::core::primitive::str> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.value)
				}
			}

			unsafe impl<T> ::js_sys::hazard::Output for JsString<T> {
				const ASM_IMPORT_FUNC: ::core::option::Option<&::core::primitive::str> = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&::core::primitive::str> = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_CONV;

				type Type = <::js_sys::JsValue as ::js_sys::hazard::Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self {
						value: ::js_sys::hazard::Output::from_raw(raw),
						_type: ::core::marker::PhantomData,
					}
				}
			}

			impl<T> JsString<T> {
				#[must_use]
				pub fn unchecked_from(value: ::js_sys::JsValue) -> Self {
					Self {
						value,
						_type: ::core::marker::PhantomData,
					}
				}
			}
		},
		None,
		None,
	);
}

#[test]
fn r#trait() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "js-sys" {
				pub type JsString<T: Sized = JsValue>;
			}
		},
		quote! {
			#[repr(transparent)]
			pub struct JsString<T: Sized = JsValue> {
				value: ::js_sys::JsValue,
				_type: ::core::marker::PhantomData<T>
			}

			impl<T: Sized> ::core::ops::Deref for JsString<T> {
				type Target = ::js_sys::JsValue;

				fn deref(&self) -> &Self::Target {
					&self.value
				}
			}

			impl<T: Sized> ::core::convert::From<JsString<T>> for ::js_sys::JsValue {
				fn from(value: JsString<T>) -> Self {
					value.value
				}
			}

			unsafe impl<T: Sized> ::js_sys::hazard::Input for &JsString<T> {
				const ASM_IMPORT_FUNC: ::core::option::Option<&'static ::core::primitive::str> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &'static ::core::primitive::str = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&'static ::core::primitive::str> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.value)
				}
			}

			unsafe impl<T: Sized> ::js_sys::hazard::Output for JsString<T> {
				const ASM_IMPORT_FUNC: ::core::option::Option<&::core::primitive::str> = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &::core::primitive::str = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&::core::primitive::str> = <::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_CONV;

				type Type = <::js_sys::JsValue as ::js_sys::hazard::Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self {
						value: ::js_sys::hazard::Output::from_raw(raw),
						_type: ::core::marker::PhantomData,
					}
				}
			}

			impl<T: Sized> JsString<T> {
				#[must_use]
				pub fn unchecked_from(value: ::js_sys::JsValue) -> Self {
					Self {
						value,
						_type: ::core::marker::PhantomData,
					}
				}
			}
		},
		None,
		None,
	);
}
