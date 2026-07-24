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
// JavaScript export adapters.
pub use crate::{
	js_export, js_export_arguments, js_export_input_arguments, js_export_output_expression,
	js_export_parameters,
};
// JavaScript import adapters.
pub use crate::{
	js_function, js_import, js_indirect_function, js_input_parameters, js_needs_adapter, js_output,
	js_parameter,
};
// WAT export adapters.
pub use crate::{
	wat_export, wat_export_direct, wat_export_imports, wat_export_indirect, wat_export_input_gets,
	wat_export_input_params, wat_export_input_raw_param, wat_export_input_raw_types,
	wat_export_result_loads, wat_export_result_types,
};
// WAT import adapters.
pub use crate::{
	wat_import, wat_import_input_types, wat_import_result, wat_imports, wat_input_gets,
	wat_input_import_types, wat_input_params, wat_output, wat_output_get, wat_output_import_param,
	wat_output_param, wat_output_result,
};
// Shared WAT and exception helpers.
pub use crate::{
	wat_import_list, wat_result_catch, wat_result_default, wat_result_try, wat_slot_params,
	wat_slot_types,
};
