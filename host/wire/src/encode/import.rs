use core::mem::size_of;

use super::{Encoder, Sizer, flag, wire_u32};
use crate::abi::{JsCatch, WatCatch};
use crate::{
	IMPORT_BINDING_CALL_EMBED, IMPORT_BINDING_CALL_GLOBAL, IMPORT_BINDING_CALL_METHOD,
	IMPORT_BINDING_CONSTRUCT, IMPORT_BINDING_GET_GLOBAL, IMPORT_BINDING_GET_MEMBER,
	IMPORT_BINDING_INDEX_DELETE, IMPORT_BINDING_INDEX_GET, IMPORT_BINDING_INDEX_SET,
	IMPORT_BINDING_SET_GLOBAL, IMPORT_BINDING_SET_MEMBER, IMPORT_CATCH_JAVASCRIPT,
	IMPORT_CATCH_WASM, IMPORT_HAS_BINDING, IMPORT_HAS_OUTPUT, IMPORT_OUTPUT_DIRECT,
	IMPORT_OUTPUT_RESULT, IMPORT_SUSPENDING, WireGlobalPath, WireImport, WireImportBinding,
	WireImportCatch, WireImportInput, WireImportInputType, WireImportOutput, WireImportOutputType,
	WireImportTypeTable,
};

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
	pub(super) const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.slots(&self.slots);
		match self.conversion {
			Some(conversion) => {
				encoder.u8(1);
				conversion.encode(encoder);
			}
			None => encoder.u8(0),
		}
	}

	pub(super) const fn size(&self, sizer: &mut Sizer) {
		sizer.slots(&self.slots);
		sizer.add(1);
		if let Some(conversion) = self.conversion {
			conversion.size(sizer);
		}
	}
}

impl WireImportOutputType {
	pub(super) const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
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

	pub(super) const fn size(&self, sizer: &mut Sizer) {
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
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		encoder.u32(wire_u32(self.type_index));
	}
}

impl WireImportOutput {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		encoder.u32(wire_u32(self.type_index));
	}
}

impl WireImportBinding {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		match self {
			Self::CallGlobal { target, variadic } => {
				encoder.u8(IMPORT_BINDING_CALL_GLOBAL);
				target.encode(encoder);
				encoder.u8(variadic as u8);
			}
			Self::CallMethod { name, variadic } => {
				encoder.u8(IMPORT_BINDING_CALL_METHOD);
				encoder.string(name);
				encoder.u8(variadic as u8);
			}
			Self::Construct { target, variadic } => {
				encoder.u8(IMPORT_BINDING_CONSTRUCT);
				target.encode(encoder);
				encoder.u8(variadic as u8);
			}
			Self::GetGlobal(target) => {
				encoder.u8(IMPORT_BINDING_GET_GLOBAL);
				target.encode(encoder);
			}
			Self::GetMember(name) => {
				encoder.u8(IMPORT_BINDING_GET_MEMBER);
				encoder.string(name);
			}
			Self::SetGlobal(target) => {
				encoder.u8(IMPORT_BINDING_SET_GLOBAL);
				target.encode(encoder);
			}
			Self::SetMember(name) => {
				encoder.u8(IMPORT_BINDING_SET_MEMBER);
				encoder.string(name);
			}
			Self::IndexGet => encoder.u8(IMPORT_BINDING_INDEX_GET),
			Self::IndexSet => encoder.u8(IMPORT_BINDING_INDEX_SET),
			Self::IndexDelete => encoder.u8(IMPORT_BINDING_INDEX_DELETE),
			Self::CallEmbed(embed) => {
				encoder.u8(IMPORT_BINDING_CALL_EMBED);
				encoder.string(embed.module);
				encoder.string(embed.name);
			}
		}
	}

	const fn size(self, sizer: &mut Sizer) {
		sizer.add(size_of::<u8>());
		match self {
			Self::CallGlobal { target, .. } | Self::Construct { target, .. } => {
				target.size(sizer);
				sizer.add(size_of::<u8>());
			}
			Self::CallMethod { name, .. } => {
				sizer.string(name);
				sizer.add(size_of::<u8>());
			}
			Self::GetGlobal(target) | Self::SetGlobal(target) => target.size(sizer),
			Self::GetMember(name) | Self::SetMember(name) => sizer.string(name),
			Self::IndexGet | Self::IndexSet | Self::IndexDelete => {}
			Self::CallEmbed(embed) => {
				sizer.string(embed.module);
				sizer.string(embed.name);
			}
		}
	}
}

impl WireGlobalPath {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		encoder.optional_string(self.namespace);
		encoder.optional_string(self.object);
		encoder.string(self.name);
	}

	const fn size(self, sizer: &mut Sizer) {
		sizer.optional_string(self.namespace);
		sizer.optional_string(self.object);
		sizer.string(self.name);
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
		sizer.add(self.inputs.len() * size_of::<u32>());
		if self.output.is_some() {
			sizer.add(size_of::<u32>());
		}
		if let Some(binding) = self.binding {
			binding.size(sizer);
		}
	}
}
