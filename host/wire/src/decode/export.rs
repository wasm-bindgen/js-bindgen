use alloc::vec::Vec;

use crate::{
	EXPORT_FLAGS, EXPORT_HAS_OUTPUT, EXPORT_OUTPUT_DIRECT, EXPORT_OUTPUT_FLAGS,
	EXPORT_OUTPUT_RESULT, EXPORT_PROMISING, Error, PointerWidth, SLOT_COUNT,
	model::{
		Callee, Embed, Export, ExportInput, ExportInputConversion, ExportInputKind, ExportOutput,
		FrameSlot, ResultLayout, ReturnFrame,
	},
};

use super::{Decode, Decoder, value};

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
		let callee = Callee::decode(decoder)?;

		let input_count = decoder.count("export input")?;
		let mut inputs = Vec::with_capacity(input_count);
		let mut embeds = Vec::new();
		for index in 0..input_count {
			let kind = if matches!(callee, Callee::Closure { .. }) && index == 0 {
				ExportInputKind::ClosureData
			} else {
				ExportInputKind::Value
			};
			let (input, embed) = ExportInput::decode_with(decoder, kind)?;
			inputs.push(input);
			embeds.extend(embed);
		}
		if matches!(callee, Callee::Closure { .. }) {
			decoder.ensure(
				inputs.first().is_some_and(|input| {
					input.slots.len() == 1 && input.slots[0].abi == pointer_width.wat_type()
				}),
				"closure export data must occupy one pointer slot",
			)?;
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

impl<'a> Decode<'a> for Callee<'a> {
	fn decode(decoder: &mut Decoder<'a>) -> Result<Self, Error> {
		match decoder.tag("export callee", 1)? {
			0 => {
				let name = decoder.string()?;
				decoder.ensure(!name.is_empty(), "export callee has an empty symbol name")?;
				Ok(Self::Symbol { name })
			}
			1 => Ok(Self::Closure {
				call_shim_offset: decoder.u64()?,
			}),
			_ => unreachable!(),
		}
	}
}

impl<'a> ExportInput<'a> {
	fn decode_with(
		decoder: &mut Decoder<'a>,
		kind: ExportInputKind,
	) -> Result<(Self, Option<Embed<'a>>), Error> {
		let name = decoder.string()?;
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
				name,
				slots,
				conversion,
			},
			embed,
		))
	}
}

impl<'a> ExportOutput<'a> {
	fn decode_with(decoder: &mut Decoder<'a>) -> Result<(Self, Option<Embed<'a>>), Error> {
		let flags = decoder.flags("export output", EXPORT_OUTPUT_FLAGS)?;
		let slots = value::slots(decoder)?;
		let conversion = value::optional_into_js_conversion(decoder)?;
		let (embed, js_conversion) = match conversion {
			Some((embed, template)) => (embed, Some(template)),
			None => (None, None),
		};
		let frame_size = decoder.u64()?;
		let slot_offsets = [
			decoder.u64()?,
			decoder.u64()?,
			decoder.u64()?,
			decoder.u64()?,
		];
		let direct = flags & EXPORT_OUTPUT_DIRECT != 0;
		let result = flags & EXPORT_OUTPUT_RESULT != 0;
		let result_layout = if result {
			Some(ResultLayout {
				discriminant: decoder.u8()?,
				error: decoder.u8()?,
			})
		} else {
			None
		};

		decoder.ensure(
			!direct || frame_size == 0 && slot_offsets == [0; SLOT_COUNT],
			"direct export output unexpectedly has a return frame",
		)?;
		decoder.ensure(
			direct || frame_size != 0,
			"indirect export output is missing its return frame",
		)?;
		let output = if direct {
			let slots = value::compact(&slots);
			decoder.ensure(slots.len() == 1, "direct export output must have one slot")?;
			decoder.ensure(!result, "direct export output cannot be a Result")?;
			ExportOutput::Direct {
				slot: slots[0].clone(),
				js_conversion,
			}
		} else {
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
