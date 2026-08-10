use proc_macro2::TokenStream;
use quote::quote;

fn expand(input: TokenStream) -> String {
	crate::closure::closure_with(input, "test-crate", "test-package", "1.2.3")
		.unwrap()
		.to_string()
}

#[test]
fn symbols_are_stable_and_disambiguated() {
	let input = quote!(dyn FnMut(i32) -> i32, move |value| value + 1);
	assert_eq!(expand(input.clone()), expand(input));

	let first = crate::closure::closure_with(
		quote!(dyn Fn(), || {}),
		"test-crate",
		"test-package",
		"1.2.3",
	)
	.unwrap()
	.to_string();
	let second = crate::closure::closure_with(
		quote!(dyn Fn(), || {}),
		"test-crate",
		"test-package",
		"2.0.0",
	)
	.unwrap()
	.to_string();

	assert_ne!(first, second);
}

#[test]
fn invalid_trait_object() {
	let error = crate::closure::closure_with(
		quote!(dyn Clone, || {}),
		"test-crate",
		"test-package",
		"1.2.3",
	)
	.unwrap_err();
	assert_eq!(error.to_string(), "expected `Fn`, `FnMut`, or `FnOnce`");

	let error = crate::closure::closure_with(
		quote!(dyn Fn() + Send, || {}),
		"test-crate",
		"test-package",
		"1.2.3",
	)
	.unwrap_err();
	assert_eq!(error.to_string(), "expected exactly one closure trait");
}
