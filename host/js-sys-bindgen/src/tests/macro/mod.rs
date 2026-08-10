use std::io::Cursor;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

use anyhow::{Context, Result, anyhow, bail, ensure};
use cargo_metadata::{Artifact, CompilerMessage, Message, Target};
use itertools::Itertools;
use js_bindgen_ld_shared::{
	IMPORT_SECTION, JsBindgenJsSectionParser, JsBindgenWatSectionParser, WAT_SECTION,
};
use proc_macro2::TokenStream;
use syn::parse_quote;
use wasmparser::{Parser, Payload};

use crate::r#macro;

mod export;
mod function;
mod member;
mod r#type;

fn macro_error(input: syn::ItemForeignMod) -> String {
	let (_, error) = r#macro::expand_for_test(TokenStream::new(), input, "test_crate").unwrap_err();

	error.to_string()
}

fn inner(tmp: &Path, source: &str) -> Result<(Option<String>, Option<String>, Option<String>)> {
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

		[dependencies]
		js-sys = {{ path = '{}' }}
		"#,
		js_sys.display(),
	);
	fs::write(tmp.join("Cargo.toml"), cargo_toml)?;

	let js_test = r#macro::expand_for_test(
		TokenStream::new(),
		parse_quote! { extern "js-sys" { pub type JsTest; } },
		"test_crate",
	)
	.unwrap()
	.into_token_stream();

	let src = tmp.join("src");
	fs::create_dir(&src)?;
	let lib = src.join("lib.rs");
	fs::write(
		&lib,
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

			{js_test}

			fn assert_optional_js_test()
			where
				::core::option::Option<JsTest>:
					::js_sys::hazard::IntoJS + ::js_sys::hazard::FromJS,
			{{}}

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

		bail!("Cargo failed with status: {}", output.status)
	}

	let reader = Cursor::new(output.stdout);

	let mut wat_output = None;
	let mut js_import_output = None;
	let mut js_export_output = None;

	for message in Message::parse_stream(reader) {
		if let Message::CompilerArtifact(Artifact {
			target: Target { src_path, .. },
			filenames,
			..
		}) = message?
			&& src_path.canonicalize()? == lib.canonicalize()?
		{
			for filename in filenames {
				js_bindgen_ld_shared::ld_input_parser(filename.as_os_str(), |_, data, _| {
					for payload in Parser::new(0).parse_all(data) {
						let payload = payload?;

						match payload {
							Payload::CustomSection(c) if c.name() == WAT_SECTION => {
								let wat = JsBindgenWatSectionParser::new(&c)
									.exactly_one()
									.map_err(|wats| {
										anyhow!(
											"found multiple WAT outputs in a single section: \
											 {wats:?}"
										)
									})?;
								ensure!(wat_output.is_none(), "found multiple WAT outputs");
								wat_output = Some(wat.to_owned());
								js_bindgen_ld_shared::wat_to_object(false, wat).unwrap();
							}
							Payload::CustomSection(c) if c.name() == IMPORT_SECTION => {
								let mut parser = JsBindgenJsSectionParser::new(&c);

								let import = parser.next().unwrap();

								if import.module != "test_crate" {
									continue;
								}

								ensure!(
									parser.next().is_none(),
									"found multiple JS import outputs in a single section: \
									 {parser:?}"
								);

								js_import_output = Some(import.js.to_owned());
							}
							Payload::CustomSection(c) if c.name() == "js_bindgen.export" => {
								let mut parser = JsBindgenJsSectionParser::new(&c);
								let export = parser.next().unwrap();

								if export.module != "test_crate" {
									continue;
								}

								ensure!(
									parser.next().is_none(),
									"found multiple JS export outputs in a single section: \
									 {parser:?}"
								);

								js_export_output = Some(export.js.to_owned());
							}
							_ => (),
						}
					}

					Ok(())
				})??;
			}
		}
	}

	Ok((wat_output, js_import_output, js_export_output))
}
