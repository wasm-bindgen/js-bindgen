use std::io::Write;
use std::str;

use hashbrown::{HashMap, HashSet};
use itertools::{Itertools, Position};
use js_bindgen_ld_shared::{JsBindgenEmbedSectionParser, JsBindgenImportSectionParser};
use wasm_encoder::{EntityType, ImportSection, Module, RawSection, Section};
use wasmparser::{CustomSectionReader, Encoding, Import, Parser, Payload, TypeRef};

const IMPORTS_JS: &str = include_str!("js/imports.mjs");

/// This removes our custom sections and generates the JS import file.
pub fn post_processing(wasm_input: &[u8], mut js_output: impl Write) -> Vec<u8> {
	// Find main memory first.
	let main_memory = Parser::new(0)
		.parse_all(wasm_input)
		.find_map(|payload| {
			let payload = payload.expect("object file should be valid Wasm");

			if let Payload::CustomSection(c) = payload
				&& c.name() == "js_bindgen.main_memory"
			{
				let mut data = c.data();
				let module_len = u16::from_le_bytes(
					data.split_off(..2)
						.expect("invalid main memory encoding")
						.try_into()
						.unwrap(),
				);
				let module = data
					.split_off(..module_len.into())
					.and_then(|b| str::from_utf8(b).ok())
					.expect("invalid main memory encoding");
				let name_len = u16::from_le_bytes(
					data.split_off(..2)
						.expect("invalid main memory encoding")
						.try_into()
						.unwrap(),
				);
				let name = data
					.split_off(..name_len.into())
					.and_then(|b| str::from_utf8(b).ok())
					.expect("invalid main memory encoding");
				assert!(data.is_empty(), "invalid main memory encoding");

				Some((module, name))
			} else {
				None
			}
		})
		.expect("no main memory found");

	let mut wasm_output = Vec::new();

	let mut js_store = JsStore::default();
	let mut memory = None;

	for payload in Parser::new(0).parse_all(wasm_input) {
		let payload = payload.expect("object file should be valid Wasm");

		match payload {
			Payload::Version { encoding, .. } => wasm_output.extend_from_slice(match encoding {
				Encoding::Module => &Module::HEADER,
				Encoding::Component => {
					unimplemented!("objects with components are not supported")
				}
			}),
			// Read what imports we need. This has already undergone dead-code elimination by LLD.
			Payload::ImportSection(i) => {
				let mut import_section = ImportSection::new();

				for i in i.into_imports() {
					let mut import = i.expect("import should be parsable");

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
							.expect("`wasmparser` type should be convertible"),
					);

					// The main memory has its own dedicated JS output handling.
					if let TypeRef::Memory(m) = import.ty
						&& import.module == main_memory.0
						&& import.name == main_memory.1
					{
						memory = Some(m);
						continue;
					}

					js_store.add_import(import);
				}

				import_section.append_to(&mut wasm_output);
			}
			// Don't write back our assembly sections.
			Payload::CustomSection(c) if c.name() == "js_bindgen.assembly" => (),
			// Don't write back our main memory section.
			Payload::CustomSection(c) if c.name() == "js_bindgen.main_memory" => (),
			// Extract all JS imports.
			Payload::CustomSection(c) if c.name().starts_with("js_bindgen.import.") => {
				let stripped = c.name().strip_prefix("js_bindgen.import.").unwrap();
				let (module, name) = stripped.split_once('.').unwrap_or_else(|| {
					panic!("found incorrectly formatted JS import custom section name: {stripped}")
				});

				js_store.add_js_import(module, name, c);
			}
			// Extract all JS embeds.
			Payload::CustomSection(c) if c.name().starts_with("js_bindgen.embed.") => {
				let stripped = c.name().strip_prefix("js_bindgen.embed.").unwrap();
				let (module, name) = stripped.split_once('.').unwrap_or_else(|| {
					panic!("found incorrectly formatted JS import custom section name: {stripped}")
				});

				js_store.add_js_embed(module, name, c);
			}
			Payload::CodeSectionEntry(_) | Payload::End(_) => (),
			payload => {
				let (id, range) = payload
					.as_section()
					.unwrap_or_else(|| panic!("expected parsable Wasm payload:\n{payload:?}"));
				RawSection {
					id,
					data: &wasm_input[range],
				}
				.append_to(&mut wasm_output);
			}
		}
	}

	let memory = memory.expect("main memory should be present");
	js_store.assert_expected();

	let (js_memory, rest) = IMPORTS_JS.split_once("JBG_PLACEHOLDER_MEMORY").unwrap();
	let (js_embed, rest) = rest.split_once("JBG_PLACEHOLDER_JS_EMBED").unwrap();
	let (js_import_object, js_rest) = rest.split_once("JBG_PLACEHOLDER_IMPORT_OBJECT").unwrap();

	// `WebAssembly.Memory`.
	js_output.write_all(js_memory.as_bytes()).unwrap();

	js_output.write_all(b"new WebAssembly.Memory({ ").unwrap();

	if memory.memory64 {
		write!(js_output, "initial: {}n", memory.initial).unwrap();
	} else {
		write!(js_output, "initial: {}", memory.initial).unwrap();
	}

	if let Some(max) = memory.maximum {
		if memory.memory64 {
			write!(js_output, ", maximum: {max}n").unwrap();
		} else {
			write!(js_output, ", maximum: {max}").unwrap();
		}
	}

	if memory.memory64 {
		js_output.write_all(b", address: 'i64'").unwrap();
	}

	if memory.shared {
		js_output.write_all(b", shared: true").unwrap();
	}

	js_output.write_all(b" })").unwrap();

	// Requested embedded JS.
	js_output.write_all(js_embed.as_bytes()).unwrap();

	js_output.write_all(b"{\n").unwrap();

	for (package, embeds) in js_store.js_embed() {
		writeln!(js_output, "\t\t\t{package}: {{").unwrap();

		for (name, js) in embeds {
			write!(js_output, "\t\t\t\t'{name}': ").unwrap();

			for (position, line) in js.lines().with_position() {
				js_output.write_all(line.as_bytes()).unwrap();

				if let Position::First | Position::Middle = position {
					js_output.write_all(b"\n\t\t\t\t").unwrap();
				}
			}

			js_output.write_all(b",\n").unwrap();
		}

		js_output.write_all(b"\t\t\t},\n").unwrap();
	}

	js_output.write_all(b"\t\t}").unwrap();

	// `importObject`
	js_output.write_all(js_import_object.as_bytes()).unwrap();

	js_output.write_all(b"{\n").unwrap();
	js_output
		.write_all(b"\t\t\tjs_bindgen: { memory: this.#memory },\n")
		.unwrap();

	for (module, names) in js_store
		.js_import()
		.into_iter()
		.filter(|(_, names)| !names.values().all(Option::is_none))
	{
		writeln!(js_output, "\t\t\t{module}: {{").unwrap();

		for (name, js) in names
			.into_iter()
			.filter_map(|(name, js)| js.map(|js| (name, js)))
		{
			write!(js_output, "\t\t\t\t'{name}': ").unwrap();

			for (position, line) in js.lines().with_position() {
				js_output.write_all(line.as_bytes()).unwrap();

				if let Position::First | Position::Middle = position {
					js_output.write_all(b"\n\t\t\t\t").unwrap();
				}
			}

			js_output.write_all(b",\n").unwrap();
		}

		js_output.write_all(b"\t\t\t},\n").unwrap();
	}

	js_output.write_all(b"\t\t}").unwrap();

	// Finish
	js_output.write_all(js_rest.as_bytes()).unwrap();

	wasm_output
}

#[derive(Default)]
struct JsStore<'a> {
	import: HashMap<&'a str, HashMap<&'a str, Option<&'a str>>>,
	expected_import: HashMap<&'a str, HashSet<&'a str>>,
	provided_import: HashMap<&'a str, HashMap<&'a str, Option<JsWithEmbed<'a>>>>,
	embed: HashMap<&'a str, HashMap<&'a str, &'a str>>,
	expected_embed: HashMap<&'a str, HashSet<&'a str>>,
	provided_embed: HashMap<&'a str, HashMap<&'a str, JsWithEmbed<'a>>>,
}

#[derive(Clone, Copy)]
struct JsWithEmbed<'a> {
	js: &'a str,
	embed: Option<&'a str>,
}

impl<'a> JsStore<'a> {
	fn add_import(&mut self, import: Import<'a>) {
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
			panic!(
				"found duplicate JS import: `{}:{}`",
				import.module, import.name
			);
		}
	}

	fn add_js_import(
		&mut self,
		module: &'a str,
		name: &'a str,
		custom_section: CustomSectionReader<'a>,
	) {
		let mut parser = JsBindgenImportSectionParser::new(custom_section);
		let import = parser
			.next()
			.unwrap_or_else(|| panic!("found no JS import for `{module}:{name}`"));

		if let Some(import_new) = parser.next() {
			panic!(
				"found multiple JS imports for `{module}:{name}`\n\tJS Import 1:\n{:?}\n\tJS \
				 Import 2:\n{:?}",
				import.js(),
				import_new.js(),
			);
		}

		if self
			.expected_import
			.get_mut(module)
			.map(|names| names.remove(name))
			.unwrap_or_default()
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
			panic!(
				"found multiple JS imports for `{module}:{name}`\n\tJS Import 1:\n{:?}\n\tJS \
				 Import 2:\n{:?}",
				import_old.map(|js| js.js),
				import.js()
			);
		}
	}

	fn add_js_embed(
		&mut self,
		module: &'a str,
		name: &'a str,
		custom_section: CustomSectionReader<'a>,
	) {
		let mut parser = JsBindgenEmbedSectionParser::new(custom_section);
		let embed = parser
			.next()
			.unwrap_or_else(|| panic!("found no JS embed for `{module}:{name}`"));

		if let Some(embed_new) = parser.next() {
			panic!(
				"found multiple JS embeds for `{module}:{name}`\n\tJS Embed 1:\n{}\n\tJS Embed \
				 2:\n{}",
				embed.js(),
				embed_new.js(),
			);
		}

		if self
			.expected_embed
			.get_mut(module)
			.map(|names| names.remove(name))
			.unwrap_or_default()
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
			panic!(
				"found multiple JS embeds for `{module}:{name}`\n\tJS Embed 1:\n{}\n\tJS Embed \
				 2:\n{}",
				embed_old.js,
				embed.js()
			);
		}
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

	fn assert_expected(&self) {
		assert!(
			self.expected_import.values().all(HashSet::is_empty),
			"missing JS imports: {:?}",
			self.expected_import
		);
		assert!(
			self.expected_embed.values().all(HashSet::is_empty),
			"missing JS embed: {:?}",
			self.expected_embed
		);
	}

	fn js_import(&self) -> &HashMap<&'a str, HashMap<&'a str, Option<&'a str>>> {
		&self.import
	}

	fn js_embed(&self) -> &HashMap<&'a str, HashMap<&'a str, &'a str>> {
		&self.embed
	}
}
