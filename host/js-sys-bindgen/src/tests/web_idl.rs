use syn::Visibility;

#[test]
fn basic() {
	let file = crate::web_idl("interface Test { };", None, &Visibility::Inherited).unwrap();

	test!(
		{ #file },
		{
			use js_sys::JsValue;
			use js_sys::hazard::{IntoJS, JsCast};

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
		},
	);
}
