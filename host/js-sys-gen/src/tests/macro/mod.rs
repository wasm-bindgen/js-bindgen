mod function;
mod r#type;

use std::ffi::OsStr;
use std::io::{self, Cursor};
use std::path::Path;
use std::process::Command;
use std::{env, fs};

use anyhow::{Context, Result};
use cargo_metadata::{Artifact, CompilerMessage, Message, Target};
use itertools::Itertools;
use js_bindgen_ld_shared::{JsBindgenAssemblySectionParser, JsBindgenImportSectionParser};
use proc_macro2::TokenStream;
use wasmparser::{Parser, Payload};

#[track_caller]
#[expect(clippy::needless_pass_by_value, reason = "test")]
fn test(
	attr: TokenStream,
	input: TokenStream,
	expected: TokenStream,
	assembly: impl Into<Option<&'static str>>,
	js_import: impl Into<Option<&'static str>>,
) {
	let output = crate::js_sys(attr.clone(), input.clone()).unwrap_or_else(|e| e);

	let output = syn::parse2(output).unwrap();
	let output = prettyplease::unparse(&output);
	let expected = syn::parse2(expected).unwrap();
	let expected = prettyplease::unparse(&expected);

	similar_asserts::assert_eq!(expected, output);

	let dir = tempfile::tempdir().unwrap();
	let (assembly_output, js_import_output) =
		inner(dir.path(), &format!("#[js_sys({attr})]\n{input}")).unwrap();

	let assembly = assembly.into();
	match (assembly, assembly_output) {
		(Some(assembly), Some(assembly_output)) => {
			similar_asserts::assert_eq!(assembly, assembly_output);
		}
		(None, None) => (),
		(assembly, assembly_output) => {
			similar_asserts::assert_eq!(assembly, assembly_output.as_deref());
		}
	}

	let js_import = js_import.into();
	match (js_import, js_import_output) {
		(Some(js_import), Some(js_import_output)) => {
			similar_asserts::assert_eq!(js_import, js_import_output);
		}
		(None, None) => (),
		(js_import, js_import_output) => {
			similar_asserts::assert_eq!(js_import, js_import_output.as_deref());
		}
	}
}

fn inner(tmp: &Path, source: &str) -> Result<(Option<String>, Option<String>)> {
	let js_sys = env::current_dir()?
		.parent()
		.and_then(Path::parent)
		.context("unexpected directory structure")?
		.join("client")
		.join("js-sys");
	let cargo_toml = indoc::formatdoc!(
		r#"[package]
		name = "test-crate"
		edition = "2024"
		publish = false
		resolver = "2"

		[dependencies]
		js-sys = {{ path = '{}' }}
		"#,
		js_sys.display(),
	);
	fs::write(tmp.join("Cargo.toml"), cargo_toml)?;

	let src = tmp.join("src");
	fs::create_dir(&src)?;
	let src = src.join("lib.rs");
	fs::write(
		&src,
		indoc::formatdoc!(
			r#"#![no_std]
			#![cfg_attr(target_arch = "wasm64", feature(simd_wasm64))]

			extern crate alloc;

			use alloc::alloc::{{GlobalAlloc, Layout}};
			#[cfg(target_arch = "wasm32")]
			use core::arch::wasm32::unreachable;
			#[cfg(target_arch = "wasm64")]
			use core::arch::wasm64::unreachable;

			use js_sys::*;

			#[panic_handler]
			fn panic(_: &core::panic::PanicInfo<'_>) -> ! {{
				unreachable();
			}}

			struct Allocator;

			unsafe impl GlobalAlloc for Allocator {{
				unsafe fn alloc(&self, _: Layout) -> *mut u8 {{
					unimplemented!()
				}}

				unsafe fn dealloc(&self, _: *mut u8, _: Layout) {{
					unimplemented!()
				}}
			}}

			#[global_allocator]
			static ALLOC: Allocator = Allocator;

			{source}
			"#
		),
	)?;

	let output = Command::new("cargo")
		.current_dir(tmp)
		.arg("build")
		.args(["--target", "wasm32-unknown-unknown"])
		.args(["--message-format", "json"])
		.output()?;

	if !output.status.success() {
		if !output.stderr.is_empty() {
			eprintln!(
				"------ cargo stderr ------\n{}",
				String::from_utf8_lossy(&output.stderr)
			);

			if !output.stderr.ends_with(b"\n") {
				eprintln!();
			}
		}

		let reader = Cursor::new(output.stdout);

		for message in Message::parse_stream(reader) {
			if let Message::CompilerMessage(CompilerMessage { message, .. }) = message? {
				println!("{message}");
			}
		}

		anyhow::bail!("Cargo failed with status: {}", output.status)
	}

	let reader = Cursor::new(output.stdout);

	let mut assembly_output = None;
	let mut js_import_output = None;

	for message in Message::parse_stream(reader) {
		if let Message::CompilerArtifact(Artifact {
			target: Target { src_path, .. },
			filenames,
			..
		}) = message?
			&& src_path.canonicalize()? == src.canonicalize()?
		{
			for filename in filenames {
				js_bindgen_ld_shared::ld_input_parser(filename.as_os_str(), |_, data| {
					for payload in Parser::new(0).parse_all(data) {
						let payload = payload?;

						match payload {
							Payload::CustomSection(c) if c.name() == "js_bindgen.assembly" => {
								let assembly = JsBindgenAssemblySectionParser::new(&c)
									.exactly_one()
									.map_err(|asms| {
										anyhow::anyhow!(
											"found multiple assembly outputs in a single section: \
											 {asms:?}"
										)
									})?;
								anyhow::ensure!(
									assembly_output.is_none(),
									"found multiple assembly outputs"
								);
								assembly_output = Some(assembly.to_owned());
								js_bindgen_ld_shared::assembly_to_object(
									OsStr::new("wasm32"),
									assembly,
									&mut io::sink(),
								)?;
							}
							Payload::CustomSection(c)
								if c.name().starts_with("js_bindgen.import.test_crate.") =>
							{
								let import = JsBindgenImportSectionParser::new(&c)
									.exactly_one()
									.map_err(|imports| {
										anyhow::anyhow!(
											"found multiple JS import outputs in a single \
											 section: {imports:?}"
										)
									})?
									.js();

								anyhow::ensure!(
									js_import_output.is_none(),
									"found multiple JS import outputs"
								);
								js_import_output = import.map(str::to_owned);
							}
							_ => (),
						}
					}

					Ok(())
				})?;
			}
		}
	}

	Ok((assembly_output, js_import_output))
}
