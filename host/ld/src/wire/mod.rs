//! Rendering of decoded `js-sys` wire records.

mod closure;
mod export;
mod import;
mod js;
mod wat;

use js_bindgen_wire::model::{Embed, Record};
use js_bindgen_wire::{Error, decode};

/// One JavaScript binding ready for the linker store.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct JsBinding<'a> {
	pub(crate) module: &'a str,
	pub(crate) name: &'a str,
	pub(crate) js: String,
	pub(crate) embeds: Vec<Embed<'a>>,
}

/// The JavaScript bindings and WAT shims emitted for one wire record.
pub(crate) struct RenderedGroup<'a> {
	pub(crate) bindings: Vec<JsBinding<'a>>,
	pub(crate) wat: Option<String>,
}

/// One generated closure binding and its matching Wasm shim.
pub(crate) struct RenderedClosureShim<'a> {
	pub(crate) name: String,
	pub(crate) js: String,
	pub(crate) embeds: Vec<Embed<'a>>,
	pub(crate) wat: String,
}

impl<'a> RenderedClosureShim<'a> {
	fn new(name: &str, js: String, mut embeds: Vec<Embed<'a>>, wat: String) -> Self {
		embeds.sort_unstable_by_key(|embed| (embed.module, embed.name));
		embeds.dedup_by_key(|embed| (embed.module, embed.name));
		Self {
			name: name.to_owned(),
			js,
			embeds,
			wat,
		}
	}
}

/// One rendered wire record.
pub(crate) enum RenderedRecord<'a> {
	Imports(RenderedGroup<'a>),
	Exports(RenderedGroup<'a>),
	Closure(closure::RenderedClosure<'a>),
}

pub(crate) fn decode_and_render(bytes: &[u8]) -> Result<RenderedRecord<'_>, Error> {
	Ok(match decode(bytes)? {
		Record::Imports(group) => RenderedRecord::Imports(import::render(&group)),
		Record::Exports(exports) => RenderedRecord::Exports(export::render(&exports)),
		Record::Closure(item) => RenderedRecord::Closure(closure::render(&item)),
	})
}
