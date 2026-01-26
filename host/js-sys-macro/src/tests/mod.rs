mod function;

use std::ffi::OsStr;
use std::io::{self, Cursor};
use std::path::Path;
use std::process::Command;
use std::{env, fs};

use anyhow::{Context, Result};
use cargo_metadata::{Artifact, CompilerMessage, Message, Target};
use itertools::Itertools;
use js_bindgen_ld_shared::CustomSectionParser;
use proc_macro2::TokenStream;
use rand::Rng;
use rand::distr::Alphanumeric;
use wasmparser::{Parser, Payload};

#[track_caller]
fn test(
	attr: TokenStream,
	input: TokenStream,
	expected: TokenStream,
	assembly: &str,
	js_import: &str,
) {
	let output = crate::js_sys(attr.clone(), input.clone());

	let output = syn::parse2(output).unwrap();
	let output = prettyplease::unparse(&output);
	let expected = syn::parse2(expected).unwrap();
	let expected = prettyplease::unparse(&expected);

	similar_asserts::assert_eq!(expected, output);

	let dir = env::temp_dir();
	let rand: String = rand::rng()
		.sample_iter(Alphanumeric)
		.take(32)
		.map(char::from)
		.collect();
	let dir = dir.join(rand);
	fs::create_dir(&dir).unwrap();

	let result = inner(&dir, &format!("#[js_sys({})]\n{}", attr, input));

	fs::remove_dir_all(dir).unwrap();

	let (assembly_output, js_import_output) = result.unwrap();

	similar_asserts::assert_eq!(assembly, assembly_output);
	similar_asserts::assert_eq!(js_import, js_import_output);
}

fn inner(tmp: &Path, source: &str) -> Result<(String, String)> {
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
		js-sys = {{ path = "{}" }}
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
					todo!()
				}}

				unsafe fn dealloc(&self, _: *mut u8, _: Layout) {{
					todo!()
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
			&& src_path == src
		{
			for filename in filenames {
				js_bindgen_ld_shared::ld_input_parser(filename.as_os_str(), |_, data| {
					for payload in Parser::new(0).parse_all(data) {
						let payload = payload?;

						match payload {
							Payload::CustomSection(c) if c.name() == "js_bindgen.assembly" => {
								let assembly = CustomSectionParser::new(c, false)
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
								let mut import: &[u8] = CustomSectionParser::new(c, true)
									.exactly_one()
									.map_err(|imports| {
										anyhow::anyhow!(
											"found multiple JS import outputs in a single \
											 section: {imports:?}"
										)
									})?;

								let js_name = import
									.get(0..2)
									.and_then(|length| {
										let length = usize::from(u16::from_le_bytes(
											length.try_into().unwrap(),
										));
										import.get(2..2 + length)
									})
									.context("found invalid JS import encoding")?;
								import = &import[2 + js_name.len()..];

								anyhow::ensure!(
									js_import_output.is_none(),
									"found multiple JS import outputs"
								);
								js_import_output = Some(import.to_owned());
							}
							_ => (),
						}
					}

					Ok(())
				})?;
			}
		}
	}

	let assembly_output = assembly_output.context("found no assembly output")?;
	let assembly_output = String::from_utf8(assembly_output)?;
	let js_import_output = js_import_output.context("found no JS import output")?;
	let js_import_output = String::from_utf8(js_import_output)?;

	Ok((assembly_output, js_import_output))
}
