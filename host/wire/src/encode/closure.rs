use core::mem::size_of;

use super::{Encoder, Sizer, flag};
use crate::{CLOSURE_HAS_OUTPUT, WireClosure, WireClosureFactory};

impl<const N: usize> Encoder<N> {
	pub(super) const fn closure(&mut self, closure: &WireClosure) {
		closure.encode(self);
	}
}

impl Sizer {
	pub(super) const fn closure(&mut self, closure: &WireClosure) {
		closure.size(self);
	}
}

impl WireClosureFactory {
	const fn encode<const N: usize>(self, encoder: &mut Encoder<N>) {
		encoder.string(self.raw_symbol);
		encoder.string(self.helper.module);
		encoder.string(self.helper.name);
		self.input.encode(encoder);
		self.output.encode(encoder);
	}

	const fn size(self, sizer: &mut Sizer) {
		sizer.string(self.raw_symbol);
		sizer.string(self.helper.module);
		sizer.string(self.helper.name);
		self.input.size(sizer);
		self.output.size(sizer);
	}
}

impl WireClosure {
	const fn encode<const N: usize>(&self, encoder: &mut Encoder<N>) {
		self.factory.encode(encoder);
		encoder.u8(flag(self.output.is_some(), CLOSURE_HAS_OUTPUT));
		encoder.u64(self.call_shim_offset as u64);
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
		self.factory.size(sizer);
		sizer.add(size_of::<u8>() + size_of::<u64>() + size_of::<u32>());
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
