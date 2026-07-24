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

			unsafe impl ::js_sys::hazard::JsCast for JsString {}

			unsafe impl ::js_sys::hazard::IntoJS for JsString {
				type Abi = <::js_sys::JsValue as ::js_sys::hazard::IntoJS>::Abi;

				fn into_abi(self) -> Self::Abi {
					::js_sys::hazard::IntoJS::into_abi(::js_sys::JsValue::from(self))
				}
			}

			unsafe impl ::js_sys::hazard::OptionIntoJS for JsString {
				type OptionAbi = <::js_sys::JsValue as ::js_sys::hazard::OptionIntoJS>::OptionAbi;

				fn option_into_abi(value: ::core::option::Option<Self>) -> Self::OptionAbi {
					::js_sys::hazard::OptionIntoJS::option_into_abi(
						value.map(::js_sys::JsValue::from),
					)
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

			unsafe impl<T> ::js_sys::hazard::JsCast for JsString<T> {}

			unsafe impl<T> ::js_sys::hazard::IntoJS for JsString<T> {
				type Abi = <::js_sys::JsValue as ::js_sys::hazard::IntoJS>::Abi;

				fn into_abi(self) -> Self::Abi {
					::js_sys::hazard::IntoJS::into_abi(::js_sys::JsValue::from(self))
				}
			}

			unsafe impl<T> ::js_sys::hazard::OptionIntoJS for JsString<T> {
				type OptionAbi = <::js_sys::JsValue as ::js_sys::hazard::OptionIntoJS>::OptionAbi;

				fn option_into_abi(value: ::core::option::Option<Self>) -> Self::OptionAbi {
					::js_sys::hazard::OptionIntoJS::option_into_abi(
						value.map(::js_sys::JsValue::from),
					)
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

			unsafe impl<T> ::js_sys::hazard::JsCast for JsString<T> {}

			unsafe impl<T> ::js_sys::hazard::IntoJS for JsString<T> {
				type Abi = <::js_sys::JsValue as ::js_sys::hazard::IntoJS>::Abi;

				fn into_abi(self) -> Self::Abi {
					::js_sys::hazard::IntoJS::into_abi(::js_sys::JsValue::from(self))
				}
			}

			unsafe impl<T> ::js_sys::hazard::OptionIntoJS for JsString<T> {
				type OptionAbi = <::js_sys::JsValue as ::js_sys::hazard::OptionIntoJS>::OptionAbi;

				fn option_into_abi(value: ::core::option::Option<Self>) -> Self::OptionAbi {
					::js_sys::hazard::OptionIntoJS::option_into_abi(
						value.map(::js_sys::JsValue::from),
					)
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

			unsafe impl<T: Sized> ::js_sys::hazard::JsCast for JsString<T> {}

			unsafe impl<T: Sized> ::js_sys::hazard::IntoJS for JsString<T> {
				type Abi = <::js_sys::JsValue as ::js_sys::hazard::IntoJS>::Abi;

				fn into_abi(self) -> Self::Abi {
					::js_sys::hazard::IntoJS::into_abi(::js_sys::JsValue::from(self))
				}
			}

			unsafe impl<T: Sized> ::js_sys::hazard::OptionIntoJS for JsString<T> {
				type OptionAbi = <::js_sys::JsValue as ::js_sys::hazard::OptionIntoJS>::OptionAbi;

				fn option_into_abi(value: ::core::option::Option<Self>) -> Self::OptionAbi {
					::js_sys::hazard::OptionIntoJS::option_into_abi(
						value.map(::js_sys::JsValue::from),
					)
				}
			}
		},
		None,
		None,
	);
}
