use alloc::{rc::Rc, vec::Vec};

use crate::{
	Error, IMPORT_CATCH_JAVASCRIPT, IMPORT_CATCH_WASM, IMPORT_CLOSURE_FACTORY, IMPORT_FLAGS,
	IMPORT_HAS_BINDING, IMPORT_HAS_OUTPUT, IMPORT_OUTPUT_DIRECT, IMPORT_OUTPUT_FLAGS,
	IMPORT_OUTPUT_RESULT, IMPORT_SUSPENDING, PointerWidth, WireImportKind,
	abi::WatType,
	model::{
		DirectImportConversion, Embed, Import, ImportBinding, ImportCatch, ImportErrorMode,
		ImportGroup, ImportInput, ImportOutput, ImportOutputAbi, ImportRetptr, ImportWriter,
		JsCatch, Slot, WatCatch,
	},
};

use super::{Decode, Decoder, value};

struct InputType<'a> {
	slots: Vec<Slot<'a>>,
	js_conversion: Option<&'a str>,
	embed: Option<Embed<'a>>,
}

struct OutputType<'a> {
	abi: ImportOutputAbi<'a>,
	embeds: [Option<Embed<'a>>; 2],
	result: bool,
}

pub(super) fn decode<'a>(
	decoder: &mut Decoder<'a>,
	pointer_width: PointerWidth,
) -> Result<ImportGroup<'a>, Error> {
	let input_type_count = decoder.count("import input type")?;
	let mut input_types = Vec::with_capacity(input_type_count);
	for _ in 0..input_type_count {
		input_types.push(InputType::decode(decoder)?);
	}

	let output_type_count = decoder.count("import output type")?;
	let pointer_type = if output_type_count == 0 {
		None
	} else {
		let pointer = InputType::decode(decoder)?;
		decoder.ensure(
			pointer.slots.len() == 1 && pointer.slots[0].abi == pointer_width.wat_type(),
			"import return pointer does not match the target pointer width",
		)?;
		Some(pointer)
	};
	let mut output_types = Vec::with_capacity(output_type_count);
	for _ in 0..output_type_count {
		let Some(pointer) = pointer_type.as_ref() else {
			return decoder.invalid("a non-empty output table has no pointer type");
		};
		output_types.push(OutputType::decode_with(decoder, pointer)?);
	}
	let catch = if output_types.iter().any(|output| output.result) {
		Some(decode_catch(decoder)?)
	} else {
		None
	};

	let import_count = decoder.count("import")?;
	let mut imports = Vec::with_capacity(import_count);
	for _ in 0..import_count {
		imports.push(Import::decode_with(
			decoder,
			&input_types,
			&output_types,
			catch.as_ref(),
		)?);
	}
	Ok(ImportGroup { catch, imports })
}

fn decode_catch<'a>(decoder: &mut Decoder<'a>) -> Result<ImportCatch<'a>, Error> {
	Ok(match decoder.tag("import catch", IMPORT_CATCH_WASM)? {
		IMPORT_CATCH_JAVASCRIPT => {
			let embed_count = decoder.count("import catch embed")?;
			let mut embeds = Vec::with_capacity(embed_count);
			for _ in 0..embed_count {
				embeds.push(Embed {
					module: decoder.string()?,
					name: decoder.string()?,
				});
			}
			let direct = decoder.string()?;
			let indirect = decoder.string()?;
			decoder.ensure(!direct.is_empty(), "direct import catch template is empty")?;
			decoder.ensure(
				!indirect.is_empty(),
				"indirect import catch template is empty",
			)?;
			ImportCatch::JavaScript(JsCatch {
				embeds: Rc::from(embeds),
				direct,
				indirect,
			})
		}
		IMPORT_CATCH_WASM => {
			let imports = value::decode_imports(decoder)?;
			let locals = value::decode_locals(decoder)?;
			let try_ = decoder.string()?;
			let catch = decoder.string()?;
			decoder.ensure(!try_.is_empty(), "WAT import try template is empty")?;
			decoder.ensure(!catch.is_empty(), "WAT import catch template is empty")?;
			ImportCatch::Wasm(WatCatch {
				imports,
				locals,
				try_,
				catch,
			})
		}
		_ => unreachable!(),
	})
}

impl<'a> Decode<'a> for InputType<'a> {
	fn decode(decoder: &mut Decoder<'a>) -> Result<Self, Error> {
		let slots = value::slots(decoder)?;
		let slots = value::compact_abi(decoder, &slots)?;
		let conversion = value::optional_into_js_conversion(decoder)?;
		let (embed, js_conversion) = match conversion {
			Some((embed, template)) => (embed, Some(template)),
			None => (None, None),
		};
		decoder.ensure(
			js_conversion.is_some() || slots.len() <= 1,
			"multi-slot import input has no JavaScript conversion",
		)?;
		Ok(Self {
			slots,
			js_conversion,
			embed,
		})
	}
}

impl<'a> OutputType<'a> {
	fn decode_with(decoder: &mut Decoder<'a>, pointer: &InputType<'a>) -> Result<Self, Error> {
		let flags = decoder.flags("import output", IMPORT_OUTPUT_FLAGS)?;

		let direct = flags & IMPORT_OUTPUT_DIRECT != 0;
		let result = flags & IMPORT_OUTPUT_RESULT != 0;
		let slots = value::slots(decoder)?;
		value::validate_abi(decoder, &slots)?;
		let conversion = value::optional_from_js_conversion(decoder)?;
		let has_conversion = conversion.is_some();
		if let Some(conversion) = &conversion {
			value::validate_templates(decoder, &slots, &conversion.templates)?;
		}
		let sret = if direct {
			None
		} else {
			Some(value::sret(decoder)?)
		};
		let (embed, prepare, templates, writer) = match conversion {
			Some(conversion) => {
				let writer = match sret {
					Some(value::Sret::Slots(function)) => Some(ImportWriter::Slots {
						function,
						prepare: conversion.prepare,
						expressions: conversion.templates.iter().copied().flatten().collect(),
					}),
					Some(value::Sret::Value(function)) => Some(ImportWriter::Value { function }),
					None => None,
				};
				(
					conversion.embed,
					conversion.prepare,
					conversion.templates,
					writer,
				)
			}
			None => (None, None, [None; 4], None),
		};
		decoder.ensure(
			has_conversion || sret.is_none(),
			"import output has a writer without a JavaScript conversion",
		)?;
		decoder.ensure(
			pointer.slots.len() == 1,
			"import return pointer must have one slot",
		)?;
		let abi = if direct {
			let slots = value::compact(&slots);
			decoder.ensure(slots.len() == 1, "direct import output must have one slot")?;
			if result {
				decoder.ensure(
					matches!(
						slots[0].abi,
						WatType::I32 | WatType::I64 | WatType::F32 | WatType::F64
					),
					"unsupported direct Result return slot",
				)?;
			}
			decoder.ensure(writer.is_none(), "direct import output has an sret writer")?;
			ImportOutputAbi::Direct {
				slot: slots[0].clone(),
				conversion: if let Some(expression) = templates[0] {
					Some(DirectImportConversion {
						prepare,
						expression,
					})
				} else {
					decoder.ensure(
						prepare.is_none(),
						"import output prepares a missing conversion",
					)?;
					None
				},
			}
		} else {
			decoder.ensure(
				templates[0].is_some(),
				"indirect import output has no conversion",
			)?;
			let Some(writer) = writer else {
				return decoder.invalid("indirect import output has no sret writer");
			};
			ImportOutputAbi::Indirect {
				retptr: ImportRetptr {
					slot: pointer.slots[0].clone(),
					js_conversion: pointer.js_conversion,
				},
				writer,
			}
		};

		let pointer_embed = if direct { None } else { pointer.embed };
		Ok(Self {
			abi,
			embeds: [pointer_embed, embed],
			result,
		})
	}
}

impl<'a> Import<'a> {
	fn decode_with(
		decoder: &mut Decoder<'a>,
		input_types: &[InputType<'a>],
		output_types: &[OutputType<'a>],
		catch: Option<&ImportCatch<'a>>,
	) -> Result<Self, Error> {
		let module = decoder.string()?;
		let name = decoder.string()?;
		let flags = decoder.flags("import", IMPORT_FLAGS)?;
		let kind = if flags & IMPORT_CLOSURE_FACTORY == 0 {
			WireImportKind::Normal
		} else {
			WireImportKind::ClosureFactory
		};

		let input_count = decoder.count("import input")?;
		let mut inputs = Vec::with_capacity(input_count);
		let mut conversion_embeds = Vec::new();
		for _ in 0..input_count {
			let name = decoder.string()?;
			let index = decoder.u32()?;
			let ty = value::table_entry(decoder, input_types, index, "import input type")?;
			conversion_embeds.extend(ty.embed);
			inputs.push(ImportInput::from_type(name, ty));
		}

		let output = if flags & IMPORT_HAS_OUTPUT != 0 {
			let index = decoder.u32()?;
			let ty = value::table_entry(decoder, output_types, index, "import output type")?;
			conversion_embeds.extend(ty.embeds.iter().flatten().copied());
			let error = if ty.result {
				match catch {
					Some(ImportCatch::JavaScript(_)) => ImportErrorMode::CatchInJavaScript,
					Some(ImportCatch::Wasm(_)) => ImportErrorMode::CatchInWasm,
					None => return decoder.invalid("Result import has no catch metadata"),
				}
			} else {
				ImportErrorMode::Infallible
			};
			Some(ImportOutput {
				abi: ty.abi.clone(),
				error,
			})
		} else {
			None
		};

		let binding = if flags & IMPORT_HAS_BINDING != 0 {
			let mut binding = ImportBinding::decode(decoder)?;
			conversion_embeds.append(&mut binding.embeds);
			binding.embeds = conversion_embeds;
			Some(binding)
		} else {
			decoder.ensure(
				conversion_embeds.is_empty(),
				"an import without a JavaScript binding requires an embed",
			)?;
			None
		};
		let suspending = flags & IMPORT_SUSPENDING != 0;
		decoder.ensure(
			output
				.as_ref()
				.is_none_or(|output| output.error != ImportErrorMode::CatchInJavaScript)
				|| binding.is_some(),
			"a JavaScript-catching Result import has no binding",
		)?;
		decoder.ensure(
			!suspending || binding.is_some(),
			"suspending import has no JavaScript binding",
		)?;
		decoder.ensure(
			!suspending
				|| output
					.as_ref()
					.is_none_or(|output| output.error != ImportErrorMode::CatchInJavaScript),
			"suspending Result imports require exception handling",
		)?;

		Ok(Self {
			module,
			name,
			kind,
			inputs,
			output,
			binding,
			suspending,
		})
	}
}

impl<'a> ImportInput<'a> {
	fn from_type(name: &'a str, ty: &InputType<'a>) -> Self {
		Self {
			name,
			slots: ty.slots.clone(),
			js_conversion: ty.js_conversion,
		}
	}
}

impl<'a> Decode<'a> for ImportBinding<'a> {
	fn decode(decoder: &mut Decoder<'a>) -> Result<Self, Error> {
		let direct_expression = decoder.optional_string()?;
		let call_expression = decoder.string()?;
		let embed_count = decoder.count("import embed")?;
		let mut embeds = Vec::with_capacity(embed_count);
		for _ in 0..embed_count {
			embeds.push(Embed {
				module: decoder.string()?,
				name: decoder.string()?,
			});
		}
		Ok(Self {
			direct_expression,
			call_expression,
			embeds,
		})
	}
}
