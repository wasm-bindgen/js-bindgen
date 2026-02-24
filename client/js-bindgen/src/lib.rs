#![no_std]

#[doc(hidden)]
pub mod r#macro;

pub use js_bindgen_macro::{embed_js, import_js, unsafe_embed_asm};
