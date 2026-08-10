use anyhow::{Result, bail, ensure};
use foldhash::fast::FixedState;
use hashbrown::{HashMap, HashSet};
use js_bindgen_cli_lib::{JsOutput, MainMemory};
use js_bindgen_ld_shared::{JsBindgenJsSectionParser, JsRequiredEmbed};
use wasmparser::{CustomSectionReader, Import};

type FixedHashMap<K, V> = HashMap<K, V, FixedState>;

#[derive(Default)]
pub struct JsStore {
	import: FixedHashMap<String, FixedHashMap<String, String>>,
	expected_import: HashMap<String, HashSet<String>>,
	// Keep canonical definitions after resolution so later records can be
	// checked for equality.
	provided_import: HashMap<String, HashMap<String, JsWithEmbeds>>,
	embed: FixedHashMap<String, FixedHashMap<String, String>>,
	expected_embed: HashMap<String, HashSet<String>>,
	provided_embed: HashMap<String, HashMap<String, JsWithEmbeds>>,
	export: FixedHashMap<String, JsExport>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct JsWithEmbeds {
	js: String,
	embeds: Vec<JsEmbed>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct JsEmbed {
	module: String,
	name: String,
}

#[derive(Debug, PartialEq, Eq)]
struct JsExport {
	module: String,
	binding: JsWithEmbeds,
	kind: JsExportKind,
}

#[derive(Debug, PartialEq, Eq)]
enum JsExportKind {
	Symbol,
	Closure { shim: String },
}

impl JsStore {
	pub fn add_js_import(
		&mut self,
		module: &str,
		name: &str,
		js: String,
		embeds: impl IntoIterator<Item = (String, String)>,
	) -> Result<()> {
		let binding = JsWithEmbeds {
			js,
			embeds: embeds
				.into_iter()
				.map(|(module, name)| JsEmbed { module, name })
				.collect(),
		};
		let definitions = self.provided_import.entry_ref(module).or_default();

		if let Some(previous) = definitions.get(name) {
			if previous != &binding {
				bail!(
					"found multiple JS imports for `{module}:{name}`\n\tJS Import \
					 1:\n{previous:?}\n\tJS Import 2:\n{binding:?}",
				);
			}
		} else {
			definitions.insert(name.to_owned(), binding.clone());
		}

		if self
			.expected_import
			.get_mut(module)
			.is_some_and(|names| names.remove(name))
		{
			self.import
				.entry_ref(module)
				.or_default()
				.insert(name.to_owned(), binding.js.clone());

			for embed in binding.embeds {
				self.require_js_embed(embed);
			}
		}

		Ok(())
	}

	pub fn add_symbol_export(
		&mut self,
		module: &str,
		name: &str,
		js: String,
		embeds: impl IntoIterator<Item = (String, String)>,
	) -> Result<()> {
		let definition = JsExport {
			module: module.to_owned(),
			binding: JsWithEmbeds {
				js,
				embeds: embeds
					.into_iter()
					.map(|(module, name)| JsEmbed { module, name })
					.collect(),
			},
			kind: JsExportKind::Symbol,
		};

		if let Some(previous) = self.export.get(name) {
			bail!(
				"found multiple JS exports named `{name}` from `{}` and `{}`\n\tJS Export \
				 1:\n{:?}\n\tJS Export 2:\n{:?}",
				previous.module,
				definition.module,
				previous.binding,
				definition.binding
			);
		}

		for embed in definition.binding.embeds.iter().cloned() {
			self.require_js_embed(embed);
		}

		self.export.insert(name.into(), definition);
		Ok(())
	}

	pub fn add_closure_export(
		&mut self,
		module: &str,
		name: &str,
		js: String,
		embeds: impl IntoIterator<Item = (String, String)>,
		shim: &str,
	) -> Result<bool> {
		let definition = JsExport {
			module: module.to_owned(),
			binding: JsWithEmbeds {
				js,
				embeds: embeds
					.into_iter()
					.map(|(module, name)| JsEmbed { module, name })
					.collect(),
			},
			kind: JsExportKind::Closure {
				shim: shim.to_owned(),
			},
		};

		if let Some(previous) = self.export.get(name) {
			if previous == &definition {
				return Ok(false);
			}
			bail!(
				"found incompatible closure exports named `{name}` from `{}` and `{}`\n\tClosure \
				 1:\n{:?}\n\tClosure 2:\n{:?}",
				previous.module,
				definition.module,
				previous,
				definition,
			);
		}

		for embed in definition.binding.embeds.iter().cloned() {
			self.require_js_embed(embed);
		}

		self.export.insert(name.into(), definition);
		Ok(true)
	}

	pub fn add_import(&mut self, import: Import<'_>) -> Result<()> {
		if let Some(js) = self
			.provided_import
			.get(import.module)
			.and_then(|names| names.get(import.name))
			.cloned()
		{
			self.import
				.entry(import.module.to_owned())
				.or_default()
				.insert(import.name.to_owned(), js.js);

			for embed in js.embeds {
				self.require_js_embed(embed);
			}
		} else if !self
			.expected_import
			.entry(import.module.to_owned())
			.or_default()
			.insert(import.name.to_owned())
		{
			bail!(
				"found duplicate JS import: `{}:{}`",
				import.module,
				import.name
			);
		}

		Ok(())
	}

	pub fn add_js_imports(&mut self, custom_section: &CustomSectionReader<'_>) -> Result<()> {
		for import in JsBindgenJsSectionParser::new(custom_section) {
			self.add_js_import(
				import.module,
				import.name,
				import.js.to_owned(),
				import
					.embeds
					.into_iter()
					.map(|embed| (embed.module.to_owned(), embed.name.to_owned())),
			)?;
		}

		Ok(())
	}

	pub fn add_js_embeds(&mut self, custom_section: &CustomSectionReader<'_>) -> Result<()> {
		for embed in JsBindgenJsSectionParser::new(custom_section) {
			let binding = JsWithEmbeds {
				js: embed.js.to_owned(),
				embeds: embed.embeds.into_iter().map(JsEmbed::from).collect(),
			};
			let definitions = self.provided_embed.entry_ref(embed.module).or_default();

			if let Some(previous) = definitions.get(embed.name) {
				if previous != &binding {
					bail!(
						"found multiple JS embeds for `{}:{}`\n\tJS Embed 1:\n{:?}\n\tJS Embed \
						 2:\n{:?}",
						embed.module,
						embed.name,
						previous,
						binding
					);
				}
			} else {
				definitions.insert(embed.name.to_owned(), binding.clone());
			}

			if self
				.expected_embed
				.get_mut(embed.module)
				.is_some_and(|names| names.remove(embed.name))
			{
				self.embed
					.entry_ref(embed.module)
					.or_default()
					.insert(embed.name.to_owned(), binding.js.clone());

				for required_embed in binding.embeds {
					self.require_js_embed(required_embed);
				}
			}
		}

		Ok(())
	}

	fn require_js_embed(&mut self, embed: JsEmbed) {
		if !self
			.embed
			.get(&embed.module)
			.is_some_and(|names| names.contains_key(&embed.name))
		{
			if let Some(js) = self
				.provided_embed
				.get(&embed.module)
				.and_then(|names| names.get(&embed.name))
				.cloned()
			{
				self.embed
					.entry_ref(&embed.module)
					.or_default()
					.insert(embed.name, js.js);

				for embed in js.embeds {
					self.require_js_embed(embed);
				}
			} else {
				self.expected_embed
					.entry(embed.module)
					.or_default()
					.insert(embed.name);
			}
		}
	}

	pub fn assert_expected(&self) -> Result<()> {
		ensure!(
			self.expected_embed.values().all(HashSet::is_empty),
			"missing JS embed: {:?}",
			self.expected_embed
		);

		Ok(())
	}

	pub fn into_output(self, main_memory: MainMemory<'_>) -> JsOutput<'_, String> {
		JsOutput {
			main_memory,
			js_import: self.import,
			js_embed: self.embed,
			js_export: self
				.export
				.into_iter()
				.map(|(name, export)| (name, export.binding.js))
				.collect(),
		}
	}
}

impl From<JsRequiredEmbed<'_>> for JsEmbed {
	fn from(value: JsRequiredEmbed<'_>) -> Self {
		Self {
			module: value.module.to_owned(),
			name: value.name.to_owned(),
		}
	}
}
