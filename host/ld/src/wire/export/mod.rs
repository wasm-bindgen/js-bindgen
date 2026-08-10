//! Host-side rendering for Rust exports and closure dispatchers.

mod js;
mod wat;

use js_bindgen_wire::model::{Callee, Export};

use crate::wire::RenderedExport;

pub(super) fn render<'a>(exports: &[Export<'a>]) -> Vec<RenderedExport<'a>> {
	exports
		.iter()
		.map(|export| {
			let binding = js::render(export);
			let shim = wat::render(core::slice::from_ref(export))
				.expect("one export always produces a Wasm shim");

			match export.callee {
				Callee::Symbol { .. } => RenderedExport::Symbol { binding, shim },
				Callee::Closure { .. } => RenderedExport::Closure { binding, shim },
			}
		})
		.collect()
}
