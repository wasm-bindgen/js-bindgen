use std::io::Write;
use std::str;

use anyhow::{Context, Result, bail};
use itertools::{Itertools, Position};
use wasm_encoder::{
	EntityType, ImportSection, Module, ProducersField, ProducersSection, RawSection, Section,
};
use wasmparser::{Encoding, KnownCustom, Parser, Payload, TypeRef};

use crate::js::JsStore;
use crate::pre::MainMemory;

const IMPORTS_JS: &str = include_str!("js/imports.mjs");

/// This removes our custom sections and generates the JS import file.
pub fn processing(
	wasm_input: &[u8],
	mut js_output: impl Write,
	main_memory: MainMemory<'_>,
	mut js_store: JsStore,
) -> Result<Vec<u8>> {
	// Start building final Wasm and JS.
	let mut wasm_output = Vec::new();

	let mut memory = None;

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
					let mut import = i.context("import should be parsable")?;

					// This is `llvm-mc` workaround for 32-bit tables when compiling to Wasm64.
					// See https://github.com/llvm/llvm-project/issues/172907.
					// TODO: This linker is supposed to be agnostic towards `js-sys`.
					if let TypeRef::Table(t) = &mut import.ty
						&& t.table64 && import.module == "js_sys"
						&& import.name == "externref.table"
					{
						t.table64 = false;
					}

					import_section.import(
						import.module,
						import.name,
						EntityType::try_from(import.ty)
							.context("`wasmparser` type should be convertible")?,
					);

					// The main memory has its own dedicated JS output handling.
					if let TypeRef::Memory(m) = import.ty
						&& import.module == main_memory.module
						&& import.name == main_memory.name
					{
						memory = Some(m);
						continue;
					}

					js_store.add_import(import)?;
				}

				import_section.append_to(&mut wasm_output);
			}
			// Don't write back our own custom sections.
			Payload::CustomSection(c) if c.name() == "js_bindgen.assembly" => (),
			Payload::CustomSection(c) if c.name().starts_with("js_bindgen.import.") => (),
			Payload::CustomSection(c) if c.name().starts_with("js_bindgen.embed.") => (),
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

	let memory = memory.context("main memory should be present")?;
	js_store.assert_expected()?;

	let (js_memory, rest) = IMPORTS_JS.split_once("JBG_PLACEHOLDER_MEMORY").unwrap();
	let (js_embed, rest) = rest.split_once("JBG_PLACEHOLDER_JS_EMBED").unwrap();
	let (js_import_object, js_rest) = rest.split_once("JBG_PLACEHOLDER_IMPORT_OBJECT").unwrap();

	// `WebAssembly.Memory`.
	js_output.write_all(js_memory.as_bytes())?;

	js_output.write_all(b"new WebAssembly.Memory({ ")?;

	if memory.memory64 {
		write!(js_output, "initial: {}n", memory.initial)?;
	} else {
		write!(js_output, "initial: {}", memory.initial)?;
	}

	if let Some(max) = memory.maximum {
		if memory.memory64 {
			write!(js_output, ", maximum: {max}n")?;
		} else {
			write!(js_output, ", maximum: {max}")?;
		}
	}

	if memory.memory64 {
		js_output.write_all(b", address: 'i64'")?;
	}

	if memory.shared {
		js_output.write_all(b", shared: true")?;
	}

	js_output.write_all(b" })")?;

	// Requested embedded JS.
	js_output.write_all(js_embed.as_bytes())?;

	js_output.write_all(b"{\n")?;

	for (package, embeds) in js_store.js_embed() {
		writeln!(js_output, "\t\t\t{package}: {{")?;

		for (name, js) in embeds {
			write!(js_output, "\t\t\t\t'{name}': ")?;

			for (position, line) in js.lines().with_position() {
				js_output.write_all(line.as_bytes())?;

				if let Position::First | Position::Middle = position {
					js_output.write_all(b"\n\t\t\t\t")?;
				}
			}

			js_output.write_all(b",\n")?;
		}

		js_output.write_all(b"\t\t\t},\n")?;
	}

	js_output.write_all(b"\t\t}")?;

	// `importObject`
	js_output.write_all(js_import_object.as_bytes())?;

	js_output.write_all(b"{\n")?;
	js_output.write_all(b"\t\t\tjs_bindgen: { memory: this.#memory },\n")?;

	for (module, names) in js_store
		.js_import()
		.into_iter()
		.filter(|(_, names)| !names.values().all(Option::is_none))
	{
		writeln!(js_output, "\t\t\t{module}: {{")?;

		for (name, js) in names
			.into_iter()
			.filter_map(|(name, js)| js.as_ref().map(|js| (name, js)))
		{
			write!(js_output, "\t\t\t\t'{name}': ")?;

			for (position, line) in js.lines().with_position() {
				js_output.write_all(line.as_bytes())?;

				if let Position::First | Position::Middle = position {
					js_output.write_all(b"\n\t\t\t\t")?;
				}
			}

			js_output.write_all(b",\n")?;
		}

		js_output.write_all(b"\t\t\t},\n")?;
	}

	js_output.write_all(b"\t\t}")?;

	// Finish
	js_output.write_all(js_rest.as_bytes())?;

	Ok(wasm_output)
}
