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

			impl ::core::convert::AsRef<::js_sys::JsValue> for JsString {
				fn as_ref(&self) -> &::js_sys::JsValue {
					&self.0
				}
			}

			impl ::core::convert::From<JsString> for ::js_sys::JsValue {
				fn from(value: JsString) -> Self {
					value.0
				}
			}

			unsafe impl ::js_sys::hazard::Input for &JsString {
				const WAT_TYPE: &'static ::core::primitive::str =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::WAT_TYPE;
				const WAT_CONV: ::core::option::Option<::js_sys::hazard::InputWatConv> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::WAT_CONV;
				const JS_CONV: ::core::option::Option<::js_sys::hazard::InputJsConv> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.0)
				}
			}

			unsafe impl ::js_sys::hazard::JsCast for JsString {}

			unsafe impl ::js_sys::hazard::Output for JsString {
				const WAT_TYPE: &::core::primitive::str =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::WAT_TYPE;
				const WAT_CONV: ::core::option::Option<::js_sys::hazard::OutputWatConv> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::WAT_CONV;
				const JS_CONV: ::core::option::Option<::js_sys::hazard::OutputJsConv> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::JS_CONV;

				type Type = <::js_sys::JsValue as ::js_sys::hazard::Output>::Type;

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

			impl<T> ::core::convert::AsRef<::js_sys::JsValue> for JsString<T> {
				fn as_ref(&self) -> &::js_sys::JsValue {
					&self.value
				}
			}

			impl<T> ::core::convert::From<JsString<T>> for ::js_sys::JsValue {
				fn from(value: JsString<T>) -> Self {
					value.value
				}
			}

			unsafe impl<T> ::js_sys::hazard::Input for &JsString<T> {
				const WAT_TYPE: &'static ::core::primitive::str =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::WAT_TYPE;
				const WAT_CONV: ::core::option::Option<::js_sys::hazard::InputWatConv> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::WAT_CONV;
				const JS_CONV: ::core::option::Option<::js_sys::hazard::InputJsConv> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.value)
				}
			}

			unsafe impl<T> ::js_sys::hazard::JsCast for JsString<T> {}

			unsafe impl<T> ::js_sys::hazard::Output for JsString<T> {
				const WAT_TYPE: &::core::primitive::str =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::WAT_TYPE;
				const WAT_CONV: ::core::option::Option<::js_sys::hazard::OutputWatConv> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::WAT_CONV;
				const JS_CONV: ::core::option::Option<::js_sys::hazard::OutputJsConv> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::JS_CONV;

				type Type = <::js_sys::JsValue as ::js_sys::hazard::Output>::Type;

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

			impl<T> ::core::convert::AsRef<::js_sys::JsValue> for JsString<T> {
				fn as_ref(&self) -> &::js_sys::JsValue {
					&self.value
				}
			}

			impl<T> ::core::convert::From<JsString<T>> for ::js_sys::JsValue {
				fn from(value: JsString<T>) -> Self {
					value.value
				}
			}

			unsafe impl<T> ::js_sys::hazard::Input for &JsString<T> {
				const WAT_TYPE: &'static ::core::primitive::str =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::WAT_TYPE;
				const WAT_CONV: ::core::option::Option<::js_sys::hazard::InputWatConv> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::WAT_CONV;
				const JS_CONV: ::core::option::Option<::js_sys::hazard::InputJsConv> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.value)
				}
			}

			unsafe impl<T> ::js_sys::hazard::JsCast for JsString<T> {}

			unsafe impl<T> ::js_sys::hazard::Output for JsString<T> {
				const WAT_TYPE: &::core::primitive::str =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::WAT_TYPE;
				const WAT_CONV: ::core::option::Option<::js_sys::hazard::OutputWatConv> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::WAT_CONV;
				const JS_CONV: ::core::option::Option<::js_sys::hazard::OutputJsConv> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::JS_CONV;

				type Type = <::js_sys::JsValue as ::js_sys::hazard::Output>::Type;

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

			impl<T: Sized> ::core::convert::AsRef<::js_sys::JsValue> for JsString<T> {
				fn as_ref(&self) -> &::js_sys::JsValue {
					&self.value
				}
			}

			impl<T: Sized> ::core::convert::From<JsString<T>> for ::js_sys::JsValue {
				fn from(value: JsString<T>) -> Self {
					value.value
				}
			}

			unsafe impl<T: Sized> ::js_sys::hazard::Input for &JsString<T> {
				const WAT_TYPE: &'static ::core::primitive::str =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::WAT_TYPE;
				const WAT_CONV: ::core::option::Option<::js_sys::hazard::InputWatConv> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::WAT_CONV;
				const JS_CONV: ::core::option::Option<::js_sys::hazard::InputJsConv> =
					<&::js_sys::JsValue as ::js_sys::hazard::Input>::JS_CONV;

				type Type = <&'static ::js_sys::JsValue as ::js_sys::hazard::Input>::Type;

				fn into_raw(self) -> Self::Type {
					::js_sys::hazard::Input::into_raw(&self.value)
				}
			}

			unsafe impl<T: Sized> ::js_sys::hazard::JsCast for JsString<T> {}

			unsafe impl<T: Sized> ::js_sys::hazard::Output for JsString<T> {
				const WAT_TYPE: &::core::primitive::str =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::WAT_TYPE;
				const WAT_CONV: ::core::option::Option<::js_sys::hazard::OutputWatConv> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::WAT_CONV;
				const JS_CONV: ::core::option::Option<::js_sys::hazard::OutputJsConv> =
					<::js_sys::JsValue as ::js_sys::hazard::Output>::JS_CONV;

				type Type = <::js_sys::JsValue as ::js_sys::hazard::Output>::Type;

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
