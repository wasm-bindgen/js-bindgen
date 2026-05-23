use std::ops::ControlFlow;

use anyhow::{Context, Result};
use serde::Serialize;
use wasmparser::{ExternalKind, FuncType, Payload, TypeRef, ValType};

#[derive(Serialize)]
pub struct MainMemory {
	module: String,
	name: String,
}

#[derive(Default)]
pub struct BinaryParser {
	types: Vec<FuncType>,
	imported_functions: u32,
	defined_function_types: Vec<u32>,
	wasm64: bool,
	main_memory: Option<MainMemory>,
}

impl BinaryParser {
	pub fn parse(mut self, payload: &Payload<'_>) -> Result<ControlFlow<Option<MainExport>, Self>> {
		match payload {
			Payload::TypeSection(section) => {
				for ty in section.clone().into_iter_err_on_gc_types() {
					self.types.push(ty?);
				}
			}
			Payload::ImportSection(section) => {
				for import in section.clone().into_imports() {
					let import = import?;

					match import.ty {
						TypeRef::Func(_) | TypeRef::FuncExact(_) => self.imported_functions += 1,
						TypeRef::Memory(_) => {
							self.main_memory = Some(MainMemory {
								module: import.module.into(),
								name: import.name.into(),
							});
						}
						_ => {}
					}
				}
			}
			Payload::FunctionSection(section) => {
				for ty in section.clone() {
					self.defined_function_types.push(ty?);
				}
			}
			Payload::MemorySection(section) => {
				for memory in section.clone() {
					self.wasm64 |= memory?.memory64;
				}
			}
			Payload::ExportSection(exports) => {
				for export in exports.clone() {
					let export = export?;

					if export.name == "main"
						&& matches!(export.kind, ExternalKind::Func | ExternalKind::FuncExact)
						&& let Some(function_index) =
							export.index.checked_sub(self.imported_functions)
						&& let Some(type_index) =
							self.defined_function_types.get(function_index as usize)
						&& let Some(ty) = self.types.get(*type_index as usize)
					{
						if !is_main_type(ty, self.wasm64) {
							return Ok(ControlFlow::Break(None));
						}

						// Import sections must always precede export sections.
						let main_memory = self
							.main_memory
							.take()
							.context("main memory should be present")?;

						let main_export = MainExport {
							wasm64: self.wasm64,
							memory: main_memory,
						};

						return Ok(ControlFlow::Break(Some(main_export)));
					}
				}

				// Only one export section can exist.
				return Ok(ControlFlow::Break(None));
			}
			_ => {}
		}

		Ok(ControlFlow::Continue(self))
	}
}

pub struct MainExport {
	pub wasm64: bool,
	pub memory: MainMemory,
}

/// Rust's standard `main` export takes `argc`, `argv`, and returns a process
/// status code. The `argv` pointer follows the module's memory width.
fn is_main_type(ty: &FuncType, wasm64: bool) -> bool {
	let ptr_type = if wasm64 { ValType::I64 } else { ValType::I32 };
	ty.params() == [ValType::I32, ptr_type] && ty.results() == [ValType::I32]
}
