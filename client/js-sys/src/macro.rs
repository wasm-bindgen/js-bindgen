mod abi;
mod closure;
mod export;
mod import;
mod result;
mod text;
mod wat;

pub use abi::*;
pub use import::*;
pub use result::*;
pub use text::*;
pub use wat::*;

// Text rendering.
pub use crate::{const_concat, const_concat_if, const_integer_str, js_template};
// JavaScript export shims.
pub use crate::{
	js_export, js_export_arguments, js_export_input_arguments, js_export_output_expression,
	js_export_parameters, js_export_promising, js_export_promising_then, js_export_result_throw,
};
// WAT closure shims.
pub use crate::{
	wat_closure, wat_closure_call, wat_closure_direct, wat_closure_indirect,
	wat_closure_table_import,
};
// WAT export shims.
pub use crate::{wat_export, wat_export_direct, wat_export_indirect};
// Shared WAT helpers.
pub use crate::{wat_imports, wat_input, wat_locals, wat_slots, wat_unique_list};
