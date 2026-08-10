use alloc::{rc::Rc, vec::Vec};

use crate::{
	Error, ErrorKind, SLOT_COUNT, WAT_IMPORT_FUNCTION, WAT_IMPORT_TABLE, WAT_IMPORT_TAG,
	abi::{RefType, WatIndexType, WatType},
	model::{Embed, Slot, WatConversion, WatImport, WatImportKind, WatLocal},
};

use super::{Decode, Decoder};

pub(super) type WireSlots<'a> = [Option<Slot<'a>>; SLOT_COUNT];

pub(super) struct FromJsConversion<'a> {
	pub embed: Option<Embed<'a>>,
	pub prepare: Option<&'a str>,
	pub templates: [Option<&'a str>; SLOT_COUNT],
}

#[derive(Clone, Copy)]
pub(super) enum Sret<'a> {
	Slots(&'a str),
	Value(&'a str),
}

impl<'de> Decode<'de> for Slot<'de> {
	fn decode(decoder: &mut Decoder<'de>) -> Result<Self, Error> {
		let abi = decode_wat_type(decoder)?;
		let wat = if decoder.boolean("slot WAT presence")? {
			Some(WatConversion {
				imports: decode_imports(decoder)?,
				locals: decode_locals(decoder)?,
				instruction: decoder.string()?,
				boundary: decode_wat_type(decoder)?,
			})
		} else {
			None
		};
		Ok(Self { abi, wat })
	}
}

impl<'de> Decode<'de> for WatImport<'de> {
	fn decode(decoder: &mut Decoder<'de>) -> Result<Self, Error> {
		let tag = decoder.tag("WAT import", WAT_IMPORT_TAG)?;
		let module = required_nonempty(decoder, "WAT import has an empty module")?;
		let name = required_nonempty(decoder, "WAT import has an empty name")?;
		let identifier = required_nonempty(decoder, "WAT import has an empty identifier")?;
		let symbol_name = decoder.optional_string()?;
		decoder.ensure(
			symbol_name.is_none_or(|symbol_name| !symbol_name.is_empty()),
			"WAT import has an empty symbol name",
		)?;
		let kind = match tag {
			WAT_IMPORT_FUNCTION => WatImportKind::Function {
				parameters: decode_types(decoder, "WAT function parameter")?,
				results: decode_types(decoder, "WAT function result")?,
			},
			WAT_IMPORT_TABLE => {
				let index_type = decode_index_type(decoder)?;
				let minimum = decoder.u64()?;
				let maximum = decoder.optional_u64("WAT table maximum presence")?;
				decoder.ensure(
					maximum.is_none_or(|maximum| maximum >= minimum),
					"WAT table import maximum is smaller than its minimum",
				)?;
				let element = decode_ref_type(decoder)?;
				WatImportKind::Table {
					index_type,
					minimum,
					maximum,
					element,
				}
			}
			WAT_IMPORT_TAG => WatImportKind::Tag {
				parameters: decode_types(decoder, "WAT tag parameter")?,
			},
			_ => unreachable!(),
		};
		Ok(Self {
			module,
			name,
			identifier,
			symbol_name,
			kind,
		})
	}
}

impl<'de> Decode<'de> for WatLocal<'de> {
	fn decode(decoder: &mut Decoder<'de>) -> Result<Self, Error> {
		Ok(Self {
			name: required_nonempty(decoder, "WAT local has an empty name")?,
			ty: decode_wat_type(decoder)?,
		})
	}
}

pub(super) fn decode_imports<'de>(
	decoder: &mut Decoder<'de>,
) -> Result<Rc<[WatImport<'de>]>, Error> {
	let count = decoder.count("WAT import")?;
	let mut imports = Vec::with_capacity(count);
	for _ in 0..count {
		imports.push(WatImport::decode(decoder)?);
	}
	Ok(imports.into())
}

pub(super) fn decode_locals<'de>(decoder: &mut Decoder<'de>) -> Result<Rc<[WatLocal<'de>]>, Error> {
	let count = decoder.count("WAT local")?;
	let mut locals = Vec::with_capacity(count);
	for _ in 0..count {
		locals.push(WatLocal::decode(decoder)?);
	}
	Ok(locals.into())
}

fn decode_types(
	decoder: &mut Decoder<'_>,
	error_context: &'static str,
) -> Result<Vec<WatType>, Error> {
	let count = decoder.count(error_context)?;
	let mut types = Vec::with_capacity(count);
	for _ in 0..count {
		types.push(decode_wat_type(decoder)?);
	}
	Ok(types)
}

fn decode_wat_type(decoder: &mut Decoder<'_>) -> Result<WatType, Error> {
	let tag = decoder.tag("WAT type", WatType::MAX_TAG)?;
	let Some(ty) = WatType::from_tag(tag) else {
		unreachable!();
	};
	Ok(ty)
}

fn decode_index_type(decoder: &mut Decoder<'_>) -> Result<WatIndexType, Error> {
	let tag = decoder.tag("WAT table index type", WatIndexType::MAX_TAG)?;
	let Some(ty) = WatIndexType::from_tag(tag) else {
		unreachable!();
	};
	Ok(ty)
}

fn decode_ref_type(decoder: &mut Decoder<'_>) -> Result<RefType, Error> {
	let tag = decoder.tag("WAT reference type", RefType::MAX_TAG)?;
	let Some(ty) = RefType::from_tag(tag) else {
		unreachable!();
	};
	Ok(ty)
}

fn required_nonempty<'de>(
	decoder: &mut Decoder<'de>,
	error_message: &'static str,
) -> Result<&'de str, Error> {
	let value = decoder.string()?;
	decoder.ensure(!value.is_empty(), error_message)?;
	Ok(value)
}

pub(super) fn optional_embed<'de>(decoder: &mut Decoder<'de>) -> Result<Option<Embed<'de>>, Error> {
	if decoder.boolean("embed presence")? {
		Ok(Some(Embed {
			module: decoder.string()?,
			name: decoder.string()?,
		}))
	} else {
		Ok(None)
	}
}

pub(super) fn optional_into_js_conversion<'de>(
	decoder: &mut Decoder<'de>,
) -> Result<Option<(Option<Embed<'de>>, &'de str)>, Error> {
	if !decoder.boolean("JavaScript conversion presence")? {
		return Ok(None);
	}
	Ok(Some((optional_embed(decoder)?, decoder.string()?)))
}

pub(super) fn optional_from_js_conversion<'de>(
	decoder: &mut Decoder<'de>,
) -> Result<Option<FromJsConversion<'de>>, Error> {
	if !decoder.boolean("JavaScript conversion presence")? {
		return Ok(None);
	}
	let embed = optional_embed(decoder)?;
	let prepare = decoder.optional_string()?;
	let templates = templates(decoder)?;
	Ok(Some(FromJsConversion {
		embed,
		prepare,
		templates,
	}))
}

pub(super) fn sret<'de>(decoder: &mut Decoder<'de>) -> Result<Sret<'de>, Error> {
	Ok(match decoder.tag("JavaScript return writer", 1)? {
		0 => Sret::Slots(decoder.string()?),
		1 => Sret::Value(decoder.string()?),
		_ => unreachable!(),
	})
}

pub(super) fn slots<'de>(decoder: &mut Decoder<'de>) -> Result<WireSlots<'de>, Error> {
	let offset = decoder.position();
	let mask = decoder.u8()?;
	let allowed = (1 << SLOT_COUNT) - 1;
	if mask & !allowed != 0 {
		return Err(Error::new(
			offset,
			ErrorKind::InvalidValue("invalid wire slot mask"),
		));
	}
	let mut slots = [const { None }; SLOT_COUNT];
	let mut index = 0;
	while index < slots.len() {
		if mask & (1 << index) != 0 {
			slots[index] = Some(Slot::decode(decoder)?);
		}
		index += 1;
	}
	Ok(slots)
}

pub(super) fn templates<'de>(
	decoder: &mut Decoder<'de>,
) -> Result<[Option<&'de str>; SLOT_COUNT], Error> {
	let offset = decoder.position();
	let mask = decoder.u8()?;
	let allowed = (1 << SLOT_COUNT) - 1;
	if mask & !allowed != 0 {
		return Err(Error::new(
			offset,
			ErrorKind::InvalidValue("invalid wire template mask"),
		));
	}
	let mut templates = [None; SLOT_COUNT];
	let mut index = 0;
	while index < templates.len() {
		if mask & (1 << index) != 0 {
			templates[index] = Some(decoder.string()?);
		}
		index += 1;
	}
	Ok(templates)
}

pub(super) fn compact<'a>(slots: &WireSlots<'a>) -> Vec<Slot<'a>> {
	slots.iter().flatten().cloned().collect()
}

pub(super) fn compact_abi<'a>(
	decoder: &Decoder<'_>,
	slots: &WireSlots<'a>,
) -> Result<Vec<Slot<'a>>, Error> {
	validate_abi(decoder, slots)?;
	Ok(compact(slots))
}

pub(super) fn validate_abi(decoder: &Decoder<'_>, slots: &WireSlots<'_>) -> Result<(), Error> {
	let mut empty = false;
	for slot in slots {
		if slot.is_some() {
			decoder.ensure(!empty, "wire ABI has a populated slot after an empty slot")?;
		} else {
			empty = true;
		}
	}
	Ok(())
}

pub(super) fn validate_templates(
	decoder: &Decoder<'_>,
	slots: &WireSlots<'_>,
	templates: &[Option<&str>; SLOT_COUNT],
) -> Result<(), Error> {
	for (slot, template) in slots.iter().zip(templates) {
		decoder.ensure(
			slot.is_some() == template.is_some(),
			"JavaScript conversion does not match its populated slots",
		)?;
	}
	Ok(())
}

pub(super) fn table_entry<'a, T>(
	decoder: &Decoder<'_>,
	table: &'a [T],
	index: u32,
	name: &'static str,
) -> Result<&'a T, Error> {
	table.get(index as usize).ok_or_else(|| {
		Error::new(
			decoder.position(),
			ErrorKind::IndexOutOfBounds {
				table: name,
				index,
				len: table.len(),
			},
		)
	})
}
