//! Host-side rendering for Rust exports and closure dispatchers.

mod js;
mod wat;

use js_bindgen_wire::model::{Export, ExportInput, ExportInputKind};

use crate::wire::RenderedGroup;

pub(super) fn render<'a>(exports: &[Export<'a>]) -> RenderedGroup<'a> {
	RenderedGroup {
		bindings: exports.iter().map(js::render).collect(),
		wat: wat::render(exports),
	}
}

fn input_names(inputs: &[ExportInput<'_>]) -> Vec<String> {
	let mut value_index = 0;
	inputs
		.iter()
		.map(|input| match input.kind {
			ExportInputKind::ClosureData => "data".to_owned(),
			ExportInputKind::Value => {
				let name = format!("arg{value_index}");
				value_index += 1;
				name
			}
		})
		.collect()
}
