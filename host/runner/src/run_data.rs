use std::ffi::OsString;

use anyhow::{Context, Ok, Result};
use serde::Serialize;
use wasmparser::{ExternalKind, FuncType, Parser, Payload, TypeRef, ValType};

use crate::test::TestEntry;

#[derive(Serialize)]
#[serde(
	tag = "kind",
	rename_all = "camelCase",
	rename_all_fields = "camelCase"
)]
pub enum RunData {
	Test {
		no_capture: bool,
		filtered_count: usize,
		tests: Vec<TestEntry>,
	},
	Binary {
		wasm64: bool,
		memory: MainMemory,
		args: Vec<OsString>,
	},
}

#[derive(Serialize)]
pub struct MainMemory {
	module: String,
	name: String,
}

pub fn main_export(wasm_bytes: &[u8]) -> Result<Option<MainExport>> {
	let mut types = Vec::new();
	let mut imported_functions = 0;
	let mut defined_function_types = Vec::new();
	let mut wasm64 = false;
	let mut main_memory = None;

	for payload in Parser::new(0).parse_all(wasm_bytes) {
		match payload? {
			Payload::TypeSection(section) => {
				for ty in section.into_iter_err_on_gc_types() {
					types.push(ty?);
				}
			}
			Payload::ImportSection(section) => {
				for import in section.into_imports() {
					let import = import?;

					match import.ty {
						TypeRef::Func(_) | TypeRef::FuncExact(_) => imported_functions += 1,
						TypeRef::Memory(_) => {
							main_memory = Some(MainMemory {
								module: import.module.into(),
								name: import.name.into(),
							});
						}
						_ => {}
					}
				}
			}
			Payload::FunctionSection(section) => {
				for ty in section {
					defined_function_types.push(ty?);
				}
			}
			Payload::MemorySection(section) => {
				for memory in section {
					wasm64 |= memory?.memory64;
				}
			}
			Payload::ExportSection(exports) => {
				for export in exports {
					let export = export?;

					if export.name == "main"
						&& matches!(export.kind, ExternalKind::Func | ExternalKind::FuncExact)
						&& let Some(function_index) = export.index.checked_sub(imported_functions)
						&& let Some(type_index) =
							defined_function_types.get(function_index as usize)
						&& let Some(ty) = types.get(*type_index as usize)
					{
						if !is_main_type(ty, wasm64) {
							return Ok(None);
						}

						// Import sections are always processed before export sections.
						let main_memory = main_memory.context("main memory should be present")?;
						return Ok(Some(MainExport {
							wasm64,
							memory: main_memory,
						}));
					}
				}
				break;
			}
			_ => {}
		}
	}

	Ok(None)
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
