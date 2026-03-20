use std::fs::{File, Metadata};
use std::io::{Error, ErrorKind, Read};
use std::ops::Deref;
use std::path::Path;
use std::time::SystemTime;

use memmap2::Mmap;

pub struct ReadFile {
	file: File,
	reader: Reader,
}

enum Reader {
	Mmap(Mmap),
	File(Vec<u8>),
}

impl ReadFile {
	pub fn new(path: &Path) -> Result<Self, Error> {
		let mut file = File::open(path)?;

		// SAFETY: the file is not mutated while the mapping is in use.
		let result = unsafe { Mmap::map(&file) };

		match result {
			Ok(mmap) => Ok(Self {
				reader: Reader::Mmap(mmap),
				file,
			}),
			Err(error) if matches!(error.kind(), ErrorKind::Unsupported) => {
				let mut output = Vec::new();
				file.read_to_end(&mut output)?;
				Ok(Self {
					reader: Reader::File(output),
					file,
				})
			}
			Err(error) => Err(error),
		}
	}

	pub fn mtime(&self) -> Result<Option<SystemTime>, Error> {
		let metadata = self.file.metadata()?;
		mtime(&metadata)
	}
}

pub fn mtime(metadata: &Metadata) -> Result<Option<SystemTime>, Error> {
	match metadata.modified() {
		Ok(mtime) => Ok(Some(mtime)),
		Err(error) if matches!(error.kind(), ErrorKind::Unsupported) => Ok(None),
		Err(error) => Err(error),
	}
}

impl Deref for ReadFile {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		match &self.reader {
			Reader::Mmap(mmap) => mmap.deref(),
			Reader::File(data) => data.as_slice(),
		}
	}
}
