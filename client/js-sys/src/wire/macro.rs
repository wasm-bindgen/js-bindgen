#[doc(hidden)]
#[macro_export]
macro_rules! const_concat {
	($($value:expr),* $(,)?) => {{
		const VALUES: &[&::core::primitive::str] = &[$($value),*];
		const LEN: ::core::primitive::usize = $crate::wire::const_concat_len(VALUES);
		const VALUE: [::core::primitive::u8; LEN] =
			$crate::wire::render_concat::<LEN>(VALUES);

		// SAFETY: Joining valid strings keeps the result valid.
		unsafe { ::core::str::from_utf8_unchecked(&VALUE) }
	}};
}

#[must_use]
pub const fn const_concat_len(values: &[&str]) -> usize {
	let mut len = 0;
	let mut index = 0;

	while index < values.len() {
		len += values[index].len();
		index += 1;
	}

	len
}

#[must_use]
pub const fn render_concat<const LEN: usize>(values: &[&str]) -> [u8; LEN] {
	let mut output = [0; LEN];
	let mut offset = 0;
	let mut index = 0;

	while index < values.len() {
		offset = append_str(&mut output, offset, values[index]);
		index += 1;
	}

	output
}

const fn append_str<const LEN: usize>(output: &mut [u8; LEN], offset: usize, value: &str) -> usize {
	let bytes = value.as_bytes();
	let Some(end) = offset.checked_add(bytes.len()) else {
		panic!("string append overflows usize");
	};
	assert!(end <= LEN);

	// SAFETY: `end <= LEN` proves that the destination range is in bounds.
	// The source is a valid string slice and cannot overlap the output array.
	unsafe {
		core::ptr::copy_nonoverlapping(
			bytes.as_ptr(),
			output.as_mut_ptr().add(offset),
			bytes.len(),
		);
	}

	end
}
