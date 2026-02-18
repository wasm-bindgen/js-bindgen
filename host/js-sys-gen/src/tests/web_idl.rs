use quote::{ToTokens, quote};
use syn::Visibility;

#[test]
fn basic() {
	let file = crate::from_web_idl("interface Test { };", None, &Visibility::Inherited).unwrap();

	super::test(
		file.into_token_stream(),
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
				const IMPORT_FUNC: &'static str = <JsValue as Input>::IMPORT_FUNC;
				const IMPORT_TYPE: &'static str = <JsValue as Input>::IMPORT_TYPE;
				const TYPE: &'static str = <JsValue as Input>::TYPE;
				const CONV: &'static str = <JsValue as Input>::CONV;

				type Type = <JsValue as Input>::Type;

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
