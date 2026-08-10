use core::mem::size_of;

use crate::{
	IMPORT_CATCH_JAVASCRIPT, IMPORT_CATCH_WASM, IMPORT_HAS_BINDING, IMPORT_HAS_OUTPUT,
	IMPORT_OUTPUT_DIRECT, IMPORT_OUTPUT_RESULT, IMPORT_SUSPENDING, WireImport, WireImportBinding,
	WireImportCatch, WireImportInput, WireImportInputType, WireImportOutput, WireImportOutputType,
	WireImportTypeTable,
	abi::{JsCatch, WatCatch},
};

use super::{Encoder, Sizer};

impl<const N: usize> Encoder<N> {
	pub(super) const fn imports(&mut self, table: &WireImportTypeTable, imports: &[WireImport]) {
		table.encode(self);
		self.count(imports.len());
		let mut index = 0;
		while index < imports.len() {
			imports[index].encode(self);
			index += 1;
		}
	}
}

impl Sizer {
	pub(super) const fn imports(&mut self, table: &WireImportTypeTable, imports: &[WireImport]) {
		table.size(self);
		self.add(size_of::<u32>());
		let mut index = 0;
		while index < imports.len() {
			imports[index].size(self);
			index += 1;
		}
	}
}

impl WireImportTypeTable {
	const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.count(self.input_types.len());
		let mut index = 0;
		while index < self.input_types.len() {
			self.input_types[index].encode(encoder);
			index += 1;
		}

		encoder.count(self.output_types.len());
		if !self.output_types.is_empty() {
			self.retptr_type.encode(encoder);
		}
		index = 0;
		while index < self.output_types.len() {
			self.output_types[index].encode(encoder);
			index += 1;
		}
		if self.has_result() {
			self.catch.encode(encoder);
		}
	}

	const fn size(&self, sizer: &mut Sizer) {
		sizer.add(size_of::<u32>());
		let mut index = 0;
		while index < self.input_types.len() {
			self.input_types[index].size(sizer);
			index += 1;
		}

		sizer.add(size_of::<u32>());
		if !self.output_types.is_empty() {
			self.retptr_type.size(sizer);
		}
		index = 0;
		while index < self.output_types.len() {
			self.output_types[index].size(sizer);
			index += 1;
		}
		if self.has_result() {
			self.catch.size(sizer);
		}
	}
}

impl WireImportCatch {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		match self {
			Self::JavaScript(catch) => {
				encoder.u8(IMPORT_CATCH_JAVASCRIPT);
				catch.encode(encoder);
			}
			Self::Wasm(catch) => {
				encoder.u8(IMPORT_CATCH_WASM);
				catch.encode(encoder);
			}
		}
	}

	const fn size(self, sizer: &mut Sizer) {
		sizer.add(1);
		match self {
			Self::JavaScript(catch) => catch.size(sizer),
			Self::Wasm(catch) => catch.size(sizer),
		}
	}
}

impl JsCatch {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		encoder.count(self.embeds.len());
		let mut index = 0;
		while index < self.embeds.len() {
			encoder.string(self.embeds[index].module);
			encoder.string(self.embeds[index].name);
			index += 1;
		}
		encoder.string(self.direct);
		encoder.string(self.indirect);
	}

	const fn size(self, sizer: &mut Sizer) {
		sizer.add(size_of::<u32>());
		let mut index = 0;
		while index < self.embeds.len() {
			sizer.string(self.embeds[index].module);
			sizer.string(self.embeds[index].name);
			index += 1;
		}
		sizer.string(self.direct);
		sizer.string(self.indirect);
	}
}

impl WatCatch {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		encoder.count(self.imports.len());
		let mut index = 0;
		while index < self.imports.len() {
			self.imports[index].encode(encoder);
			index += 1;
		}
		encoder.count(self.locals.len());
		index = 0;
		while index < self.locals.len() {
			self.locals[index].encode(encoder);
			index += 1;
		}
		encoder.string(self.try_);
		encoder.string(self.catch);
	}

	const fn size(self, sizer: &mut Sizer) {
		sizer.add(size_of::<u32>());
		let mut index = 0;
		while index < self.imports.len() {
			self.imports[index].size(sizer);
			index += 1;
		}
		sizer.add(size_of::<u32>());
		index = 0;
		while index < self.locals.len() {
			self.locals[index].size(sizer);
			index += 1;
		}
		sizer.string(self.try_);
		sizer.string(self.catch);
	}
}

impl WireImportInputType {
	const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.slots(&self.slots);
		match self.conversion {
			Some(conversion) => {
				encoder.u8(1);
				conversion.encode(encoder);
			}
			None => encoder.u8(0),
		}
	}

	const fn size(&self, sizer: &mut Sizer) {
		sizer.slots(&self.slots);
		sizer.add(1);
		if let Some(conversion) = self.conversion {
			conversion.size(sizer);
		}
	}
}

impl WireImportOutputType {
	const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.u8(flag(self.mode.is_direct(), IMPORT_OUTPUT_DIRECT)
			| flag(self.conversion.is_result(), IMPORT_OUTPUT_RESULT));
		encoder.slots(&self.slots);
		match self.conversion.conversion() {
			Some(conversion) => {
				encoder.u8(1);
				conversion.encode(encoder);
			}
			None => encoder.u8(0),
		}
		if let Some(sret) = self.sret {
			encoder.sret(sret);
		}
	}

	const fn size(&self, sizer: &mut Sizer) {
		sizer.add(1);
		sizer.slots(&self.slots);
		sizer.add(1);
		if let Some(conversion) = self.conversion.conversion() {
			conversion.size(sizer);
		}
		if let Some(sret) = self.sret {
			sizer.sret(sret);
		}
	}
}

impl WireImportInput {
	const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.string(self.name);
		encoder.u32(wire_u32(self.type_index));
	}

	const fn size(&self, sizer: &mut Sizer) {
		sizer.string(self.name);
		sizer.add(size_of::<u32>());
	}
}

impl WireImportOutput {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		encoder.u32(wire_u32(self.type_index));
	}
}

impl WireImportBinding {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		encoder.optional_string(self.direct);
		encoder.string(self.call);
		encoder.count(self.required_embeds.len());
		let mut index = 0;
		while index < self.required_embeds.len() {
			let embed = self.required_embeds[index];
			encoder.string(embed.module);
			encoder.string(embed.name);
			index += 1;
		}
	}

	const fn size(self, sizer: &mut Sizer) {
		sizer.optional_string(self.direct);
		sizer.string(self.call);
		sizer.add(size_of::<u32>());
		let mut index = 0;
		while index < self.required_embeds.len() {
			let embed = self.required_embeds[index];
			sizer.string(embed.module);
			sizer.string(embed.name);
			index += 1;
		}
	}
}

impl WireImport {
	const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.string(self.module);
		encoder.string(self.name);
		encoder.u8(flag(self.suspending, IMPORT_SUSPENDING)
			| flag(self.output.is_some(), IMPORT_HAS_OUTPUT)
			| flag(self.binding.is_some(), IMPORT_HAS_BINDING));
		encoder.count(self.inputs.len());
		let mut index = 0;
		while index < self.inputs.len() {
			self.inputs[index].encode(encoder);
			index += 1;
		}
		if let Some(output) = self.output {
			output.encode(encoder);
		}
		if let Some(binding) = self.binding {
			binding.encode(encoder);
		}
	}

	const fn size(&self, sizer: &mut Sizer) {
		sizer.string(self.module);
		sizer.string(self.name);
		sizer.add(1 + size_of::<u32>());
		let mut index = 0;
		while index < self.inputs.len() {
			self.inputs[index].size(sizer);
			index += 1;
		}
		if self.output.is_some() {
			sizer.add(size_of::<u32>());
		}
		if let Some(binding) = self.binding {
			binding.size(sizer);
		}
	}
}

const fn flag(enabled: bool, value: u8) -> u8 {
	if enabled { value } else { 0 }
}

#[expect(
	clippy::cast_possible_truncation,
	reason = "the function asserts that the value fits in u32"
)]
const fn wire_u32(value: usize) -> u32 {
	assert!(value <= u32::MAX as usize);
	value as u32
}
