use alloc::vec::Vec;
use core::error::Error;
use core::fmt::{self, Display, Formatter};
use core::mem::MaybeUninit;
use core::ops::Range;
use core::ptr;

use crate::builtins::{
	BigInt64Array, BigUint64Array, Float16Array, Float32Array, Float64Array, Int8Array, Int16Array,
	Int32Array, Uint8Array, Uint8ClampedArray, Uint16Array, Uint32Array,
};
use crate::util::{PtrConst, PtrLength, PtrMut};
use crate::{JsValue, js_sys};

/// A JavaScript typed-array type with a corresponding Rust storage element.
/// `Float16Array` uses `u16` to preserve raw `binary16` bits.
pub trait TypedArray: AsRef<JsValue> {
	type Element: Copy;
	type Value: Copy;

	const BYTES_PER_ELEMENT: u32;

	#[doc(hidden)]
	fn typed_array_length(&self) -> usize;

	#[doc(hidden)]
	fn typed_array_get(&self, index: usize) -> Option<Self::Value>;
}

/// A borrowed Rust iterator over a JavaScript typed array.
pub struct TypedArrayIter<'array, A: TypedArray> {
	array: &'array A,
	range: Range<usize>,
}

fn typed_array_iter_next<A: TypedArray>(array: &A, range: &mut Range<usize>) -> Option<A::Value> {
	let index = range.next()?;
	if let Some(value) = array.typed_array_get(index) {
		Some(value)
	} else {
		*range = 0..0;
		None
	}
}

fn typed_array_iter_nth_back<A: TypedArray>(
	array: &A,
	range: &mut Range<usize>,
	mut n: usize,
) -> Option<A::Value> {
	for index in range.rev() {
		if let Some(value) = array.typed_array_get(index) {
			if n == 0 {
				return Some(value);
			}
			n -= 1;
		}
	}
	None
}

impl<A: TypedArray> core::iter::Iterator for TypedArrayIter<'_, A> {
	type Item = A::Value;

	fn next(&mut self) -> Option<Self::Item> {
		typed_array_iter_next(self.array, &mut self.range)
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		let index = self.range.nth(n)?;
		if let Some(value) = self.array.typed_array_get(index) {
			Some(value)
		} else {
			self.range = 0..0;
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, Some(self.range.len()))
	}
}

impl<A: TypedArray> DoubleEndedIterator for TypedArrayIter<'_, A> {
	fn next_back(&mut self) -> Option<Self::Item> {
		typed_array_iter_nth_back(self.array, &mut self.range, 0)
	}

	fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
		typed_array_iter_nth_back(self.array, &mut self.range, n)
	}
}

impl<A: TypedArray> core::iter::FusedIterator for TypedArrayIter<'_, A> {}

/// An owned Rust iterator over a JavaScript typed array.
pub struct TypedArrayIntoIter<A: TypedArray> {
	array: A,
	range: Range<usize>,
}

impl<A: TypedArray> core::iter::Iterator for TypedArrayIntoIter<A> {
	type Item = A::Value;

	fn next(&mut self) -> Option<Self::Item> {
		typed_array_iter_next(&self.array, &mut self.range)
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		let index = self.range.nth(n)?;
		if let Some(value) = self.array.typed_array_get(index) {
			Some(value)
		} else {
			self.range = 0..0;
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, Some(self.range.len()))
	}
}

impl<A: TypedArray> DoubleEndedIterator for TypedArrayIntoIter<A> {
	fn next_back(&mut self) -> Option<Self::Item> {
		typed_array_iter_nth_back(&self.array, &mut self.range, 0)
	}

	fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
		typed_array_iter_nth_back(&self.array, &mut self.range, n)
	}
}

impl<A: TypedArray> core::iter::FusedIterator for TypedArrayIntoIter<A> {}

#[derive(Debug)]
#[non_exhaustive]
pub enum TypedArrayCopyError {
	LengthMismatch,
	LengthOutOfRange,
	JavaScript(JsValue),
}

impl Display for TypedArrayCopyError {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::LengthMismatch => {
				formatter.write_str("typed array length does not match the Rust slice length")
			}
			Self::LengthOutOfRange => {
				formatter.write_str("typed array length does not fit in a Rust `usize`")
			}
			Self::JavaScript(_) => {
				formatter.write_str("JavaScript threw while copying the typed array")
			}
		}
	}
}

impl Error for TypedArrayCopyError {}

fn check_copy(result: Result<bool, JsValue>) -> Result<(), TypedArrayCopyError> {
	match result {
		Ok(true) => Ok(()),
		Ok(false) => Err(TypedArrayCopyError::LengthMismatch),
		Err(error) => Err(TypedArrayCopyError::JavaScript(error)),
	}
}

const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

#[expect(
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss,
	reason = "the value is checked before converting it"
)]
fn length_to_usize(length: f64) -> Option<usize> {
	#[cfg(target_arch = "wasm32")]
	let maximum = f64::from(u32::MAX);
	#[cfg(target_arch = "wasm64")]
	let maximum = MAX_SAFE_INTEGER;

	(length >= 0.0 && length <= maximum && length % 1.0 == 0.0).then_some(length as usize)
}

fn expect_usize_length(length: f64) -> usize {
	length_to_usize(length).expect("typed array length does not fit in a Rust `usize`")
}

#[expect(
	clippy::cast_precision_loss,
	reason = "typed-array indices are limited to JavaScript's exact integer range"
)]
fn index_to_number(index: usize) -> Option<f64> {
	let number = index as f64;
	(number <= MAX_SAFE_INTEGER).then_some(number)
}

// The public `length()` binding intentionally follows normal JavaScript
// property lookup. Rust iteration and conversions describe the typed array's
// actual elements, so use the built-in `getter` just as native typed-array
// algorithms do.
js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.length",
	required_embeds = [("js_sys", "typed_array.intrinsics")],
	"(array) => this.#jsEmbed.js_sys['typed_array.intrinsics'].length.call(array)",
);

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "typed_array.length")]
	fn intrinsic_typed_array_length(array: &JsValue) -> f64;
}

macro_rules! typed_array_traits {
	($name:ident: $element:ty => $value:ty, bytes = $bytes:literal, get = $get:ident) => {
		impl $name {
			#[must_use]
			pub fn iter(&self) -> TypedArrayIter<'_, Self> {
				TypedArrayIter {
					array: self,
					range: 0..expect_usize_length(intrinsic_typed_array_length(self.as_ref())),
				}
			}
		}

		impl TypedArray for $name {
			type Element = $element;
			type Value = $value;

			const BYTES_PER_ELEMENT: u32 = $bytes;

			fn typed_array_length(&self) -> usize {
				expect_usize_length(intrinsic_typed_array_length(self.as_ref()))
			}

			fn typed_array_get(&self, index: usize) -> Option<Self::Value> {
				self.$get(index_to_number(index)?)
			}
		}

		impl<'array> IntoIterator for &'array $name {
			type Item = <$name as TypedArray>::Value;
			type IntoIter = TypedArrayIter<'array, $name>;

			fn into_iter(self) -> Self::IntoIter {
				self.iter()
			}
		}

		impl IntoIterator for $name {
			type Item = <$name as TypedArray>::Value;
			type IntoIter = TypedArrayIntoIter<$name>;

			fn into_iter(self) -> Self::IntoIter {
				let range = 0..expect_usize_length(intrinsic_typed_array_length(self.as_ref()));
				TypedArrayIntoIter { array: self, range }
			}
		}
	};
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.intrinsics",
	"(() => {{",
	"    const prototype = Object.getPrototypeOf(Uint8Array.prototype)",
	"    return {{",
	"        buffer: Object.getOwnPropertyDescriptor(prototype, 'buffer').get,",
	"        byteOffset: Object.getOwnPropertyDescriptor(prototype, 'byteOffset').get,",
	"        length: Object.getOwnPropertyDescriptor(prototype, 'length').get,",
	"        set: prototype.set,",
	"    }}",
	"}})()",
);

macro_rules! typed_array_interop {
	(
		$name:ident : $element:ty,
		constructor = $constructor:literal,
		view = $view:literal,
		bytes = $bytes:literal,
		copy_to = $copy_to:ident,
		copy_from = $copy_from:ident,
		from_slice = $from_slice:ident,
		copy_to_embed = $copy_to_embed:literal,
		copy_from_embed = $copy_from_embed:literal,
		from_slice_embed = $from_slice_embed:literal,
	) => {
		#[js_sys(js_sys = crate)]
		extern "js-sys" {
			#[js_sys(js_embed = $copy_to_embed)]
			unsafe fn $copy_to(
				array: &$name,
				ptr: PtrMut<$element>,
				len: PtrLength<$element>,
			) -> Result<bool, JsValue>;

			#[js_sys(js_embed = $copy_from_embed)]
			unsafe fn $copy_from(
				array: &$name,
				ptr: PtrConst<$element>,
				len: PtrLength<$element>,
			) -> Result<bool, JsValue>;

			#[js_sys(js_embed = $from_slice_embed)]
			unsafe fn $from_slice(
				ptr: PtrConst<$element>,
				len: PtrLength<$element>,
			) -> $name;
		}

		impl $name {
			pub fn copy_to(
				&self,
				destination: &mut [$element],
			) -> Result<(), TypedArrayCopyError> {
				// SAFETY: The pointer and length describe `destination`.
				let result = unsafe {
					$copy_to(
						self,
						PtrMut::new(destination),
						PtrLength::new(destination),
					)
				};
				check_copy(result)
			}

			pub fn copy_to_uninit<'destination>(
				&self,
				destination: &'destination mut [MaybeUninit<$element>],
			) -> Result<&'destination mut [$element], TypedArrayCopyError> {
				// SAFETY: The pointer and length describe `destination`.
				let result = unsafe {
					$copy_to(
						self,
						PtrMut::from_uninit_slice(destination),
						PtrLength::from_uninit_slice(destination),
					)
				};
				check_copy(result)?;
				// SAFETY: JavaScript initialized every element after checking the length.
				Ok(unsafe { assume_init_mut(destination) })
			}

			pub fn copy_from(
				&self,
				source: &[$element],
			) -> Result<(), TypedArrayCopyError> {
				// SAFETY: The pointer and length describe `source`.
				let result = unsafe {
					$copy_from(self, PtrConst::new(source), PtrLength::new(source))
				};
				check_copy(result)
			}

			pub fn to_vec(&self) -> Result<Vec<$element>, TypedArrayCopyError> {
				let len = length_to_usize(intrinsic_typed_array_length(self.as_ref()))
					.ok_or(TypedArrayCopyError::LengthOutOfRange)?;
				let mut output = Vec::with_capacity(len);
				self.copy_to_uninit(&mut output.spare_capacity_mut()[..len])?;
				// SAFETY: `copy_to_uninit` initialized all `len` elements.
				unsafe { output.set_len(len) };
				Ok(output)
			}
		}

		impl From<&[$element]> for $name {
			fn from(source: &[$element]) -> Self {
				// SAFETY: The pointer and length describe `source`; the constructor
				// copies it before returning.
				unsafe { $from_slice(PtrConst::new(source), PtrLength::new(source)) }
			}
		}

		impl<const N: usize> From<&[$element; N]> for $name {
			fn from(source: &[$element; N]) -> Self {
				Self::from(source.as_slice())
			}
		}

		typed_array_traits!($name: $element => $element, bytes = $bytes, get = get);

		js_bindgen::embed_js!(
			module = "js_sys",
			name = $copy_to_embed,
			required_embeds = [
				("js_sys", "typed_array.intrinsics"),
				("js_sys", concat!("view.set", $view))
			],
			"(array, ptr, len) => {{",
			"    const intrinsics = this.#jsEmbed.js_sys['typed_array.intrinsics']",
			"    if (intrinsics.length.call(array) !== len) return false",
			"    this.#jsEmbed.js_sys['view.set{view}'](ptr, array, len)",
			"    return true",
			"}}",
			view = interpolate $view,
		);

		js_bindgen::embed_js!(
			module = "js_sys",
			name = $copy_from_embed,
			required_embeds = [
				("js_sys", "typed_array.intrinsics"),
				("js_sys", concat!("view.get", $view))
			],
			"(array, ptr, len) => {{",
			"    const intrinsics = this.#jsEmbed.js_sys['typed_array.intrinsics']",
			"    if (intrinsics.length.call(array) !== len) return false",
			"    intrinsics.set.call(",
			"        array, this.#jsEmbed.js_sys['view.get{view}'](ptr, len)",
			"    )",
			"    return true",
			"}}",
			view = interpolate $view,
		);

		js_bindgen::embed_js!(
			module = "js_sys",
			name = $from_slice_embed,
			required_embeds = [("js_sys", concat!("view.get", $view))],
			"(ptr, len) => new {constructor}(",
			"    this.#jsEmbed.js_sys['view.get{view}'](ptr, len)",
			")",
			constructor = interpolate $constructor,
			view = interpolate $view,
		);
	};
}

typed_array_interop! {
	Int8Array: i8,
	constructor = "Int8Array",
	view = "Int8",
	bytes = 1,
	copy_to = int8_array_copy_to,
	copy_from = int8_array_copy_from,
	from_slice = int8_array_from_slice,
	copy_to_embed = "typed_array.Int8Array.copy_to",
	copy_from_embed = "typed_array.Int8Array.copy_from",
	from_slice_embed = "typed_array.Int8Array.from",
}

typed_array_interop! {
	Uint8Array: u8,
	constructor = "Uint8Array",
	view = "Uint8",
	bytes = 1,
	copy_to = uint8_array_copy_to,
	copy_from = uint8_array_copy_from,
	from_slice = uint8_array_from_slice,
	copy_to_embed = "typed_array.Uint8Array.copy_to",
	copy_from_embed = "typed_array.Uint8Array.copy_from",
	from_slice_embed = "typed_array.Uint8Array.from",
}

typed_array_interop! {
	Uint8ClampedArray: u8,
	constructor = "Uint8ClampedArray",
	view = "Uint8",
	bytes = 1,
	copy_to = uint8_clamped_array_copy_to,
	copy_from = uint8_clamped_array_copy_from,
	from_slice = uint8_clamped_array_from_slice,
	copy_to_embed = "typed_array.Uint8ClampedArray.copy_to",
	copy_from_embed = "typed_array.Uint8ClampedArray.copy_from",
	from_slice_embed = "typed_array.Uint8ClampedArray.from",
}

typed_array_interop! {
	Int16Array: i16,
	constructor = "Int16Array",
	view = "Int16",
	bytes = 2,
	copy_to = int16_array_copy_to,
	copy_from = int16_array_copy_from,
	from_slice = int16_array_from_slice,
	copy_to_embed = "typed_array.Int16Array.copy_to",
	copy_from_embed = "typed_array.Int16Array.copy_from",
	from_slice_embed = "typed_array.Int16Array.from",
}

typed_array_interop! {
	Uint16Array: u16,
	constructor = "Uint16Array",
	view = "Uint16",
	bytes = 2,
	copy_to = uint16_array_copy_to,
	copy_from = uint16_array_copy_from,
	from_slice = uint16_array_from_slice,
	copy_to_embed = "typed_array.Uint16Array.copy_to",
	copy_from_embed = "typed_array.Uint16Array.copy_from",
	from_slice_embed = "typed_array.Uint16Array.from",
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "typed_array.Float16Array.copy_to_u16")]
	unsafe fn float16_array_copy_to_u16(
		array: &Float16Array,
		ptr: PtrMut<u16>,
		len: PtrLength<u16>,
	) -> Result<bool, JsValue>;

	#[js_sys(js_embed = "typed_array.Float16Array.copy_from_u16")]
	unsafe fn float16_array_copy_from_u16(
		array: &Float16Array,
		ptr: PtrConst<u16>,
		len: PtrLength<u16>,
	) -> Result<bool, JsValue>;

	#[js_sys(js_embed = "typed_array.Float16Array.from_u16")]
	unsafe fn float16_array_from_u16(
		ptr: PtrConst<u16>,
		len: PtrLength<u16>,
	) -> Result<Float16Array, JsValue>;
}

impl Float16Array {
	/// Copies raw `IEEE 754 binary16` bit patterns into a new array.
	pub fn new_from_u16_slice(source: &[u16]) -> Result<Self, JsValue> {
		// SAFETY: The pointer and length describe `source`; JavaScript copies it
		// before returning.
		unsafe { float16_array_from_u16(PtrConst::new(source), PtrLength::new(source)) }
	}

	/// Copies the array's raw `IEEE 754 binary16` bit patterns into a slice.
	pub fn copy_to_u16_slice(&self, destination: &mut [u16]) -> Result<(), TypedArrayCopyError> {
		// SAFETY: The pointer and length describe `destination`.
		let result = unsafe {
			float16_array_copy_to_u16(self, PtrMut::new(destination), PtrLength::new(destination))
		};
		check_copy(result)
	}

	/// Copies raw `IEEE 754 binary16` bit patterns into uninitialized storage.
	pub fn copy_to_uninit_u16_slice<'destination>(
		&self,
		destination: &'destination mut [MaybeUninit<u16>],
	) -> Result<&'destination mut [u16], TypedArrayCopyError> {
		// SAFETY: The pointer and length describe `destination`.
		let result = unsafe {
			float16_array_copy_to_u16(
				self,
				PtrMut::from_uninit_slice(destination),
				PtrLength::from_uninit_slice(destination),
			)
		};
		check_copy(result)?;
		// SAFETY: JavaScript initialized every element after checking the length.
		Ok(unsafe { assume_init_mut(destination) })
	}

	/// Copies raw `IEEE 754 binary16` bit patterns into the array.
	pub fn copy_from_u16_slice(&self, source: &[u16]) -> Result<(), TypedArrayCopyError> {
		// SAFETY: The pointer and length describe `source`.
		let result = unsafe {
			float16_array_copy_from_u16(self, PtrConst::new(source), PtrLength::new(source))
		};
		check_copy(result)
	}

	/// Returns the array's raw `IEEE 754 binary16` bit patterns.
	pub fn to_u16_vec(&self) -> Result<Vec<u16>, TypedArrayCopyError> {
		let len = length_to_usize(intrinsic_typed_array_length(self.as_ref()))
			.ok_or(TypedArrayCopyError::LengthOutOfRange)?;
		let mut output = Vec::with_capacity(len);
		self.copy_to_uninit_u16_slice(&mut output.spare_capacity_mut()[..len])?;
		// SAFETY: `copy_to_uninit_u16_slice` initialized all `len` elements.
		unsafe { output.set_len(len) };
		Ok(output)
	}
}

typed_array_traits!(Float16Array: u16 => f32, bytes = 2, get = get_as_f32);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.Float16Array.copy_to_u16",
	required_embeds = [
		("js_sys", "typed_array.intrinsics"),
		("js_sys", "view.setUint16")
	],
	"(array, ptr, len) => {{",
	"    const intrinsics = this.#jsEmbed.js_sys['typed_array.intrinsics']",
	"    const length = intrinsics.length.call(array)",
	"    const bits = new Uint16Array(",
	"        intrinsics.buffer.call(array),",
	"        intrinsics.byteOffset.call(array),",
	"        length,",
	"    )",
	"    if (length !== len) return false",
	"    this.#jsEmbed.js_sys['view.setUint16'](ptr, bits, len)",
	"    return true",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.Float16Array.copy_from_u16",
	required_embeds = [
		("js_sys", "typed_array.intrinsics"),
		("js_sys", "view.getUint16")
	],
	"(array, ptr, len) => {{",
	"    const intrinsics = this.#jsEmbed.js_sys['typed_array.intrinsics']",
	"    const length = intrinsics.length.call(array)",
	"    const bits = new Uint16Array(",
	"        intrinsics.buffer.call(array),",
	"        intrinsics.byteOffset.call(array),",
	"        length,",
	"    )",
	"    if (length !== len) return false",
	"    intrinsics.set.call(bits, this.#jsEmbed.js_sys['view.getUint16'](ptr, len))",
	"    return true",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.Float16Array.from_u16",
	required_embeds = [
		("js_sys", "typed_array.intrinsics"),
		("js_sys", "view.getUint16")
	],
	"(ptr, len) => {{",
	"    const array = new Float16Array(len)",
	"    const intrinsics = this.#jsEmbed.js_sys['typed_array.intrinsics']",
	"    const bits = new Uint16Array(",
	"        intrinsics.buffer.call(array),",
	"        intrinsics.byteOffset.call(array),",
	"        intrinsics.length.call(array),",
	"    )",
	"    intrinsics.set.call(bits, this.#jsEmbed.js_sys['view.getUint16'](ptr, len))",
	"    return array",
	"}}",
);

typed_array_interop! {
	Int32Array: i32,
	constructor = "Int32Array",
	view = "Int32",
	bytes = 4,
	copy_to = int32_array_copy_to,
	copy_from = int32_array_copy_from,
	from_slice = int32_array_from_slice,
	copy_to_embed = "typed_array.Int32Array.copy_to",
	copy_from_embed = "typed_array.Int32Array.copy_from",
	from_slice_embed = "typed_array.Int32Array.from",
}

typed_array_interop! {
	Uint32Array: u32,
	constructor = "Uint32Array",
	view = "Uint32",
	bytes = 4,
	copy_to = uint32_array_copy_to,
	copy_from = uint32_array_copy_from,
	from_slice = uint32_array_from_slice,
	copy_to_embed = "typed_array.Uint32Array.copy_to",
	copy_from_embed = "typed_array.Uint32Array.copy_from",
	from_slice_embed = "typed_array.Uint32Array.from",
}

typed_array_interop! {
	Float32Array: f32,
	constructor = "Float32Array",
	view = "Float32",
	bytes = 4,
	copy_to = float32_array_copy_to,
	copy_from = float32_array_copy_from,
	from_slice = float32_array_from_slice,
	copy_to_embed = "typed_array.Float32Array.copy_to",
	copy_from_embed = "typed_array.Float32Array.copy_from",
	from_slice_embed = "typed_array.Float32Array.from",
}

typed_array_interop! {
	Float64Array: f64,
	constructor = "Float64Array",
	view = "Float64",
	bytes = 8,
	copy_to = float64_array_copy_to,
	copy_from = float64_array_copy_from,
	from_slice = float64_array_from_slice,
	copy_to_embed = "typed_array.Float64Array.copy_to",
	copy_from_embed = "typed_array.Float64Array.copy_from",
	from_slice_embed = "typed_array.Float64Array.from",
}

typed_array_interop! {
	BigInt64Array: i64,
	constructor = "BigInt64Array",
	view = "BigInt64",
	bytes = 8,
	copy_to = big_int64_array_copy_to,
	copy_from = big_int64_array_copy_from,
	from_slice = big_int64_array_from_slice,
	copy_to_embed = "typed_array.BigInt64Array.copy_to",
	copy_from_embed = "typed_array.BigInt64Array.copy_from",
	from_slice_embed = "typed_array.BigInt64Array.from",
}

typed_array_interop! {
	BigUint64Array: u64,
	constructor = "BigUint64Array",
	view = "BigUint64",
	bytes = 8,
	copy_to = big_uint64_array_copy_to,
	copy_from = big_uint64_array_copy_from,
	from_slice = big_uint64_array_from_slice,
	copy_to_embed = "typed_array.BigUint64Array.copy_to",
	copy_from_embed = "typed_array.BigUint64Array.copy_from",
	from_slice_embed = "typed_array.BigUint64Array.from",
}

// MSRV: Stable on v1.93.
const unsafe fn assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
	// SAFETY: copied from Std.
	unsafe { &mut *(ptr::from_mut::<[MaybeUninit<T>]>(slice) as *mut [T]) }
}
