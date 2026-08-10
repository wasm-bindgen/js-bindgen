//! Rendering of JavaScript imports and their Wasm `ABI` shims.

mod js;
mod wat;

use js_bindgen_wire::model::{ClosureFactory, Import, ImportGroup};

use super::{RenderedClosureShim, RenderedGroup};

pub(super) fn render<'a>(group: &ImportGroup<'a>) -> RenderedGroup<'a> {
	RenderedGroup {
		bindings: js::render(group),
		wat: wat::render(group),
	}
}

fn input_name(index: usize) -> String {
	format!("arg{index}")
}

pub(super) fn render_closure_factory<'a>(
	factory: &ClosureFactory<'a>,
	name: &str,
	call_name: &str,
) -> RenderedClosureShim<'a> {
	let import = Import {
		module: factory.helper.module,
		name,
		inputs: vec![factory.input.clone()],
		output: Some(factory.output.clone()),
		binding: None,
		suspending: false,
	};
	let js = js::render_closure_factory(&import, factory.helper, call_name);
	let wat = wat::render_closure_factory(&import, factory.raw_symbol);
	let mut embeds = factory.embeds.clone();
	embeds.push(factory.helper);
	RenderedClosureShim::new(name, js, embeds, wat)
}
