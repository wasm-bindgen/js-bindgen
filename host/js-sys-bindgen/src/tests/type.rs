use quote::quote;
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

			impl From<Test> for JsValue {
				fn from(value: Test) -> Self {
					value.0
				}
			}

			unsafe impl Input for &Test {
				const ASM_IMPORT_FUNC: Option<&'static str> = <&JsValue as Input>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &'static str = <&JsValue as Input>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &'static str = <&JsValue as Input>::ASM_TYPE;
				const ASM_CONV: Option<&'static str> = <&JsValue as Input>::ASM_CONV;

				type Type = <&'static JsValue as Input>::Type;

				fn into_raw(self) -> Self::Type {
					Input::into_raw(&self.0)
				}
			}

			unsafe impl Output for Test {
				const ASM_IMPORT_FUNC: Option<&str> = <JsValue as Output>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &str = <JsValue as Output>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &str = <JsValue as Output>::ASM_TYPE;
				const ASM_CONV: Option<&str> = <JsValue as Output>::ASM_CONV;

				type Type = <JsValue as Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self(Output::from_raw(raw))
				}
			}

			impl Test {
				#[must_use]
				fn unchecked_from(value: JsValue) -> Self {
					Self(value)
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

			impl<T> From<Test<T>> for JsValue {
				fn from(value: Test<T>) -> Self {
					value.value
				}
			}

			unsafe impl<T> Input for &Test<T> {
				const ASM_IMPORT_FUNC: Option<&'static str> = <&JsValue as Input>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &'static str = <&JsValue as Input>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &'static str = <&JsValue as Input>::ASM_TYPE;
				const ASM_CONV: Option<&'static str> = <&JsValue as Input>::ASM_CONV;

				type Type = <&'static JsValue as Input>::Type;

				fn into_raw(self) -> Self::Type {
					Input::into_raw(&self.value)
				}
			}

			unsafe impl<T> Output for Test<T> {
				const ASM_IMPORT_FUNC: Option<&str> = <JsValue as Output>::ASM_IMPORT_FUNC;
				const ASM_IMPORT_TYPE: &str = <JsValue as Output>::ASM_IMPORT_TYPE;
				const ASM_TYPE: &str = <JsValue as Output>::ASM_TYPE;
				const ASM_CONV: Option<&str> = <JsValue as Output>::ASM_CONV;

				type Type = <JsValue as Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self {
						value: Output::from_raw(raw),
						_type: PhantomData,
					}
				}
			}

			impl<T> Test<T> {
				#[must_use]
				fn unchecked_from(value: JsValue) -> Self {
					Self {
						value,
						_type: PhantomData,
					}
				}
			}
		},
	);
}
