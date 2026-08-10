//! Shared JavaScript-boundary `ABI` descriptions.

#[cfg(feature = "alloc")]
use core::fmt;

/// One primitive `WebAssembly` value type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WatType {
	I32,
	I64,
	F32,
	F64,
	V128,
	ExternRef,
	FuncRef,
}

impl WatType {
	#[cfg(feature = "alloc")]
	pub(crate) const MAX_TAG: u8 = 6;

	#[must_use]
	#[cfg(feature = "alloc")]
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::I32 => "i32",
			Self::I64 => "i64",
			Self::F32 => "f32",
			Self::F64 => "f64",
			Self::V128 => "v128",
			Self::ExternRef => "externref",
			Self::FuncRef => "funcref",
		}
	}

	/// Returns an instruction that places this type's default value on the
	/// stack.
	#[must_use]
	#[cfg(feature = "alloc")]
	pub const fn zero(self) -> &'static str {
		match self {
			Self::I32 => "i32.const 0",
			Self::I64 => "i64.const 0",
			Self::F32 => "f32.const 0",
			Self::F64 => "f64.const 0",
			Self::V128 => "v128.const i32x4 0 0 0 0",
			Self::ExternRef => "ref.null extern",
			Self::FuncRef => "ref.null func",
		}
	}

	pub(crate) const fn tag(self) -> u8 {
		match self {
			Self::I32 => 0,
			Self::I64 => 1,
			Self::F32 => 2,
			Self::F64 => 3,
			Self::V128 => 4,
			Self::ExternRef => 5,
			Self::FuncRef => 6,
		}
	}

	#[cfg(feature = "alloc")]
	pub(crate) const fn from_tag(tag: u8) -> Option<Self> {
		match tag {
			0 => Some(Self::I32),
			1 => Some(Self::I64),
			2 => Some(Self::F32),
			3 => Some(Self::F64),
			4 => Some(Self::V128),
			5 => Some(Self::ExternRef),
			6 => Some(Self::FuncRef),
			_ => None,
		}
	}
}

#[cfg(feature = "alloc")]
impl fmt::Display for WatType {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_str(self.as_str())
	}
}

/// The index type of a `WebAssembly` table.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum WatIndexType {
	I32,
	I64,
}

impl WatIndexType {
	#[cfg(feature = "alloc")]
	pub(crate) const MAX_TAG: u8 = 1;

	#[must_use]
	#[cfg(feature = "alloc")]
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::I32 => "i32",
			Self::I64 => "i64",
		}
	}

	/// Returns the explicit table-index prefix used by canonical WAT.
	#[must_use]
	#[cfg(feature = "alloc")]
	pub const fn wat_prefix(self) -> &'static str {
		match self {
			Self::I32 => "",
			Self::I64 => "i64 ",
		}
	}

	pub(crate) const fn tag(self) -> u8 {
		match self {
			Self::I32 => 0,
			Self::I64 => 1,
		}
	}

	#[cfg(feature = "alloc")]
	pub(crate) const fn from_tag(tag: u8) -> Option<Self> {
		match tag {
			0 => Some(Self::I32),
			1 => Some(Self::I64),
			_ => None,
		}
	}
}

#[cfg(feature = "alloc")]
impl fmt::Display for WatIndexType {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_str(self.as_str())
	}
}

/// A reference type accepted by a `WebAssembly` table.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RefType {
	ExternRef,
	FuncRef,
}

impl RefType {
	#[cfg(feature = "alloc")]
	pub(crate) const MAX_TAG: u8 = 1;

	#[must_use]
	#[cfg(feature = "alloc")]
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::ExternRef => "externref",
			Self::FuncRef => "funcref",
		}
	}

	pub(crate) const fn tag(self) -> u8 {
		match self {
			Self::ExternRef => 0,
			Self::FuncRef => 1,
		}
	}

	#[cfg(feature = "alloc")]
	pub(crate) const fn from_tag(tag: u8) -> Option<Self> {
		match tag {
			0 => Some(Self::ExternRef),
			1 => Some(Self::FuncRef),
			_ => None,
		}
	}
}

#[cfg(feature = "alloc")]
impl fmt::Display for RefType {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_str(self.as_str())
	}
}

/// One JavaScript source fragment required by a generated binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JsEmbed {
	pub module: &'static str,
	pub name: &'static str,
}

impl JsEmbed {
	#[must_use]
	pub const fn new(module: &'static str, name: &'static str) -> Self {
		Self { module, name }
	}
}

/// JavaScript exception-catching support shared by one import type table.
///
/// The `renderer` places the appropriate catch suffix after the successful
/// direct or indirect result path. `embeds` contains the JavaScript `runtime`
/// values referenced by those suffixes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JsCatch {
	pub embeds: &'static [JsEmbed],
	pub direct: &'static str,
	pub indirect: &'static str,
}

impl JsCatch {
	#[must_use]
	pub const fn new(
		embeds: &'static [JsEmbed],
		direct: &'static str,
		indirect: &'static str,
	) -> Self {
		Self {
			embeds,
			direct,
			indirect,
		}
	}
}

/// One structured WAT import required by a boundary conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatImport {
	pub module: &'static str,
	pub name: &'static str,
	pub identifier: &'static str,
	pub symbol_name: Option<&'static str>,
	pub kind: WatImportKind,
}

impl WatImport {
	#[must_use]
	pub const fn new(
		module: &'static str,
		name: &'static str,
		identifier: &'static str,
		symbol_name: Option<&'static str>,
		kind: WatImportKind,
	) -> Self {
		Self {
			module,
			name,
			identifier,
			symbol_name,
			kind,
		}
	}
}

/// The kind and type of one structured WAT import.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatImportKind {
	Function {
		parameters: &'static [WatType],
		results: &'static [WatType],
	},
	Table {
		index_type: WatIndexType,
		minimum: u64,
		maximum: Option<u64>,
		element: RefType,
	},
	Tag {
		parameters: &'static [WatType],
	},
}

/// One structured local required by a boundary conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatLocal {
	pub name: &'static str,
	pub ty: WatType,
}

impl WatLocal {
	#[must_use]
	pub const fn new(name: &'static str, ty: WatType) -> Self {
		Self { name, ty }
	}
}

/// Wasm exception-catching support shared by one import type table.
///
/// `try_` is inserted before the imported call and its boundary conversions;
/// `catch` is inserted after the successful path. The `renderer` remains
/// responsible for the direct result's type-specific fallback value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatCatch {
	pub imports: &'static [WatImport],
	pub locals: &'static [WatLocal],
	pub try_: &'static str,
	pub catch: &'static str,
}

impl WatCatch {
	#[must_use]
	pub const fn new(
		imports: &'static [WatImport],
		locals: &'static [WatLocal],
		try_: &'static str,
		catch: &'static str,
	) -> Self {
		Self {
			imports,
			locals,
			try_,
			catch,
		}
	}
}

/// `WAT` required to translate one primitive `WebAssembly` slot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatConv {
	pub imports: &'static [WatImport],
	pub locals: &'static [WatLocal],
	pub instruction: &'static str,
	pub js: WatType,
}

impl WatConv {
	#[must_use]
	pub const fn new(
		imports: &'static [WatImport],
		locals: &'static [WatLocal],
		instruction: &'static str,
		js: WatType,
	) -> Self {
		Self {
			imports,
			locals,
			instruction,
			js,
		}
	}
}

/// The Rust-facing type of one primitive `WebAssembly` slot and its optional
/// JavaScript-facing conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatSlot {
	pub rust: WatType,
	pub wat: Option<WatConv>,
}

impl WatSlot {
	#[must_use]
	pub const fn new(rust: WatType, wat: Option<WatConv>) -> Self {
		Self { rust, wat }
	}

	#[must_use]
	pub const fn plain(rust: WatType) -> Self {
		Self::new(rust, None)
	}
}

/// Converts primitive `ABI` slots into one JavaScript value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntoJsConv {
	pub embed: Option<JsEmbed>,
	pub template: &'static str,
}

impl IntoJsConv {
	#[must_use]
	pub const fn new(template: &'static str) -> Self {
		Self {
			embed: None,
			template,
		}
	}

	#[must_use]
	pub const fn with_embed(mut self, module: &'static str, name: &'static str) -> Self {
		self.embed = Some(JsEmbed::new(module, name));
		self
	}
}

/// Converts one JavaScript value into primitive `ABI` slots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FromJsConv {
	pub embed: Option<JsEmbed>,
	pub prepare: Option<&'static str>,
	pub templates: [Option<&'static str>; 4],
}

impl FromJsConv {
	#[must_use]
	pub const fn slot1(template: &'static str) -> Self {
		Self {
			embed: None,
			prepare: None,
			templates: [Some(template), None, None, None],
		}
	}

	#[must_use]
	pub const fn prepare(mut self, template: &'static str) -> Self {
		self.prepare = Some(template);
		self
	}

	#[must_use]
	pub const fn slot2(mut self, template: &'static str) -> Self {
		self.templates[1] = Some(template);
		self
	}

	#[must_use]
	pub const fn slot3(mut self, template: &'static str) -> Self {
		self.templates[2] = Some(template);
		self
	}

	#[must_use]
	pub const fn slot4(mut self, template: &'static str) -> Self {
		self.templates[3] = Some(template);
		self
	}

	#[must_use]
	pub const fn with_embed(mut self, module: &'static str, name: &'static str) -> Self {
		self.embed = Some(JsEmbed::new(module, name));
		self
	}
}

/// Selects how an indirect JavaScript import result is written to Rust.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Sret {
	Slots(&'static str),
	Value(&'static str),
}

/// Describes how a function return is handled at the JavaScript boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReturnConv<T> {
	Value(Option<T>),
	Result(Option<T>),
}

impl<T: Copy> ReturnConv<T> {
	#[must_use]
	pub const fn conversion(self) -> Option<T> {
		match self {
			Self::Value(value) | Self::Result(value) => value,
		}
	}

	#[must_use]
	pub const fn is_result(self) -> bool {
		matches!(self, Self::Result(_))
	}
}

/// Describes how a Rust function return is represented in its C `ABI`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReturnMode {
	Direct,
	Indirect,
}

impl ReturnMode {
	#[must_use]
	pub const fn is_direct(self) -> bool {
		matches!(self, Self::Direct)
	}
}

/// Positions of an exported Result's control slots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResultLayout {
	pub discriminant: u8,
	pub error: u8,
}

impl ResultLayout {
	#[must_use]
	pub const fn new(discriminant: u8, error: u8) -> Self {
		Self {
			discriminant,
			error,
		}
	}
}
