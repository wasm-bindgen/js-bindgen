use core::marker::PhantomData;
use core::mem;
use core::mem::MaybeUninit;

use crate::hazard::Input;

#[repr(C)]
pub struct ExternSlice<T> {
	ptr: <*const T as Input>::Type,
	len: PtrLength<T>,
}

#[expect(dead_code, reason = "custom sections are considered dead-code")]
impl<T> ExternSlice<T> {
	#[cfg(target_arch = "wasm32")]
	const DATA_VIEW_GET: &str = "Uint32";
	#[cfg(target_arch = "wasm64")]
	const DATA_VIEW_GET: &str = "Float64";

	pub(crate) fn new(value: &[T]) -> Self {
		Self {
			ptr: <*const T>::into_raw(value.as_ptr()),
			len: PtrLength::new(value),
		}
	}
}

// SAFETY: Implementation.
unsafe impl<T> Input for ExternSlice<T> {
	const IMPORT_TYPE: &'static str = <*const T>::IMPORT_TYPE;
	#[cfg(target_arch = "wasm32")]
	const TYPE: &'static str = "i32";
	#[cfg(target_arch = "wasm64")]
	const TYPE: &'static str = "i64";
	#[cfg(target_arch = "wasm64")]
	const CONV: &'static str = "f64.convert_i64_u";

	type Type = Self;

	fn into_raw(self) -> Self::Type {
		// Only for validation on Wasm64.
		<*const Self>::into_raw(&raw const self);

		self
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "extern_slice",
	"(slicePtr) => {{",
	"	const view = new DataView(this.#memory.buffer, slicePtr, {})",
	"	const ptr = view.get{data_view}(0, true)",
	"	const len = view.get{data_view}({}, true)",
	"	return {{ ptr, len }}",
	"}}",
	data_view = interpolate ExternSlice::<()>::DATA_VIEW_GET,
	const mem::size_of::<ExternSlice<()>>(),
	const mem::offset_of!(ExternSlice::<()>, len),
);

#[repr(transparent)]
pub(crate) struct PtrLength<T> {
	len: <Self as Input>::Type,
	_ty: PhantomData<T>,
}

impl<T> PtrLength<T> {
	pub(crate) fn new(value: &[T]) -> Self {
		Self::internal(value.as_ptr(), value.len())
	}

	pub(crate) fn from_uninit_array<const N: usize>(value: &MaybeUninit<[T; N]>) -> Self {
		Self::internal(value.as_ptr().cast(), N)
	}

	pub(crate) fn from_uninit_slice(value: &[MaybeUninit<T>]) -> Self {
		Self::internal(value.as_ptr().cast(), value.len())
	}

	fn internal(
		#[cfg_attr(
			not(target_arch = "wasm64"),
			expect(unused_variables, reason = "only needed for Wasm64")
		)]
		ptr: *const T,
		len: usize,
	) -> Self {
		#[cfg(target_arch = "wasm64")]
		let len = {
			debug_assert!(
				ptr.addr() + len * core::mem::size_of::<T>() < 0x20000000000000,
				"found pointer + length bigger than `Number.MAX_SAFE_INTEGER`"
			);
			len as f64
		};

		Self {
			len,
			_ty: PhantomData,
		}
	}
}

// SAFETY: Delegated to already implemented types.
unsafe impl<T> Input for PtrLength<T> {
	const IMPORT_TYPE: &str = Self::Type::IMPORT_TYPE;
	const TYPE: &str = Self::Type::TYPE;
	const JS_CONV: &str = Self::Type::JS_CONV;

	#[cfg(target_arch = "wasm32")]
	type Type = usize;
	#[cfg(target_arch = "wasm64")]
	type Type = f64;

	fn into_raw(self) -> Self::Type {
		self.len
	}
}
