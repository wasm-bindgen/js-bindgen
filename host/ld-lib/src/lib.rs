use std::io::Write;

use hashbrown::{HashMap, HashSet};
use itertools::{Itertools, Position};
use js_bindgen_ld_shared::{
	JsBindgenEmbedSection, JsBindgenEmbedSectionParser, JsBindgenImportSection,
	JsBindgenImportSectionParser,
};
use wasm_encoder::{EntityType, ImportSection, Module, RawSection, Section};
use wasmparser::{Encoding, Parser, Payload, TypeRef};

const IMPORTS_JS: &str = include_str!("js/imports.mjs");

pub struct MainMemory<'a> {
	pub module: &'a str,
	pub name: &'a str,
}

/// This removes our custom sections and generates the JS import file.
pub fn post_processing(
	wasm_input: &[u8],
	mut js_output: impl Write,
	main_memory: MainMemory<'_>,
) -> Vec<u8> {
	let mut wasm_output = Vec::new();

	let mut found_import: HashMap<&str, HashMap<&str, Option<&str>>> = HashMap::new();
	let mut expected_import: HashMap<&str, HashSet<&str>> = HashMap::new();
	let mut provided_import: HashMap<&str, HashMap<&str, Option<&str>>> = HashMap::new();
	let mut found_embed: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
	let mut expected_embed: HashMap<&str, HashSet<&str>> = HashMap::new();
	let mut provided_embed: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
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

					// The main memory has its own dedicated output handling.
					if let TypeRef::Memory(m) = import.ty
						&& import.module == main_memory.module
						&& import.name == main_memory.name
					{
						memory = Some(m);
						continue;
					}

					if let Some(code) = provided_import
						.get_mut(import.module)
						.and_then(|names| names.remove(import.name))
					{
						found_import
							.entry(import.module)
							.or_default()
							.insert(import.name, code);
					} else {
						assert!(
							expected_import
								.entry(import.module)
								.or_default()
								.insert(import.name),
							"found duplicate JS import: `{}:{}`",
							import.module,
							import.name
						);
					}
				}

				import_section.append_to(&mut wasm_output);
			}
			// Don't write back our assembly sections.
			Payload::CustomSection(c) if c.name() == "js_bindgen.assembly" => (),
			// Extract all JS imports.
			Payload::CustomSection(c) if c.name().starts_with("js_bindgen.import.") => {
				let stripped = c.name().strip_prefix("js_bindgen.import.").unwrap();
				let (module, name) = stripped.split_once('.').unwrap_or_else(|| {
					panic!("found incorrectly formatted JS import custom section name: {stripped}")
				});

				let mut parser = JsBindgenImportSectionParser::new(c);
				let import = parser
					.next()
					.unwrap_or_else(|| panic!("found no JS import for `{module}:{name}`"));

				if let Some(import_new) = parser.next() {
					panic!(
						"found multiple JS imports for `{module}:{name}`\n\tJS Import \
						 1:\n{:?}\n\tJS Import 2:\n{:?}",
						import.js(),
						import_new.js(),
					);
				}

				if expected_import
					.get_mut(module)
					.map(|names| names.remove(name))
					.unwrap_or_default()
				{
					found_import
						.entry(module)
						.or_default()
						.insert(name, import.js());
				} else if let Some(import_old) = provided_import
					.entry_ref(module)
					.or_default()
					.insert(name, import.js())
				{
					panic!(
						"found multiple JS imports for `{module}:{name}`\n\tJS Import \
						 1:\n{:?}\n\tJS Import 2:\n{:?}",
						import_old,
						import.js()
					);
				}

				if let JsBindgenImportSection::WithEmbed { embed, .. } = import {
					if found_embed
						.get_mut(module)
						.map(|names| names.contains_key(embed))
						.unwrap_or(false)
					{
					} else if let Some(code) = provided_embed
						.get_mut(module)
						.and_then(|names| names.remove(embed))
					{
						found_embed.entry(module).or_default().insert(embed, code);
					} else {
						expected_embed.entry(module).or_default().insert(embed);
					}
				}
			}
			// Extract all JS embeds.
			Payload::CustomSection(c) if c.name().starts_with("js_bindgen.embed.") => {
				let stripped = c.name().strip_prefix("js_bindgen.embed.").unwrap();
				let (module, name) = stripped.split_once('.').unwrap_or_else(|| {
					panic!("found incorrectly formatted JS import custom section name: {stripped}")
				});

				let mut parser = JsBindgenEmbedSectionParser::new(c);
				let embed = parser
					.next()
					.unwrap_or_else(|| panic!("found no JS embed for `{module}:{name}`"));

				if let Some(embed_new) = parser.next() {
					panic!(
						"found multiple JS embeds for `{module}:{name}`\n\tJS Embed 1:\n{}\n\tJS \
						 Embed 2:\n{}",
						embed.js(),
						embed_new.js(),
					);
				}

				if expected_embed
					.get_mut(module)
					.map(|names| names.remove(name))
					.unwrap_or_default()
				{
					found_embed
						.entry(module)
						.or_default()
						.insert(name, embed.js());
				} else if let Some(embed_old) = provided_embed
					.entry_ref(module)
					.or_default()
					.insert(name, embed.js())
				{
					panic!(
						"found multiple JS embeds for `{module}:{name}`\n\tJS Embed 1:\n{}\n\tJS \
						 Embed 2:\n{}",
						embed_old,
						embed.js()
					);
				}

				if let JsBindgenEmbedSection::WithEmbed { embed, .. } = embed {
					if found_embed
						.get_mut(module)
						.map(|names| names.contains_key(embed))
						.unwrap_or(false)
					{
					} else if let Some(code) = provided_embed
						.get_mut(module)
						.and_then(|names| names.remove(embed))
					{
						found_embed.entry(module).or_default().insert(embed, code);
					} else {
						expected_embed.entry(module).or_default().insert(embed);
					}
				}
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
	assert!(
		expected_import.values().all(HashSet::is_empty),
		"missing JS imports: {expected_import:?}"
	);
	assert!(
		expected_embed.values().all(HashSet::is_empty),
		"missing JS embed: {expected_embed:?}"
	);

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

	for (package, embeds) in found_embed {
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

	for (module, names) in found_import
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
