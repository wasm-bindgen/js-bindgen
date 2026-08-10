//! Constant serialization of static wire descriptions.

mod export;
mod import;

use core::mem::size_of;

use crate::abi::{
	FromJsConv, IntoJsConv, JsEmbed, Sret, WatImport, WatImportKind, WatLocal, WatSlot, WatType,
};
use crate::schema::WireKind;
use crate::{MAGIC, VERSION, WAT_IMPORT_FUNCTION, WAT_IMPORT_TABLE, WAT_IMPORT_TAG, Wire};

/// A raw, self-contained wire record without custom-section framing.
#[derive(Clone, Copy)]
pub struct WireRecord<const N: usize> {
	bytes: [u8; N],
}

impl<const N: usize> WireRecord<N> {
	#[must_use]
	pub const fn new(wire: &Wire) -> Self {
		Self {
			bytes: encode::<N>(wire),
		}
	}

	#[must_use]
	pub const fn as_bytes(&self) -> &[u8; N] {
		&self.bytes
	}

	#[must_use]
	pub const fn into_bytes(self) -> [u8; N] {
		self.bytes
	}
}

/// A length-prefixed wire record stored in the `js_bindgen.wire` custom
/// section.
#[repr(C)]
pub struct WireBlob<const N: usize> {
	record_len: [u8; 4],
	bytes: [u8; N],
}

impl<const N: usize> WireBlob<N> {
	#[must_use]
	pub const fn new(wire: &Wire) -> Self {
		let record_len = wire_u32(N);

		Self {
			record_len: record_len.to_le_bytes(),
			bytes: encode::<N>(wire),
		}
	}
}

/// Computes the exact encoded size of a wire record.
#[must_use]
pub const fn wire_blob_len(wire: &Wire) -> usize {
	let mut sizer = Sizer::new();
	sizer.header();
	match &wire.kind {
		WireKind::Imports { table, imports } => sizer.imports(table, imports),
		WireKind::Exports(exports) => sizer.exports(exports),
	}
	sizer.position
}

const fn encode<const N: usize>(wire: &Wire) -> [u8; N] {
	let mut encoder = Encoder::<N>::new();
	encoder.header(wire.pointer_width.bytes(), wire.kind.tag());
	match &wire.kind {
		WireKind::Imports { table, imports } => encoder.imports(table, imports),
		WireKind::Exports(exports) => encoder.exports(exports),
	}
	assert!(encoder.position == N);
	encoder.bytes
}

impl WireKind {
	const fn tag(&self) -> u8 {
		match self {
			Self::Imports { .. } => crate::KIND_IMPORT,
			Self::Exports(_) => crate::KIND_EXPORT,
		}
	}
}

pub(crate) struct Encoder<const N: usize> {
	bytes: [u8; N],
	position: usize,
}

impl<const N: usize> Encoder<N> {
	const fn new() -> Self {
		Self {
			bytes: [0; N],
			position: 0,
		}
	}

	const fn header(&mut self, pointer_width: u8, kind: u8) {
		self.bytes(&MAGIC);
		self.u16(VERSION);
		self.u8(pointer_width);
		self.u8(kind);
	}

	pub(crate) const fn string(&mut self, value: &'static str) {
		self.optional_string(Some(value));
	}

	pub(crate) const fn optional_string(&mut self, value: Option<&'static str>) {
		let Some(value) = value else {
			self.u32(u32::MAX);
			return;
		};
		let len = wire_string_len(value.len());
		let position = self.position;
		let value_position = position + size_of::<u32>();
		let end = value_position + len as usize;
		assert!(end <= N);
		let length = len.to_le_bytes();
		self.bytes[position] = length[0];
		self.bytes[position + 1] = length[1];
		self.bytes[position + 2] = length[2];
		self.bytes[position + 3] = length[3];
		if len != 0 {
			// SAFETY: The bounds check covers the destination, and the source
			// is a distinct immutable string.
			unsafe {
				core::ptr::copy_nonoverlapping(
					value.as_ptr(),
					self.bytes.as_mut_ptr().add(value_position),
					len as usize,
				);
			}
		}
		self.position = end;
	}

	pub(crate) const fn count(&mut self, value: usize) {
		self.u32(wire_u32(value));
	}

	pub(crate) const fn wat_types(&mut self, values: &[WatType]) {
		self.count(values.len());
		let mut index = 0;
		while index < values.len() {
			self.u8(values[index].tag());
			index += 1;
		}
	}

	pub(crate) const fn slots(&mut self, slots: &[Option<WatSlot>; 4]) {
		let mask = slot_mask(slots);
		self.u8(mask);
		let mut index = 0;
		while index < slots.len() {
			if let Some(slot) = slots[index] {
				slot.encode(self);
			}
			index += 1;
		}
	}

	pub(crate) const fn templates(&mut self, templates: &[Option<&'static str>; 4]) {
		let mask = template_mask(templates);
		self.u8(mask);
		let mut index = 0;
		while index < templates.len() {
			if mask & (1 << index) != 0 {
				self.string(match templates[index] {
					Some(template) => template,
					None => unreachable!(),
				});
			}
			index += 1;
		}
	}

	pub(crate) const fn embed(&mut self, embed: Option<JsEmbed>) {
		match embed {
			Some(embed) => {
				self.u8(1);
				self.string(embed.module);
				self.string(embed.name);
			}
			None => self.u8(0),
		}
	}

	pub(crate) const fn u64(&mut self, value: u64) {
		self.bytes(&value.to_le_bytes());
	}

	pub(crate) const fn optional_u64(&mut self, value: Option<u64>) {
		match value {
			Some(value) => {
				self.u8(1);
				self.u64(value);
			}
			None => self.u8(0),
		}
	}

	pub(crate) const fn u32(&mut self, value: u32) {
		self.bytes(&value.to_le_bytes());
	}

	const fn u16(&mut self, value: u16) {
		self.bytes(&value.to_le_bytes());
	}

	pub(crate) const fn u8(&mut self, value: u8) {
		assert!(self.position < N);
		self.bytes[self.position] = value;
		self.position += 1;
	}

	const fn bytes(&mut self, value: &[u8]) {
		let Some(end) = self.position.checked_add(value.len()) else {
			panic!("wire record write overflow");
		};
		assert!(end <= N);
		if !value.is_empty() {
			// SAFETY: The bounds check covers the destination, and the private
			// output buffer cannot overlap `value`.
			unsafe {
				core::ptr::copy_nonoverlapping(
					value.as_ptr(),
					self.bytes.as_mut_ptr().add(self.position),
					value.len(),
				);
			}
		}
		self.position = end;
	}
}

// Sizing remains separate from `Encoder`: using `Encoder<0>` adds a branch
// to every write during constant evaluation and slows large import groups.
pub(crate) struct Sizer {
	position: usize,
}

impl Sizer {
	const fn new() -> Self {
		Self { position: 0 }
	}

	const fn header(&mut self) {
		self.position += MAGIC.len() + size_of::<u16>() + 2 * size_of::<u8>();
	}

	pub(crate) const fn string(&mut self, value: &'static str) {
		self.optional_string(Some(value));
	}

	pub(crate) const fn optional_string(&mut self, value: Option<&'static str>) {
		self.position += size_of::<u32>();
		if let Some(value) = value {
			assert!(value.len() < u32::MAX as usize);
			self.position += value.len();
		}
	}

	pub(crate) const fn wat_types(&mut self, values: &[WatType]) {
		self.add(size_of::<u32>() + values.len());
	}

	pub(crate) const fn add(&mut self, bytes: usize) {
		let Some(position) = self.position.checked_add(bytes) else {
			panic!("wire record size overflow");
		};
		self.position = position;
	}

	pub(crate) const fn slots(&mut self, slots: &[Option<WatSlot>; 4]) {
		self.add(1);
		let mut index = 0;
		while index < slots.len() {
			if let Some(slot) = slots[index] {
				slot.size(self);
			}
			index += 1;
		}
	}

	pub(crate) const fn templates(&mut self, templates: &[Option<&'static str>; 4]) {
		self.add(1);
		let mask = template_mask(templates);
		let mut index = 0;
		while index < templates.len() {
			if mask & (1 << index) != 0 {
				self.optional_string(templates[index]);
			}
			index += 1;
		}
	}

	pub(crate) const fn embed(&mut self, embed: Option<JsEmbed>) {
		self.add(1);
		if let Some(embed) = embed {
			self.string(embed.module);
			self.string(embed.name);
		}
	}

	pub(crate) const fn optional_u64(&mut self, value: Option<u64>) {
		self.add(size_of::<u8>());
		if value.is_some() {
			self.add(size_of::<u64>());
		}
	}
}

impl WatSlot {
	const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.u8(self.rust.tag());
		match self.wat {
			Some(wat) => {
				encoder.u8(1);
				encoder.count(wat.imports.len());
				let mut index = 0;
				while index < wat.imports.len() {
					wat.imports[index].encode(encoder);
					index += 1;
				}
				encoder.count(wat.locals.len());
				index = 0;
				while index < wat.locals.len() {
					wat.locals[index].encode(encoder);
					index += 1;
				}
				encoder.string(wat.instruction);
				encoder.u8(wat.js.tag());
			}
			None => encoder.u8(0),
		}
	}

	const fn size(&self, sizer: &mut Sizer) {
		sizer.add(1);
		sizer.add(1);
		if let Some(wat) = self.wat {
			sizer.add(size_of::<u32>());
			let mut index = 0;
			while index < wat.imports.len() {
				wat.imports[index].size(sizer);
				index += 1;
			}
			sizer.add(size_of::<u32>());
			index = 0;
			while index < wat.locals.len() {
				wat.locals[index].size(sizer);
				index += 1;
			}
			sizer.string(wat.instruction);
			sizer.add(1);
		}
	}
}

impl WatImport {
	const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.u8(self.kind.tag());
		encoder.string(self.module);
		encoder.string(self.name);
		encoder.string(self.identifier);
		encoder.optional_string(self.symbol_name);
		match self.kind {
			WatImportKind::Function {
				parameters,
				results,
			} => {
				encoder.wat_types(parameters);
				encoder.wat_types(results);
			}
			WatImportKind::Table {
				index_type,
				minimum,
				maximum,
				element,
			} => {
				encoder.u8(index_type.tag());
				encoder.u64(minimum);
				encoder.optional_u64(maximum);
				encoder.u8(element.tag());
			}
			WatImportKind::Tag { parameters } => encoder.wat_types(parameters),
		}
	}

	const fn size(&self, sizer: &mut Sizer) {
		sizer.add(size_of::<u8>());
		sizer.string(self.module);
		sizer.string(self.name);
		sizer.string(self.identifier);
		sizer.optional_string(self.symbol_name);
		match self.kind {
			WatImportKind::Function {
				parameters,
				results,
			} => {
				sizer.wat_types(parameters);
				sizer.wat_types(results);
			}
			WatImportKind::Table {
				index_type: _,
				minimum: _,
				maximum,
				element: _,
			} => {
				sizer.add(1);
				sizer.add(size_of::<u64>());
				sizer.optional_u64(maximum);
				sizer.add(1);
			}
			WatImportKind::Tag { parameters } => sizer.wat_types(parameters),
		}
	}
}

impl WatImportKind {
	const fn tag(self) -> u8 {
		match self {
			Self::Function { .. } => WAT_IMPORT_FUNCTION,
			Self::Table { .. } => WAT_IMPORT_TABLE,
			Self::Tag { .. } => WAT_IMPORT_TAG,
		}
	}
}

impl WatLocal {
	const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.string(self.name);
		encoder.u8(self.ty.tag());
	}

	const fn size(&self, sizer: &mut Sizer) {
		sizer.string(self.name);
		sizer.add(1);
	}
}

impl IntoJsConv {
	pub(crate) const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.embed(self.embed);
		encoder.string(self.template);
	}

	pub(crate) const fn size(&self, sizer: &mut Sizer) {
		sizer.embed(self.embed);
		sizer.string(self.template);
	}
}

impl FromJsConv {
	pub(crate) const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.embed(self.embed);
		encoder.optional_string(self.prepare);
		encoder.templates(&self.templates);
	}

	pub(crate) const fn size(&self, sizer: &mut Sizer) {
		sizer.embed(self.embed);
		sizer.optional_string(self.prepare);
		sizer.templates(&self.templates);
	}
}

impl<const N: usize> Encoder<N> {
	pub(crate) const fn sret(&mut self, sret: Sret) {
		match sret {
			Sret::Slots(function) => {
				self.u8(0);
				self.string(function);
			}
			Sret::Value(function) => {
				self.u8(1);
				self.string(function);
			}
		}
	}
}

impl Sizer {
	pub(crate) const fn sret(&mut self, sret: Sret) {
		self.add(1);
		let function = match sret {
			Sret::Slots(function) | Sret::Value(function) => function,
		};
		self.string(function);
	}
}

const fn slot_mask(slots: &[Option<WatSlot>; 4]) -> u8 {
	let mut mask = 0;
	let mut index = 0;
	while index < slots.len() {
		if slots[index].is_some() {
			mask |= 1 << index;
		}
		index += 1;
	}
	mask
}

const fn template_mask(templates: &[Option<&str>; 4]) -> u8 {
	let mut mask = 0;
	let mut index = 0;
	while index < templates.len() {
		if templates[index].is_some() {
			mask |= 1 << index;
		}
		index += 1;
	}
	mask
}

#[expect(
	clippy::cast_possible_truncation,
	reason = "the function asserts that the value fits in u32"
)]
pub(crate) const fn wire_u32(value: usize) -> u32 {
	assert!(value <= u32::MAX as usize);
	value as u32
}

pub(crate) const fn flag(enabled: bool, value: u8) -> u8 {
	if enabled { value } else { 0 }
}

#[expect(
	clippy::cast_possible_truncation,
	reason = "the function asserts that the value fits in u32"
)]
const fn wire_string_len(value: usize) -> u32 {
	assert!(value < u32::MAX as usize);
	value as u32
}
