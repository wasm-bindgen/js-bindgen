use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser as _;
use js_bindgen_cli_lib::{JS_OUTPUT_SECTION, JsOutput};
use js_bindgen_shared::ReadFile;
use wasm_encoder::{Module, RawSection};
use wasmparser::{MemoryType, Parser, Payload, TypeRef};

#[derive(clap::Parser)]
#[command(name = "js-bindgen", version, about, long_about = None)]
struct Cli {
	/// Final linked Wasm artifact containing js-bindgen `metadata`.
	input: PathBuf,

	/// Directory in which to write the generated `.wasm` and `.mjs` files.
	#[arg(short, long)]
	out_dir: PathBuf,

	/// Keep custom sections other than `js_bindgen.js_output` in the output
	/// Wasm.
	#[arg(long)]
	keep_custom_sections: bool,
}

fn main() -> Result<()> {
	Cli::parse().run()
}

impl Cli {
	fn run(self) -> Result<()> {
		let input = ReadFile::new(&self.input)
			.with_context(|| format!("failed to read Wasm file: {}", self.input.display()))?;
		let output = process(&input, self.keep_custom_sections)?;
		let file_name = self
			.input
			.file_name()
			.context("input path must have a file name")?;

		fs::create_dir_all(&self.out_dir).with_context(|| {
			format!(
				"failed to create output directory: {}",
				self.out_dir.display()
			)
		})?;

		let wasm_path = self.out_dir.join(file_name);
		let js_path = wasm_path.with_extension("mjs");

		fs::write(&wasm_path, output.wasm)
			.with_context(|| format!("failed to write Wasm file: {}", wasm_path.display()))?;
		fs::write(&js_path, output.js)
			.with_context(|| format!("failed to write JS file: {}", js_path.display()))?;

		println!("{}", wasm_path.display());
		println!("{}", js_path.display());

		Ok(())
	}
}

struct Output {
	wasm: Vec<u8>,
	js: Vec<u8>,
}

struct Memory<'a> {
	module: &'a str,
	name: &'a str,
	data: MemoryType,
}

fn process(input: &[u8], keep_custom_sections: bool) -> Result<Output> {
	let mut module = Module::new();
	let mut js_output = None;
	let mut memories = Vec::new();

	for payload in Parser::new(0).parse_all(input) {
		let payload = payload.context("input should be valid Wasm")?;
		let section = payload.as_section();

		match payload {
			Payload::ImportSection(imports) => {
				for import in imports.into_imports() {
					let import = import.context("import should be parsable")?;

					if let TypeRef::Memory(data) = import.ty {
						memories.push(Memory {
							module: import.module,
							name: import.name,
							data,
						});
					}
				}

				copy_section(&mut module, input, section)?;
			}
			Payload::CustomSection(custom) => {
				if custom.name() == JS_OUTPUT_SECTION {
					js_output = Some(
						postcard::from_bytes(custom.data())
							.context("JS output section should be valid")?,
					);
				} else if keep_custom_sections {
					copy_section(&mut module, input, section)?;
				}
			}
			Payload::Version { .. } | Payload::CodeSectionEntry(_) | Payload::End(_) => {}
			_ => copy_section(&mut module, input, section)?,
		}
	}

	let js_output: JsOutput<&str> = js_output.context("unable to find JS output section")?;
	let main_memory = memories
		.iter()
		.find(|memory| {
			memory.module == js_output.main_memory.module
				&& memory.name == js_output.main_memory.name
		})
		.context("unable to find the encoded main memory import")?;
	let mut js = Vec::new();

	js_output.js(&mut js, main_memory.data)?;

	Ok(Output {
		wasm: module.finish(),
		js,
	})
}

fn copy_section(
	output: &mut Module,
	input: &[u8],
	section: Option<(u8, core::ops::Range<usize>)>,
) -> Result<()> {
	let (id, range) = section.context("expected a complete Wasm section")?;

	output.section(&RawSection {
		id,
		data: &input[range],
	});

	Ok(())
}
