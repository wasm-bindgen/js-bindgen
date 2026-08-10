use std::fmt::Write;

use js_bindgen_wire::abi::WatType;
use js_bindgen_wire::model::{Callee, Export, ExportInput, ExportInputKind, ExportOutput, Slot};

use crate::wire::wat::{WatImports, WatLocals, quoted, write_conversion};

use super::input_names;

struct ExportRenderer<'export, 'wire> {
	index: usize,
	export: &'export Export<'wire>,
}

/// Renders one WAT module fragment containing all decoded exports.
pub(super) fn render(exports: &[Export<'_>]) -> Option<String> {
	if exports.is_empty() {
		return None;
	}

	let pointer_type = exports[0].pointer_type();
	let mut imports = WatImports::default();
	let mut has_closure = false;
	let mut has_indirect = false;
	// WAT requires imports before the types and functions which use them.
	for (index, export) in exports.iter().enumerate() {
		let renderer = ExportRenderer { index, export };
		has_closure |= renderer.is_closure();
		has_indirect |= renderer.is_indirect();
		for_each_conversion_slot(export, |slot| {
			imports.extend(slot.imports());
		});
		if let Callee::Symbol { name } = renderer.export.callee {
			let identifier = format!("js_sys.export.symbol.{index}");
			imports.insert(
				&identifier,
				renderer.render_symbol_import(&identifier, name),
			);
		}
	}

	if has_closure {
		imports.insert(
			"js_sys.closure.table",
			format!(
				"(import \"env\" \"__indirect_function_table\" (table $js_sys.closure.table (@sym \
				 (name \"__indirect_function_table\")) {pointer_type} 0 funcref))",
			),
		);
	}

	if has_indirect {
		imports.insert(
			"__stack_pointer",
			format!(
				"(import \"env\" \"__stack_pointer\" (global $__stack_pointer (mut \
				 {pointer_type})))",
			),
		);
	}

	let mut wat = imports.render();

	for (index, export) in exports.iter().enumerate() {
		let renderer = ExportRenderer { index, export };
		if renderer.is_closure() {
			if !wat.is_empty() {
				wat.push('\n');
			}
			renderer.write_closure_type(&mut wat);
		}
	}

	for (index, export) in exports.iter().enumerate() {
		if !wat.is_empty() {
			wat.push('\n');
		}
		ExportRenderer { index, export }.write_shim(&mut wat);
	}

	Some(wat)
}

impl ExportRenderer<'_, '_> {
	fn pointer_type(&self) -> WatType {
		self.export.pointer_type()
	}

	fn is_indirect(&self) -> bool {
		self.export
			.output
			.as_ref()
			.is_some_and(|output| !output.is_direct())
	}

	fn is_closure(&self) -> bool {
		matches!(self.export.callee, Callee::Closure { .. })
	}

	fn closure_data_slot(&self) -> &Slot<'_> {
		self.export
			.inputs
			.iter()
			.find(|input| input.kind == ExportInputKind::ClosureData)
			.and_then(|input| input.slots.first())
			.expect("a closure export has one data slot")
	}

	fn render_symbol_import(&self, identifier: &str, symbol: &str) -> String {
		let mut wat = format!(
			"(import \"env\" \"symbol\" (func ${identifier} (@sym (name {}))",
			quoted(symbol),
		);
		if self.is_indirect() {
			write!(wat, " (param {})", self.pointer_type())
				.expect("writing to a String cannot fail");
		}
		for input in &self.export.inputs {
			if input.slots.is_empty() {
				continue;
			}
			wat.push_str(" (param");
			for slot in &input.slots {
				write!(wat, " {}", slot.rust).expect("writing to a String cannot fail");
			}
			wat.push(')');
		}
		if let Some(ExportOutput::Direct { slot, .. }) = self.export.output.as_ref() {
			write!(wat, " (result {})", slot.rust).expect("writing to a String cannot fail");
		}
		wat.push_str("))");
		wat
	}

	fn write_closure_type(&self, wat: &mut String) {
		write!(wat, "(type $js_sys.closure.call.{} (func", self.index)
			.expect("writing to a String cannot fail");
		if self.is_indirect() {
			write!(wat, " (param {})", self.pointer_type())
				.expect("writing to a String cannot fail");
		}
		write!(wat, " (param {})", self.pointer_type()).expect("writing to a String cannot fail");
		for input in &self.export.inputs {
			if input.kind != ExportInputKind::Value || input.slots.is_empty() {
				continue;
			}
			wat.push_str(" (param");
			for slot in &input.slots {
				write!(wat, " {}", slot.rust).expect("writing to a String cannot fail");
			}
			wat.push(')');
		}
		if let Some(ExportOutput::Direct { slot, .. }) = self.export.output.as_ref() {
			write!(wat, " (result {})", slot.rust).expect("writing to a String cannot fail");
		}
		wat.push_str("))");
	}

	fn write_shim(&self, wat: &mut String) {
		let names = input_names(&self.export.inputs);
		write!(
			wat,
			"(func $js_sys.export.{} (@sym (name {}))",
			self.index,
			quoted(self.export.name),
		)
		.expect("writing to a String cannot fail");
		if self.is_closure() {
			let comdat = closure_comdat(self.export.name);
			write!(wat, " (@comdat {})", quoted(&comdat)).expect("writing to a String cannot fail");
		}
		for (input, name) in self.export.inputs.iter().zip(&names) {
			for (slot_index, slot) in input.slots.iter().enumerate() {
				let js = slot.js();
				if input.kind == ExportInputKind::ClosureData {
					write!(wat, " (param ${name} {js})")
				} else {
					write!(wat, " (param ${name}_{slot_index} {js})")
				}
				.expect("writing to a String cannot fail");
			}
		}
		if let Some(output) = self.export.output.as_ref() {
			match output {
				ExportOutput::Direct { slot, .. } => {
					write!(wat, " (result {})", slot.js())
						.expect("writing to a String cannot fail");
				}
				ExportOutput::Indirect { frame, .. } => {
					wat.push_str(" (result");
					for frame_slot in &frame.slots {
						write!(wat, " {}", frame_slot.slot.js())
							.expect("writing to a String cannot fail");
					}
					wat.push(')');
				}
			}
		}

		self.write_prologue(wat);
		self.write_call(wat, &names);
		self.write_epilogue(wat);
		wat.push_str("\n)");
	}

	fn write_prologue(&self, wat: &mut String) {
		if self.is_indirect() {
			write!(wat, "\n  (local $retptr {})", self.pointer_type())
				.expect("writing to a String cannot fail");
		}
		if self.is_closure() {
			write!(
				wat,
				"\n  (local $js_sys.closure.data {})",
				self.pointer_type()
			)
			.expect("writing to a String cannot fail");
		}

		let mut locals = WatLocals::default();
		for_each_conversion_slot(self.export, |slot| {
			locals.extend(slot.locals());
		});
		locals.write_into(wat);

		if self.is_closure() {
			let slot = self.closure_data_slot();
			wat.push_str("\n  local.get $data");
			write_conversion(wat, slot.instruction());
			wat.push_str("\n  local.set $js_sys.closure.data");
		}

		if let Some(ExportOutput::Indirect { frame, .. }) = self.export.output.as_ref() {
			write!(
				wat,
				"\n  global.get $__stack_pointer\n  {}.const {}\n  {}.sub\n  local.tee $retptr\n  \
				 global.set $__stack_pointer",
				self.pointer_type(),
				frame.size,
				self.pointer_type(),
			)
			.expect("writing to a String cannot fail");
		}
	}

	fn write_call(&self, wat: &mut String, names: &[String]) {
		match self.export.callee {
			Callee::Symbol { .. } => {
				if self.is_indirect() {
					wat.push_str("\n  local.get $retptr");
				}
				for (input, name) in self.export.inputs.iter().zip(names) {
					write_abi_arguments(wat, input, name);
				}
				write!(
					wat,
					"\n  call $js_sys.export.symbol.{} (@reloc)",
					self.index,
				)
				.expect("writing to a String cannot fail");
			}
			Callee::Closure { call_shim_offset } => {
				if self.is_indirect() {
					wat.push_str("\n  local.get $retptr");
				}
				wat.push_str("\n  local.get $js_sys.closure.data");
				for (input, name) in self.export.inputs.iter().zip(names) {
					if input.kind == ExportInputKind::Value {
						write_abi_arguments(wat, input, name);
					}
				}
				write!(
					wat,
					"\n  local.get $js_sys.closure.data\n  {}.load offset={call_shim_offset}\n  \
					 call_indirect $js_sys.closure.table (type $js_sys.closure.call.{}) (@reloc)",
					self.pointer_type(),
					self.index,
				)
				.expect("writing to a String cannot fail");
			}
		}
	}

	fn write_epilogue(&self, wat: &mut String) {
		if let Some(output) = self.export.output.as_ref() {
			match output {
				ExportOutput::Direct { slot, .. } => {
					write_conversion(wat, slot.instruction());
				}
				ExportOutput::Indirect { frame, .. } => {
					for frame_slot in &frame.slots {
						write!(
							wat,
							"\n  local.get $retptr\n  {}.load offset={}",
							frame_slot.slot.rust, frame_slot.offset,
						)
						.expect("writing to a String cannot fail");
						write_conversion(wat, frame_slot.slot.instruction());
					}
					write!(
						wat,
						"\n  local.get $retptr\n  {}.const {}\n  {}.add\n  global.set \
						 $__stack_pointer",
						self.pointer_type(),
						frame.size,
						self.pointer_type(),
					)
					.expect("writing to a String cannot fail");
				}
			}
		}
	}
}

fn closure_comdat(name: &str) -> String {
	format!("js_sys.closure.call.{name}")
}

fn for_each_conversion_slot<'export, 'wire>(
	export: &'export Export<'wire>,
	mut visit: impl FnMut(&'export Slot<'wire>),
) {
	for input in &export.inputs {
		for slot in &input.slots {
			visit(slot);
		}
	}
	if let Some(output) = export.output.as_ref() {
		match output {
			ExportOutput::Direct { slot, .. } => visit(slot),
			ExportOutput::Indirect { frame, .. } => {
				for frame_slot in &frame.slots {
					visit(&frame_slot.slot);
				}
			}
		}
	}
}

fn write_abi_arguments(wat: &mut String, input: &ExportInput<'_>, name: &str) {
	for (slot_index, slot) in input.slots.iter().enumerate() {
		write!(wat, "\n  local.get ${name}_{slot_index}").expect("writing to a String cannot fail");
		write_conversion(wat, slot.instruction());
	}
}

#[cfg(test)]
mod tests {
	use std::rc::Rc;

	use js_bindgen_wire::PointerWidth;
	use js_bindgen_wire::abi::WatType;
	use js_bindgen_wire::model::{
		Callee, Export, ExportInput, ExportInputKind, ExportOutput, Slot, WatConversion,
	};

	use super::render;

	#[test]
	fn arbitrary_names_produce_valid_wat() {
		const NAME: &str = "single' double\" slash\\ line\n雪";
		let export = Export {
			module: NAME,
			name: NAME,
			pointer_width: PointerWidth::Wasm32,
			inputs: Vec::new(),
			output: None,
			embeds: Vec::new(),
			promising: false,
			callee: Callee::Symbol { name: NAME },
		};

		let wat = render(core::slice::from_ref(&export)).expect("one export produces WAT");
		js_bindgen_ld_shared::wat_to_object(false, &wat).expect("escaped WAT should parse");
	}

	#[test]
	fn converted_slots_use_js_types_for_shim_and_rust_types_for_symbol() {
		let input = Slot {
			rust: WatType::I32,
			wat: Some(WatConversion {
				js: WatType::ExternRef,
				imports: Rc::from([]),
				locals: Rc::from([]),
				instruction: "drop\n  i32.const 0",
			}),
		};
		let output = Slot {
			rust: WatType::I32,
			wat: Some(WatConversion {
				js: WatType::ExternRef,
				imports: Rc::from([]),
				locals: Rc::from([]),
				instruction: "drop\n  ref.null extern",
			}),
		};
		let export = Export {
			module: "test",
			name: "converted",
			pointer_width: PointerWidth::Wasm32,
			inputs: vec![ExportInput {
				kind: ExportInputKind::Value,
				slots: vec![input],
				conversion: None,
			}],
			output: Some(ExportOutput::Direct {
				slot: output,
				js_conversion: None,
			}),
			embeds: Vec::new(),
			promising: false,
			callee: Callee::Symbol { name: "test.raw" },
		};

		let wat = render(core::slice::from_ref(&export)).expect("one export produces WAT");
		assert_eq!(
			wat,
			r#"(import "env" "symbol" (func $js_sys.export.symbol.0 (@sym (name "test.raw")) (param i32) (result i32)))
(func $js_sys.export.0 (@sym (name "converted")) (param $arg0_0 externref) (result externref)
  local.get $arg0_0
  drop
  i32.const 0
  call $js_sys.export.symbol.0 (@reloc)
  drop
  ref.null extern
)"#,
		);
		js_bindgen_ld_shared::wat_to_object(false, &wat).expect("rendered WAT should parse");
	}
}
