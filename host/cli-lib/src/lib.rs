use std::fmt::Display;
use std::hash::Hash;
use std::io::Write;
use std::ops::Deref;

use anyhow::Result;
use foldhash::fast::FixedState;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use wasmparser::MemoryType;

pub const JS_OUTPUT_SECTION: &str = "js_bindgen.js_output";

type FixedHashMap<K, V> = HashMap<K, V, FixedState>;

#[derive(Deserialize, Serialize)]
pub struct JsOutput<'a, T: Deref<Target = str> + Display + Eq + Hash + Serialize> {
	#[serde(borrow)]
	pub main_memory: MainMemory<'a>,
	pub js_import: FixedHashMap<T, FixedHashMap<T, T>>,
	pub js_embed: FixedHashMap<T, FixedHashMap<T, T>>,
	pub js_export: FixedHashMap<T, T>,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct MainMemory<'a> {
	pub module: &'a str,
	pub name: &'a str,
}

impl<T: Deref<Target = str> + Display + Eq + Hash + Serialize> JsOutput<'_, T> {
	pub fn js(&self, mut output: impl Write, main_memory: MemoryType) -> Result<()> {
		const IMPORTS_JS: &str = include_str!("js/imports.mjs");

		let (js_file_memory, rest) = IMPORTS_JS.split_once("JBG_PLACEHOLDER_MEMORY").unwrap();
		let (js_file_embed, rest) = rest.split_once("JBG_PLACEHOLDER_JS_EMBED").unwrap();
		let (js_file_import, rest) = rest.split_once("JBG_PLACEHOLDER_IMPORT_OBJECT").unwrap();
		let (js_file_export, js_file_finish) =
			rest.split_once("JBG_PLACEHOLDER_JS_EXPORT").unwrap();

		// `WebAssembly.Memory`.
		output.write_all(js_file_memory.as_bytes())?;

		output.write_all(b"new WebAssembly.Memory({ ")?;

		if main_memory.memory64 {
			write!(output, "initial: {}n", main_memory.initial)?;
		} else {
			write!(output, "initial: {}", main_memory.initial)?;
		}

		if let Some(max) = main_memory.maximum {
			if main_memory.memory64 {
				write!(output, ", maximum: {max}n")?;
			} else {
				write!(output, ", maximum: {max}")?;
			}
		}

		if main_memory.memory64 {
			output.write_all(b", address: 'i64'")?;
		}

		if main_memory.shared {
			output.write_all(b", shared: true")?;
		}

		output.write_all(b" })")?;

		// Requested embedded JS.
		output.write_all(js_file_embed.as_bytes())?;

		output.write_all(b"{\n")?;

		for (package, embeds) in &self.js_embed {
			writeln!(output, "\t\t\t{package}: {{")?;

			for (name, js) in embeds {
				write!(output, "\t\t\t\t'{name}': ")?;
				write_indented_js(&mut output, js, b"\t\t\t\t")?;
				output.write_all(b",\n")?;
			}

			output.write_all(b"\t\t\t},\n")?;
		}

		output.write_all(b"\t\t}")?;

		// `importObject`
		output.write_all(js_file_import.as_bytes())?;

		output.write_all(b"{\n")?;
		writeln!(
			output,
			"\t\t\t{}: {{ {}: this.#memory }},\n",
			self.main_memory.module, self.main_memory.name
		)?;

		for (module, names) in &self.js_import {
			writeln!(output, "\t\t\t{module}: {{")?;

			for (name, js) in names {
				write!(output, "\t\t\t\t'{name}': ")?;
				write_indented_js(&mut output, js, b"\t\t\t\t")?;
				output.write_all(b",\n")?;
			}

			output.write_all(b"\t\t\t},\n")?;
		}

		output.write_all(b"\t\t}")?;

		// JS export wrappers.
		output.write_all(js_file_export.as_bytes())?;
		output.write_all(b"{\n")?;

		for (name, js) in &self.js_export {
			write!(output, "                '{name}': ")?;
			write_indented_js(&mut output, js, b"                ")?;
			output.write_all(b",\n")?;
		}

		output.write_all(b"            }")?;

		// Finish.
		output.write_all(js_file_finish.as_bytes())?;

		Ok(())
	}
}

fn write_indented_js(
	output: &mut impl Write,
	js: &str,
	continuation_indent: &[u8],
) -> std::io::Result<()> {
	for (index, line) in js.lines().enumerate() {
		if index != 0 {
			output.write_all(b"\n")?;

			if !line.is_empty() {
				output.write_all(continuation_indent)?;
			}
		}

		output.write_all(line.as_bytes())?;
	}

	Ok(())
}
