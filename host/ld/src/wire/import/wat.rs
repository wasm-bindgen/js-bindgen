use std::fmt::Write;

use js_bindgen_wire::WireImportKind;
use js_bindgen_wire::abi::WatType;
use js_bindgen_wire::model::{
	Import, ImportCatch, ImportErrorMode, ImportGroup, ImportOutput, ImportOutputAbi, Slot,
	WatCatch,
};

use crate::wire::wat::{WatImports, WatLocals, write_conversion};

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
	let mut shims = Vec::with_capacity(group.imports.len());
	for import in &group.imports {
		let catches = import
			.output
			.as_ref()
			.is_some_and(|output| output.error == ImportErrorMode::CatchInWasm);
		let catch = catches.then(|| {
			group_catch.expect("a Wasm-catching Result import has no Wasm catch metadata")
		});
		render_wat_import(&mut imports, import);

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
//   (func $js_sys.import.notify
//     (@sym (name "js_sys.import.notify"))
//     (param i32)))
//
// ;; fn identity(value: u32) -> u32
// (import "js_sys" "identity"
//   (func $js_sys.import.identity
//     (@sym (name "js_sys.import.identity"))
//     (param i32)
//     (result i32)))
//
// ;; fn wide(value: u128) -> u128
// (import "js_sys" "wide"
//   (func $js_sys.import.wide
//     (@sym (name "js_sys.import.wide"))
//     (param $retptr i32)
//     (param i64 i64)))
// ```
fn render_wat_import(imports: &mut WatImports, import: &Import<'_>) {
	let identifier = format!("{}.import.{}", import.module, import.name);
	let mut wat = format!(
		"(import \"{}\" \"{}\" (func ${identifier} \
		 (@sym (name \"{identifier}\"))",
		import.module, import.name,
	);

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Indirect { retptr, .. },
		..
	}) = import.output.as_ref()
	{
		write!(wat, " (param $retptr {})", retptr.slot.boundary())
			.expect("writing to a String cannot fail");
	}

	let input_types = import
		.inputs
		.iter()
		.flat_map(|input| &input.slots)
		.map(Slot::boundary)
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
		write!(wat, " (result {})", slot.boundary()).expect("writing to a String cannot fail");
	}

	wat.push_str("))");
	imports.insert(&identifier, wat);
}

// The three imports shown above are exposed to Rust through these `ABI` shims:
//
// ```wat
// ;; fn notify(value: u32)
// (func $js_sys.notify (@sym) (param $value_0 i32)
//   local.get $value_0
//   call $js_sys.import.notify (@reloc)
// )
//
// ;; fn identity(value: u32) -> u32
// (func $js_sys.identity (@sym) (param $value_0 i32) (result i32)
//   local.get $value_0
//   call $js_sys.import.identity (@reloc)
// )
//
// ;; fn wide(value: u128) -> u128
// (func $js_sys.wide (@sym) (param $retptr i32) (param $value_0 i64) (param $value_1 i64)
//   local.get $retptr
//   local.get $value_0
//   local.get $value_1
//   call $js_sys.import.wide (@reloc)
// )
// ```
fn render_shim(wat: &mut String, shim: Shim<'_, '_>) {
	let Shim {
		import,
		catch,
		locals,
	} = shim;
	write!(wat, "(func ${}.{} (@sym)", import.module, import.name)
		.expect("writing to a String cannot fail");
	if import.kind == WireImportKind::ClosureFactory {
		write!(wat, " (@comdat \"{}.{}\")", import.module, import.name)
			.expect("writing to a String cannot fail");
	}

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Indirect { retptr, .. },
		..
	}) = import.output.as_ref()
	{
		write!(wat, " (param $retptr {})", retptr.slot.abi)
			.expect("writing to a String cannot fail");
	}

	for input in &import.inputs {
		for (index, slot) in input.slots.iter().enumerate() {
			write!(wat, " (param ${}_{index} {})", input.name, slot.abi)
				.expect("writing to a String cannot fail");
		}
	}

	if let Some(ImportOutput {
		abi: ImportOutputAbi::Direct { slot, .. },
		..
	}) = import.output.as_ref()
	{
		write!(wat, " (result {})", slot.abi).expect("writing to a String cannot fail");
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
		"\n  call ${}.import.{} (@reloc)",
		import.module, import.name,
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
			let zero = slot.abi.zero();
			write!(wat, "\n  {zero}").expect("writing to a String cannot fail");
		}
	}

	wat.push_str("\n)");
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
