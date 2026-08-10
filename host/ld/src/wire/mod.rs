//! Rendering of decoded `js-sys` wire records.

mod export;
mod import;
mod js;
mod wat;

use js_bindgen_wire::{
	Error, decode,
	model::{Embed, Record},
};

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

/// One Rust export with its JavaScript binding and Wasm boundary shim.
pub(crate) enum RenderedExport<'a> {
	Symbol {
		binding: JsBinding<'a>,
		shim: String,
	},
	Closure {
		binding: JsBinding<'a>,
		shim: String,
	},
}

/// Whether a rendered group describes imports or exports.
pub(crate) enum RenderedRecord<'a> {
	Imports(RenderedGroup<'a>),
	Exports(Vec<RenderedExport<'a>>),
}

pub(crate) fn decode_and_render(bytes: &[u8]) -> Result<RenderedRecord<'_>, Error> {
	Ok(match decode(bytes)? {
		Record::Imports(group) => RenderedRecord::Imports(import::render(&group)),
		Record::Exports(exports) => RenderedRecord::Exports(export::render(&exports)),
	})
}
