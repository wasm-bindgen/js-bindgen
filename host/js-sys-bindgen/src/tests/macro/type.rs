#[test]
fn basic() {
	test!(
		{},
		{
			extern "js-sys" {
				pub type JsString;
			}
		},
		{
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
				const ASM_IMPORT_FUNC: ::core::option::Option<&'static ::core::primitive::str> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &'static ::core::primitive::str =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &'static ::core::primitive::str =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&'static ::core::primitive::str> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_CONV;
				const JS_EMBED: ::core::option::Option<(
					&'static ::core::primitive::str,
					&'static ::core::primitive::str,
				)> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_EMBED;
				const JS_CONV: ::core::option::Option<(
					&'static ::core::primitive::str,
					::core::option::Option<&'static ::core::primitive::str>,
				)> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.0)
				}
			}

			unsafe impl ::js_sys::hazard::Output for JsString {
				const ASM_IMPORT_FUNC: ::core::option::Option<&::core::primitive::str> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &::core::primitive::str =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_TYPE;
				const ASM_DIRECT: ::core::primitive::bool =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_DIRECT;
				const ASM_TYPE: &::core::primitive::str =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&::core::primitive::str> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_CONV;
				const JS_EMBED: ::core::option::Option<(
					&'static ::core::primitive::str,
					&'static ::core::primitive::str,
				)> = <::js_sys::JsValue as ::js_sys::hazard::Output>::JS_EMBED;
				const JS_CONV: ::core::option::Option<(
					&'static ::core::primitive::str,
					&'static ::core::primitive::str,
				)> = <::js_sys::JsValue as ::js_sys::hazard::Output>::JS_CONV;

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
	test!(
		{},
		{
			extern "js-sys" {
				pub type JsString<T>;
			}
		},
		{
			#[repr(transparent)]
			pub struct JsString<T> {
				value: ::js_sys::JsValue,
				_type: ::core::marker::PhantomData<T>,
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
				const ASM_IMPORT_FUNC: ::core::option::Option<&'static ::core::primitive::str> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &'static ::core::primitive::str =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &'static ::core::primitive::str =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&'static ::core::primitive::str> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_CONV;
				const JS_EMBED: ::core::option::Option<(
					&'static ::core::primitive::str,
					&'static ::core::primitive::str,
				)> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_EMBED;
				const JS_CONV: ::core::option::Option<(
					&'static ::core::primitive::str,
					::core::option::Option<&'static ::core::primitive::str>,
				)> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.value)
				}
			}

			unsafe impl<T> ::js_sys::hazard::Output for JsString<T> {
				const ASM_IMPORT_FUNC: ::core::option::Option<&::core::primitive::str> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &::core::primitive::str =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_TYPE;
				const ASM_DIRECT: ::core::primitive::bool =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_DIRECT;
				const ASM_TYPE: &::core::primitive::str =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&::core::primitive::str> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_CONV;
				const JS_EMBED: ::core::option::Option<(
					&'static ::core::primitive::str,
					&'static ::core::primitive::str,
				)> = <::js_sys::JsValue as ::js_sys::hazard::Output>::JS_EMBED;
				const JS_CONV: ::core::option::Option<(
					&'static ::core::primitive::str,
					&'static ::core::primitive::str,
				)> = <::js_sys::JsValue as ::js_sys::hazard::Output>::JS_CONV;

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
	test!(
		{},
		{
			extern "js-sys" {
				pub type JsString<T = JsValue>;
			}
		},
		{
			#[repr(transparent)]
			pub struct JsString<T = JsValue> {
				value: ::js_sys::JsValue,
				_type: ::core::marker::PhantomData<T>,
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
				const ASM_IMPORT_FUNC: ::core::option::Option<&'static ::core::primitive::str> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &'static ::core::primitive::str =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &'static ::core::primitive::str =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&'static ::core::primitive::str> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_CONV;
				const JS_EMBED: ::core::option::Option<(
					&'static ::core::primitive::str,
					&'static ::core::primitive::str,
				)> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_EMBED;
				const JS_CONV: ::core::option::Option<(
					&'static ::core::primitive::str,
					::core::option::Option<&'static ::core::primitive::str>,
				)> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.value)
				}
			}

			unsafe impl<T> ::js_sys::hazard::Output for JsString<T> {
				const ASM_IMPORT_FUNC: ::core::option::Option<&::core::primitive::str> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &::core::primitive::str =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_TYPE;
				const ASM_DIRECT: ::core::primitive::bool =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_DIRECT;
				const ASM_TYPE: &::core::primitive::str =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&::core::primitive::str> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_CONV;
				const JS_EMBED: ::core::option::Option<(
					&'static ::core::primitive::str,
					&'static ::core::primitive::str,
				)> = <::js_sys::JsValue as ::js_sys::hazard::Output>::JS_EMBED;
				const JS_CONV: ::core::option::Option<(
					&'static ::core::primitive::str,
					&'static ::core::primitive::str,
				)> = <::js_sys::JsValue as ::js_sys::hazard::Output>::JS_CONV;

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
	test!(
		{},
		{
			extern "js-sys" {
				pub type JsString<T: Sized = JsValue>;
			}
		},
		{
			#[repr(transparent)]
			pub struct JsString<T: Sized = JsValue> {
				value: ::js_sys::JsValue,
				_type: ::core::marker::PhantomData<T>,
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
				const ASM_IMPORT_FUNC: ::core::option::Option<&'static ::core::primitive::str> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &'static ::core::primitive::str =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &'static ::core::primitive::str =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&'static ::core::primitive::str> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_CONV;
				const JS_EMBED: ::core::option::Option<(
					&'static ::core::primitive::str,
					&'static ::core::primitive::str,
				)> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_EMBED;
				const JS_CONV: ::core::option::Option<(
					&'static ::core::primitive::str,
					::core::option::Option<&'static ::core::primitive::str>,
				)> = <&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.value)
				}
			}

			unsafe impl<T: Sized> ::js_sys::hazard::Output for JsString<T> {
				const ASM_IMPORT_FUNC: ::core::option::Option<&::core::primitive::str> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &::core::primitive::str =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_IMPORT_TYPE;
				const ASM_DIRECT: ::core::primitive::bool =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_DIRECT;
				const ASM_TYPE: &::core::primitive::str =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_TYPE;
				const ASM_CONV: ::core::option::Option<&::core::primitive::str> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::ASM_CONV;
				const JS_EMBED: ::core::option::Option<(
					&'static ::core::primitive::str,
					&'static ::core::primitive::str,
				)> = <::js_sys::JsValue as ::js_sys::hazard::Output>::JS_EMBED;
				const JS_CONV: ::core::option::Option<(
					&'static ::core::primitive::str,
					&'static ::core::primitive::str,
				)> = <::js_sys::JsValue as ::js_sys::hazard::Output>::JS_CONV;

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
