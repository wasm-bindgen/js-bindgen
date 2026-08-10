use super::{
	JsEmbed, Wire, WireClosure, WireClosureFactory, WireExportInput, WireExportOutput,
	wire_import_input_type, wire_import_output_type,
};
use crate::runtime::JsValue;

/// The JavaScript lifetime and invocation semantics of a Rust closure.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum ClosureKind {
	Shared,
	Mutable,
	Once,
}

/// Builds the wire description for a generated closure.
#[doc(hidden)]
#[must_use]
pub const fn wire_closure(
	raw_symbol: &'static str,
	kind: ClosureKind,
	call_shim_offset: usize,
	inputs: &'static [WireExportInput],
	output: Option<WireExportOutput>,
) -> Wire {
	let helper = match kind {
		ClosureKind::Shared => JsEmbed::new("js_sys", "closure.make"),
		ClosureKind::Mutable => JsEmbed::new("js_sys", "closure.make_mut"),
		ClosureKind::Once => JsEmbed::new("js_sys", "closure.make_once"),
	};
	Wire::closure(WireClosure::new(
		WireClosureFactory::new(
			raw_symbol,
			helper,
			wire_import_input_type::<usize>(),
			wire_import_output_type::<JsValue>(),
		),
		call_shim_offset,
		inputs,
		output,
	))
}
