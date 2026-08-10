mod abi;
mod closure;
mod export;
mod import;
mod result;
mod text;
mod wat;
mod writer;

pub use abi::*;
pub use export::*;
pub use import::*;
pub use result::*;
pub use text::*;
pub use wat::*;

// Text rendering.
pub use crate::{const_concat, const_concat_if, const_integer_str, js_template};
// JavaScript export shims.
pub use crate::{js_export, js_export_promising};
// WAT closure shims.
pub use crate::{
	wat_closure, wat_closure_call, wat_closure_direct, wat_closure_indirect,
	wat_closure_table_import,
};
// WAT export shims.
pub use crate::{wat_export, wat_export_direct, wat_export_indirect};
// Shared WAT helpers.
pub use crate::{wat_imports, wat_input, wat_locals, wat_slots, wat_unique_list};
