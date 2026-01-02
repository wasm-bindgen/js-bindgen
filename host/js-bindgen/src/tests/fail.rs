use proc_macro::{Ident, Punct, Spacing, Span, TokenTree};
use quote::quote;

#[test]
fn empty() {
	super::error(
		crate::unsafe_embed_asm(quote! {}),
		"requires at least a string argument",
	);
}

#[test]
fn no_string() {
	super::error(
		crate::unsafe_embed_asm(quote! { 42 }),
		"requires at least a string argument",
	);
}

#[test]
fn escape() {
	super::error(
		crate::unsafe_embed_asm(quote! { "\r" }),
		"escaping `r` is not supported",
	);
}

#[test]
fn comma() {
	super::error(
		crate::unsafe_embed_asm(quote! { "" 42 }),
		"expected a `,` after string literal",
	);
}

#[test]
fn cfg_hash() {
	super::error(
		crate::unsafe_embed_asm(quote! { . }),
		"requires at least a string argument",
	);
}

#[test]
fn cfg_no_brackets() {
	super::error(crate::unsafe_embed_asm(quote! { # }), "expected `#[...]`");
}

#[test]
fn cfg_not_group() {
	super::error(
		crate::unsafe_embed_asm(
			[
				TokenTree::from(Punct::new('#', Spacing::Alone)),
				Ident::new("foo", Span::call_site()).into(),
			]
			.into_iter()
			.collect(),
		),
		"expected `#[...]`",
	);
}

#[test]
fn cfg_not_brackets() {
	super::error(crate::unsafe_embed_asm(quote! { #() }), "expected `#[...]`");
}

#[test]
fn cfg_not_cfg() {
	super::error(crate::unsafe_embed_asm(quote! { #[foo] }), "expected `cfg`");
}

#[test]
fn cfg_double() {
	super::error(
		crate::unsafe_embed_asm(quote! { #[cfg()] #[cfg()] }),
		"multiple `cfg`s in a row not supported",
	);
}

#[test]
fn cfg_empty() {
	super::error(
		crate::unsafe_embed_asm(quote! { #[cfg()] }),
		"requires at least a string argument",
	);
}

#[test]
fn bracers_escape_close() {
	super::error(
		crate::unsafe_embed_asm(quote! { "{}}" }),
		"no corresponding closing bracers found",
	);
}

#[test]
fn interpolate() {
	super::error(
		crate::unsafe_embed_asm(quote! { "{}", foo }),
		"expected `interpolate`",
	);
}

#[test]
fn interpolate_no_value() {
	super::error(
		crate::unsafe_embed_asm(quote! { "{}", interpolate }),
		"expected a value",
	);
}

#[test]
fn interpolate_not_value() {
	super::error(
		crate::unsafe_embed_asm(quote! { "{}", interpolate # }),
		"expected a value",
	);
}

#[test]
fn interpolate_ptr_const() {
	super::error(
		crate::unsafe_embed_asm(quote! { "{}", interpolate *mut }),
		"expected `*const`",
	);
}

#[test]
fn interpolate_angular() {
	super::error(
		crate::unsafe_embed_asm(quote! { "{}", interpolate < Foo }),
		"type not completed, missing `>`",
	);
}

#[test]
fn interpolate_empty() {
	super::error(
		crate::unsafe_embed_asm(quote! { "{}", interpolate, }),
		"expected a value",
	);
}

#[test]
fn interpolate_comma() {
	super::error(
		crate::unsafe_embed_asm(quote! { "{}", interpolate Foo & }),
		"expected a `,` between formatting parameters",
	);
}

#[test]
fn interpolate_missing_1() {
	super::error(
		crate::unsafe_embed_asm(quote! { "{}" }),
		"expected an argument for `{}`",
	);
}

#[test]
fn interpolate_missing_2() {
	super::error(
		crate::unsafe_embed_asm(quote! { "{}", "{}", interpolate "test", }),
		"expected an argument for `{}`",
	);
}

#[test]
fn bracers_no_close() {
	super::error(
		crate::unsafe_embed_asm(quote! { "{" }),
		"no corresponding closing bracers found",
	);
}

#[test]
fn bracers_no_open() {
	super::error(
		crate::unsafe_embed_asm(quote! { "}" }),
		"no corresponding opening bracers found",
	);
}

#[test]
fn after() {
	super::error(
		crate::unsafe_embed_asm(quote! { "", 42 }),
		"expected no tokens after string literals and formatting parameters",
	);
}

#[test]
fn import_empty() {
	super::error(crate::js_import(quote! {}), "expected `name = \"...\"`");
}

#[test]
fn import_not_attribute() {
	super::error(crate::js_import(quote! { 42 }), "expected `name = \"...\"`");
}

#[test]
fn import_wrong_attribute() {
	super::error(
		crate::js_import(quote! { foo }),
		"expected `name = \"...\"`",
	);
}

#[test]
fn import_no_equal() {
	super::error(
		crate::js_import(quote! { name }),
		"expected `name = \"...\"`",
	);
}

#[test]
fn import_not_equal() {
	super::error(
		crate::js_import(quote! { name + }),
		"expected `name = \"...\"`",
	);
}

#[test]
fn import_no_value() {
	super::error(
		crate::js_import(quote! { name = }),
		"expected `name = \"...\"`",
	);
}

#[test]
fn import_not_literal() {
	super::error(
		crate::js_import(quote! { name = 42 }),
		"expected `name = \"...\"`",
	);
}

#[test]
fn import_not_string() {
	super::error(
		crate::js_import(quote! { name = Foo }),
		"expected `name = \"...\"`",
	);
}

#[test]
fn import_no_strings() {
	super::error(
		crate::js_import(quote! { name = "foo" }),
		"expected `name = \"...\",` and a list of string literals",
	);
}

#[test]
fn import_no_strings_but_comma() {
	super::error(
		crate::js_import(quote! { name = "foo", }),
		"requires at least a string argument",
	);
}

#[test]
fn import_not_strings() {
	super::error(
		crate::js_import(quote! { name = "foo", 42}),
		"requires at least a string argument",
	);
}
