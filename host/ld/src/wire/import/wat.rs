use std::collections::HashMap;
use std::fmt::Write;

use js_bindgen_wire::model::{
	Import, ImportCatch, ImportErrorMode, ImportGroup, ImportOutput, ImportOutputAbi, Slot,
	WatCatch,
};

use crate::wire::wat::{WatImports, WatLocals, quoted, write_conversion};

use super::input_name;

/// Renders the imported functions followed by their Rust `ABI` shims.
pub(super) fn render(group: &ImportGroup<'_>) -> Option<String> {
	render_impl(&group.imports, group.catch.as_ref(), None)
}

pub(super) fn render_closure_factory(import: &Import<'_>, raw_symbol: &str) -> String {
	render_impl(core::slice::from_ref(import), None, Some(raw_symbol))
		.expect("one closure factory produces one Wasm import shim")
}

fn render_impl(
	group_imports: &[Import<'_>],
	group_catch: Option<&ImportCatch<'_>>,
	raw_shim_symbol: Option<&str>,
) -> Option<String> {
	if group_imports.is_empty() {
		return None;
	}

	let group_catch = match group_catch {
		Some(ImportCatch::Wasm(catch)) => Some(catch),
		Some(ImportCatch::JavaScript(_)) | None => None,
	};
	let mut imports = WatImports::default();
	let mut boundaries = HashMap::new();
	let mut shims = Vec::with_capacity(group_imports.len());
	// Conversion imports must precede every function body, so collect each
	// shim's inputs while the shared import set is still being built.
	for (index, import) in group_imports.iter().enumerate() {
		let boundary_index = *boundaries
			.entry((import.module, import.name))
			.or_insert(index);
		let catches = import
			.output
			.as_ref()
			.is_some_and(|output| output.error == ImportErrorMode::CatchInWasm);
		let catch = catches.then(|| {
			group_catch.expect("a Wasm-catching Result import has no Wasm catch metadata")
		});
		render_wat_import(&mut imports, boundary_index, import);

		let mut locals = WatLocals::default();
		for slot in conversion_slots(import) {
			imports.extend(slot.imports());
			locals.extend(slot.locals());
		}
		if let Some(catch) = catch {
			imports.extend(&catch.imports);
			locals.extend(&catch.locals);
		}
		let (symbol, comdat) = raw_shim_symbol.map_or_else(
			|| (shim_symbol(import), None),
			|raw_symbol| {
				(
					raw_symbol.to_owned(),
					Some(factory_comdat(raw_symbol, import.name)),
				)
			},
		);
		shims.push(Shim {
			index,
			boundary_index,
			import,
			symbol,
			comdat,
			catch,
			locals,
		});
	}

	let mut wat = imports.render();
	for shim in shims {
		wat.push('\n');
		render_shim(&mut wat, shim);
	}
	Some(wat)
}

struct Shim<'group, 'wire> {
	index: usize,
	boundary_index: usize,
	import: &'group Import<'wire>,
	symbol: String,
	comdat: Option<String>,
	catch: Option<&'group WatCatch<'wire>>,
	locals: WatLocals<'wire>,
}

fn render_wat_import(imports: &mut WatImports, index: usize, import: &Import<'_>) {
	let symbol = boundary_symbol(import);
	let mut wat = format!(
		"(import {} {} (func $js_sys.import.boundary.{index} (@sym (name {}))",
		quoted(import.module),
		quoted(import.name),
		quoted(&symbol),
	);

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Indirect { retptr, .. },
		..
	}) = import.output.as_ref()
	{
		write!(wat, " (param $retptr {})", retptr.slot.js())
			.expect("writing to a String cannot fail");
	}

	let mut wrote_parameter = false;
	for slot in import.inputs.iter().flat_map(|input| &input.slots) {
		if !wrote_parameter {
			wat.push_str(" (param");
			wrote_parameter = true;
		}
		write!(wat, " {}", slot.js()).expect("writing to a String cannot fail");
	}
	if wrote_parameter {
		wat.push(')');
	}

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Direct { slot, .. },
		..
	}) = import.output.as_ref()
	{
		write!(wat, " (result {})", slot.js()).expect("writing to a String cannot fail");
	}

	wat.push_str("))");
	imports.insert(&symbol, wat);
}

fn render_shim(wat: &mut String, shim: Shim<'_, '_>) {
	let Shim {
		index,
		boundary_index,
		import,
		symbol,
		comdat,
		catch,
		locals,
	} = shim;
	write!(
		wat,
		"(func $js_sys.import.shim.{index} (@sym (name {}))",
		quoted(&symbol),
	)
	.expect("writing to a String cannot fail");
	if let Some(comdat) = comdat {
		write!(wat, " (@comdat {})", quoted(&comdat)).expect("writing to a String cannot fail");
	}

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Indirect { retptr, .. },
		..
	}) = import.output.as_ref()
	{
		write!(wat, " (param $retptr {})", retptr.slot.rust)
			.expect("writing to a String cannot fail");
	}

	for (input_index, input) in import.inputs.iter().enumerate() {
		let name = input_name(input_index);
		for (slot_index, slot) in input.slots.iter().enumerate() {
			write!(wat, " (param ${name}_{slot_index} {})", slot.rust)
				.expect("writing to a String cannot fail");
		}
	}

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Direct { slot, .. },
		..
	}) = import.output.as_ref()
	{
		write!(wat, " (result {})", slot.rust).expect("writing to a String cannot fail");
	}

	locals.write_into(wat);

	if let Some(catch) = catch {
		wat.push_str(catch.try_);
	}

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Indirect { retptr, .. },
		..
	}) = import.output.as_ref()
	{
		wat.push_str("\n  local.get $retptr");
		write_conversion(wat, retptr.slot.instruction());
	}

	for (input_index, input) in import.inputs.iter().enumerate() {
		let name = input_name(input_index);
		for (slot_index, slot) in input.slots.iter().enumerate() {
			write!(wat, "\n  local.get ${name}_{slot_index}")
				.expect("writing to a String cannot fail");
			write_conversion(wat, slot.instruction());
		}
	}

	write!(
		wat,
		"\n  call $js_sys.import.boundary.{boundary_index} (@reloc)",
	)
	.expect("writing to a String cannot fail");

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Direct { slot, .. },
		..
	}) = import.output.as_ref()
	{
		write_conversion(wat, slot.instruction());
	}

	if let Some(catch) = catch {
		wat.push_str(catch.catch);
		if let Some(ImportOutput {
			abi: ImportOutputAbi::Direct { slot, .. },
			..
		}) = import.output.as_ref()
		{
			let zero = slot.rust.zero();
			write!(wat, "\n  {zero}").expect("writing to a String cannot fail");
		}
	}

	wat.push_str("\n)");
}

fn boundary_symbol(import: &Import<'_>) -> String {
	format!("{}.import.{}", import.module, import.name)
}

fn shim_symbol(import: &Import<'_>) -> String {
	format!("{}.{}", import.module, import.name)
}

fn factory_comdat(raw_symbol: &str, canonical_factory_name: &str) -> String {
	format!(
		"js_sys.closure.factory.{}.{raw_symbol}.{canonical_factory_name}",
		raw_symbol.len(),
	)
}

fn conversion_slots<'import, 'wire>(
	import: &'import Import<'wire>,
) -> impl Iterator<Item = &'import Slot<'wire>> {
	let retptr = import.output.as_ref().and_then(|output| match &output.abi {
		ImportOutputAbi::Indirect { retptr, .. } => Some(&retptr.slot),
		ImportOutputAbi::Direct { .. } => None,
	});
	let result = import.output.as_ref().and_then(|output| match &output.abi {
		ImportOutputAbi::Direct { slot, .. } => Some(slot),
		ImportOutputAbi::Indirect { .. } => None,
	});
	retptr
		.into_iter()
		.chain(import.inputs.iter().flat_map(|input| input.slots.iter()))
		.chain(result)
}

#[cfg(test)]
mod tests {
	use std::rc::Rc;

	use js_bindgen_wire::abi::WatType;
	use js_bindgen_wire::model::{Import, ImportGroup, ImportInput, Slot, WatConversion};

	use super::{factory_comdat, quoted, render, render_closure_factory};

	#[test]
	fn arbitrary_names_produce_valid_wat() {
		const NAME: &str = "single' double\" slash\\ line\n雪";
		let group = ImportGroup {
			catch: None,
			imports: vec![Import {
				module: NAME,
				name: NAME,
				inputs: Vec::new(),
				output: None,
				binding: None,
				suspending: false,
			}],
		};

		let wat = render(&group).expect("one import produces WAT");
		assert!(!wat.contains("@comdat"));
		js_bindgen_ld_shared::wat_to_object(false, &wat).expect("escaped WAT should parse");
	}

	#[test]
	fn closure_factory_uses_raw_symbol_and_canonical_name_in_comdat() {
		let canonical_name = "closure_new_mut_0123456789abcdef";
		let raw_symbol = "crate.raw_factory";
		let import = Import {
			module: "js_sys",
			name: canonical_name,
			inputs: Vec::new(),
			output: None,
			binding: None,
			suspending: false,
		};

		let wat = render_closure_factory(&import, raw_symbol);
		assert!(wat.contains("(@sym (name \"crate.raw_factory\"))"));
		assert!(wat.contains(&format!(
			"(@comdat {})",
			quoted(&factory_comdat(raw_symbol, canonical_name)),
		)));
		js_bindgen_ld_shared::wat_to_object(false, &wat).expect("rendered WAT should parse");
	}

	#[test]
	fn converted_slot_uses_js_type_for_import_and_rust_type_for_shim() {
		let group = ImportGroup {
			catch: None,
			imports: vec![Import {
				module: "test",
				name: "converted",
				inputs: vec![ImportInput {
					slots: vec![Slot {
						rust: WatType::I32,
						wat: Some(WatConversion {
							js: WatType::ExternRef,
							imports: Rc::from([]),
							locals: Rc::from([]),
							instruction: "drop\n  ref.null extern",
						}),
					}],
					js_conversion: None,
				}],
				output: None,
				binding: None,
				suspending: false,
			}],
		};

		let wat = render(&group).expect("one import produces WAT");
		assert_eq!(
			wat,
			r#"(import "test" "converted" (func $js_sys.import.boundary.0 (@sym (name "test.import.converted")) (param externref)))
(func $js_sys.import.shim.0 (@sym (name "test.converted")) (param $arg0_0 i32)
  local.get $arg0_0
  drop
  ref.null extern
  call $js_sys.import.boundary.0 (@reloc)
)"#,
		);
		js_bindgen_ld_shared::wat_to_object(false, &wat).expect("rendered WAT should parse");
	}
}
