use std::collections::HashMap;
use std::fmt::Write;

use js_bindgen_wire::ImportShimKind;
use js_bindgen_wire::abi::WatType;
use js_bindgen_wire::model::{
	Import, ImportCatch, ImportErrorMode, ImportGroup, ImportOutput, ImportOutputAbi, Slot,
	WatCatch,
};

use crate::wire::wat::{WatImports, WatLocals, quote_string, write_conversion};

/// Renders the imported functions followed by their Rust `ABI` shims.
pub(super) fn render(group: &ImportGroup<'_>) -> Option<String> {
	if group.imports.is_empty() {
		return None;
	}

	let group_catch = match group.catch.as_ref() {
		Some(ImportCatch::Wasm(catch)) => Some(catch),
		Some(ImportCatch::JavaScript(_)) | None => None,
	};
	let mut imports = WatImports::default();
	let mut boundaries = HashMap::new();
	let mut shims = Vec::with_capacity(group.imports.len());
	for (index, import) in group.imports.iter().enumerate() {
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
		shims.push(Shim {
			index,
			boundary_index,
			import,
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
	catch: Option<&'group WatCatch<'wire>>,
	locals: WatLocals<'wire>,
}

// On Wasm32, imports without a return value, with a direct return, and with an
// indirect return respectively render as:
//
// ```wat
// ;; fn notify(value: u32)
// (import "js_sys" "notify"
//   (func $js_sys.import.boundary.0
//     (@sym (name "js_sys.import.notify"))
//     (param i32)))
//
// ;; fn identity(value: u32) -> u32
// (import "js_sys" "identity"
//   (func $js_sys.import.boundary.1
//     (@sym (name "js_sys.import.identity"))
//     (param i32)
//     (result i32)))
//
// ;; fn wide(value: u128) -> u128
// (import "js_sys" "wide"
//   (func $js_sys.import.boundary.2
//     (@sym (name "js_sys.import.wide"))
//     (param $retptr i32)
//     (param i64 i64)))
// ```
fn render_wat_import(imports: &mut WatImports, index: usize, import: &Import<'_>) {
	let identifier = boundary_identifier(index);
	let symbol = boundary_symbol(import);
	let mut wat = format!(
		"(import {} {} (func ${identifier} (@sym (name {}))",
		quote_string(import.module),
		quote_string(import.name),
		quote_string(&symbol),
	);

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Indirect { retptr, .. },
		..
	}) = import.output.as_ref()
	{
		write!(wat, " (param $retptr {})", retptr.slot.js())
			.expect("writing to a String cannot fail");
	}

	let input_types = import
		.inputs
		.iter()
		.flat_map(|input| &input.slots)
		.map(Slot::js)
		.map(WatType::as_str)
		.collect::<Vec<_>>();
	if !input_types.is_empty() {
		write!(wat, " (param {})", input_types.join(" ")).expect("writing to a String cannot fail");
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

// The three imports shown above are exposed to Rust through these `ABI` shims:
//
// ```wat
// ;; fn notify(value: u32)
// (func $js_sys.import.shim.0 (@sym (name "js_sys.notify")) (param $value_0 i32)
//   local.get $value_0
//   call $js_sys.import.boundary.0 (@reloc)
// )
//
// ;; fn identity(value: u32) -> u32
// (func $js_sys.import.shim.1 (@sym (name "js_sys.identity")) (param $value_0 i32) (result i32)
//   local.get $value_0
//   call $js_sys.import.boundary.1 (@reloc)
// )
//
// ;; fn wide(value: u128) -> u128
// (func $js_sys.import.shim.2 (@sym (name "js_sys.wide")) (param $retptr i32) (param $value_0 i64) (param $value_1 i64)
//   local.get $retptr
//   local.get $value_0
//   local.get $value_1
//   call $js_sys.import.boundary.2 (@reloc)
// )
// ```
fn render_shim(wat: &mut String, shim: Shim<'_, '_>) {
	let Shim {
		index,
		boundary_index,
		import,
		catch,
		locals,
	} = shim;
	write!(
		wat,
		"(func ${} (@sym (name {}))",
		shim_identifier(index),
		quote_string(&shim_symbol(import)),
	)
	.expect("writing to a String cannot fail");
	if import.shim_kind == ImportShimKind::ClosureFactory {
		write!(wat, " (@comdat {})", quote_string(&shim_symbol(import)))
			.expect("writing to a String cannot fail");
	}

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Indirect { retptr, .. },
		..
	}) = import.output.as_ref()
	{
		write!(wat, " (param $retptr {})", retptr.slot.rust)
			.expect("writing to a String cannot fail");
	}

	for input in &import.inputs {
		for (index, slot) in input.slots.iter().enumerate() {
			write!(wat, " (param ${}_{index} {})", input.name, slot.rust)
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

	let locals = locals.render();
	if !locals.is_empty() {
		wat.push('\n');
		wat.push_str(&locals);
	}

	if let Some(catch) = catch {
		wat.push_str(catch.try_);
	}

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Indirect { retptr, .. },
		..
	}) = import.output.as_ref()
	{
		write_slot_get(wat, "$retptr", &retptr.slot);
	}

	for input in &import.inputs {
		for (index, slot) in input.slots.iter().enumerate() {
			write!(wat, "\n  local.get ${}_{index}", input.name)
				.expect("writing to a String cannot fail");
			if let Some(instruction) = slot.instruction() {
				write_conversion(wat, instruction);
			}
		}
	}

	write!(
		wat,
		"\n  call ${} (@reloc)",
		boundary_identifier(boundary_index),
	)
	.expect("writing to a String cannot fail");

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Direct { slot, .. },
		..
	}) = import.output.as_ref()
		&& let Some(instruction) = slot.instruction()
	{
		write_conversion(wat, instruction);
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

fn boundary_identifier(index: usize) -> String {
	format!("js_sys.import.boundary.{index}")
}

fn shim_identifier(index: usize) -> String {
	format!("js_sys.import.shim.{index}")
}

fn boundary_symbol(import: &Import<'_>) -> String {
	format!("{}.import.{}", import.module, import.name)
}

fn shim_symbol(import: &Import<'_>) -> String {
	format!("{}.{}", import.module, import.name)
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

fn write_slot_get(wat: &mut String, local: &str, slot: &Slot<'_>) {
	write!(wat, "\n  local.get {local}").expect("writing to a String cannot fail");
	if let Some(instruction) = slot.instruction() {
		write_conversion(wat, instruction);
	}
}

#[cfg(test)]
mod tests {
	use std::rc::Rc;

	use js_bindgen_wire::ImportShimKind;
	use js_bindgen_wire::abi::WatType;
	use js_bindgen_wire::model::{Import, ImportGroup, ImportInput, Slot, WatConversion};

	use super::render;

	#[test]
	fn arbitrary_names_produce_valid_wat() {
		const NAME: &str = "single' double\" slash\\ line\n雪";
		let group = ImportGroup {
			catch: None,
			imports: vec![Import {
				module: NAME,
				name: NAME,
				shim_kind: ImportShimKind::ClosureFactory,
				inputs: Vec::new(),
				output: None,
				binding: None,
				suspending: false,
			}],
		};

		let wat = render(&group).expect("one import produces WAT");
		js_bindgen_ld_shared::wat_to_object(false, &wat).expect("escaped WAT should parse");
	}

	#[test]
	fn converted_slot_uses_js_type_for_import_and_rust_type_for_shim() {
		let group = ImportGroup {
			catch: None,
			imports: vec![Import {
				module: "test",
				name: "converted",
				shim_kind: ImportShimKind::Normal,
				inputs: vec![ImportInput {
					name: "value",
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
(func $js_sys.import.shim.0 (@sym (name "test.converted")) (param $value_0 i32)
  local.get $value_0
  drop
  ref.null extern
  call $js_sys.import.boundary.0 (@reloc)
)"#,
		);
		js_bindgen_ld_shared::wat_to_object(false, &wat).expect("rendered WAT should parse");
	}
}
