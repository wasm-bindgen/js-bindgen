//! The wire protocol shared by `js-sys` and `js-bindgen-ld`.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod encode;
mod schema;

#[doc(hidden)]
pub mod abi;

#[cfg(feature = "alloc")]
mod decode;
#[cfg(feature = "alloc")]
pub mod model;

pub use encode::{WireBlob, WireRecord, wire_blob_len};
pub use schema::*;

#[cfg(feature = "alloc")]
pub use decode::{Error, ErrorKind, decode};

/// Identifies a wire record independently of its payload kind.
pub const MAGIC: [u8; 8] = *b"JBGWIRE\0";

/// The single protocol version used by imports and exports.
pub const VERSION: u16 = 1;

#[cfg(feature = "alloc")]
pub(crate) const SLOT_COUNT: usize = 4;
pub(crate) const KIND_IMPORT: u8 = 0;
pub(crate) const KIND_EXPORT: u8 = 1;

pub(crate) const WAT_IMPORT_FUNCTION: u8 = 0;
pub(crate) const WAT_IMPORT_TABLE: u8 = 1;
pub(crate) const WAT_IMPORT_TAG: u8 = 2;

pub(crate) const IMPORT_SUSPENDING: u8 = 1 << 0;
pub(crate) const IMPORT_HAS_OUTPUT: u8 = 1 << 1;
pub(crate) const IMPORT_HAS_BINDING: u8 = 1 << 2;
#[cfg(feature = "alloc")]
pub(crate) const IMPORT_FLAGS: u8 = IMPORT_SUSPENDING | IMPORT_HAS_OUTPUT | IMPORT_HAS_BINDING;

pub(crate) const IMPORT_OUTPUT_DIRECT: u8 = 1 << 0;
pub(crate) const IMPORT_OUTPUT_RESULT: u8 = 1 << 1;
#[cfg(feature = "alloc")]
pub(crate) const IMPORT_OUTPUT_FLAGS: u8 = IMPORT_OUTPUT_DIRECT | IMPORT_OUTPUT_RESULT;

pub(crate) const IMPORT_CATCH_JAVASCRIPT: u8 = 0;
pub(crate) const IMPORT_CATCH_WASM: u8 = 1;

pub(crate) const EXPORT_PROMISING: u8 = 1 << 0;
pub(crate) const EXPORT_HAS_OUTPUT: u8 = 1 << 1;
#[cfg(feature = "alloc")]
pub(crate) const EXPORT_FLAGS: u8 = EXPORT_PROMISING | EXPORT_HAS_OUTPUT;

pub(crate) const EXPORT_OUTPUT_DIRECT: u8 = 1 << 0;
pub(crate) const EXPORT_OUTPUT_RESULT: u8 = 1 << 1;
#[cfg(feature = "alloc")]
pub(crate) const EXPORT_OUTPUT_FLAGS: u8 = EXPORT_OUTPUT_DIRECT | EXPORT_OUTPUT_RESULT;

#[cfg(all(test, feature = "alloc"))]
mod tests;
