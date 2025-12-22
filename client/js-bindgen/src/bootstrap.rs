use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Cursor, ErrorKind, Read, Seek, Write};
use std::process::{Command, Stdio};

use ar_archive_writer::{ArchiveKind, NewArchiveMember, ObjectReader};
use proc_macro::TokenStream;
use proc_macro2::Span;
use sanitize_filename::OptionsForCheck;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::{self, Punctuated};
use syn::{Error, Ident, LitStr, Token};

use crate::Library;

/// This parses the given assembly and converts them into archive files. These
/// archive files are then imported via `#[link(...)] extern "C"`.
pub(crate) fn run(input: TokenStream, library: Library) -> Result<TokenStream, Error> {
	let span = Span::mixed_site();
	let InputParser { name, assembly } = Parser::parse(InputParser::parse, input)?;

	// For Rust Analyzer we just want parse errors, the rest doesn't work.
	if env::var_os("RUST_ANALYZER_INTERNALS_DO_NOT_USE")
		.filter(|value| value == "this is unstable")
		.is_some()
	{
		return Ok(TokenStream::new());
	}

	let source_path = span
		.local_file()
		.ok_or_else(|| Error::new(span, "unable to get path to source file"))?;
	let library_file = library.file(&name.to_string());
	let object_file_name = format!("lib{library_file}");
	let object_file = format!("{object_file_name}.o");
	let archive_dir = library.dir();
	let archive_path = archive_dir
		.join(&object_file_name)
		.with_added_extension("a");

	let mtime = fs::metadata(source_path)
		.and_then(|m| m.modified())
		.map_err(|e| Error::new(span, e))?;

	let object = assembly_to_object(span, &mut AssemblyReader::new(assembly))?;

	// Create the folder if it doesn't exist yet.
	fs::create_dir_all(archive_dir).map_err(|e| Error::new(span, e))?;

	// We check if the file exists
	match OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(&archive_path)
	{
		// If it does not, we write the archive.
		Ok(mut file) => {
			// There can be some strange race conditions with Rust Analyzer.
			file.lock().map_err(|e| Error::new(span, e))?;
			object_to_archive(&object, object_file, &mut file).map_err(|e| Error::new(span, e))?;
			// We use mtime to check freshness in non-bootstrap mode.
			file.set_modified(mtime).map_err(|e| Error::new(span, e))?;
			file.sync_all().map_err(|e| Error::new(span, e))?;
		}
		// If it does, we make sure its fresh. This only happens with Rust Analyzer because
		// `build.rs` is always supposed to delete the old cache.
		Err(error) if matches!(error.kind(), ErrorKind::AlreadyExists) => {
			let mut file = OpenOptions::new()
				.read(true)
				.open(archive_path)
				.map_err(|e| Error::new(span, e))?;
			file.lock_shared().map_err(|e| Error::new(span, e))?;

			let mut archive = Vec::new();
			object_to_archive(&object, object_file, &mut Cursor::new(&mut archive))
				.map_err(|e| Error::new(span, e))?;

			let metadata = file.metadata().map_err(|e| Error::new(span, e))?;

			if metadata.len() != archive.len().try_into().unwrap()
				|| !file_matches(&mut file, &archive).map_err(|e| Error::new(span, e))?
			{
				return Err(Error::new(
					span,
					"existing archive didn't match or unable to generate unique file name",
				));
			}
		}
		Err(error) => return Err(Error::new(span, error)),
	}

	Ok(crate::output(proc_macro::Span::mixed_site(), &library_file))
}

struct InputParser {
	name: String,
	assembly: Punctuated<LitStr, Token![,]>,
}

impl Parse for InputParser {
	fn parse(input: ParseStream) -> Result<Self, Error> {
		let span = input.span();
		let ident: Ident = input.parse()?;

		if ident != "name" {
			return Err(Error::new(ident.span(), "expected `name = \"...\"`"));
		}

		input.parse::<Token![=]>()?;
		let name_ident: LitStr = input.parse()?;
		let name = name_ident.value();

		if !sanitize_filename::is_sanitized_with_options(
			&name,
			OptionsForCheck {
				windows: true,
				truncate: true,
			},
		) {
			return Err(Error::new(name_ident.span(), "not a valid filename"));
		}

		if input.is_empty() {
			return Err(Error::new(span, "requires at least a string argument"));
		}

		input.parse::<Token![,]>()?;
		let assembly = Punctuated::parse_terminated(input)?;

		Ok(Self { name, assembly })
	}
}

/// Turns the input assembly into a [`Read`]er.
struct AssemblyReader {
	assembly: punctuated::IntoIter<LitStr>,
	current: Option<String>,
	offset: usize,
	pending_newline: bool,
}

impl AssemblyReader {
	fn new(assembly: Punctuated<LitStr, Token![,]>) -> Self {
		let mut assembly = assembly.into_iter();
		let current = assembly.next().map(|lit| lit.value());

		Self {
			assembly,
			current,
			offset: 0,
			pending_newline: false,
		}
	}
}

impl Read for AssemblyReader {
	fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
		let mut written = 0;

		while written < buffer.len() {
			if self.current.is_some() && self.pending_newline {
				buffer[written] = b'\n';
				written += 1;
				self.pending_newline = false;
				continue;
			}

			let Some(current) = self.current.as_ref() else {
				break;
			};
			let remaining_bytes = &current.as_bytes()[self.offset..];
			let bytes_to_write = (buffer.len() - written).min(remaining_bytes.len());

			buffer[written..written + bytes_to_write]
				.copy_from_slice(&remaining_bytes[..bytes_to_write]);
			written += bytes_to_write;
			self.offset += bytes_to_write;

			if self.offset == current.len() {
				self.current = self.assembly.next().map(|lit| lit.value());
				self.offset = 0;
				self.pending_newline = true;
			}
		}

		Ok(written)
	}
}

fn assembly_to_object(span: Span, assembly: &mut dyn Read) -> Result<Vec<u8>, Error> {
	let mut child = Command::new("llvm-mc")
		.arg("-arch=wasm32")
		.arg("-mattr=+reference-types")
		.arg("-filetype=obj")
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.stdin(Stdio::piped())
		.spawn()
		.map_err(|e| Error::new(span, e))?;

	let mut stdin = child
		.stdin
		.as_mut()
		.ok_or_else(|| Error::new(span, "no stdin found for `llvm-mc` process"))?;
	io::copy(assembly, &mut stdin).map_err(|e| Error::new(span, e))?;

	let output = child.wait_with_output().map_err(|e| Error::new(span, e))?;

	if output.status.success() {
		Ok(output.stdout)
	} else {
		let mut error = format!("`llvm-mc` process failed with status: {}\n", output.status);

		if !output.stdout.is_empty() {
			error.push_str("\n------ llvm-mc stdout ------\n");
			error.push_str(&String::from_utf8_lossy(&output.stdout));

			if !output.stdout.ends_with(b"\n") {
				error.push('\n');
			}
		}

		if !output.stderr.is_empty() {
			error.push_str("\n------ llvm-mc stderr ------\n");
			error.push_str(&String::from_utf8_lossy(&output.stderr));

			if !output.stderr.ends_with(b"\n") {
				error.push('\n');
			}
		}

		Err(Error::new(span, error))
	}
}

fn object_to_archive<W: Seek + Write>(
	object: &[u8],
	archive_file_name: String,
	output: &mut W,
) -> io::Result<()> {
	const OBJECT_READER: ObjectReader = ObjectReader {
		get_symbols: |_, _| Ok(true),
		is_64_bit_object_file: |_| false,
		is_ec_object_file: |_| false,
		is_any_arm64_coff: |_| false,
		get_xcoff_member_alignment: |_| 0,
	};
	let member = NewArchiveMember::new(object, &OBJECT_READER, archive_file_name);

	ar_archive_writer::write_archive_to_stream(output, &[member], ArchiveKind::Gnu, false, None)
}

fn file_matches(file: &mut File, data: &[u8]) -> io::Result<bool> {
	let mut offset = 0;
	let mut buffer = [0; 1024];

	loop {
		let bytes_read = file.read(&mut buffer)?;
		assert!(bytes_read > 0, "we should have interrupted beforehand");

		if buffer[..bytes_read] != data[offset..offset + bytes_read] {
			return Ok(false);
		}

		offset += bytes_read;

		if offset == data.len() {
			break Ok(true);
		}
	}
}
