//! Canonical model consumed by JavaScript and `WAT` `renderers`.

use alloc::{rc::Rc, vec::Vec};

pub use crate::PointerWidth;
pub use crate::abi::ResultLayout;
use crate::abi::{RefType, WatIndexType, WatType};

/// One primitive `Wasm` `ABI` slot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Slot<'a> {
	pub abi: WatType,
	pub wat: Option<WatConversion<'a>>,
}

impl<'a> Slot<'a> {
	#[must_use]
	pub fn boundary(&self) -> WatType {
		self.wat
			.as_ref()
			.map_or(self.abi, |conversion| conversion.boundary)
	}

	#[must_use]
	pub fn imports(&self) -> &[WatImport<'a>] {
		self.wat
			.as_ref()
			.map_or(&[], |conversion| conversion.imports.as_ref())
	}

	#[must_use]
	pub fn locals(&self) -> &[WatLocal<'a>] {
		self.wat
			.as_ref()
			.map_or(&[], |conversion| conversion.locals.as_ref())
	}

	#[must_use]
	pub fn instruction(&self) -> Option<&'a str> {
		self.wat.as_ref().map(|conversion| conversion.instruction)
	}
}

/// `WAT` required to translate one slot across the JavaScript boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatConversion<'a> {
	pub boundary: WatType,
	pub imports: Rc<[WatImport<'a>]>,
	pub locals: Rc<[WatLocal<'a>]>,
	pub instruction: &'a str,
}

/// One decoded structured `WAT` import.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatImport<'a> {
	pub module: &'a str,
	pub name: &'a str,
	pub identifier: &'a str,
	pub symbol_name: Option<&'a str>,
	pub kind: WatImportKind,
}

/// The kind and type of one decoded structured `WAT` import.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WatImportKind {
	Function {
		parameters: Vec<WatType>,
		results: Vec<WatType>,
	},
	Table {
		index_type: WatIndexType,
		minimum: u64,
		maximum: Option<u64>,
		element: RefType,
	},
	Tag {
		parameters: Vec<WatType>,
	},
}

/// One decoded structured local required by a boundary conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatLocal<'a> {
	pub name: &'a str,
	pub ty: WatType,
}

/// One JavaScript source fragment required by a generated binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Embed<'a> {
	pub module: &'a str,
	pub name: &'a str,
}

/// JavaScript exception-catching support shared by an import group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsCatch<'a> {
	pub embeds: Rc<[Embed<'a>]>,
	pub direct: &'a str,
	pub indirect: &'a str,
}

/// Wasm exception-catching support shared by an import group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatCatch<'a> {
	pub imports: Rc<[WatImport<'a>]>,
	pub locals: Rc<[WatLocal<'a>]>,
	pub try_: &'a str,
	pub catch: &'a str,
}

/// Where and how imported JavaScript exceptions are lowered.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImportCatch<'a> {
	JavaScript(JsCatch<'a>),
	Wasm(WatCatch<'a>),
}

/// One named argument passed from Rust to an imported JavaScript function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportInput<'a> {
	pub name: &'a str,
	pub slots: Vec<Slot<'a>>,
	pub js_conversion: Option<&'a str>,
}

/// Where an imported `Result` catches a JavaScript exception.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImportErrorMode {
	Infallible,
	CatchInJavaScript,
	CatchInWasm,
}

/// JavaScript conversion for a direct import result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DirectImportConversion<'a> {
	pub prepare: Option<&'a str>,
	pub expression: &'a str,
}

/// The return pointer accepted by an indirect import.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportRetptr<'a> {
	pub slot: Slot<'a>,
	pub js_conversion: Option<&'a str>,
}

/// How JavaScript writes an indirect import result into Rust's return area.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImportWriter<'a> {
	Slots {
		function: &'a str,
		prepare: Option<&'a str>,
		expressions: Vec<&'a str>,
	},
	Value {
		function: &'a str,
	},
}

/// The `ABI` shape of one imported result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImportOutputAbi<'a> {
	Direct {
		slot: Slot<'a>,
		conversion: Option<DirectImportConversion<'a>>,
	},
	Indirect {
		retptr: ImportRetptr<'a>,
		writer: ImportWriter<'a>,
	},
}

/// One value returned by an imported JavaScript function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportOutput<'a> {
	pub abi: ImportOutputAbi<'a>,
	pub error: ImportErrorMode,
}

impl ImportOutput<'_> {
	#[must_use]
	pub const fn is_direct(&self) -> bool {
		matches!(&self.abi, ImportOutputAbi::Direct { .. })
	}
}

/// JavaScript call expressions and required embeds for one import.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportBinding<'a> {
	pub direct_expression: Option<&'a str>,
	pub call_expression: &'a str,
	pub embeds: Vec<Embed<'a>>,
}

/// One decoded JavaScript import.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Import<'a> {
	pub module: &'a str,
	pub name: &'a str,
	pub inputs: Vec<ImportInput<'a>>,
	pub output: Option<ImportOutput<'a>>,
	pub binding: Option<ImportBinding<'a>>,
	pub suspending: bool,
}

/// One decoded group of JavaScript imports and its shared exception lowering.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportGroup<'a> {
	pub catch: Option<ImportCatch<'a>>,
	pub imports: Vec<Import<'a>>,
}

/// Why an export input exists in the Rust call `ABI`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExportInputKind {
	Value,
	ClosureData,
}

/// JavaScript conversion for one exported argument.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportInputConversion<'a> {
	pub prepare: Option<&'a str>,
	pub expressions: Vec<&'a str>,
}

/// One JavaScript argument accepted by a `Wasm` export.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportInput<'a> {
	pub kind: ExportInputKind,
	pub name: &'a str,
	pub slots: Vec<Slot<'a>>,
	pub conversion: Option<ExportInputConversion<'a>>,
}

/// One slot loaded from an indirect Rust return frame.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameSlot<'a> {
	pub slot: Slot<'a>,
	pub offset: u64,
}

/// Stack storage used by an indirect Rust return.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReturnFrame<'a> {
	pub size: u64,
	pub slots: Vec<FrameSlot<'a>>,
}

/// One value returned from Rust to JavaScript.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExportOutput<'a> {
	Direct {
		slot: Slot<'a>,
		js_conversion: Option<&'a str>,
	},
	Indirect {
		frame: ReturnFrame<'a>,
		js_conversion: Option<&'a str>,
		result: Option<ResultLayout>,
	},
}

impl<'a> ExportOutput<'a> {
	#[must_use]
	pub const fn is_direct(&self) -> bool {
		matches!(self, Self::Direct { .. })
	}

	#[must_use]
	pub const fn js_conversion(&self) -> Option<&'a str> {
		match self {
			Self::Direct { js_conversion, .. } | Self::Indirect { js_conversion, .. } => {
				*js_conversion
			}
		}
	}

	#[must_use]
	pub const fn result(&self) -> Option<ResultLayout> {
		match self {
			Self::Direct { .. } => None,
			Self::Indirect { result, .. } => *result,
		}
	}
}

/// How a public export shim reaches Rust code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Callee<'a> {
	Symbol { name: &'a str },
	Closure { call_shim_offset: u64 },
}

/// One decoded JavaScript-facing `Wasm` export.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Export<'a> {
	pub module: &'a str,
	pub name: &'a str,
	pub pointer_width: PointerWidth,
	pub inputs: Vec<ExportInput<'a>>,
	pub output: Option<ExportOutput<'a>>,
	pub embeds: Vec<Embed<'a>>,
	pub promising: bool,
	pub callee: Callee<'a>,
}

impl Export<'_> {
	#[must_use]
	pub const fn pointer_type(&self) -> WatType {
		self.pointer_width.wat_type()
	}
}

/// One decoded import or export group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Record<'a> {
	Imports(ImportGroup<'a>),
	Exports(Vec<Export<'a>>),
}
