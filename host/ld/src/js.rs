use anyhow::{Context, Result, bail, ensure};
use foldhash::fast::FixedState;
use hashbrown::{HashMap, HashSet};
use js_bindgen_ld_shared::{JsBindgenEmbedSectionParser, JsBindgenImportSectionParser};
use wasmparser::{CustomSectionReader, Import};

type FixedHashMap<K, V> = HashMap<K, V, FixedState>;

#[derive(Default)]
pub struct JsStore {
	import: FixedHashMap<String, FixedHashMap<String, Option<String>>>,
	expected_import: HashMap<String, HashSet<String>>,
	provided_import: HashMap<String, HashMap<String, Option<JsWithEmbed>>>,
	embed: FixedHashMap<String, FixedHashMap<String, String>>,
	expected_embed: HashMap<String, HashSet<String>>,
	provided_embed: HashMap<String, HashMap<String, JsWithEmbed>>,
}

struct JsWithEmbed {
	js: String,
	embed: Option<String>,
}

impl JsStore {
	pub fn add_import(&mut self, import: Import<'_>) -> Result<()> {
		if let Some(js) = self
			.provided_import
			.get_mut(import.module)
			.and_then(|names| names.remove(import.name))
		{
			let (js, embed) = if let Some(js) = js {
				(Some(js.js), js.embed)
			} else {
				(None, None)
			};

			self.import
				.entry(import.module.to_owned())
				.or_default()
				.insert(import.name.to_owned(), js);

			if let Some(embed) = embed {
				self.require_js_embed(import.module.to_owned(), embed);
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

	pub fn add_js_import(
		&mut self,
		module: String,
		name: String,
		custom_section: &CustomSectionReader<'_>,
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
			.get_mut(&module)
			.is_some_and(|names| names.remove(&name))
		{
			self.import
				.entry_ref(&module)
				.or_default()
				.insert(name, import.js().map(str::to_owned));

			if let Some(embed) = import.embed() {
				self.require_js_embed(module, embed.to_owned());
			}
		} else if let Err(error) = self
			.provided_import
			.entry_ref(&module)
			.or_default()
			.try_insert(
				name,
				import.js().map(|js| JsWithEmbed {
					js: js.to_owned(),
					embed: import.embed().map(str::to_owned),
				}),
			) {
			bail!(
				"found multiple JS imports for `{module}:{}`\n\tJS Import 1:\n{:?}\n\tJS Import \
				 2:\n{:?}",
				error.entry.key(),
				error.entry.get().as_ref().map(|js| &js.js),
				import.js()
			);
		}

		Ok(())
	}

	pub fn add_js_embed(
		&mut self,
		module: String,
		name: String,
		custom_section: &CustomSectionReader<'_>,
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
			.get_mut(&module)
			.is_some_and(|names| names.remove(&name))
		{
			self.embed
				.entry_ref(&module)
				.or_default()
				.insert(name, embed.js().to_owned());

			if let Some(embed) = embed.embed() {
				self.require_js_embed(module, embed.to_owned());
			}
		} else if let Err(error) = self
			.provided_embed
			.entry_ref(&module)
			.or_default()
			.try_insert(
				name,
				JsWithEmbed {
					js: embed.js().to_owned(),
					embed: embed.embed().map(str::to_owned),
				},
			) {
			bail!(
				"found multiple JS embeds for `{module}:{}`\n\tJS Embed 1:\n{}\n\tJS Embed 2:\n{}",
				error.entry.key(),
				error.entry.get().js,
				embed.js()
			);
		}

		Ok(())
	}

	fn require_js_embed(&mut self, module: String, name: String) {
		if !self
			.embed
			.get(&module)
			.iter()
			.any(|names| names.contains_key(&name))
		{
			if let Some(embed) = self
				.provided_embed
				.get_mut(&module)
				.and_then(|names| names.remove(&name))
			{
				self.embed
					.entry_ref(&module)
					.or_default()
					.insert(name, embed.js);

				if let Some(name) = embed.embed {
					self.require_js_embed(module, name);
				}
			} else {
				self.expected_embed.entry(module).or_default().insert(name);
			}
		}
	}

	pub fn assert_expected(&self) -> Result<()> {
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

	pub fn js_import(&self) -> &FixedHashMap<String, FixedHashMap<String, Option<String>>> {
		&self.import
	}

	pub fn js_embed(&self) -> &FixedHashMap<String, FixedHashMap<String, String>> {
		&self.embed
	}
}
