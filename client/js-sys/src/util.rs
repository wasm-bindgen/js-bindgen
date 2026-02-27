use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::{mem, ptr};

use crate::hazard::Input;

macro_rules! thread_local {
	($($vis:vis static $name:ident: $ty:ty = $value:expr;)*) => {
		#[cfg_attr(target_feature = "atomics", thread_local)]
		$($vis static $name: $crate::util::LocalKey<$ty> = $crate::util::LocalKey::new($value);)*
	};
}

pub(crate) struct LocalKey<T>(T);

// SAFETY: Multi-threading is not possible without `atomics`.
#[cfg(not(target_feature = "atomics"))]
unsafe impl<T> Send for LocalKey<T> {}

// SAFETY: Multi-threading is not possible without `atomics`.
#[cfg(not(target_feature = "atomics"))]
unsafe impl<T> Sync for LocalKey<T> {}

impl<T> LocalKey<T> {
	pub(crate) const fn new(value: T) -> Self {
		Self(value)
	}

	pub(crate) fn with<F, R>(&self, f: F) -> R
	where
		F: FnOnce(&T) -> R,
	{
		f(&self.0)
	}
}

#[repr(C)]
pub struct ExternValue<T> {
	len: PtrLength<T>,
	value: T,
}

#[expect(dead_code, reason = "custom sections are considered dead-code")]
impl<T> ExternValue<T> {
	pub(crate) const ASM_IMPORT_TYPE: &str = <*const Self>::ASM_IMPORT_TYPE;
	#[cfg(target_arch = "wasm32")]
	pub(crate) const ASM_TYPE: &str = "i32";
	#[cfg(target_arch = "wasm64")]
	pub(crate) const ASM_TYPE: &str = "i64";
	#[cfg(target_arch = "wasm32")]
	pub(crate) const ASM_CONV: Option<&str> = None;
	#[cfg(target_arch = "wasm64")]
	pub(crate) const ASM_CONV: Option<&str> = Some("f64.convert_i64_u");

	#[cfg(target_arch = "wasm32")]
	const DATA_VIEW_GET: &str = "Uint32";
	#[cfg(target_arch = "wasm64")]
	const DATA_VIEW_GET: &str = "Float64";

	pub(crate) fn new(value: T) -> Self {
		Self {
			len: PtrLength::internal(ptr::null(), mem::size_of::<T>()),
			value,
		}
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "extern_value",
	"(valuePtr) => {{",
	"	const view = new DataView(this.#memory.buffer, valuePtr)",
	"	const ptr = valuePtr + {}",
	"	const len = view.get{}(0, true)",
	"	return {{ ptr, len }}",
	"}}",
	const mem::offset_of!(ExternValue::<()>, value),
	interpolate ExternValue::<()>::DATA_VIEW_GET,
);

#[repr(C)]
pub struct ExternSlice<T> {
	ptr: <*const T as Input>::Type,
	len: PtrLength<T>,
}

impl<T> ExternSlice<T> {
	pub(crate) const ASM_IMPORT_TYPE: &str = <*const Self>::ASM_IMPORT_TYPE;
	pub(crate) const ASM_TYPE: &str = ExternValue::<()>::ASM_TYPE;
	pub(crate) const ASM_CONV: Option<&str> = ExternValue::<()>::ASM_CONV;

	pub(crate) fn new(value: &[T]) -> Self {
		Self {
			ptr: <*const T>::into_raw(value.as_ptr()),
			len: PtrLength::new(value),
		}
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "extern_ref",
	"(refPtr) => {{",
	"	const view = new DataView(this.#memory.buffer, refPtr, {})",
	"	const ptr = view.get{data_view}(0, true)",
	"	const len = view.get{data_view}({}, true)",
	"	return {{ ptr, len }}",
	"}}",
	data_view = interpolate ExternValue::<()>::DATA_VIEW_GET,
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
		#[expect(clippy::cast_precision_loss, reason = "checked")]
		let len = {
			debug_assert!(
				ptr.addr() + len * mem::size_of::<T>() < 0x0020_0000_0000_0000,
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
	const ASM_IMPORT_TYPE: &str = Self::Type::ASM_IMPORT_TYPE;
	const ASM_TYPE: &str = Self::Type::ASM_TYPE;
	const JS_CONV: Option<(&str, Option<&str>)> = Self::Type::JS_CONV;

	#[cfg(target_arch = "wasm32")]
	type Type = usize;
	#[cfg(target_arch = "wasm64")]
	type Type = f64;

	fn into_raw(self) -> Self::Type {
		self.len
	}
}
