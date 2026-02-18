use std::io::Write;
use std::str;

use anyhow::{Context, Result, bail, ensure};
use foldhash::fast::FixedState;
use hashbrown::{HashMap, HashSet};
use itertools::{Itertools, Position};
use js_bindgen_ld_shared::{JsBindgenEmbedSectionParser, JsBindgenImportSectionParser};
use wasm_encoder::{
	EntityType, ImportSection, Module, ProducersField, ProducersSection, RawSection, Section,
};
use wasmparser::{CustomSectionReader, Encoding, Import, KnownCustom, Parser, Payload, TypeRef};

const IMPORTS_JS: &str = include_str!("js/imports.mjs");

#[derive(Clone, Copy)]
pub struct MainMemory<'a> {
	pub module: &'a str,
	pub name: &'a str,
}

/// This removes our custom sections and generates the JS import file.
pub fn processing(
	wasm_input: &[u8],
	mut js_output: impl Write,
	main_memory: MainMemory<'_>,
) -> Result<Vec<u8>> {
	// Start building final Wasm and JS.
	let mut wasm_output = Vec::new();

	let mut js_store = JsStore::default();
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
			// Don't write back our assembly sections.
			Payload::CustomSection(c) if c.name() == "js_bindgen.assembly" => (),
			// Extract all JS imports.
			Payload::CustomSection(c) if c.name().starts_with("js_bindgen.import.") => {
				let stripped = c.name().strip_prefix("js_bindgen.import.").unwrap();
				let (module, name) = stripped.split_once('.').with_context(|| {
					format!("found incorrectly formatted JS import custom section name: {stripped}")
				})?;

				js_store.add_js_import(module, name, &c)?;
			}
			// Extract all JS embeds.
			Payload::CustomSection(c) if c.name().starts_with("js_bindgen.embed.") => {
				let stripped = c.name().strip_prefix("js_bindgen.embed.").unwrap();
				let (module, name) = stripped.split_once('.').with_context(|| {
					format!("found incorrectly formatted JS import custom section name: {stripped}",)
				})?;

				js_store.add_js_embed(module, name, &c)?;
			}
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
			.filter_map(|(name, js)| js.map(|js| (name, js)))
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

type FixedHashMap<K, V> = HashMap<K, V, FixedState>;

#[derive(Default)]
struct JsStore<'a> {
	import: FixedHashMap<&'a str, FixedHashMap<&'a str, Option<&'a str>>>,
	expected_import: HashMap<&'a str, HashSet<&'a str>>,
	provided_import: HashMap<&'a str, HashMap<&'a str, Option<JsWithEmbed<'a>>>>,
	embed: FixedHashMap<&'a str, FixedHashMap<&'a str, &'a str>>,
	expected_embed: HashMap<&'a str, HashSet<&'a str>>,
	provided_embed: HashMap<&'a str, HashMap<&'a str, JsWithEmbed<'a>>>,
}

#[derive(Clone, Copy)]
struct JsWithEmbed<'a> {
	js: &'a str,
	embed: Option<&'a str>,
}

impl<'a> JsStore<'a> {
	fn add_import(&mut self, import: Import<'a>) -> Result<()> {
		if let Some(js) = self
			.provided_import
			.get_mut(import.module)
			.and_then(|names| names.remove(import.name))
		{
			self.import
				.entry(import.module)
				.or_default()
				.insert(import.name, js.map(|js| js.js));

			if let Some(embed) = js.and_then(|js| js.embed) {
				self.require_js_embed(import.module, embed);
			}
		} else if !self
			.expected_import
			.entry(import.module)
			.or_default()
			.insert(import.name)
		{
			bail!(
				"found duplicate JS import: `{}:{}`",
				import.module,
				import.name
			);
		}

		Ok(())
	}

	fn add_js_import(
		&mut self,
		module: &'a str,
		name: &'a str,
		custom_section: &CustomSectionReader<'a>,
	) -> Result<()> {
		let mut parser = JsBindgenImportSectionParser::new(custom_section);
		let import = parser
			.next()
			.with_context(|| format!("found no JS import for `{module}:{name}`"))?;

		if let Some(import_new) = parser.next() {
			bail!(
				"found multiple JS imports for `{module}:{name}`\n\tJS Import 1:\n{:?}\n\tJS \
				 Import 2:\n{:?}",
				import.js(),
				import_new.js(),
			);
		}

		if self
			.expected_import
			.get_mut(module)
			.is_some_and(|names| names.remove(name))
		{
			self.import
				.entry(module)
				.or_default()
				.insert(name, import.js());

			if let Some(embed) = import.embed() {
				self.require_js_embed(module, embed);
			}
		} else if let Some(import_old) = self.provided_import.entry_ref(module).or_default().insert(
			name,
			import.js().map(|js| JsWithEmbed {
				js,
				embed: import.embed(),
			}),
		) {
			bail!(
				"found multiple JS imports for `{module}:{name}`\n\tJS Import 1:\n{:?}\n\tJS \
				 Import 2:\n{:?}",
				import_old.map(|js| js.js),
				import.js()
			);
		}

		Ok(())
	}

	fn add_js_embed(
		&mut self,
		module: &'a str,
		name: &'a str,
		custom_section: &CustomSectionReader<'a>,
	) -> Result<()> {
		let mut parser = JsBindgenEmbedSectionParser::new(custom_section);
		let embed = parser
			.next()
			.with_context(|| format!("found no JS embed for `{module}:{name}`"))?;

		if let Some(embed_new) = parser.next() {
			bail!(
				"found multiple JS embeds for `{module}:{name}`\n\tJS Embed 1:\n{}\n\tJS Embed \
				 2:\n{}",
				embed.js(),
				embed_new.js(),
			);
		}

		if self
			.expected_embed
			.get_mut(module)
			.is_some_and(|names| names.remove(name))
		{
			self.embed
				.entry(module)
				.or_default()
				.insert(name, embed.js());

			if let Some(embed) = embed.embed() {
				self.require_js_embed(module, embed);
			}
		} else if let Some(embed_old) = self.provided_embed.entry_ref(module).or_default().insert(
			name,
			JsWithEmbed {
				js: embed.js(),
				embed: embed.embed(),
			},
		) {
			bail!(
				"found multiple JS embeds for `{module}:{name}`\n\tJS Embed 1:\n{}\n\tJS Embed \
				 2:\n{}",
				embed_old.js,
				embed.js()
			);
		}

		Ok(())
	}

	fn require_js_embed(&mut self, module: &'a str, name: &'a str) {
		if !self
			.embed
			.get(module)
			.iter()
			.any(|names| names.contains_key(name))
		{
			if let Some(embed) = self
				.provided_embed
				.get_mut(module)
				.and_then(|names| names.remove(name))
			{
				self.embed.entry(module).or_default().insert(name, embed.js);

				if let Some(name) = embed.embed {
					self.require_js_embed(module, name);
				}
			} else {
				self.expected_embed.entry(module).or_default().insert(name);
			}
		}
	}

	fn assert_expected(&self) -> Result<()> {
		ensure!(
			self.expected_import.values().all(HashSet::is_empty),
			"missing JS imports: {:?}",
			self.expected_import
		);
		ensure!(
			self.expected_embed.values().all(HashSet::is_empty),
			"missing JS embed: {:?}",
			self.expected_embed
		);

		Ok(())
	}

	fn js_import(&self) -> &FixedHashMap<&'a str, FixedHashMap<&'a str, Option<&'a str>>> {
		&self.import
	}

	fn js_embed(&self) -> &FixedHashMap<&'a str, FixedHashMap<&'a str, &'a str>> {
		&self.embed
	}
}
