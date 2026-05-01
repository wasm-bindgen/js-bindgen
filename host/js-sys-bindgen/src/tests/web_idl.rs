use syn::Visibility;

#[test]
fn basic() {
	let file = crate::web_idl("interface Test { };", None, &Visibility::Inherited).unwrap();

	test!(
		{ #file },
		{
			use js_sys::JsValue;
			use js_sys::hazard::{Input, InputJsConv, InputAsmConv, OutputAsmConv, Output, JsCast, OutputJsConv};

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

			unsafe impl JsCast for Test {}

			unsafe impl Output for Test {
				const ASM_TYPE: &str = <JsValue as Output>::ASM_TYPE;
				const ASM_CONV: Option<OutputAsmConv> = <JsValue as Output>::ASM_CONV;
				const JS_CONV: Option<OutputJsConv> = <JsValue as Output>::JS_CONV;

				type Type = <JsValue as Output>::Type;

				fn from_raw(raw: Self::Type) -> Self {
					Self(Output::from_raw(raw))
				}
			}
		},
	);
}
