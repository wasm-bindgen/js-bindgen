use core::error::Error;
use core::fmt::{self, Display, Formatter};
use core::mem::MaybeUninit;
use core::ops::Range;
use core::ptr;

use crate::JsValue;
use crate::builtins::Array;
use crate::hazard::JsCast;
use crate::interop::slice::array_from_js_value_slice;
use crate::runtime::externref;
use crate::util::{PtrConst, PtrLength, PtrMut};

macro_rules! primitive_arrays {
	(
		$(
			$(#[$attr:meta])*
			$ty:ty {
				encode = $encode:ident,
				from_slice = $from_slice:ident,
				embed = $embed:literal,
				constructor = $constructor:literal,
				view = $view:literal $(,)?
			}
		)*
	) => {
		#[crate::js_sys(js_sys = crate)]
		extern "js-sys" {
			#[js_sys(js_embed = "array.checked_length")]
			fn array_checked_length(array: &Array) -> Result<u32, JsValue>;

			// SAFETY: Every pointer and length pair must describe its matching
			// output slice.
			#[js_sys(js_embed = "array.js_value.encode")]
			#[expect(
				clippy::allow_attributes,
				reason = "the macro emits an unsafe ABI call"
			)]
			#[allow(
				clippy::undocumented_unsafe_blocks,
				reason = "the safety requirement is documented on this declaration"
			)]
			unsafe fn array_js_value_encode(
				array: &Array,
				array_ptr: PtrMut<JsValue>,
				array_len: PtrLength<JsValue>,
				externref_ptr: PtrConst<i32>,
				externref_len: PtrLength<i32>,
				write_output: bool,
			) -> Result<u32, JsValue>;

			$(
				$(#[$attr])*
				// SAFETY: The pointer and length must describe a valid output slice.
				#[js_sys(js_embed = $embed)]
				#[expect(
					clippy::allow_attributes,
					reason = "the macro emits an unsafe ABI call"
				)]
				#[allow(
					clippy::undocumented_unsafe_blocks,
					reason = "the safety requirement is documented on this declaration"
				)]
				unsafe fn $encode(
					array: &Array<$ty>,
					ptr: PtrMut<$ty>,
					len: PtrLength<$ty>,
				) -> Result<u32, JsValue>;
			)*
		}

		$(
			$(#[$attr])*
			js_bindgen::embed_js!(
				module = "js_sys",
				name = $embed,
				required_embeds = [("js_sys", concat!("view.set", $view))],
				"(array, ptr, len) => {{",
				"    const rawLength = array.length",
				"    const length = rawLength >>> 0",
				"    if (rawLength !== length) throw new TypeError('invalid array length')",
				"    if (length !== len) return length",
				"",
				"    const values = new {constructor}(len)",
				"    for (let index = 0; index < len; index++) {{",
				"        values[index] = array[index]",
				"    }}",
				"    this.#jsEmbed.js_sys['view.set{view}'](ptr, values, len)",
				"    return length",
				"}}",
				constructor = interpolate $constructor,
				view = interpolate $view,
			);

			$(#[$attr])*
			impl Array<$ty> {
				pub fn to_slice(&self, slice: &mut [$ty]) -> Result<(), TryFromArrayError> {
					// SAFETY: Parameters are correct.
					let result = unsafe {
						$encode(self, PtrMut::new(slice), PtrLength::new(slice))
					};

					check_copy_length(result, slice.len())
				}

				pub fn to_uninit_slice<'slice>(
					&self,
					slice: &'slice mut [MaybeUninit<$ty>],
				) -> Result<&'slice mut [$ty], TryFromArrayError> {
					// SAFETY: Parameters are correct.
					let result = unsafe {
						$encode(
							self,
							PtrMut::from_uninit_slice(slice),
							PtrLength::from_uninit_slice(slice),
						)
					};

					check_copy_length(result, slice.len())?;
					// SAFETY: The staging typed array was fully initialized before it
					// was copied into `slice`.
					Ok(unsafe { assume_init_mut(slice) })
				}

				pub fn to_array<const N: usize>(&self) -> Result<[$ty; N], TryFromArrayError> {
					let mut array: MaybeUninit<[$ty; N]> = MaybeUninit::uninit();

					// SAFETY: Parameters are correct.
					let result = unsafe {
						$encode(
							self,
							PtrMut::from_uninit_array(&mut array),
							PtrLength::from_uninit_array(&array),
						)
					};

					check_copy_length(result, N)?;
					// SAFETY: The staging typed array was fully initialized before it
					// was copied into `array`.
					Ok(unsafe { array.assume_init() })
				}
			}

			$(#[$attr])*
			impl From<&[$ty]> for Array<$ty> {
				fn from(value: &[$ty]) -> Self {
					crate::interop::slice::$from_slice(value)
				}
			}
		)*
	};
}

primitive_arrays! {
	i8 {
		encode = array_i8_encode,
		from_slice = array_from_i8_slice,
		embed = "array.i8.encode",
		constructor = "Int8Array",
		view = "Int8",
	}
	u8 {
		encode = array_u8_encode,
		from_slice = array_from_u8_slice,
		embed = "array.u8.encode",
		constructor = "Uint8Array",
		view = "Uint8",
	}
	i16 {
		encode = array_i16_encode,
		from_slice = array_from_i16_slice,
		embed = "array.i16.encode",
		constructor = "Int16Array",
		view = "Int16",
	}
	u16 {
		encode = array_u16_encode,
		from_slice = array_from_u16_slice,
		embed = "array.u16.encode",
		constructor = "Uint16Array",
		view = "Uint16",
	}
	i32 {
		encode = array_i32_encode,
		from_slice = array_from_i32_slice,
		embed = "array.i32.encode",
		constructor = "Int32Array",
		view = "Int32",
	}
	u32 {
		encode = array_u32_encode,
		from_slice = array_from_u32_slice,
		embed = "array.u32.encode",
		constructor = "Uint32Array",
		view = "Uint32",
	}
	i64 {
		encode = array_i64_encode,
		from_slice = array_from_i64_slice,
		embed = "array.i64.encode",
		constructor = "BigInt64Array",
		view = "BigInt64",
	}
	u64 {
		encode = array_u64_encode,
		from_slice = array_from_u64_slice,
		embed = "array.u64.encode",
		constructor = "BigUint64Array",
		view = "BigUint64",
	}
	f32 {
		encode = array_f32_encode,
		from_slice = array_from_f32_slice,
		embed = "array.f32.encode",
		constructor = "Float32Array",
		view = "Float32",
	}
	f64 {
		encode = array_f64_encode,
		from_slice = array_from_f64_slice,
		embed = "array.f64.encode",
		constructor = "Float64Array",
		view = "Float64",
	}
	#[cfg(target_arch = "wasm32")]
	isize {
		encode = array_isize32_encode,
		from_slice = array_from_isize_slice,
		embed = "array.isize.encode",
		constructor = "Int32Array",
		view = "Int32",
	}
	#[cfg(target_arch = "wasm64")]
	isize {
		encode = array_isize64_encode,
		from_slice = array_from_isize_slice,
		embed = "array.isize.encode",
		constructor = "BigInt64Array",
		view = "BigInt64",
	}
	#[cfg(target_arch = "wasm32")]
	usize {
		encode = array_usize32_encode,
		from_slice = array_from_usize_slice,
		embed = "array.usize.encode",
		constructor = "Uint32Array",
		view = "Uint32",
	}
	#[cfg(target_arch = "wasm64")]
	usize {
		encode = array_usize64_encode,
		from_slice = array_from_usize_slice,
		embed = "array.usize.encode",
		constructor = "BigUint64Array",
		view = "BigUint64",
	}
}

impl<T: JsCast> Array<T> {
	#[must_use]
	pub fn iter(&self) -> ArrayIter<'_, T> {
		ArrayIter {
			range: 0..self.length(),
			array: self,
		}
	}
}

/// A borrowed Rust iterator over an [`Array`].
pub struct ArrayIter<'array, T = JsValue> {
	array: &'array Array<T>,
	range: Range<u32>,
}

impl<T: JsCast> core::iter::Iterator for ArrayIter<'_, T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.range
			.next()
			.map(|index| self.array.get_unchecked(index))
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.range
			.nth(n)
			.map(|index| self.array.get_unchecked(index))
	}

	fn count(self) -> usize {
		self.range.count()
	}

	fn last(self) -> Option<Self::Item> {
		self.range
			.last()
			.map(|index| self.array.get_unchecked(index))
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.range.size_hint()
	}
}

impl<T: JsCast> DoubleEndedIterator for ArrayIter<'_, T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.range
			.next_back()
			.map(|index| self.array.get_unchecked(index))
	}

	fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
		self.range
			.nth_back(n)
			.map(|index| self.array.get_unchecked(index))
	}
}

impl<T: JsCast> ExactSizeIterator for ArrayIter<'_, T> {}
impl<T: JsCast> core::iter::FusedIterator for ArrayIter<'_, T> {}

/// An owned Rust iterator over an [`Array`].
pub struct ArrayIntoIter<T = JsValue> {
	array: Array<T>,
	range: Range<u32>,
}

impl<T: JsCast> core::iter::Iterator for ArrayIntoIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.range
			.next()
			.map(|index| self.array.get_unchecked(index))
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.range
			.nth(n)
			.map(|index| self.array.get_unchecked(index))
	}

	fn count(self) -> usize {
		self.range.count()
	}

	fn last(self) -> Option<Self::Item> {
		self.range
			.last()
			.map(|index| self.array.get_unchecked(index))
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.range.size_hint()
	}
}

impl<T: JsCast> DoubleEndedIterator for ArrayIntoIter<T> {
	fn next_back(&mut self) -> Option<Self::Item> {
		self.range
			.next_back()
			.map(|index| self.array.get_unchecked(index))
	}

	fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
		self.range
			.nth_back(n)
			.map(|index| self.array.get_unchecked(index))
	}
}

impl<T: JsCast> ExactSizeIterator for ArrayIntoIter<T> {}
impl<T: JsCast> core::iter::FusedIterator for ArrayIntoIter<T> {}

impl<'array, T: JsCast> IntoIterator for &'array Array<T> {
	type Item = T;
	type IntoIter = ArrayIter<'array, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<T: JsCast> IntoIterator for Array<T> {
	type Item = T;
	type IntoIter = ArrayIntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		let range = 0..self.length();
		ArrayIntoIter { array: self, range }
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "array.checked_length",
	"(array) => {{",
	"    const rawLength = array.length",
	"    const length = rawLength >>> 0",
	"    if (rawLength !== length) throw new TypeError('invalid array length')",
	"    return length",
	"}}",
);

impl<T, const N: usize> From<&[T; N]> for Array<T>
where
	Self: for<'a> From<&'a [T]>,
{
	fn from(value: &[T; N]) -> Self {
		value.as_slice().into()
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum TryFromArrayError {
	LengthMismatch { actual: u32, expected: usize },
	JavaScript(JsValue),
}

impl Display for TryFromArrayError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::LengthMismatch { actual, expected } => {
				write!(
					f,
					"array length {actual} does not match destination length {expected}"
				)
			}
			Self::JavaScript(_) => f.write_str("JavaScript threw while copying the array"),
		}
	}
}

impl Error for TryFromArrayError {}

fn check_copy_length(
	result: Result<u32, JsValue>,
	expected: usize,
) -> Result<(), TryFromArrayError> {
	let actual = result.map_err(TryFromArrayError::JavaScript)?;

	if usize::try_from(actual) == Ok(expected) {
		Ok(())
	} else {
		Err(TryFromArrayError::LengthMismatch { actual, expected })
	}
}

impl<T: JsCast> Array<T> {
	pub fn to_slice(&self, slice: &mut [T]) -> Result<(), TryFromArrayError> {
		check_copy_length(array_checked_length(self.as_untyped()), slice.len())?;
		let slots = externref::reserve_slots(slice.len());

		let result = {
			let js_slice = JsValue::from_slice_mut(slice);
			// SAFETY: Parameters are correct. `write_output` is false, so JavaScript
			// does not write through the destination pointer.
			unsafe {
				array_js_value_encode(
					self.as_untyped(),
					PtrMut::new(js_slice),
					PtrLength::new(js_slice),
					slots.ptr(),
					slots.len(),
					false,
				)
			}
		};

		check_copy_length(result, slice.len())?;
		slots.replace(slice);
		Ok(())
	}

	pub fn to_uninit_slice<'slice>(
		&self,
		slice: &'slice mut [MaybeUninit<T>],
	) -> Result<&'slice mut [T], TryFromArrayError> {
		check_copy_length(array_checked_length(self.as_untyped()), slice.len())?;
		let js_slice = JsValue::from_uninit_slice_mut(slice);
		let slots = externref::reserve_slots(js_slice.len());

		// SAFETY: Parameters are correct.
		let result = unsafe {
			array_js_value_encode(
				self.as_untyped(),
				PtrMut::from_uninit_slice(js_slice),
				PtrLength::from_uninit_slice(js_slice),
				slots.ptr(),
				slots.len(),
				true,
			)
		};

		check_copy_length(result, slice.len())?;
		slots.commit();
		// SAFETY: Correctly initialized in JS.
		Ok(unsafe { assume_init_mut(slice) })
	}

	pub fn to_array<const N: usize>(&self) -> Result<[T; N], TryFromArrayError> {
		check_copy_length(array_checked_length(self.as_untyped()), N)?;
		let mut array: MaybeUninit<[T; N]> = MaybeUninit::uninit();
		let slots = externref::reserve_slots(N);
		let js_array = JsValue::from_mut_uninit_array(&mut array);

		// SAFETY: Parameters are correct.
		let result = unsafe {
			array_js_value_encode(
				self.as_untyped(),
				PtrMut::from_uninit_array(js_array),
				PtrLength::from_uninit_array(js_array),
				slots.ptr(),
				slots.len(),
				true,
			)
		};

		check_copy_length(result, N)?;
		slots.commit();
		// SAFETY: Correctly initialized in JS.
		Ok(unsafe { array.assume_init() })
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "array.js_value.encode",
	required_embeds = [
		("js_sys", "externref.table"),
		("js_sys", "view.getUint32"),
		("js_sys", "view.setUint32")
	],
	"(array, arrPtr, arrLen, refPtr, refLen, writeOutput) => {{",
	"    const rawLength = array.length",
	"    const length = rawLength >>> 0",
	"    if (rawLength !== length) throw new TypeError('invalid array length')",
	"    if (length !== arrLen) return length",
	"",
	"    const table = this.#jsEmbed.js_sys['externref.table']",
	"    const refIndices = new Uint32Array(",
	"        this.#jsEmbed.js_sys['view.getUint32'](refPtr, refLen),",
	"    )",
	"    if (writeOutput) {{",
	"        for (let arrayIndex = 0; arrayIndex < arrLen; arrayIndex++) {{",
	"            const elemIndex = refIndices[arrayIndex]",
	"            table.set(elemIndex, array[arrayIndex])",
	"        }}",
	"        this.#jsEmbed.js_sys['view.setUint32'](arrPtr, refIndices, arrLen)",
	"    }} else {{",
	"        for (let arrayIndex = 0; arrayIndex < arrLen; arrayIndex++) {{",
	"            table.set(refIndices[arrayIndex], array[arrayIndex])",
	"        }}",
	"    }}",
	"    return length",
	"}}",
);

impl<T: JsCast> From<&[T]> for Array<T> {
	fn from(value: &[T]) -> Self {
		array_from_js_value_slice(value)
	}
}

// MSRV: Stable on v1.93.
const unsafe fn assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
	// SAFETY: copied from Std.
	unsafe { &mut *(ptr::from_mut::<[MaybeUninit<T>]>(slice) as *mut [T]) }
}
