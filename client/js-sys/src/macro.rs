mod abi;
mod export;
mod js_import;
mod result;
mod text;
mod wat;
mod wat_import;

pub use abi::*;
pub use result::*;
pub use text::*;
pub use wat::*;

// Text rendering.
pub use crate::{const_concat, const_concat_if, const_integer_str, js_template};
// JavaScript export shims.
pub use crate::{
	js_export, js_export_arguments, js_export_input_arguments, js_export_output_expression,
	js_export_parameters,
};
// JavaScript import shims.
pub use crate::{
	js_function, js_import, js_indirect_function, js_input_parameters, js_needs_shim, js_output,
	js_parameter,
};
// WAT export shims.
pub use crate::{wat_export, wat_export_direct, wat_export_indirect, wat_export_needs_shim};
// WAT import shims.
pub use crate::{wat_import, wat_import_output};
// Shared WAT helpers.
pub use crate::{wat_import_list, wat_imports, wat_input, wat_slots};
