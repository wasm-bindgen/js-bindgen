use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::ops::Deref;
use std::path::Path;
use std::time::SystemTime;

use memmap2::Mmap;

pub struct ReadFile {
	reader: Reader,
	mtime: Option<SystemTime>,
}

enum Reader {
	Mmap(Mmap),
	File(Vec<u8>),
}

impl ReadFile {
	pub fn new(path: &Path) -> Result<Self, Error> {
		let mut file = File::open(path)?;

		let metadata = file.metadata()?;
		let mtime = match metadata.modified() {
			Ok(mtime) => Some(mtime),
			Err(error) if matches!(error.kind(), ErrorKind::Unsupported) => None,
			Err(error) => return Err(error),
		};

		// SAFETY: the file is not mutated while the mapping is in use.
		let result = unsafe { Mmap::map(&file) };

		match result {
			Ok(mmap) => Ok(Self {
				reader: Reader::Mmap(mmap),
				mtime,
			}),
			Err(error) if matches!(error.kind(), ErrorKind::Unsupported) => {
				let mut output = Vec::new();
				file.read_to_end(&mut output)?;
				Ok(Self {
					reader: Reader::File(output),
					mtime,
				})
			}
			Err(error) => Err(error),
		}
	}

	#[must_use]
	pub fn mtime(&self) -> Option<SystemTime> {
		self.mtime
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
