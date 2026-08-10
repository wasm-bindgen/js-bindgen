//! Static, const-constructible wire descriptions.

use core::mem::size_of;

use crate::{
	EXPORT_HAS_OUTPUT, EXPORT_PROMISING,
	abi::{
		FromJsConv, IntoJsConv, JsCatch, JsEmbed, ResultLayout, ReturnConv, ReturnMode, Sret,
		WatCatch, WatSlot, WatType,
	},
};

/// The target's native pointer width.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PointerWidth {
	Wasm32,
	Wasm64,
}

impl PointerWidth {
	/// Returns the width of a native pointer in bytes.
	#[must_use]
	pub const fn bytes(self) -> u8 {
		match self {
			Self::Wasm32 => 4,
			Self::Wasm64 => 8,
		}
	}

	/// Returns the `WAT` type of a native pointer.
	#[must_use]
	pub const fn wat_type(self) -> WatType {
		match self {
			Self::Wasm32 => WatType::I32,
			Self::Wasm64 => WatType::I64,
		}
	}

	pub(crate) const fn native() -> Self {
		if size_of::<usize>() == 4 {
			Self::Wasm32
		} else {
			assert!(size_of::<usize>() == 8);
			Self::Wasm64
		}
	}
}

/// Type-level data shared by imported arguments of the same Rust type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireImportInputType {
	pub(crate) slots: [Option<WatSlot>; 4],
	pub(crate) conversion: Option<IntoJsConv>,
}

impl WireImportInputType {
	#[must_use]
	pub const fn new(slots: [Option<WatSlot>; 4], conversion: Option<IntoJsConv>) -> Self {
		Self { slots, conversion }
	}
}

/// Type-level data shared by imported results of the same Rust type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireImportOutputType {
	pub(crate) mode: ReturnMode,
	pub(crate) conversion: ReturnConv<FromJsConv>,
	pub(crate) sret: Option<Sret>,
	pub(crate) slots: [Option<WatSlot>; 4],
}

impl WireImportOutputType {
	#[must_use]
	pub const fn new(
		mode: ReturnMode,
		conversion: ReturnConv<FromJsConv>,
		sret: Option<Sret>,
		slots: [Option<WatSlot>; 4],
	) -> Self {
		assert!(mode.is_direct() == sret.is_none());
		assert!(conversion.conversion().is_some() || sret.is_none());
		Self {
			mode,
			conversion,
			sret,
			slots,
		}
	}
}

/// Exception lowering shared by all `Result` entries in an import type table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WireImportCatch {
	/// JavaScript wraps the call in `try`/`catch` and records the exception.
	JavaScript(JsCatch),
	/// Wasm exception handling catches and records the exception in the `ABI` shim.
	Wasm(WatCatch),
}

/// Type definitions shared by one group of JavaScript imports.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireImportTypeTable {
	pub(crate) retptr_type: &'static WireImportInputType,
	pub(crate) input_types: &'static [&'static WireImportInputType],
	pub(crate) output_types: &'static [&'static WireImportOutputType],
	pub(crate) catch: WireImportCatch,
}

impl WireImportTypeTable {
	#[must_use]
	pub const fn new(
		retptr_type: &'static WireImportInputType,
		input_types: &'static [&'static WireImportInputType],
		output_types: &'static [&'static WireImportOutputType],
		catch: WireImportCatch,
	) -> Self {
		Self {
			retptr_type,
			input_types,
			output_types,
			catch,
		}
	}

	#[must_use]
	pub(crate) const fn has_result(&self) -> bool {
		let mut index = 0;
		while index < self.output_types.len() {
			if self.output_types[index].conversion.is_result() {
				return true;
			}
			index += 1;
		}
		false
	}
}

/// One named argument accepted by a JavaScript import.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireImportInput {
	pub(crate) name: &'static str,
	pub(crate) type_index: usize,
}

impl WireImportInput {
	#[must_use]
	pub const fn new(name: &'static str, type_index: usize) -> Self {
		Self { name, type_index }
	}
}

/// JavaScript binding data for one import.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireImportBinding {
	pub(crate) direct: Option<&'static str>,
	pub(crate) call: &'static str,
	pub(crate) required_embeds: &'static [JsEmbed],
}

impl WireImportBinding {
	#[must_use]
	pub const fn new(
		direct: Option<&'static str>,
		call: &'static str,
		required_embeds: &'static [JsEmbed],
	) -> Self {
		Self {
			direct,
			call,
			required_embeds,
		}
	}
}

/// The result type referenced by one JavaScript import.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireImportOutput {
	pub(crate) type_index: usize,
}

impl WireImportOutput {
	#[must_use]
	pub const fn new(type_index: usize) -> Self {
		Self { type_index }
	}
}

/// One semantic JavaScript import.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireImport {
	pub(crate) module: &'static str,
	pub(crate) name: &'static str,
	pub(crate) inputs: &'static [WireImportInput],
	pub(crate) output: Option<WireImportOutput>,
	pub(crate) binding: Option<WireImportBinding>,
	pub(crate) suspending: bool,
}

impl WireImport {
	#[must_use]
	pub const fn new(
		module: &'static str,
		name: &'static str,
		inputs: &'static [WireImportInput],
		output: Option<WireImportOutput>,
		binding: Option<WireImportBinding>,
		suspending: bool,
	) -> Self {
		assert!(!suspending || binding.is_some());
		Self {
			module,
			name,
			inputs,
			output,
			binding,
			suspending,
		}
	}
}

/// Type-level data shared by exported arguments with the same Rust type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireExportInputType {
	pub(crate) slots: [Option<WatSlot>; 4],
	pub(crate) conversion: Option<FromJsConv>,
}

impl WireExportInputType {
	#[must_use]
	pub const fn new(slots: [Option<WatSlot>; 4], conversion: Option<FromJsConv>) -> Self {
		Self { slots, conversion }
	}
}

/// One named argument accepted by a JavaScript-facing `Wasm` export.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireExportInput {
	pub(crate) name: &'static str,
	pub(crate) ty: &'static WireExportInputType,
}

impl WireExportInput {
	#[must_use]
	pub const fn new(name: &'static str, ty: &'static WireExportInputType) -> Self {
		Self { name, ty }
	}
}

/// Type-level data shared by exported results with the same Rust type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireExportOutputType {
	pub(crate) mode: ReturnMode,
	pub(crate) conversion: ReturnConv<IntoJsConv>,
	pub(crate) slots: [Option<WatSlot>; 4],
	pub(crate) frame_size: usize,
	pub(crate) slot_offsets: [usize; 4],
	pub(crate) result: Option<ResultLayout>,
}

impl WireExportOutputType {
	#[must_use]
	pub const fn new(
		mode: ReturnMode,
		conversion: ReturnConv<IntoJsConv>,
		slots: [Option<WatSlot>; 4],
		frame_size: usize,
		slot_offsets: [usize; 4],
		result: Option<ResultLayout>,
	) -> Self {
		assert!(conversion.is_result() == result.is_some());
		Self {
			mode,
			conversion,
			slots,
			frame_size,
			slot_offsets,
			result,
		}
	}
}

/// The result type referenced by one JavaScript-facing `Wasm` export.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireExportOutput {
	pub(crate) ty: &'static WireExportOutputType,
}

impl WireExportOutput {
	#[must_use]
	pub const fn new(ty: &'static WireExportOutputType) -> Self {
		Self { ty }
	}
}

/// Describes how an exported `Wasm` shim reaches Rust code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WireExportCallee {
	Symbol(&'static str),
	Closure { call_shim_offset: usize },
}

impl WireExportCallee {
	#[must_use]
	const fn symbol(name: &'static str) -> Self {
		Self::Symbol(name)
	}

	#[must_use]
	const fn closure(call_shim_offset: usize) -> Self {
		Self::Closure { call_shim_offset }
	}
}

/// One semantic JavaScript-facing export.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireExport {
	pub(crate) module: &'static str,
	pub(crate) name: &'static str,
	pub(crate) inputs: &'static [WireExportInput],
	pub(crate) output: Option<WireExportOutput>,
	pub(crate) callee: WireExportCallee,
	pub(crate) flags: u8,
}

impl WireExport {
	#[must_use]
	const fn new(
		module: &'static str,
		name: &'static str,
		inputs: &'static [WireExportInput],
		output: Option<WireExportOutput>,
		callee: WireExportCallee,
		promising: bool,
	) -> Self {
		Self {
			module,
			name,
			inputs,
			output,
			callee,
			flags: flag(promising, EXPORT_PROMISING) | flag(output.is_some(), EXPORT_HAS_OUTPUT),
		}
	}

	#[must_use]
	pub const fn new_symbol(
		module: &'static str,
		name: &'static str,
		symbol: &'static str,
		inputs: &'static [WireExportInput],
		output: Option<WireExportOutput>,
	) -> Self {
		Self::new(
			module,
			name,
			inputs,
			output,
			WireExportCallee::symbol(symbol),
			false,
		)
	}

	#[must_use]
	pub const fn new_symbol_promising(
		module: &'static str,
		name: &'static str,
		symbol: &'static str,
		inputs: &'static [WireExportInput],
		output: Option<WireExportOutput>,
	) -> Self {
		Self::new(
			module,
			name,
			inputs,
			output,
			WireExportCallee::symbol(symbol),
			true,
		)
	}

	#[must_use]
	pub const fn new_closure(
		module: &'static str,
		name: &'static str,
		call_shim_offset: usize,
		inputs: &'static [WireExportInput],
		output: Option<WireExportOutput>,
	) -> Self {
		assert!(!inputs.is_empty(), "closure exports require a data input");
		Self::new(
			module,
			name,
			inputs,
			output,
			WireExportCallee::closure(call_shim_offset),
			false,
		)
	}
}

#[derive(Clone, Copy)]
pub(crate) enum WireKind {
	Imports {
		table: &'static WireImportTypeTable,
		imports: &'static [WireImport],
	},
	Exports(&'static [WireExport]),
}

/// One import or export group encoded into a wire record.
#[derive(Clone, Copy)]
pub struct Wire {
	pub(crate) pointer_width: PointerWidth,
	pub(crate) kind: WireKind,
}

impl Wire {
	#[must_use]
	pub const fn imports(
		table: &'static WireImportTypeTable,
		imports: &'static [WireImport],
	) -> Self {
		Self::imports_with(PointerWidth::native(), table, imports)
	}

	#[must_use]
	pub(crate) const fn imports_with(
		pointer_width: PointerWidth,
		table: &'static WireImportTypeTable,
		imports: &'static [WireImport],
	) -> Self {
		validate_imports(table, imports);
		Self {
			pointer_width,
			kind: WireKind::Imports { table, imports },
		}
	}

	#[must_use]
	pub const fn exports(exports: &'static [WireExport]) -> Self {
		Self::exports_with(PointerWidth::native(), exports)
	}

	#[must_use]
	pub(crate) const fn exports_with(
		pointer_width: PointerWidth,
		exports: &'static [WireExport],
	) -> Self {
		Self {
			pointer_width,
			kind: WireKind::Exports(exports),
		}
	}
}

const fn validate_imports(table: &WireImportTypeTable, imports: &[WireImport]) {
	let mut index = 0;
	while index < imports.len() {
		let import = &imports[index];
		let mut input_index = 0;
		while input_index < import.inputs.len() {
			assert!(
				import.inputs[input_index].type_index < table.input_types.len(),
				"import input type index is out of bounds",
			);
			input_index += 1;
		}
		if let Some(output) = import.output {
			assert!(
				output.type_index < table.output_types.len(),
				"import output type index is out of bounds",
			);
		}
		if import.suspending {
			if let Some(output) = import.output {
				assert!(
					!table.output_types[output.type_index].conversion.is_result()
						|| matches!(table.catch, WireImportCatch::Wasm(_)),
					"suspending Result imports require the Wasm exception-handling target feature",
				);
			}
		}
		index += 1;
	}
}

const fn flag(enabled: bool, value: u8) -> u8 {
	if enabled { value } else { 0 }
}
