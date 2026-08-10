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
		},
		None,
		None,
	);
}

#[test]
fn multiple_generic_kinds() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			pub type Generic<'a, T, U, const N: usize>;
		}
	};
	let output =
		crate::r#macro::expand_for_test(proc_macro2::TokenStream::new(), input, "test_crate")
			.unwrap()
			.into_items()
			.unwrap();
	let output = prettyplease::unparse(&syn::File {
		shebang: None,
		attrs: Vec::new(),
		items: output,
	});
	let dir = tempfile::tempdir().unwrap();

	super::inner(dir.path(), &output).unwrap();
}

#[test]
fn cfg_attr_only_applies_to_the_declared_type() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[cfg_attr(all(), derive(Clone))]
			pub type JsString;
		}
	};
	let output =
		crate::r#macro::expand_for_test(proc_macro2::TokenStream::new(), input, "test_crate")
			.unwrap()
			.into_items()
			.unwrap();

	assert_eq!(output.len(), 5);
	for (index, item) in output.into_iter().enumerate() {
		let attrs = match item {
			syn::Item::Struct(item) => item.attrs,
			syn::Item::Impl(item) => item.attrs,
			item => panic!("unexpected generated item: {item:?}"),
		};
		let has_cfg_attr = attrs.iter().any(|attr| attr.path().is_ident("cfg_attr"));

		assert_eq!(has_cfg_attr, index == 0);
	}
}

#[test]
fn duplicate_type_names_do_not_panic_in_the_macro() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			pub type Duplicate;
			pub type Duplicate;
		}
	};
	let output =
		crate::r#macro::expand_for_test(proc_macro2::TokenStream::new(), input, "test_crate")
			.unwrap()
			.into_items()
			.unwrap();

	assert_eq!(output.len(), 10);
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
		},
		None,
		None,
	);
}

#[test]
fn extends() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(extends = JsTest)]
			#[js_sys(extends = JsArray)]
			pub type Child;
		}
	};
	let output =
		crate::r#macro::expand_for_test(proc_macro2::TokenStream::new(), input, "test_crate")
			.unwrap()
			.into_items()
			.unwrap();
	let mut output = prettyplease::unparse(&syn::File {
		shebang: None,
		attrs: Vec::new(),
		items: output,
	});
	output.push_str(
		r"
fn assert_extends(value: &Child) {
	let _: &JsTest = value;
	let _: &JsArray = ::core::convert::AsRef::<JsArray>::as_ref(value);
}

fn into_parent(value: Child) -> JsArray {
	value.into()
}
",
	);

	let dir = tempfile::tempdir().unwrap();
	super::inner(dir.path(), &output).unwrap();
}
