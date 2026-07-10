#![no_std]

// Defer this nonexistent library to the final link, where `js-bindgen-ld`
// removes it.
#[link(
	name = "js-bindgen-needs-js-bindgen-ld",
	kind = "static",
    // See <https://doc.rust-lang.org/reference/items/external-blocks.html#r-items.extern.attributes.link.modifiers.bundle.behavior-negative>
	modifiers = "-bundle"
)]
unsafe extern "C" {}

#[doc(hidden)]
pub mod r#macro;

pub use js_bindgen_macro::{embed_js, import_js, unsafe_global_wat};
