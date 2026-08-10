use alloc::vec::Vec;

use super::import::{InputType, OutputType};
use super::{Decode, Decoder};
use crate::abi::WatType;
use crate::model::{
	Closure, ClosureFactory, Embed, ExportInput, ExportInputKind, ExportOutput, ImportErrorMode,
	ImportInput, ImportOutput, ImportOutputAbi,
};
use crate::{CLOSURE_FLAGS, CLOSURE_HAS_OUTPUT, Error, PointerWidth};

pub(super) fn decode<'a>(
	decoder: &mut Decoder<'a>,
	pointer_width: PointerWidth,
) -> Result<Closure<'a>, Error> {
	Closure::decode_with(decoder, pointer_width)
}

impl<'a> Closure<'a> {
	fn decode_with(decoder: &mut Decoder<'a>, pointer_width: PointerWidth) -> Result<Self, Error> {
		let factory = ClosureFactory::decode_with(decoder, pointer_width)?;

		let call_identity_start = decoder.position();
		let flags = decoder.flags("closure", CLOSURE_FLAGS)?;
		let call_shim_offset = decoder.u64()?;

		let input_count = decoder.count("closure dispatcher input")?;
		let mut inputs = Vec::with_capacity(input_count);
		let mut embeds = Vec::new();
		for index in 0..input_count {
			let kind = if index == 0 {
				ExportInputKind::ClosureData
			} else {
				ExportInputKind::Value
			};
			let (input, embed) = ExportInput::decode_with(decoder, kind)?;
			inputs.push(input);
			embeds.extend(embed);
		}
		decoder.ensure(
			inputs.first().is_some_and(|input| {
				input.slots.len() == 1 && input.slots[0].rust == pointer_width.wat_type()
			}),
			"closure dispatcher data must occupy one native pointer slot",
		)?;
		decoder.ensure(
			factory.input.slots[0].js() == inputs[0].slots[0].js(),
			"closure factory and dispatcher data boundary types differ",
		)?;

		let output = if flags & CLOSURE_HAS_OUTPUT != 0 {
			let (output, embed) = ExportOutput::decode_with(decoder)?;
			embeds.extend(embed);
			Some(output)
		} else {
			None
		};
		let call_identity = decoder.raw_from(call_identity_start)?;

		Ok(Self {
			factory,
			call_identity,
			pointer_width,
			inputs,
			output,
			embeds,
			call_shim_offset,
		})
	}
}

impl<'a> ClosureFactory<'a> {
	fn decode_with(decoder: &mut Decoder<'a>, pointer_width: PointerWidth) -> Result<Self, Error> {
		let raw_symbol = decoder.string()?;
		decoder.ensure(
			!raw_symbol.is_empty(),
			"closure factory raw symbol is empty",
		)?;
		let helper = Embed {
			module: decoder.string()?,
			name: decoder.string()?,
		};

		let input_type = InputType::decode(decoder)?;
		decoder.ensure(
			input_type.slots.len() == 1 && input_type.slots[0].rust == pointer_width.wat_type(),
			"closure factory data must occupy one native pointer slot",
		)?;
		let input = ImportInput::from_type(&input_type);

		let output_type = OutputType::decode_with(decoder, None)?;
		decoder.ensure(
			!output_type.result,
			"closure factory output cannot be a Result",
		)?;
		decoder.ensure(
			matches!(
				&output_type.abi,
				ImportOutputAbi::Direct {
					slot,
					conversion: None,
				} if slot.rust == WatType::I32 && slot.js() == WatType::ExternRef
			),
			"closure factory output must be a direct, infallible JsValue",
		)?;

		let mut embeds = Vec::new();
		embeds.extend(input_type.embed);
		embeds.extend(output_type.embeds.into_iter().flatten());
		Ok(Self {
			raw_symbol,
			helper,
			input,
			output: ImportOutput {
				abi: output_type.abi,
				error: ImportErrorMode::Infallible,
			},
			embeds,
		})
	}
}
