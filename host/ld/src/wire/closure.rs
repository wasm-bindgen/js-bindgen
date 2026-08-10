//! Rendering and semantic identity for Rust closures.

use js_bindgen_wire::model::{Callee, Closure, Export};
use xxhash_rust::xxh3::Xxh3;

use super::{RenderedClosureShim, export, import};

pub(crate) struct RenderedClosure<'a> {
	pub(crate) factory_module: &'a str,
	pub(crate) factory: RenderedClosureShim<'a>,
	pub(crate) call: RenderedClosureShim<'a>,
}

pub(super) fn render<'a>(closure: &Closure<'a>) -> RenderedClosure<'a> {
	let call_hash = fingerprint([closure.call_identity]);
	let call_name = format!("closure_call_{call_hash:032x}");
	let call = render_call(closure, &call_name);

	let call_hash = call_hash.to_le_bytes();
	let factory_hash = fingerprint([
		closure.factory.helper.module.as_bytes(),
		closure.factory.helper.name.as_bytes(),
		call_hash.as_slice(),
	]);
	let factory_name = format!("closure_new_{factory_hash:032x}");
	let factory = import::render_closure_factory(&closure.factory, &factory_name, &call_name);

	RenderedClosure {
		factory_module: closure.factory.helper.module,
		factory,
		call,
	}
}

fn render_call<'a>(closure: &Closure<'a>, name: &str) -> RenderedClosureShim<'a> {
	let export = Export {
		// The export renderer only uses this field when building an ordinary
		// `JsBinding`; closure dispatchers have no public module of their own.
		module: closure.factory.helper.module,
		name,
		pointer_width: closure.pointer_width,
		inputs: closure.inputs.clone(),
		output: closure.output.clone(),
		embeds: closure.embeds.clone(),
		promising: false,
		callee: Callee::Closure {
			call_shim_offset: closure.call_shim_offset,
		},
	};
	let rendered = export::render(core::slice::from_ref(&export));
	let [binding] = rendered
		.bindings
		.try_into()
		.expect("one closure produces one dispatcher");
	RenderedClosureShim::new(
		name,
		binding.js,
		closure.embeds.clone(),
		rendered
			.wat
			.expect("one closure produces one Wasm dispatcher"),
	)
}

fn fingerprint<'a>(parts: impl IntoIterator<Item = &'a [u8]>) -> u128 {
	let mut hasher = Xxh3::new();
	for part in parts {
		hasher.update(&(part.len() as u64).to_le_bytes());
		hasher.update(part);
	}
	hasher.digest128()
}
