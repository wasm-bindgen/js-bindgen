use std::fmt::Write;

use crate::wire::wat::{WatImports, WatLocals, write_conversion};
use js_bindgen_wire::abi::WatType;
use js_bindgen_wire::model::{Callee, Export, ExportInput, ExportInputKind, ExportOutput, Slot};

struct ExportRenderer<'export, 'wire> {
	index: usize,
	export: &'export Export<'wire>,
}

impl<'export, 'wire> ExportRenderer<'export, 'wire> {
	fn new(index: usize, export: &'export Export<'wire>) -> Self {
		Self { index, export }
	}
}

/// Renders one WAT module fragment containing all decoded exports.
pub(super) fn render(exports: &[Export<'_>]) -> Option<String> {
	if exports.is_empty() {
		return None;
	}

	let pointer_type = exports[0].pointer_type();
	let mut items = Vec::with_capacity(exports.len() * 2 + 2);
	let mut imports = WatImports::default();
	for (index, export) in exports.iter().enumerate() {
		let renderer = ExportRenderer::new(index, export);
		for_each_conversion_slot(export, |slot| {
			imports.extend(slot.imports());
		});
		if let Callee::Symbol { name } = renderer.export.callee {
			let mut identifier = String::new();
			renderer.write_symbol_identifier(&mut identifier);
			imports.insert(
				&identifier,
				renderer.render_symbol_import(&identifier, name),
			);
		}
	}

	if exports
		.iter()
		.any(|export| matches!(export.callee, Callee::Closure { .. }))
	{
		imports.insert(
			"js_sys.closure.table",
			format!(
				"(import \"env\" \"__indirect_function_table\" (table \
			 $js_sys.closure.table (@sym (name \"__indirect_function_table\")) \
			 {pointer_type} 0 funcref))",
			),
		);
	}

	if exports.iter().any(|export| {
		export
			.output
			.as_ref()
			.is_some_and(|output| !output.is_direct())
	}) {
		imports.insert(
			"__stack_pointer",
			format!(
				"(import \"env\" \"__stack_pointer\" \
			 (global $__stack_pointer (mut {pointer_type})))",
			),
		);
	}

	let imports = imports.render();
	if !imports.is_empty() {
		items.push(imports);
	}

	for (index, export) in exports.iter().enumerate() {
		let renderer = ExportRenderer::new(index, export);
		if renderer.is_closure() {
			items.push(renderer.render_closure_type());
		}
	}

	for (index, export) in exports.iter().enumerate() {
		items.push(ExportRenderer::new(index, export).render_shim());
	}

	Some(items.join("\n"))
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

	fn write_symbol_identifier(&self, output: &mut String) {
		write!(output, "js_sys.export.symbol.{}", self.index)
			.expect("writing to a String cannot fail");
	}

	fn render_symbol_import(&self, identifier: &str, symbol: &str) -> String {
		let retptr = if self.is_indirect() {
			format!(" (param {})", self.pointer_type())
		} else {
			String::new()
		};
		let mut parameters = String::new();
		for input in &self.export.inputs {
			if input.slots.is_empty() {
				continue;
			}
			parameters.push_str(" (param");
			for slot in &input.slots {
				write!(parameters, " {}", slot.abi).expect("writing to a String cannot fail");
			}
			parameters.push(')');
		}
		let result = match self.export.output.as_ref() {
			Some(ExportOutput::Direct { slot, .. }) => format!(" (result {})", slot.abi),
			_ => String::new(),
		};

		format!(
			"(import \"env\" \"symbol\" (func ${identifier} \
			 (@sym (name \"{symbol}\")){retptr}{parameters}{result}))",
		)
	}

	fn render_closure_type(&self) -> String {
		let retptr = if self.is_indirect() {
			format!(" (param {})", self.pointer_type())
		} else {
			String::new()
		};
		let mut parameters = String::new();
		for input in &self.export.inputs {
			if input.kind != ExportInputKind::Value || input.slots.is_empty() {
				continue;
			}
			parameters.push_str(" (param");
			for slot in &input.slots {
				write!(parameters, " {}", slot.abi).expect("writing to a String cannot fail");
			}
			parameters.push(')');
		}
		let result = match self.export.output.as_ref() {
			Some(ExportOutput::Direct { slot, .. }) => format!(" (result {})", slot.abi),
			_ => String::new(),
		};

		format!(
			"(type $js_sys.closure.call.{} (func{retptr} \
			 (param {}){parameters}{result}))",
			self.index,
			self.pointer_type(),
		)
	}

	fn render_shim(&self) -> String {
		let parameters = self
			.export
			.inputs
			.iter()
			.flat_map(|input| {
				input.slots.iter().enumerate().map(|(slot_index, slot)| {
					let boundary = slot.boundary();
					if input.kind == ExportInputKind::ClosureData {
						format!(" (param $data {boundary})")
					} else {
						format!(" (param ${}_{slot_index} {boundary})", input.name)
					}
				})
			})
			.collect::<String>();
		let result = self
			.export
			.output
			.as_ref()
			.map_or_else(String::new, |output| match output {
				ExportOutput::Direct { slot, .. } => format!(" (result {})", slot.boundary()),
				ExportOutput::Indirect { frame, .. } => {
					let types = frame
						.slots
						.iter()
						.map(|frame_slot| frame_slot.slot.boundary().as_str())
						.collect::<Vec<_>>()
						.join(" ");
					if types.is_empty() {
						" (result)".to_owned()
					} else {
						format!(" (result {types})")
					}
				}
			});

		let mut wat = format!(
			"(func $js_sys.export.{} (@sym (name \"{}\")){parameters}{result}",
			self.index, self.export.name,
		);
		self.write_prologue(&mut wat);
		self.write_call(&mut wat);
		self.write_epilogue(&mut wat);
		wat.push_str("\n)");
		wat
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
		let locals = locals.render();
		if !locals.is_empty() {
			wat.push('\n');
			wat.push_str(&locals);
		}

		if self.is_closure() {
			let slot = self.closure_data_slot();
			wat.push_str("\n  local.get $data");
			if let Some(instruction) = slot.instruction() {
				write_conversion(wat, instruction);
			}
			wat.push_str("\n  local.set $js_sys.closure.data");
		}

		if let Some(ExportOutput::Indirect { frame, .. }) = self.export.output.as_ref() {
			write!(
				wat,
				"\n  global.get $__stack_pointer\n  {}.const {}\n  \
				 {}.sub\n  local.tee $retptr\n  global.set $__stack_pointer",
				self.pointer_type(),
				frame.size,
				self.pointer_type(),
			)
			.expect("writing to a String cannot fail");
		}
	}

	fn write_call(&self, wat: &mut String) {
		match self.export.callee {
			Callee::Symbol { .. } => {
				if self.is_indirect() {
					wat.push_str("\n  local.get $retptr");
				}
				for input in &self.export.inputs {
					write_abi_arguments(wat, input);
				}
				wat.push_str("\n  call $");
				self.write_symbol_identifier(wat);
				wat.push_str(" (@reloc)");
			}
			Callee::Closure { call_shim_offset } => {
				if self.is_indirect() {
					wat.push_str("\n  local.get $retptr");
				}
				wat.push_str("\n  local.get $js_sys.closure.data");
				for input in self
					.export
					.inputs
					.iter()
					.filter(|input| input.kind == ExportInputKind::Value)
				{
					write_abi_arguments(wat, input);
				}
				write!(
					wat,
					"\n  local.get $js_sys.closure.data\n  \
				 {}.load offset={call_shim_offset}\n  \
				 call_indirect $js_sys.closure.table \
				 (type $js_sys.closure.call.{}) (@reloc)",
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
					if let Some(instruction) = slot.instruction() {
						write_conversion(wat, instruction);
					}
				}
				ExportOutput::Indirect { frame, .. } => {
					for frame_slot in &frame.slots {
						write!(
							wat,
							"\n  local.get $retptr\n  {}.load offset={}",
							frame_slot.slot.abi, frame_slot.offset,
						)
						.expect("writing to a String cannot fail");
						if let Some(instruction) = frame_slot.slot.instruction() {
							write_conversion(wat, instruction);
						}
					}
					write!(
						wat,
						"\n  local.get $retptr\n  {}.const {}\n  \
					 {}.add\n  global.set $__stack_pointer",
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

fn write_abi_arguments(wat: &mut String, input: &ExportInput<'_>) {
	for (slot_index, slot) in input.slots.iter().enumerate() {
		write!(wat, "\n  local.get ${}_{slot_index}", input.name)
			.expect("writing to a String cannot fail");
		if let Some(instruction) = slot.instruction() {
			write_conversion(wat, instruction);
		}
	}
}
