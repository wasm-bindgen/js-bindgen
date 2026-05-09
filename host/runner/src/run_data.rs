use anyhow::Result;
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
	Binary,
}

pub fn has_main_export(wasm_bytes: &[u8]) -> Result<bool> {
	let mut types = Vec::new();
	let mut imported_functions = 0;
	let mut defined_function_types = Vec::new();

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

					if matches!(import.ty, TypeRef::Func(_) | TypeRef::FuncExact(_)) {
						imported_functions += 1;
					}
				}
			}
			Payload::FunctionSection(section) => {
				for ty in section {
					defined_function_types.push(ty?);
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
						return Ok(is_main_type(ty));
					}
				}

				break;
			}
			_ => {}
		}
	}

	Ok(false)
}

/// Rust's standard `main` export takes `argc/argv` pointers and returns a
/// process status code.
fn is_main_type(ty: &FuncType) -> bool {
	ty.params() == [ValType::I32, ValType::I32] && ty.results() == [ValType::I32]
}
