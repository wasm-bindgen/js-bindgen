use std::ffi::OsStr;
use std::fmt::{self, Debug, Formatter};
use std::fs::File;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::ops::Deref;
use std::path::Path;
use std::process::{Command, Stdio};

use memmap2::Mmap;
use object::read::archive::ArchiveFile;
use wasmparser::CustomSectionReader;

/// Currently this simply passes the LLVM s-format assembly to `llvm-mc` to
/// convert to an object file the linker can consume.
pub fn assembly_to_object(
	arch_str: &OsStr,
	assembly: &[u8],
	output: &mut dyn Write,
) -> Result<(), Error> {
	let mut child = Command::new("llvm-mc")
		.arg(format!("-arch={}", arch_str.display()))
		// In the future we will switch to something supporting auto-detection.
		.arg("-mattr=+reference-types,+call-indirect-overlong")
		.arg("-filetype=obj")
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.stdin(Stdio::piped())
		.spawn()?;

	let stdin = child
		.stdin
		.as_mut()
		.ok_or_else(|| Error::other("`llvm-mc` process should have `stdin`"))?;
	stdin.write_all(assembly)?;

	let status = child.wait()?;

	let mut child_stdout = child
		.stdout
		.ok_or_else(|| Error::other("`llvm-mc` process should have `stdout`"))?;

	if status.success() {
		io::copy(&mut child_stdout, output)?;
		Ok(())
	} else {
		eprintln!(
			"------ llvm-mc input -------\n{}",
			String::from_utf8_lossy(assembly)
		);

		let mut stdout = Vec::new();
		child_stdout.read_to_end(&mut stdout)?;

		if !stdout.is_empty() {
			eprintln!(
				"------ llvm-mc stdout ------\n{}",
				String::from_utf8_lossy(&stdout)
			);

			if !stdout.ends_with(b"\n") {
				eprintln!();
			}
		}

		let mut stderr = Vec::new();
		child
			.stderr
			.ok_or_else(|| Error::other("`llvm-mc` process should have `stderr`"))?
			.read_to_end(&mut stderr)?;

		if !stderr.is_empty() {
			eprintln!(
				"------ llvm-mc stderr ------\n{}",
				String::from_utf8_lossy(&stderr)
			);

			if !stderr.ends_with(b"\n") {
				eprintln!();
			}
		}

		Err(Error::other(format!(
			"`llvm-mc` process failed with status: {status}"
		)))
	}
}

pub fn ld_input_parser<E>(
	input: &OsStr,
	mut fun: impl FnMut(&Path, &[u8]) -> Result<(), E>,
) -> Result<(), E> {
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
				return Ok(());
			}
		};
		let archive = match ArchiveFile::parse(&*archive_data) {
			Ok(archive_data) => archive_data,
			Err(error) => {
				eprintln!(
					"failed to parse archive file {}:\n{error}",
					archive_path.display()
				);
				return Ok(());
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

			fun(&archive_path.with_file_name(name), data)?;
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
				return Ok(());
			}
		};

		fun(object_path, &object)?;
	}

	Ok(())
}

pub struct ReadFile(ReadInner);

enum ReadInner {
	Mmap(Mmap),
	File(Vec<u8>),
}

impl ReadFile {
	pub fn new(path: &Path) -> Result<Self, Error> {
		let mut file = File::open(path)?;
		// SAFETY: the file is not mutated while the mapping is in use.
		let result = unsafe { Mmap::map(&file) };

		match result {
			Ok(mmap) => Ok(Self(ReadInner::Mmap(mmap))),
			Err(error) if matches!(error.kind(), ErrorKind::Unsupported) => {
				let mut output = Vec::new();
				file.read_to_end(&mut output)?;
				Ok(Self(ReadInner::File(output)))
			}
			Err(error) => Err(error),
		}
	}
}

impl Deref for ReadFile {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		match &self.0 {
			ReadInner::Mmap(mmap) => mmap.deref(),
			ReadInner::File(data) => data.as_slice(),
		}
	}
}

#[derive(Clone)]
pub struct CustomSectionParser<'cs> {
	name: &'cs str,
	data: &'cs [u8],
	prefix: bool,
}

impl<'cs> CustomSectionParser<'cs> {
	pub fn new(custom_section: CustomSectionReader<'cs>, prefix: bool) -> Self {
		Self {
			name: custom_section.name(),
			data: custom_section.data(),
			prefix,
		}
	}
}

impl<'cs> Debug for CustomSectionParser<'cs> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let rest: Vec<_> = self.clone().collect();

		f.debug_tuple("CustomSectionParser")
			.field(&rest.as_slice())
			.finish()
	}
}

impl<'cs> Iterator for CustomSectionParser<'cs> {
	type Item = &'cs [u8];

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(length) = self.data.get(..4) {
			self.data = &self.data[4..];
			let mut length = u32::from_le_bytes(length.try_into().unwrap()) as usize;

			if self.prefix {
				let prefix = &self.data[0..2];
				let prefix = u16::from_le_bytes(prefix.try_into().unwrap()) as usize;
				length += 2 + prefix;
			}

			let data = self.data.get(..length).unwrap_or_else(|| {
				panic!("invalid length encoding in custom section `{}`", self.name)
			});
			self.data = &self.data[length..];

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
