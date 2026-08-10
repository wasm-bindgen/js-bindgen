/// A const writer that renders directly into a custom section allocation.
pub(super) struct Writer<const LEN: usize> {
	bytes: [u8; LEN],
	position: usize,
}

impl<const LEN: usize> Writer<LEN> {
	pub const fn new() -> Self {
		Self {
			bytes: [0; LEN],
			position: 0,
		}
	}

	pub const fn len(&self) -> usize {
		self.position
	}

	pub const fn write_byte(&mut self, value: u8) {
		if LEN != 0 {
			assert!(self.position < LEN);
			self.bytes[self.position] = value;
		}

		self.position += 1;
	}

	pub const fn write_str(&mut self, value: &str) {
		let bytes = value.as_bytes();

		if LEN != 0 {
			assert!(bytes.len() <= LEN - self.position);

			// SAFETY: The assertion above proves that both ranges are valid and
			// a string borrowed by the descriptor cannot overlap the output.
			unsafe {
				core::ptr::copy_nonoverlapping(
					bytes.as_ptr(),
					self.bytes.as_mut_ptr().add(self.position),
					bytes.len(),
				);
			}
		}

		self.position += bytes.len();
	}

	pub const fn write_str_range(&mut self, value: &str, start: usize, end: usize) {
		assert!(start <= end && end <= value.len());
		let len = end - start;

		if LEN != 0 {
			assert!(len <= LEN - self.position);

			// SAFETY: Both assertions prove the source and destination ranges
			// are valid, and they belong to different allocations.
			unsafe {
				core::ptr::copy_nonoverlapping(
					value.as_ptr().add(start),
					self.bytes.as_mut_ptr().add(self.position),
					len,
				);
			}
		}

		self.position += len;
	}

	pub const fn write_u16(&mut self, value: usize) {
		assert!(value <= u16::MAX as usize);
		let bytes = value.to_le_bytes();
		self.write_byte(bytes[0]);
		self.write_byte(bytes[1]);
	}

	pub const fn write_u32(&mut self, value: usize) {
		assert!(value <= u32::MAX as usize);
		let bytes = value.to_le_bytes();
		self.write_byte(bytes[0]);
		self.write_byte(bytes[1]);
		self.write_byte(bytes[2]);
		self.write_byte(bytes[3]);
	}

	pub const fn set_u32(&mut self, offset: usize, value: usize) {
		assert!(value <= u32::MAX as usize);

		if LEN != 0 {
			assert!(offset <= LEN - 4);
			let bytes = value.to_le_bytes();
			self.bytes[offset] = bytes[0];
			self.bytes[offset + 1] = bytes[1];
			self.bytes[offset + 2] = bytes[2];
			self.bytes[offset + 3] = bytes[3];
		}
	}

	pub const fn finish_padded(self) -> [u8; LEN] {
		assert!(self.position <= LEN);
		self.bytes
	}
}
