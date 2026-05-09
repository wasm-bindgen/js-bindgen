use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use js_bindgen_shared::ReadFile;
use serde::Serialize;
use wasmparser::{ExternalKind, FuncType, Parser, Payload, TypeRef, ValType};

use crate::test::TestEntry;

const BENCH_FILE_NAME: &str = "benchmark.json";

#[derive(Serialize)]
#[serde(
	tag = "kind",
	rename_all = "camelCase",
	rename_all_fields = "camelCase"
)]
pub enum RunData {
	Test {
		ctx: Option<String>,
		no_capture: bool,
		filtered_count: usize,
		tests: Vec<TestEntry>,
	},
	Binary {
		ctx: Option<String>,
		wasm64: bool,
	},
}

#[derive(Serialize)]
pub struct BenchCtx {
	path: PathBuf,
	baseline: Option<String>,
}

impl BenchCtx {
	pub fn from_path(wasm_path: &Path) -> Result<Self> {
		let target_dir = wasm_path
			.ancestors()
			.find(|path| path.file_name().is_some_and(|name| name == "target"))
			.map(Path::to_path_buf)
			.with_context(|| {
				format!(
					"failed to locate Cargo target directory from wasm path: {}",
					wasm_path.display()
				)
			})?;

		let path = target_dir.join(BENCH_FILE_NAME);
		let baseline = match ReadFile::new(&path) {
			Ok(data) => Ok(std::str::from_utf8(&data).map(ToString::to_string).ok()),
			Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
			Err(error) => Err(error).with_context(|| {
				format!("failed to read benchmark baseline file: {}", path.display())
			}),
		}?;

		Ok(Self { path, baseline })
	}
}

pub fn main_export(wasm_bytes: &[u8]) -> Result<Option<MainExport>> {
	let mut types = Vec::new();
	let mut imported_functions = 0;
	let mut defined_function_types = Vec::new();
	let mut wasm64 = false;

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
						TypeRef::Memory(memory) => wasm64 |= memory.memory64,
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
						return Ok(is_main_type(ty, wasm64).then_some(MainExport { wasm64 }));
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
}

/// Rust's standard `main` export takes `argc`, `argv`, and returns a process
/// status code. The `argv` pointer follows the module's memory width.
fn is_main_type(ty: &FuncType, wasm64: bool) -> bool {
	let ptr_type = if wasm64 { ValType::I64 } else { ValType::I32 };
	ty.params() == [ValType::I32, ptr_type] && ty.results() == [ValType::I32]
}
