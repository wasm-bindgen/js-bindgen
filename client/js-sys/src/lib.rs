#![no_std]
#![cfg_attr(target_feature = "atomics", feature(thread_local))]
#![cfg_attr(target_arch = "wasm64", feature(simd_wasm64))]

extern crate alloc;

mod array;
mod externref;
pub mod hazard;
#[doc(hidden)]
pub mod r#macro;
mod numeric;
mod panic;
mod string;
mod util;
mod value;

pub use js_bindgen;
pub use js_sys_macro::js_sys;

pub use crate::array::JsArray;
pub use crate::panic::{UnwrapThrowExt, panic};
pub use crate::string::JsString;
pub use crate::value::JsValue;

#[cfg(not(target_feature = "reference-types"))]
compile_error!("`js-sys` requires the `reference-types` target feature");
