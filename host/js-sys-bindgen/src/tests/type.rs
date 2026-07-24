use syn::parse_quote;

use crate::{Hygiene, ImportManager, Type};

#[test]
fn basic() {
	let mut imports = ImportManager::new(None);
	let items = Type::new(
		&mut Hygiene::Imports(&mut imports),
		parse_quote!(
			type Test;
		),
	);

	test!(
		{
			#imports

			#items
		},
		{
			use js_sys::JsValue;
			use js_sys::hazard::{IntoJS, JsCast, OptionIntoJS};

			#[repr(transparent)]
			struct Test(JsValue);

			impl AsRef<JsValue> for Test {
				fn as_ref(&self) -> &JsValue {
					&self.0
				}
			}

			impl From<Test> for JsValue {
				fn from(value: Test) -> Self {
					value.0
				}
			}

			unsafe impl JsCast for Test {}

			unsafe impl IntoJS for Test {
				type Abi = <JsValue as IntoJS>::Abi;

				fn into_abi(self) -> Self::Abi {
					IntoJS::into_abi(JsValue::from(self))
				}
			}

			unsafe impl OptionIntoJS for Test {
				type OptionAbi = <JsValue as OptionIntoJS>::OptionAbi;

				fn option_into_abi(value: ::core::option::Option<Self>) -> Self::OptionAbi {
					OptionIntoJS::option_into_abi(value.map(JsValue::from))
				}
			}

		},
	);
}

#[test]
fn generic() {
	let mut imports = ImportManager::new(None);
	let items = Type::new(
		&mut Hygiene::Imports(&mut imports),
		parse_quote!(
			type Test<T = JsValue>;
		),
	);

	test!(
		{
			#imports

			#items
		},
		{
			use core::marker::PhantomData;
			use js_sys::JsValue;
			use js_sys::hazard::{IntoJS, JsCast, OptionIntoJS};

			#[repr(transparent)]
			struct Test<T = JsValue> {
				value: JsValue,
				_type: PhantomData<T>,
			}

			impl<T> AsRef<JsValue> for Test<T> {
				fn as_ref(&self) -> &JsValue {
					&self.value
				}
			}

			impl<T> From<Test<T>> for JsValue {
				fn from(value: Test<T>) -> Self {
					value.value
				}
			}

			unsafe impl<T> JsCast for Test<T> {}

			unsafe impl<T> IntoJS for Test<T> {
				type Abi = <JsValue as IntoJS>::Abi;

				fn into_abi(self) -> Self::Abi {
					IntoJS::into_abi(JsValue::from(self))
				}
			}

			unsafe impl<T> OptionIntoJS for Test<T> {
				type OptionAbi = <JsValue as OptionIntoJS>::OptionAbi;

				fn option_into_abi(value: ::core::option::Option<Self>) -> Self::OptionAbi {
					OptionIntoJS::option_into_abi(value.map(JsValue::from))
				}
			}

		},
	);
}
