#![no_std]
#![cfg_attr(target_feature = "atomics", feature(thread_local))]
#![cfg_attr(target_arch = "wasm64", feature(simd_wasm64))]

extern crate alloc;

#[macro_use]
mod util;
mod array;
mod bigint;
mod externref;
pub mod hazard;
#[doc(hidden)]
pub mod r#macro;
mod number;
mod numeric;
mod panic;
mod string;
mod value;

pub use js_bindgen;
#[cfg(feature = "macro")]
pub use js_sys_macro::js_sys;

pub use crate::array::JsArray;
pub use crate::bigint::JsBigInt;
pub use crate::number::JsNumber;
pub use crate::panic::{UnwrapThrowExt, panic};
pub use crate::string::JsString;
pub use crate::value::JsValue;

#[cfg(not(target_feature = "reference-types"))]
compile_error!("`js-sys` requires the `reference-types` target feature");
