use std::ffi::OsStr;
use std::fmt::{self, Debug, Formatter};
use std::io::{Error, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::SystemTime;

use js_bindgen_shared::ReadFile;
use object::read::archive::ArchiveFile;
use wasmparser::CustomSectionReader;

/// Currently this simply passes the LLVM s-format assembly to `clang` to
/// convert to an object file the linker can consume.
pub fn assembly_to_object(arch_str: &str, assembly: &str) -> Result<Vec<u8>, Error> {
	let mut child = Command::new("clang")
		.arg("-xassembler")
		.arg(format!("--target={arch_str}"))
		// In the future we will switch to something supporting auto-detection.
		.arg("-mreference-types")
		.arg("-mcall-indirect-overlong")
		.arg("-c")
		// from stdin
		.arg("-")
		.arg("-o")
		// to stdout
		.arg("-")
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.stdin(Stdio::piped())
		.spawn()?;

	let stdin = child
		.stdin
		.as_mut()
		.ok_or_else(|| Error::other("`clang` process should have `stdin`"))?;
	stdin.write_all(assembly.as_bytes())?;

	let output = child.wait_with_output()?;

	if output.status.success() {
		Ok(output.stdout)
	} else {
		eprintln!("------ clang input -------\n{assembly}");

		if !output.stdout.is_empty() {
			eprintln!(
				"------ clang stdout ------\n{}",
				String::from_utf8_lossy(&output.stdout)
			);

			if !output.stdout.ends_with(b"\n") {
				eprintln!();
			}
		}

		if !output.stderr.is_empty() {
			eprintln!(
				"------ clang stderr ------\n{}",
				String::from_utf8_lossy(&output.stderr)
			);

			if !output.stderr.ends_with(b"\n") {
				eprintln!();
			}
		}

		Err(Error::other(format!(
			"`clang` process failed with status: {}",
			output.status
		)))
	}
}

pub fn ld_input_parser<E>(
	input: &OsStr,
	mut fun: impl FnMut(&Path, &[u8], Option<SystemTime>) -> Result<(), E>,
) -> Result<Result<(), E>, Error> {
	// We found a UNIX archive.
	if input.as_encoded_bytes().ends_with(b".rlib") {
		let archive_path = Path::new(&input);
		let archive_data = match ReadFile::new(archive_path) {
			Ok(archive_data) => archive_data,
			Err(error) => {
				eprintln!(
					"failed to read archive file {}:\n{error}",
					archive_path.display()
				);
				return Ok(Ok(()));
			}
		};
		let archive = match ArchiveFile::parse(&*archive_data) {
			Ok(archive_data) => archive_data,
			Err(error) => {
				eprintln!(
					"failed to parse archive file {}:\n{error}",
					archive_path.display()
				);
				return Ok(Ok(()));
			}
		};

		for member in archive.members() {
			let member = match member {
				Ok(member) => member,
				Err(error) => {
					eprintln!(
						"unable to parse archive member in {}:\n{error}",
						archive_path.display()
					);
					continue;
				}
			};
			let name = match str::from_utf8(member.name()) {
				Ok(name) => name.to_owned(),
				Err(error) => {
					eprintln!(
						"unable to convert archive member name to UTF-8 in {}:\n{error}",
						archive_path.display()
					);
					continue;
				}
			};
			let data = match member.data(&*archive_data) {
				Ok(object) => object,
				Err(error) => {
					eprintln!(
						"unable to extract archive member data from {}:\n{error}",
						archive_path.display()
					);
					continue;
				}
			};

			if let Err(error) = fun(
				&archive_path.with_file_name(name),
				data,
				archive_data.mtime()?,
			) {
				return Ok(Err(error));
			}
		}
	} else if input.as_encoded_bytes().ends_with(b".o") {
		let object_path = Path::new(&input);
		let object = match ReadFile::new(object_path) {
			Ok(object) => object,
			Err(error) => {
				eprintln!(
					"failed to read object file {}:\n{error}",
					object_path.display()
				);
				return Ok(Ok(()));
			}
		};

		if let Err(error) = fun(object_path, &object, object.mtime()?) {
			return Ok(Err(error));
		}
	}

	Ok(Ok(()))
}

#[derive(Clone)]
pub struct JsBindgenAssemblySectionParser<'cs>(CustomSectionParser<'cs>);

impl<'cs> JsBindgenAssemblySectionParser<'cs> {
	#[must_use]
	pub fn new(custom_section: &CustomSectionReader<'cs>) -> Self {
		Self(CustomSectionParser::new(custom_section))
	}
}

impl Debug for JsBindgenAssemblySectionParser<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let rest: Vec<_> = self.clone().collect();

		f.debug_tuple("JsBindgenAssemblySectionParser")
			.field(&rest.as_slice())
			.finish()
	}
}

impl<'cs> Iterator for JsBindgenAssemblySectionParser<'cs> {
	type Item = &'cs str;

	fn next(&mut self) -> Option<Self::Item> {
		self.0
			.next()
			.map(str::from_utf8)
			.transpose()
			.unwrap_or_else(|error| {
				panic!(
					"found invalid JS assembly encoding `{}`: {error}",
					self.0.name
				)
			})
	}
}

#[derive(Clone)]
pub struct JsBindgenJsSectionParser<'cs>(CustomSectionParser<'cs>);

#[derive(Debug)]
pub struct JsBindgenJsSection<'cs> {
	pub module: &'cs str,
	pub name: &'cs str,
	pub js: &'cs str,
	pub embeds: Vec<JsRequiredEmbed<'cs>>,
}

#[derive(Debug)]
pub struct JsRequiredEmbed<'cs> {
	pub module: &'cs str,
	pub name: &'cs str,
}

impl<'cs> JsBindgenJsSectionParser<'cs> {
	#[must_use]
	pub fn new(custom_section: &CustomSectionReader<'cs>) -> Self {
		Self(CustomSectionParser::new(custom_section))
	}
}

impl Debug for JsBindgenJsSectionParser<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let rest: Vec<_> = self.clone().collect();

		f.debug_tuple("JsBindgenJsSectionParser")
			.field(&rest.as_slice())
			.finish()
	}
}

impl<'cs> Iterator for JsBindgenJsSectionParser<'cs> {
	type Item = JsBindgenJsSection<'cs>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|mut data| {
			let module = data
				.split_off(..2)
				.and_then(|length| {
					let length = usize::from(u16::from_le_bytes(length.try_into().unwrap()));

					let module = data.split_off(..length)?;
					str::from_utf8(module).ok()
				})
				.unwrap_or_else(|| panic!("found invalid JS encoding `{}`", self.0.name));

			let name = data
				.split_off(..2)
				.and_then(|length| {
					let length = usize::from(u16::from_le_bytes(length.try_into().unwrap()));

					let name = data.split_off(..length)?;
					str::from_utf8(name).ok()
				})
				.unwrap_or_else(|| panic!("found invalid JS encoding `{}`", self.0.name));

			let embeds = data
				.split_off_first()
				.and_then(|length| {
					let mut embeds = Vec::new();

					for _ in 0..*length {
						let length = usize::from(u16::from_le_bytes(
							data.split_off(..2)?.try_into().unwrap(),
						));
						let module = data.split_off(..length)?;
						let module = str::from_utf8(module).ok()?;

						let length = usize::from(u16::from_le_bytes(
							data.split_off(..2)?.try_into().unwrap(),
						));
						let name = data.split_off(..length)?;
						let name = str::from_utf8(name).ok()?;

						if module.is_empty() {
							continue;
						}

						embeds.push(JsRequiredEmbed { module, name });
					}

					Some(embeds)
				})
				.unwrap_or_else(|| panic!("found invalid JS encoding `{}`", self.0.name));

			let js = str::from_utf8(data)
				.unwrap_or_else(|e| panic!("found invalid JS encoding `{}`: {e}", self.0.name));

			JsBindgenJsSection {
				module,
				name,
				js,
				embeds,
			}
		})
	}
}

#[derive(Clone)]
struct CustomSectionParser<'cs> {
	name: &'cs str,
	data: &'cs [u8],
}

impl<'cs> CustomSectionParser<'cs> {
	fn new(custom_section: &CustomSectionReader<'cs>) -> Self {
		Self {
			name: custom_section.name(),
			data: custom_section.data(),
		}
	}
}

impl<'cs> Iterator for CustomSectionParser<'cs> {
	type Item = &'cs [u8];

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(length) = self.data.split_off(..4) {
			let length = u32::from_le_bytes(length.try_into().unwrap()) as usize;

			let data = self.data.split_off(..length).unwrap_or_else(|| {
				panic!("invalid length encoding in custom section `{}`", self.name)
			});

			Some(data)
		} else if self.data.is_empty() {
			None
		} else {
			panic!(
				"found left over bytes in custom section `{}`: {:?}",
				self.name, self.data
			);
		}
	}
}
