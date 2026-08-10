//! Static, const-constructible wire descriptions.

use core::mem::size_of;

use crate::abi::{
	FromJsConv, IntoJsConv, JsCatch, JsEmbed, ResultLayout, ReturnConv, ReturnMode, Sret, WatCatch,
	WatSlot, WatType,
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
	/// Wasm exception handling catches and records the exception in the `ABI`
	/// shim.
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

/// One argument accepted by a JavaScript import.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireImportInput {
	pub(crate) type_index: usize,
}

impl WireImportInput {
	#[must_use]
	pub const fn new(type_index: usize) -> Self {
		Self { type_index }
	}
}

/// One path rooted at JavaScript's global object.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireGlobalPath {
	pub(crate) namespace: Option<&'static str>,
	pub(crate) object: Option<&'static str>,
	pub(crate) name: &'static str,
}

impl WireGlobalPath {
	#[must_use]
	pub const fn new(
		namespace: Option<&'static str>,
		object: Option<&'static str>,
		name: &'static str,
	) -> Self {
		Self {
			namespace,
			object,
			name,
		}
	}
}

/// The JavaScript operation performed by one import.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WireImportBinding {
	CallGlobal {
		target: WireGlobalPath,
		variadic: bool,
	},
	CallMethod {
		name: &'static str,
		variadic: bool,
	},
	Construct {
		target: WireGlobalPath,
		variadic: bool,
	},
	GetGlobal(WireGlobalPath),
	GetMember(&'static str),
	SetGlobal(WireGlobalPath),
	SetMember(&'static str),
	IndexGet,
	IndexSet,
	IndexDelete,
	CallEmbed(JsEmbed),
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

/// One argument accepted by a JavaScript-facing `Wasm` export.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireExportInput {
	pub(crate) ty: &'static WireExportInputType,
}

impl WireExportInput {
	#[must_use]
	pub const fn new(ty: &'static WireExportInputType) -> Self {
		Self { ty }
	}
}

/// Type-level data shared by exported results with the same Rust type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireExportOutputType {
	pub(crate) kind: WireExportOutputKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WireExportOutputKind {
	Direct {
		slots: [Option<WatSlot>; 4],
		conversion: Option<IntoJsConv>,
	},
	Indirect {
		slots: [Option<WatSlot>; 4],
		conversion: ReturnConv<IntoJsConv>,
		frame: WireReturnFrame,
		result: Option<ResultLayout>,
	},
}

/// Stack storage used by an indirect Rust return.
///
/// Each offset corresponds to the same position in the output slot array;
/// offsets for empty slots are ignored.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireReturnFrame {
	pub(crate) size: usize,
	pub(crate) slot_offsets: [usize; 4],
}

impl WireReturnFrame {
	#[must_use]
	pub const fn new(size: usize, slot_offsets: [usize; 4]) -> Self {
		assert!(size != 0);
		Self { size, slot_offsets }
	}
}

impl WireExportOutputType {
	#[must_use]
	pub const fn direct(slots: [Option<WatSlot>; 4], conversion: Option<IntoJsConv>) -> Self {
		let mut slot_count = 0;
		let mut index = 0;
		while index < slots.len() {
			if slots[index].is_some() {
				slot_count += 1;
			}
			index += 1;
		}
		assert!(slot_count == 1);
		Self {
			kind: WireExportOutputKind::Direct { slots, conversion },
		}
	}

	#[must_use]
	pub const fn indirect(
		slots: [Option<WatSlot>; 4],
		conversion: ReturnConv<IntoJsConv>,
		frame: WireReturnFrame,
		result: Option<ResultLayout>,
	) -> Self {
		assert!(conversion.is_result() == result.is_some());
		let mut slot_count = 0;
		let mut index = 0;
		while index < slots.len() {
			if slots[index].is_some() {
				assert!(frame.slot_offsets[index] < frame.size);
				slot_count += 1;
			}
			index += 1;
		}
		if let Some(result) = result {
			let discriminant = result.discriminant as usize;
			let error = result.error as usize;
			assert!(error == discriminant + 1);
			assert!(error + 1 == slot_count);
		}
		Self {
			kind: WireExportOutputKind::Indirect {
				slots,
				conversion,
				frame,
				result,
			},
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

/// The raw linker symbol and boundary `ABI` of a closure factory.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireClosureFactory {
	pub(crate) raw_symbol: &'static str,
	pub(crate) helper: JsEmbed,
	pub(crate) input: &'static WireImportInputType,
	pub(crate) output: &'static WireImportOutputType,
}

impl WireClosureFactory {
	#[must_use]
	pub const fn new(
		raw_symbol: &'static str,
		helper: JsEmbed,
		input: &'static WireImportInputType,
		output: &'static WireImportOutputType,
	) -> Self {
		Self {
			raw_symbol,
			helper,
			input,
			output,
		}
	}
}

/// One semantic closure factory and its matching Wasm dispatcher.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireClosure {
	pub(crate) factory: WireClosureFactory,
	pub(crate) call_shim_offset: usize,
	pub(crate) inputs: &'static [WireExportInput],
	pub(crate) output: Option<WireExportOutput>,
}

impl WireClosure {
	#[must_use]
	pub const fn new(
		factory: WireClosureFactory,
		call_shim_offset: usize,
		inputs: &'static [WireExportInput],
		output: Option<WireExportOutput>,
	) -> Self {
		assert!(
			!inputs.is_empty(),
			"closure dispatchers require a data input"
		);
		Self {
			factory,
			call_shim_offset,
			inputs,
			output,
		}
	}
}

/// One semantic JavaScript-facing export.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WireExport {
	pub(crate) module: &'static str,
	pub(crate) name: &'static str,
	pub(crate) inputs: &'static [WireExportInput],
	pub(crate) output: Option<WireExportOutput>,
	pub(crate) symbol: &'static str,
	pub(crate) promising: bool,
}

impl WireExport {
	#[must_use]
	const fn new(
		module: &'static str,
		name: &'static str,
		inputs: &'static [WireExportInput],
		output: Option<WireExportOutput>,
		symbol: &'static str,
		promising: bool,
	) -> Self {
		Self {
			module,
			name,
			inputs,
			output,
			symbol,
			promising,
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
		Self::new(module, name, inputs, output, symbol, false)
	}

	#[must_use]
	pub const fn new_symbol_promising(
		module: &'static str,
		name: &'static str,
		symbol: &'static str,
		inputs: &'static [WireExportInput],
		output: Option<WireExportOutput>,
	) -> Self {
		Self::new(module, name, inputs, output, symbol, true)
	}
}

#[derive(Clone, Copy)]
pub(crate) enum WireKind {
	Imports {
		table: &'static WireImportTypeTable,
		imports: &'static [WireImport],
	},
	Exports(&'static [WireExport]),
	Closure(WireClosure),
}

/// One import, export, or closure payload encoded into a wire record.
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

	#[must_use]
	pub const fn closure(closure: WireClosure) -> Self {
		Self::closure_with(PointerWidth::native(), closure)
	}

	#[must_use]
	pub(crate) const fn closure_with(pointer_width: PointerWidth, closure: WireClosure) -> Self {
		Self {
			pointer_width,
			kind: WireKind::Closure(closure),
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
