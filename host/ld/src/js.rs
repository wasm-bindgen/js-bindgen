use anyhow::{Result, bail, ensure};
use foldhash::fast::FixedState;
use hashbrown::{HashMap, HashSet};
use js_bindgen_ld_shared::{JsBindgenJsSectionParser, JsRequiredEmbed};
use wasmparser::{CustomSectionReader, Import};

type FixedHashMap<K, V> = HashMap<K, V, FixedState>;

#[derive(Default)]
pub struct JsStore {
	import: FixedHashMap<String, FixedHashMap<String, String>>,
	expected_import: HashMap<String, HashSet<String>>,
	provided_import: HashMap<String, HashMap<String, JsWithEmbeds>>,
	embed: FixedHashMap<String, FixedHashMap<String, String>>,
	expected_embed: HashMap<String, HashSet<String>>,
	provided_embed: HashMap<String, HashMap<String, JsWithEmbeds>>,
}

struct JsWithEmbeds {
	js: String,
	embeds: Vec<JsEmbed>,
}

struct JsEmbed {
	module: String,
	name: String,
}

impl JsStore {
	pub fn add_import(&mut self, import: Import<'_>) -> Result<()> {
		if let Some(js) = self
			.provided_import
			.get_mut(import.module)
			.and_then(|names| names.remove(import.name))
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
			if self
				.expected_import
				.get_mut(import.module)
				.is_some_and(|names| names.remove(import.name))
			{
				self.import
					.entry_ref(import.module)
					.or_default()
					.insert(import.name.to_owned(), import.js.to_owned());

				for embed in import.embeds {
					self.require_js_embed(embed.into());
				}
			} else if let Err(error) = self
				.provided_import
				.entry_ref(import.module)
				.or_default()
				.try_insert(
					import.name.to_owned(),
					JsWithEmbeds {
						js: import.js.to_owned(),
						embeds: import.embeds.into_iter().map(JsEmbed::from).collect(),
					},
				) {
				bail!(
					"found multiple JS imports for `{}:{}`\n\tJS Import 1:\n{:?}\n\tJS Import \
					 2:\n{:?}",
					import.module,
					error.entry.key(),
					error.entry.get().js,
					import.js
				);
			}
		}

		Ok(())
	}

	pub fn add_js_embeds(&mut self, custom_section: &CustomSectionReader<'_>) -> Result<()> {
		for embed in JsBindgenJsSectionParser::new(custom_section) {
			if self
				.expected_embed
				.get_mut(embed.module)
				.is_some_and(|names| names.remove(embed.name))
			{
				self.embed
					.entry_ref(embed.module)
					.or_default()
					.insert(embed.name.to_owned(), embed.js.to_owned());

				for required_embed in embed.embeds {
					self.require_js_embed(required_embed.into());
				}
			} else if let Err(error) = self
				.provided_embed
				.entry_ref(embed.module)
				.or_default()
				.try_insert(
					embed.name.to_owned(),
					JsWithEmbeds {
						js: embed.js.to_owned(),
						embeds: embed.embeds.into_iter().map(JsEmbed::from).collect(),
					},
				) {
				bail!(
					"found multiple JS embeds for `{}:{}`\n\tJS Embed 1:\n{}\n\tJS Embed 2:\n{}",
					embed.module,
					error.entry.key(),
					error.entry.get().js,
					embed.js
				);
			}
		}

		Ok(())
	}

	fn require_js_embed(&mut self, embed: JsEmbed) {
		if !self
			.embed
			.get(&embed.module)
			.iter()
			.any(|names| names.contains_key(&embed.name))
		{
			if let Some(js) = self
				.provided_embed
				.get_mut(&embed.module)
				.and_then(|names| names.remove(&embed.name))
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

	pub fn js_import(&self) -> &FixedHashMap<String, FixedHashMap<String, String>> {
		&self.import
	}

	pub fn js_embed(&self) -> &FixedHashMap<String, FixedHashMap<String, String>> {
		&self.embed
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
