use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::ops::Deref;
use std::path::Path;

use memmap2::Mmap;

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
