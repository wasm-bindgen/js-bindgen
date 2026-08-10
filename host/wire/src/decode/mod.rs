//! Allocation-backed decoding of wire records.

mod export;
mod import;
mod value;

use core::{fmt, str};

use crate::{KIND_EXPORT, KIND_IMPORT, MAGIC, PointerWidth, VERSION, model::Record};

/// A type that can be decoded from a wire record.
pub(crate) trait Decode<'de>: Sized {
	fn decode(decoder: &mut Decoder<'de>) -> Result<Self, Error>;
}

/// Decodes one complete wire record.
pub fn decode(bytes: &[u8]) -> Result<Record<'_>, Error> {
	let mut decoder = Decoder::new(bytes);
	let record = Record::decode(&mut decoder)?;
	decoder.finish()?;
	Ok(record)
}

impl<'de> Decode<'de> for Record<'de> {
	fn decode(decoder: &mut Decoder<'de>) -> Result<Self, Error> {
		let magic_offset = decoder.position();
		if decoder.bytes(MAGIC.len())? != MAGIC {
			return Err(Error::new(magic_offset, ErrorKind::InvalidMagic));
		}
		let version_offset = decoder.position();
		let version = decoder.u16()?;
		if version != VERSION {
			return Err(Error::new(
				version_offset,
				ErrorKind::UnsupportedVersion(version),
			));
		}
		let width_offset = decoder.position();
		let pointer_width = match decoder.u8()? {
			4 => PointerWidth::Wasm32,
			8 => PointerWidth::Wasm64,
			width => {
				return Err(Error::new(
					width_offset,
					ErrorKind::InvalidPointerWidth(width),
				));
			}
		};
		let kind_offset = decoder.position();
		match decoder.u8()? {
			KIND_IMPORT => import::decode(decoder, pointer_width).map(Self::Imports),
			KIND_EXPORT => export::decode(decoder, pointer_width).map(Self::Exports),
			kind => Err(Error::new(kind_offset, ErrorKind::UnknownRecordKind(kind))),
		}
	}
}

/// A cursor over one borrowed wire record.
pub(crate) struct Decoder<'de> {
	bytes: &'de [u8],
	position: usize,
}

impl<'de> Decoder<'de> {
	#[must_use]
	pub(crate) const fn new(bytes: &'de [u8]) -> Self {
		Self { bytes, position: 0 }
	}

	#[must_use]
	pub(crate) const fn position(&self) -> usize {
		self.position
	}

	pub(crate) fn u8(&mut self) -> Result<u8, Error> {
		Ok(self.bytes(1)?[0])
	}

	pub(crate) fn boolean(&mut self, error_context: &'static str) -> Result<bool, Error> {
		let offset = self.position;
		match self.u8()? {
			0 => Ok(false),
			1 => Ok(true),
			value => Err(Error::new(
				offset,
				ErrorKind::InvalidBoolean {
					error_context,
					value,
				},
			)),
		}
	}

	fn u16(&mut self) -> Result<u16, Error> {
		let bytes = self.bytes(2)?;
		Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
	}

	pub(crate) fn u32(&mut self) -> Result<u32, Error> {
		let bytes = self.bytes(4)?;
		Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
	}

	pub(crate) fn u64(&mut self) -> Result<u64, Error> {
		let bytes = self.bytes(8)?;
		Ok(u64::from_le_bytes([
			bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
		]))
	}

	pub(crate) fn optional_u64(
		&mut self,
		error_context: &'static str,
	) -> Result<Option<u64>, Error> {
		if self.boolean(error_context)? {
			self.u64().map(Some)
		} else {
			Ok(None)
		}
	}

	pub(crate) fn count(&mut self, error_context: &'static str) -> Result<usize, Error> {
		let offset = self.position;
		let count = self.u32()? as usize;
		if count > self.remaining() {
			return Err(Error::new(
				offset,
				ErrorKind::CountExceedsRecord {
					error_context,
					count,
				},
			));
		}
		Ok(count)
	}

	pub(crate) fn optional_string(&mut self) -> Result<Option<&'de str>, Error> {
		let offset = self.position;
		let length = self.u32()?;
		if length == u32::MAX {
			return Ok(None);
		}
		let bytes = self.bytes(length as usize)?;
		str::from_utf8(bytes)
			.map(Some)
			.map_err(|_| Error::new(offset, ErrorKind::InvalidUtf8))
	}

	pub(crate) fn string(&mut self) -> Result<&'de str, Error> {
		let offset = self.position;
		self.optional_string()?
			.ok_or_else(|| Error::new(offset, ErrorKind::MissingString))
	}

	fn bytes(&mut self, length: usize) -> Result<&'de [u8], Error> {
		let offset = self.position;
		let Some(end) = offset.checked_add(length) else {
			return Err(Error::new(
				offset,
				ErrorKind::UnexpectedEnd { needed: length },
			));
		};
		let Some(bytes) = self.bytes.get(offset..end) else {
			return Err(Error::new(
				offset,
				ErrorKind::UnexpectedEnd { needed: length },
			));
		};
		self.position = end;
		Ok(bytes)
	}

	fn finish(self) -> Result<(), Error> {
		let remaining = self.remaining();
		if remaining == 0 {
			Ok(())
		} else {
			Err(Error::new(
				self.position,
				ErrorKind::TrailingBytes(remaining),
			))
		}
	}

	const fn remaining(&self) -> usize {
		self.bytes.len() - self.position
	}

	pub(crate) fn invalid<T>(&self, message: &'static str) -> Result<T, Error> {
		Err(Error::new(self.position, ErrorKind::InvalidValue(message)))
	}

	pub(crate) fn ensure(&self, condition: bool, message: &'static str) -> Result<(), Error> {
		if condition {
			Ok(())
		} else {
			self.invalid(message)
		}
	}

	pub(crate) fn flags(&mut self, error_context: &'static str, allowed: u8) -> Result<u8, Error> {
		let offset = self.position;
		let flags = self.u8()?;
		let unknown = flags & !allowed;
		if unknown == 0 {
			Ok(flags)
		} else {
			Err(Error::new(
				offset,
				ErrorKind::UnknownFlags {
					error_context,
					unknown,
				},
			))
		}
	}

	pub(crate) fn tag(&mut self, error_context: &'static str, maximum: u8) -> Result<u8, Error> {
		let offset = self.position;
		let tag = self.u8()?;
		if tag <= maximum {
			Ok(tag)
		} else {
			Err(Error::new(
				offset,
				ErrorKind::UnknownTag { error_context, tag },
			))
		}
	}
}

/// A structural or semantic wire decoding failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Error {
	offset: usize,
	kind: ErrorKind,
}

impl Error {
	const fn new(offset: usize, kind: ErrorKind) -> Self {
		Self { offset, kind }
	}

	#[must_use]
	pub const fn offset(&self) -> usize {
		self.offset
	}

	#[must_use]
	pub const fn kind(&self) -> &ErrorKind {
		&self.kind
	}
}

/// The precise category of a wire decoding failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorKind {
	UnexpectedEnd {
		needed: usize,
	},
	InvalidMagic,
	UnsupportedVersion(u16),
	InvalidPointerWidth(u8),
	UnknownRecordKind(u8),
	InvalidBoolean {
		error_context: &'static str,
		value: u8,
	},
	InvalidUtf8,
	MissingString,
	TrailingBytes(usize),
	CountExceedsRecord {
		error_context: &'static str,
		count: usize,
	},
	UnknownFlags {
		error_context: &'static str,
		unknown: u8,
	},
	UnknownTag {
		error_context: &'static str,
		tag: u8,
	},
	IndexOutOfBounds {
		table: &'static str,
		index: u32,
		len: usize,
	},
	InvalidValue(&'static str),
}

impl fmt::Display for Error {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(formatter, "wire error at byte {}: ", self.offset)?;
		match self.kind {
			ErrorKind::UnexpectedEnd { needed } => {
				write!(formatter, "record ends before the next {needed} bytes")
			}
			ErrorKind::InvalidMagic => formatter.write_str("invalid magic"),
			ErrorKind::UnsupportedVersion(version) => {
				write!(formatter, "unsupported version {version}")
			}
			ErrorKind::InvalidPointerWidth(width) => {
				write!(formatter, "invalid pointer width {width}")
			}
			ErrorKind::UnknownRecordKind(kind) => write!(formatter, "unknown record kind {kind}"),
			ErrorKind::InvalidBoolean {
				error_context,
				value,
			} => {
				write!(formatter, "invalid {error_context} boolean {value}")
			}
			ErrorKind::InvalidUtf8 => formatter.write_str("string is not UTF-8"),
			ErrorKind::MissingString => formatter.write_str("required string is absent"),
			ErrorKind::TrailingBytes(bytes) => write!(formatter, "{bytes} trailing bytes"),
			ErrorKind::CountExceedsRecord {
				error_context,
				count,
			} => {
				write!(
					formatter,
					"{error_context} count {count} exceeds the record"
				)
			}
			ErrorKind::UnknownFlags {
				error_context,
				unknown,
			} => {
				write!(formatter, "unknown {error_context} flags {unknown:#x}")
			}
			ErrorKind::UnknownTag { error_context, tag } => {
				write!(formatter, "unknown {error_context} tag {tag}")
			}
			ErrorKind::IndexOutOfBounds { table, index, len } => {
				write!(
					formatter,
					"{table} index {index} is out of bounds for length {len}"
				)
			}
			ErrorKind::InvalidValue(message) => formatter.write_str(message),
		}
	}
}

impl core::error::Error for Error {}
