#![no_std]
#![cfg_attr(target_feature = "atomics", feature(thread_local))]
#![cfg_attr(
	all(not(debug_assertions), target_arch = "wasm64"),
	feature(simd_wasm64)
)]

extern crate alloc;

#[macro_use]
mod util;

// JavaScript standard built-in objects.
mod builtins;
// `Runtime` support for Rust and JavaScript `interop`.
mod runtime;

pub mod hazard;
// Implementations for passing Rust standard types across the JavaScript
// boundary.
mod interop;
#[doc(hidden)]
pub mod r#macro;

pub use builtins::{
	Error, ErrorOptions, JsArray, JsBigInt, JsNumber, JsString, Object, TryFromJsArrayError,
};
pub use js_bindgen;
pub use js_sys_macro::{closure, js_sys};
pub use runtime::{Closure, ClosureAllocation, ClosureHeader, JsValue, UnwrapThrowExt, panic};

#[cfg(not(target_feature = "reference-types"))]
compile_error!("`js-sys` requires the `reference-types` target feature");

#[cfg(not(target_feature = "multivalue"))]
compile_error!("`js-sys` requires the `multivalue` target feature");
