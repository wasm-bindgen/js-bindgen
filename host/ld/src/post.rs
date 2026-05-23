use anyhow::{Context, Result, bail};
use js_bindgen_cli_lib::{JS_OUTPUT_SECTION, MainMemory};
use js_bindgen_shared::IS_TEST_SECTION;
use wasm_encoder::{
	CustomSection, EntityType, ImportSection, Module, ProducersField, ProducersSection, RawSection,
	Section,
};
use wasmparser::{Encoding, KnownCustom, Parser, Payload, TypeRef};

use crate::js::JsStore;

/// This removes our custom sections and generates the JS import file.
pub fn processing(
	wasm_input: &[u8],
	main_memory: MainMemory<'_>,
	mut js_store: JsStore,
	is_test: bool,
) -> Result<Vec<u8>> {
	// Start building final Wasm and JS.
	let mut wasm_output = Vec::new();

	for payload in Parser::new(0).parse_all(wasm_input) {
		let payload = payload.context("object file should be valid Wasm")?;

		match payload {
			Payload::Version { encoding, .. } => wasm_output.extend_from_slice(match encoding {
				Encoding::Module => &Module::HEADER,
				Encoding::Component => {
					bail!("objects with components are not supported")
				}
			}),
			// Read what imports we need. This has already undergone dead-code elimination by LLD.
			Payload::ImportSection(i) => {
				let mut import_section = ImportSection::new();

				for i in i.into_imports() {
					let import = i.context("import should be parsable")?;

					import_section.import(
						import.module,
						import.name,
						EntityType::try_from(import.ty)
							.context("`wasmparser` type should be convertible")?,
					);

					// The main memory has its own dedicated JS output handling.
					if let TypeRef::Memory(_) = import.ty
						&& import.module == main_memory.module
						&& import.name == main_memory.name
					{
						continue;
					}

					js_store.add_import(import)?;
				}

				import_section.append_to(&mut wasm_output);
			}
			// Don't write back our own custom sections.
			Payload::CustomSection(c) if c.name() == "js_bindgen.wat" => (),
			Payload::CustomSection(c) if c.name() == "js_bindgen.import" => (),
			Payload::CustomSection(c) if c.name() == "js_bindgen.embed" => (),
			// Register ourselves in the producer section.
			Payload::CustomSection(c) if c.name() == "producers" => {
				let KnownCustom::Producers(c) = c.as_known() else {
					bail!("unexpected producer section encoding")
				};

				let mut section = ProducersSection::new();

				for f in c {
					let f = f?;
					let mut field = ProducersField::new();

					for value in f.values {
						let value = value?;
						field.value(value.name, value.version);
					}

					if f.name == "processed-by" {
						field.value("js-bindgen", env!("CARGO_PKG_VERSION"));
					}

					section.field(f.name, &field);
				}

				section.append_to(&mut wasm_output);
			}
			Payload::CodeSectionEntry(_) | Payload::End(_) => (),
			payload => {
				let (id, range) = payload
					.as_section()
					.with_context(|| format!("expected parsable Wasm payload:\n{payload:?}"))?;
				RawSection {
					id,
					data: &wasm_input[range],
				}
				.append_to(&mut wasm_output);
			}
		}
	}

	js_store.assert_expected()?;

	if is_test {
		CustomSection {
			name: IS_TEST_SECTION.into(),
			data: (&[]).into(),
		}
		.append_to(&mut wasm_output);
	}

	let output = js_store.into_output(main_memory);
	let data = postcard::to_allocvec(&output)?;

	CustomSection {
		name: JS_OUTPUT_SECTION.into(),
		data: data.into(),
	}
	.append_to(&mut wasm_output);

	Ok(wasm_output)
}
