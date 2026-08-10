use core::mem::size_of;

use super::{Encoder, Sizer, flag};
use crate::{
	EXPORT_HAS_OUTPUT, EXPORT_OUTPUT_DIRECT, EXPORT_OUTPUT_RESULT, EXPORT_PROMISING, WireExport,
	WireExportCallee, WireExportInput, WireExportOutput, WireExportOutputType,
};

impl<const N: usize> Encoder<N> {
	pub(super) const fn exports(&mut self, exports: &[WireExport]) {
		self.count(exports.len());
		let mut index = 0;
		while index < exports.len() {
			exports[index].encode(self);
			index += 1;
		}
	}
}

impl Sizer {
	pub(super) const fn exports(&mut self, exports: &[WireExport]) {
		self.add(size_of::<u32>());
		let mut index = 0;
		while index < exports.len() {
			exports[index].size(self);
			index += 1;
		}
	}
}

impl WireExportCallee {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		match self {
			Self::Symbol(symbol) => {
				encoder.u8(0);
				encoder.string(symbol);
			}
			Self::Closure { call_shim_offset } => {
				encoder.u8(1);
				encoder.u64(call_shim_offset as u64);
			}
		}
	}

	const fn size(self, sizer: &mut Sizer) {
		sizer.add(1);
		match self {
			Self::Symbol(symbol) => sizer.string(symbol),
			Self::Closure { .. } => sizer.add(size_of::<u64>()),
		}
	}
}

impl WireExportInput {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		encoder.string(self.name);
		encoder.slots(&self.ty.slots);
		match self.ty.conversion {
			Some(conversion) => {
				encoder.u8(1);
				conversion.encode(encoder);
			}
			None => encoder.u8(0),
		}
	}

	const fn size(self, sizer: &mut Sizer) {
		sizer.string(self.name);
		sizer.slots(&self.ty.slots);
		sizer.add(1);
		if let Some(conversion) = self.ty.conversion {
			conversion.size(sizer);
		}
	}
}

impl WireExportOutputType {
	const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.u8(flag(self.mode.is_direct(), EXPORT_OUTPUT_DIRECT)
			| flag(self.conversion.is_result(), EXPORT_OUTPUT_RESULT));
		encoder.slots(&self.slots);
		match self.conversion.conversion() {
			Some(conversion) => {
				encoder.u8(1);
				conversion.encode(encoder);
			}
			None => encoder.u8(0),
		}
		encoder.u64(self.frame_size as u64);
		let mut index = 0;
		while index < self.slot_offsets.len() {
			encoder.u64(self.slot_offsets[index] as u64);
			index += 1;
		}
		if let Some(result) = self.result {
			encoder.u8(result.discriminant);
			encoder.u8(result.error);
		}
	}

	const fn size(&self, sizer: &mut Sizer) {
		sizer.add(1);
		sizer.slots(&self.slots);
		sizer.add(1);
		if let Some(conversion) = self.conversion.conversion() {
			conversion.size(sizer);
		}
		sizer.add(5 * size_of::<u64>());
		if self.result.is_some() {
			sizer.add(2);
		}
	}
}

impl WireExportOutput {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		self.ty.encode(encoder);
	}

	const fn size(self, sizer: &mut Sizer) {
		self.ty.size(sizer);
	}
}

impl WireExport {
	const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		encoder.string(self.module);
		encoder.string(self.name);
		let flags =
			flag(self.promising, EXPORT_PROMISING) | flag(self.output.is_some(), EXPORT_HAS_OUTPUT);
		encoder.u8(flags);
		self.callee.encode(encoder);
		encoder.count(self.inputs.len());
		let mut index = 0;
		while index < self.inputs.len() {
			self.inputs[index].encode(encoder);
			index += 1;
		}
		if let Some(output) = self.output {
			output.encode(encoder);
		}
	}

	const fn size(&self, sizer: &mut Sizer) {
		sizer.string(self.module);
		sizer.string(self.name);
		sizer.add(1);
		self.callee.size(sizer);
		sizer.add(size_of::<u32>());
		let mut index = 0;
		while index < self.inputs.len() {
			self.inputs[index].size(sizer);
			index += 1;
		}
		if let Some(output) = self.output {
			output.size(sizer);
		}
	}
}
