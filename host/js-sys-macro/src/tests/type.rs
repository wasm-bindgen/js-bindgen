use proc_macro2::TokenStream;
use quote::quote;

#[test]
fn basic() {
	super::test(
		TokenStream::new(),
		quote! {
			extern "C" {
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

			unsafe impl ::js_sys::hazard::Input for &JsString {
				const IMPORT_FUNC: &'static ::core::primitive::str = ".functype js_sys.externref.get (i32) -> (externref)";
				const IMPORT_TYPE: &'static ::core::primitive::str = "externref";
				const TYPE: &'static ::core::primitive::str = "i32";
				const CONV: &'static ::core::primitive::str = "call js_sys.externref.get";

				type Type = ::core::primitive::i32;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.0)
				}
			}

			unsafe impl ::js_sys::hazard::Output for JsString {
				const IMPORT_FUNC: &::core::primitive::str = ".functype js_sys.externref.insert (externref) -> (i32)";
				const IMPORT_TYPE: &::core::primitive::str = "externref";
				const TYPE: &::core::primitive::str = "i32";
				const CONV: &::core::primitive::str = "call js_sys.externref.insert";

				type Type = ::core::primitive::i32;

				fn from_raw(raw: Self::Type) -> Self {
					Self(::js_sys::hazard::Output::from_raw(raw))
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
			extern "C" {
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

			unsafe impl<T> ::js_sys::hazard::Input for &JsString<T> {
				const IMPORT_FUNC: &'static ::core::primitive::str = ".functype js_sys.externref.get (i32) -> (externref)";
				const IMPORT_TYPE: &'static ::core::primitive::str = "externref";
				const TYPE: &'static ::core::primitive::str = "i32";
				const CONV: &'static ::core::primitive::str = "call js_sys.externref.get";

				type Type = ::core::primitive::i32;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.value)
				}
			}

			unsafe impl<T> ::js_sys::hazard::Output for JsString<T> {
				const IMPORT_FUNC: &::core::primitive::str = ".functype js_sys.externref.insert (externref) -> (i32)";
				const IMPORT_TYPE: &::core::primitive::str = "externref";
				const TYPE: &::core::primitive::str = "i32";
				const CONV: &::core::primitive::str = "call js_sys.externref.insert";

				type Type = ::core::primitive::i32;

				fn from_raw(raw: Self::Type) -> Self {
					Self {
						value: ::js_sys::hazard::Output::from_raw(raw),
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
			extern "C" {
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

			unsafe impl<T> ::js_sys::hazard::Input for &JsString<T> {
				const IMPORT_FUNC: &'static ::core::primitive::str = ".functype js_sys.externref.get (i32) -> (externref)";
				const IMPORT_TYPE: &'static ::core::primitive::str = "externref";
				const TYPE: &'static ::core::primitive::str = "i32";
				const CONV: &'static ::core::primitive::str = "call js_sys.externref.get";

				type Type = ::core::primitive::i32;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.value)
				}
			}

			unsafe impl<T> ::js_sys::hazard::Output for JsString<T> {
				const IMPORT_FUNC: &::core::primitive::str = ".functype js_sys.externref.insert (externref) -> (i32)";
				const IMPORT_TYPE: &::core::primitive::str = "externref";
				const TYPE: &::core::primitive::str = "i32";
				const CONV: &::core::primitive::str = "call js_sys.externref.insert";

				type Type = ::core::primitive::i32;

				fn from_raw(raw: Self::Type) -> Self {
					Self {
						value: ::js_sys::hazard::Output::from_raw(raw),
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
			extern "C" {
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

			unsafe impl<T: Sized> ::js_sys::hazard::Input for &JsString<T> {
				const IMPORT_FUNC: &'static ::core::primitive::str = ".functype js_sys.externref.get (i32) -> (externref)";
				const IMPORT_TYPE: &'static ::core::primitive::str = "externref";
				const TYPE: &'static ::core::primitive::str = "i32";
				const CONV: &'static ::core::primitive::str = "call js_sys.externref.get";

				type Type = ::core::primitive::i32;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.value)
				}
			}

			unsafe impl<T: Sized> ::js_sys::hazard::Output for JsString<T> {
				const IMPORT_FUNC: &::core::primitive::str = ".functype js_sys.externref.insert (externref) -> (i32)";
				const IMPORT_TYPE: &::core::primitive::str = "externref";
				const TYPE: &::core::primitive::str = "i32";
				const CONV: &::core::primitive::str = "call js_sys.externref.insert";

				type Type = ::core::primitive::i32;

				fn from_raw(raw: Self::Type) -> Self {
					Self {
						value: ::js_sys::hazard::Output::from_raw(raw),
						_type: ::core::marker::PhantomData,
					}
				}
			}
		},
		None,
		None,
	);
}
