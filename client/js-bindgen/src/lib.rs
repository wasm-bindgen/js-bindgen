#![no_std]

// Defer this non-existent library to the linker, where `js-bindgen-ld`
// removes it.
#[link(
	name = "js-bindgen-needs-js-bindgen-ld",
	kind = "static",
    // Ensures that its only searched for during linking.
    // See <https://doc.rust-lang.org/reference/items/external-blocks.html#r-items.extern.attributes.link.modifiers.bundle.behavior-negative>.
	modifiers = "-bundle"
)]
unsafe extern "C" {}

#[doc(hidden)]
pub mod r#macro;

pub use js_bindgen_macro::{embed_js, export_js, import_js, unsafe_global_wat};
