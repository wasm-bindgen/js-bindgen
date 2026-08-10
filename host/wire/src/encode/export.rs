use core::mem::size_of;

use super::{Encoder, Sizer, flag};
use crate::abi::WatSlot;
use crate::schema::WireExportOutputKind;
use crate::{
	EXPORT_HAS_OUTPUT, EXPORT_OUTPUT_DIRECT, EXPORT_OUTPUT_RESULT, EXPORT_PROMISING, WireExport,
	WireExportInput, WireExportOutput, WireExportOutputType, WireReturnFrame,
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

impl WireExportInput {
	pub(super) const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		encoder.slots(&self.ty.slots);
		match self.ty.conversion {
			Some(conversion) => {
				encoder.u8(1);
				conversion.encode(encoder);
			}
			None => encoder.u8(0),
		}
	}

	pub(super) const fn size(self, sizer: &mut Sizer) {
		sizer.slots(&self.ty.slots);
		sizer.add(1);
		if let Some(conversion) = self.ty.conversion {
			conversion.size(sizer);
		}
	}
}

impl WireExportOutputType {
	const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		match self.kind {
			WireExportOutputKind::Direct { slots, conversion } => {
				encoder.u8(EXPORT_OUTPUT_DIRECT);
				encoder.slots(&slots);
				match conversion {
					Some(conversion) => {
						encoder.u8(1);
						conversion.encode(encoder);
					}
					None => encoder.u8(0),
				}
			}
			WireExportOutputKind::Indirect {
				slots,
				conversion,
				frame,
				result,
			} => {
				encoder.u8(flag(conversion.is_result(), EXPORT_OUTPUT_RESULT));
				encoder.slots(&slots);
				match conversion.conversion() {
					Some(conversion) => {
						encoder.u8(1);
						conversion.encode(encoder);
					}
					None => encoder.u8(0),
				}
				frame.encode(encoder, &slots);
				if let Some(result) = result {
					encoder.u8(result.discriminant);
					encoder.u8(result.error);
				}
			}
		}
	}

	const fn size(&self, sizer: &mut Sizer) {
		match self.kind {
			WireExportOutputKind::Direct { slots, conversion } => {
				sizer.add(1);
				sizer.slots(&slots);
				sizer.add(1);
				if let Some(conversion) = conversion {
					conversion.size(sizer);
				}
			}
			WireExportOutputKind::Indirect {
				slots,
				conversion,
				result,
				..
			} => {
				sizer.add(1);
				sizer.slots(&slots);
				sizer.add(1);
				if let Some(conversion) = conversion.conversion() {
					conversion.size(sizer);
				}
				sizer.add(size_of::<u64>());
				let mut index = 0;
				while index < slots.len() {
					if slots[index].is_some() {
						sizer.add(size_of::<u64>());
					}
					index += 1;
				}
				if result.is_some() {
					sizer.add(2);
				}
			}
		}
	}
}

impl WireReturnFrame {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>, slots: &[Option<WatSlot>; 4]) {
		encoder.u64(self.size as u64);
		let mut index = 0;
		while index < slots.len() {
			if slots[index].is_some() {
				encoder.u64(self.slot_offsets[index] as u64);
			}
			index += 1;
		}
	}
}

impl WireExportOutput {
	pub(super) const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		self.ty.encode(encoder);
	}

	pub(super) const fn size(self, sizer: &mut Sizer) {
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
		encoder.string(self.symbol);
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
		sizer.string(self.symbol);
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
