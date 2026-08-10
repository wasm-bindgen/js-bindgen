use alloc::vec::Vec;

use super::{Decoder, value};
use crate::model::{
	Callee, Embed, Export, ExportInput, ExportInputConversion, ExportInputKind, ExportOutput,
	FrameSlot, ResultLayout, ReturnFrame,
};
use crate::{
	EXPORT_FLAGS, EXPORT_HAS_OUTPUT, EXPORT_OUTPUT_DIRECT, EXPORT_OUTPUT_FLAGS,
	EXPORT_OUTPUT_RESULT, EXPORT_PROMISING, Error, PointerWidth,
};

pub(super) fn decode<'a>(
	decoder: &mut Decoder<'a>,
	pointer_width: PointerWidth,
) -> Result<Vec<Export<'a>>, Error> {
	let count = decoder.count("export")?;
	let mut exports = Vec::with_capacity(count);
	for _ in 0..count {
		exports.push(Export::decode_with(decoder, pointer_width)?);
	}
	Ok(exports)
}

impl<'a> Export<'a> {
	fn decode_with(decoder: &mut Decoder<'a>, pointer_width: PointerWidth) -> Result<Self, Error> {
		let module = decoder.string()?;
		let name = decoder.string()?;
		let flags = decoder.flags("export", EXPORT_FLAGS)?;
		let symbol = decoder.string()?;
		decoder.ensure(!symbol.is_empty(), "export callee has an empty symbol name")?;
		let callee = Callee::Symbol { name: symbol };

		let input_count = decoder.count("export input")?;
		let mut inputs = Vec::with_capacity(input_count);
		let mut embeds = Vec::new();
		for _ in 0..input_count {
			let (input, embed) = ExportInput::decode_with(decoder, ExportInputKind::Value)?;
			inputs.push(input);
			embeds.extend(embed);
		}

		let output = if flags & EXPORT_HAS_OUTPUT != 0 {
			let (output, embed) = ExportOutput::decode_with(decoder)?;
			embeds.extend(embed);
			Some(output)
		} else {
			None
		};

		Ok(Self {
			module,
			name,
			pointer_width,
			inputs,
			output,
			embeds,
			promising: flags & EXPORT_PROMISING != 0,
			callee,
		})
	}
}

impl<'a> ExportInput<'a> {
	pub(super) fn decode_with(
		decoder: &mut Decoder<'a>,
		kind: ExportInputKind,
	) -> Result<(Self, Option<Embed<'a>>), Error> {
		let slots = value::slots(decoder)?;
		let conversion = value::optional_from_js_conversion(decoder)?;
		let (conversion, embed) = if let Some(conversion) = conversion {
			let embed = conversion.embed;
			value::validate_templates(decoder, &slots, &conversion.templates)?;
			let conversion = Some(ExportInputConversion {
				prepare: conversion.prepare,
				expressions: conversion.templates.into_iter().flatten().collect(),
			});
			(conversion, embed)
		} else {
			(None, None)
		};
		let slots = value::compact_abi(decoder, &slots)?;
		decoder.ensure(
			conversion.is_some() || slots.len() <= 1,
			"multi-slot export input has no JavaScript conversion",
		)?;
		Ok((
			Self {
				kind,
				slots,
				conversion,
			},
			embed,
		))
	}
}

impl<'a> ExportOutput<'a> {
	pub(super) fn decode_with(
		decoder: &mut Decoder<'a>,
	) -> Result<(Self, Option<Embed<'a>>), Error> {
		let flags = decoder.flags("export output", EXPORT_OUTPUT_FLAGS)?;
		let slots = value::slots(decoder)?;
		let conversion = value::optional_into_js_conversion(decoder)?;
		let (embed, js_conversion) = match conversion {
			Some((embed, template)) => (embed, Some(template)),
			None => (None, None),
		};
		let direct = flags & EXPORT_OUTPUT_DIRECT != 0;
		let result = flags & EXPORT_OUTPUT_RESULT != 0;
		let output = if direct {
			let slots = value::compact(&slots);
			decoder.ensure(
				slots.len() <= 1,
				"direct export output has more than one slot",
			)?;
			decoder.ensure(!result, "direct export output cannot be a Result")?;
			decoder.ensure(
				!slots.is_empty() || js_conversion.is_none(),
				"zero-slot export output cannot have a JavaScript conversion",
			)?;
			ExportOutput::Direct {
				slot: slots.into_iter().next(),
				js_conversion,
			}
		} else {
			let frame_size = decoder.u64()?;
			decoder.ensure(
				frame_size != 0,
				"indirect export output is missing its return frame",
			)?;
			let mut slot_offsets = [0; 4];
			// The slot mask determines which offsets are present in the record.
			for (slot, offset) in slots.iter().zip(&mut slot_offsets) {
				if slot.is_some() {
					*offset = decoder.u64()?;
				}
			}
			let result_layout = if result {
				Some(ResultLayout {
					discriminant: decoder.u8()?,
					error: decoder.u8()?,
				})
			} else {
				None
			};
			// Result control slots can follow empty value slots, as in `Result<()>`,
			// so export outputs cannot require a contiguous `ABI` prefix.
			let frame_slots: Vec<_> = slots
				.into_iter()
				.zip(slot_offsets)
				.filter_map(|(slot, offset)| slot.map(|slot| FrameSlot { slot, offset }))
				.collect();
			let result = if let Some(result) = result_layout {
				let discriminant = usize::from(result.discriminant);
				let error = usize::from(result.error);
				decoder.ensure(
					error == discriminant + 1 && error + 1 == frame_slots.len(),
					"Result export control slots must terminate the return frame",
				)?;
				Some(result)
			} else {
				None
			};
			ExportOutput::Indirect {
				frame: ReturnFrame {
					size: frame_size,
					slots: frame_slots,
				},
				js_conversion,
				result,
			}
		};
		Ok((output, embed))
	}
}
