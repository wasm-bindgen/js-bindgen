//! staticlib:
//! ```shell
//! cargo rustc -p wabii --target wasm32-unknown-unknown --crate-type staticlib
//! ```

#![cfg_attr(feature = "rustc-dep-of-std", no_std)]

pub mod random;
pub mod stdio;
pub mod time;
