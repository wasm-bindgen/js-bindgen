use quote::quote;
use syn::parse_quote;

use crate::{Hygiene, ImportManager, Type};

#[test]
fn basic() {
	let mut imports = ImportManager::new(None);
	let items = Type::new(
		Hygiene::Imports(&mut imports),
		parse_quote!(
			type Test;
		),
	);

	super::test(
		quote! {
			#imports

			#items
		},
		quote! {
			use core::ops::Deref;

			use js_sys::JsValue;
			use js_sys::hazard::{Input, Output};

			#[repr(transparent)]
			struct Test(JsValue);

			impl Deref for Test {
				type Target = JsValue;

				fn deref(&self) -> &Self::Target {
					&self.0
				}
			}

			unsafe impl Input for &Test {
				const IMPORT_FUNC: &'static str = <&JsValue as Input>::IMPORT_FUNC;
				const IMPORT_TYPE: &'static str = <&JsValue as Input>::IMPORT_TYPE;
				const TYPE: &'static str = <&JsValue as Input>::TYPE;
				const CONV: &'static str = <&JsValue as Input>::CONV;

				type Type = <&'static JsValue as Input>::Type;

				fn into_raw(self) -> Self::Type {
					Input::into_raw(&self.0)
				}
			}

			unsafe impl Output for Test {
				const IMPORT_FUNC: &str = <JsValue as Output>::IMPORT_FUNC;
				const IMPORT_TYPE: &str = <JsValue as Output>::IMPORT_TYPE;
				const TYPE: &str = <JsValue as Output>::TYPE;
				const CONV: &str = <JsValue as Output>::CONV;

				type Type = <JsValue as Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self(Output::from_raw(raw))
				}
			}
		},
	);
}

#[test]
fn generic() {
	let mut imports = ImportManager::new(None);
	let items = Type::new(
		Hygiene::Imports(&mut imports),
		parse_quote!(
			type Test<T = JsValue>;
		),
	);

	super::test(
		quote! {
			#imports

			#items
		},
		quote! {
			use core::marker::PhantomData;
			use core::ops::Deref;

			use js_sys::JsValue;
			use js_sys::hazard::{Input, Output};

			#[repr(transparent)]
			struct Test<T = JsValue> {
				value: JsValue,
				_type: PhantomData<T>,
			}

			impl<T> Deref for Test<T> {
				type Target = JsValue;

				fn deref(&self) -> &Self::Target {
					&self.value
				}
			}

			unsafe impl<T> Input for &Test<T> {
				const IMPORT_FUNC: &'static str = <&JsValue as Input>::IMPORT_FUNC;
				const IMPORT_TYPE: &'static str = <&JsValue as Input>::IMPORT_TYPE;
				const TYPE: &'static str = <&JsValue as Input>::TYPE;
				const CONV: &'static str = <&JsValue as Input>::CONV;

				type Type = <&'static JsValue as Input>::Type;

				fn into_raw(self) -> Self::Type {
					Input::into_raw(&self.value)
				}
			}

			unsafe impl<T> Output for Test<T> {
				const IMPORT_FUNC: &str = <JsValue as Output>::IMPORT_FUNC;
				const IMPORT_TYPE: &str = <JsValue as Output>::IMPORT_TYPE;
				const TYPE: &str = <JsValue as Output>::TYPE;
				const CONV: &str = <JsValue as Output>::CONV;

				type Type = <JsValue as Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self {
						value: Output::from_raw(raw),
						_type: PhantomData,
					}
				}
			}
		},
	);
}
