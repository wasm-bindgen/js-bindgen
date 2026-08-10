//! Rendering of JavaScript imports and their Wasm `ABI` shims.

mod js;
mod wat;

use js_bindgen_wire::model::ImportGroup;

use super::RenderedGroup;

pub(super) fn render<'a>(group: &ImportGroup<'a>) -> RenderedGroup<'a> {
	RenderedGroup {
		bindings: js::render(group),
		wat: wat::render(group),
	}
}
