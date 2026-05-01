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
			use js_sys::hazard::{Input, InputJsConv, InputAsmConv, OutputAsmConv, Output, OutputJsConv};

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

			unsafe impl Input for &Test {
				const ASM_TYPE: &'static str = <&JsValue as Input>::ASM_TYPE;
				const ASM_CONV: Option<InputAsmConv> = <&JsValue as Input>::ASM_CONV;
				const JS_CONV: Option<InputJsConv> = <&JsValue as Input>::JS_CONV;

				type Type = <&'static JsValue as Input>::Type;

				fn into_raw(self) -> Self::Type {
					Input::into_raw(&self.0)
				}
			}

			unsafe impl Output for Test {
				const ASM_TYPE: &str = <JsValue as Output>::ASM_TYPE;
				const ASM_CONV: Option<OutputAsmConv> = <JsValue as Output>::ASM_CONV;
				const JS_CONV: Option<OutputJsConv> = <JsValue as Output>::JS_CONV;

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

	test!(
		{
			#imports

			#items
		},
		{
			use core::marker::PhantomData;
			use js_sys::JsValue;
			use js_sys::hazard::{Input, InputJsConv, InputAsmConv, OutputAsmConv, Output, OutputJsConv};

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

			unsafe impl<T> Input for &Test<T> {
				const ASM_TYPE: &'static str = <&JsValue as Input>::ASM_TYPE;
				const ASM_CONV: Option<InputAsmConv> = <&JsValue as Input>::ASM_CONV;
				const JS_CONV: Option<InputJsConv> = <&JsValue as Input>::JS_CONV;

				type Type = <&'static JsValue as Input>::Type;

				fn into_raw(self) -> Self::Type {
					Input::into_raw(&self.value)
				}
			}

			unsafe impl<T> Output for Test<T> {
				const ASM_TYPE: &str = <JsValue as Output>::ASM_TYPE;
				const ASM_CONV: Option<OutputAsmConv> = <JsValue as Output>::ASM_CONV;
				const JS_CONV: Option<OutputJsConv> = <JsValue as Output>::JS_CONV;

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
					Self { value, _type: PhantomData }
				}
			}
		},
	);
}
