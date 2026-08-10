//! The wire protocol shared by `js-sys` and `js-bindgen-ld`.
//!
//! Data passes through the following stages:
//!
//! ```text
//! ABI -> schema -> const encode -> decode -> model -> ld render
//! ```
//!
//! - `abi` defines the common vocabulary, such as Wasm slots, conversions, and
//!   return modes.
//! - `schema` combines those values into static descriptions that `js-sys` can
//!   construct during constant evaluation.
//! - `encode` writes each description into a compact byte record without
//!   allocation.
//! - `decode` validates records read from object files.
//! - `model` owns the decoded form used by host tools.
//! - `js-bindgen-ld` renders that model into JavaScript and WAT shims.

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

#[cfg(feature = "alloc")]
pub use decode::{Error, ErrorKind, WireRecords, decode};
pub use encode::{WireBlob, WireRecord, wire_blob_len};
pub use schema::*;

/// Identifies a wire record independently of its payload kind.
pub const MAGIC: [u8; 8] = *b"JBGWIRE\0";

/// The single protocol version used by imports, exports, and closures.
pub const VERSION: u16 = 1;

/// Custom section containing length-prefixed wire records.
pub const WIRE_SECTION: &str = "js_bindgen.wire";

#[cfg(feature = "alloc")]
pub(crate) const SLOT_COUNT: usize = 4;
pub(crate) const KIND_IMPORT: u8 = 0;
pub(crate) const KIND_EXPORT: u8 = 1;
pub(crate) const KIND_CLOSURE: u8 = 2;

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

pub(crate) const IMPORT_BINDING_CALL_GLOBAL: u8 = 0;
pub(crate) const IMPORT_BINDING_CALL_METHOD: u8 = 1;
pub(crate) const IMPORT_BINDING_CONSTRUCT: u8 = 2;
pub(crate) const IMPORT_BINDING_GET_GLOBAL: u8 = 3;
pub(crate) const IMPORT_BINDING_GET_MEMBER: u8 = 4;
pub(crate) const IMPORT_BINDING_SET_GLOBAL: u8 = 5;
pub(crate) const IMPORT_BINDING_SET_MEMBER: u8 = 6;
pub(crate) const IMPORT_BINDING_INDEX_GET: u8 = 7;
pub(crate) const IMPORT_BINDING_INDEX_SET: u8 = 8;
pub(crate) const IMPORT_BINDING_INDEX_DELETE: u8 = 9;
pub(crate) const IMPORT_BINDING_CALL_EMBED: u8 = 10;
#[cfg(feature = "alloc")]
pub(crate) const IMPORT_BINDING_MAX: u8 = IMPORT_BINDING_CALL_EMBED;

pub(crate) const EXPORT_PROMISING: u8 = 1 << 0;
pub(crate) const EXPORT_HAS_OUTPUT: u8 = 1 << 1;
#[cfg(feature = "alloc")]
pub(crate) const EXPORT_FLAGS: u8 = EXPORT_PROMISING | EXPORT_HAS_OUTPUT;

pub(crate) const EXPORT_OUTPUT_DIRECT: u8 = 1 << 0;
pub(crate) const EXPORT_OUTPUT_RESULT: u8 = 1 << 1;
#[cfg(feature = "alloc")]
pub(crate) const EXPORT_OUTPUT_FLAGS: u8 = EXPORT_OUTPUT_DIRECT | EXPORT_OUTPUT_RESULT;

pub(crate) const CLOSURE_HAS_OUTPUT: u8 = 1 << 0;
#[cfg(feature = "alloc")]
pub(crate) const CLOSURE_FLAGS: u8 = CLOSURE_HAS_OUTPUT;

#[cfg(all(test, feature = "alloc"))]
mod tests;
