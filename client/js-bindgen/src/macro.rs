//! See <https://en.wikibooks.org/wiki/C_Programming/stdlib.h/itoa>.

#![expect(clippy::cast_possible_truncation, reason = "itoa implementation")]

#[derive(Clone, Copy)]
pub struct ConstInteger<T>(pub T);

macro_rules! const_unsigned {
	($($ty:ty),+) => {$(
		impl ConstInteger<$ty> {
			pub const fn __jbg_len(self) -> usize {
				let mut n = self.0;
				let mut i = 1;

				while n > 9 {
					i += 1;
					n /= 10;
				}

				i
			}

			pub const fn __jbg_to_le_bytes<const L: usize>(self) -> [u8; L] {
				let mut s = [0; L];
				let mut i = 0;
				let mut n = self.0;

				loop {
					s[i] = (n % 10) as u8 + b'0';
					i += 1;
					n /= 10;

					if n == 0 {
						debug_assert!(i == L);
						break;
					}
				}


				reverse(&mut s);
				s
			}
		})+
	};
}

macro_rules! const_signed {
	($($ty:ty),+) => {$(
		impl ConstInteger<$ty> {
			pub const fn __jbg_len(self) -> usize {
				let n = self.0;
				let abs_len = ConstInteger(n.unsigned_abs()).__jbg_len();
				abs_len + (n < 0) as usize
			}

			pub const fn __jbg_to_le_bytes<const L: usize>(self) -> [u8; L] {
				let mut s = [0; L];
				let mut i = 0;
				let mut n = self.0.unsigned_abs();

				loop {
					s[i] = (n % 10) as u8 + b'0';
					i += 1;
					n /= 10;

					if n == 0 {
						break;
					}
				}

				if self.0 < 0 {
					s[i] = b'-';
					i += 1;
				}

				debug_assert!(i == L);
				reverse(&mut s);
				s
			}
		})+
	};
}

// MSRV: Const stable on v1.90.
const fn reverse<const L: usize>(s: &mut [u8; L]) {
	let mut i = 0;
	let mut j = L - 1;
	let mut c;

	while i < j {
		c = s[i];
		s[i] = s[j];
		s[j] = c;

		i += 1;
		j -= 1;
	}
}

const_unsigned!(u8, u16, u32, u64, u128, usize);
const_signed!(i8, i16, i32, i64, i128, isize);

#[cfg(test)]
mod test {
	extern crate alloc;
	use alloc::string::ToString;

	use js_bindgen_test::test;
	use paste::paste;

	use super::ConstInteger;

	macro_rules! test_unsigned {
		($($ty:ty),+) => {$(
			paste! {
				#[test]
				fn [<max_ $ty>]() {
					const MAX: ConstInteger<$ty> = ConstInteger($ty::MAX);
					let output = MAX.__jbg_to_le_bytes::<{ MAX.__jbg_len() }>();
					let output = str::from_utf8(&output).unwrap();
					let expected = $ty::MAX.to_string();

					assert_eq!(output, expected);
				}

				#[test]
				fn [<min_ $ty>]() {
					const MIN: ConstInteger<$ty> = ConstInteger($ty::MIN);
					let output = MIN.__jbg_to_le_bytes::<{ MIN.__jbg_len() }>();
					let output = str::from_utf8(&output).unwrap();
					let expected = $ty::MIN.to_string();

					assert_eq!(output, expected);
				}
			}
		)+};
	}

	macro_rules! test_signed {
		($($ty:ty),+) => {$(
			paste! {
				test_unsigned!($ty);

				#[test]
				fn [<null_ $ty>]() {
					const NULL: ConstInteger<$ty> = ConstInteger(0);
					let output = NULL.__jbg_to_le_bytes::<{ NULL.__jbg_len() }>();
					let output = str::from_utf8(&output).unwrap();
					let expected = 0.to_string();

					assert_eq!(output, expected);
				}
			}
		)+};
	}

	test_unsigned!(u8, u16, u32, u64, u128, usize);
	test_signed!(i8, i16, i32, i64, i128, isize);
}
