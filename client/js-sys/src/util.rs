use core::marker::PhantomData;
use core::mem;
use core::mem::MaybeUninit;

use crate::hazard::{Input, InputJsConv, InputWatConv};

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
pub struct ExternValue<T>(T);

impl<T> ExternValue<T> {
	pub(crate) const WAT_TYPE: &str = WAT_PTR_TYPE;
	#[cfg(target_arch = "wasm32")]
	pub(crate) const WAT_CONV: Option<InputWatConv> = None;
	#[cfg(target_arch = "wasm64")]
	pub(crate) const WAT_CONV: Option<InputWatConv> = Some(InputWatConv {
		import: None,
		conv: "f64.convert_i64_u",
		r#type: "f64",
	});

	pub(crate) fn new(value: T) -> Self {
		Self(value)
	}
}

#[repr(C)]
pub struct ExternSlice<T> {
	ptr: PtrConst<T>,
	len: PtrLength<T>,
}

#[expect(dead_code, reason = "custom sections are considered dead-code")]
impl<T> ExternSlice<T> {
	pub(crate) const WAT_TYPE: &str = ExternValue::<()>::WAT_TYPE;
	pub(crate) const WAT_CONV: Option<InputWatConv> = ExternValue::<()>::WAT_CONV;

	#[cfg(target_arch = "wasm32")]
	const VIEW_FN: &str = "view.getUint32";
	#[cfg(target_arch = "wasm64")]
	const VIEW_FN: &str = "view.getFloat64";

	pub(crate) fn new(value: &[T]) -> Self {
		Self {
			ptr: PtrConst::new(value),
			len: PtrLength::new(value),
		}
	}
}

// Verify that we can access `ExternSlice` via a `TypedArray` with two elements.
const _: () = {
	debug_assert!(
		mem::align_of::<ExternSlice<()>>() == mem::size_of::<<PtrConst<()> as Input>::Type>()
	);
};

js_bindgen::embed_js!(
	module = "js_sys",
	name = "extern_ref",
	required_embeds = [("js_sys", ExternSlice::<()>::VIEW_FN)],
	"(refPtr) => {{",
	"	const [ptr, len] = this.#jsEmbed.js_sys['{}'](refPtr, 2)",
	"	return {{ ptr, len }}",
	"}}",
	interpolate ExternSlice::<()>::VIEW_FN,
);

#[cfg(target_arch = "wasm32")]
type WatUsizeType = u32;
#[cfg(target_arch = "wasm64")]
type WatUsizeType = f64;

#[cfg(target_arch = "wasm32")]
pub(crate) const WAT_PTR_TYPE: &str = "i32";
#[cfg(target_arch = "wasm64")]
pub(crate) const WAT_PTR_TYPE: &str = "i64";

#[repr(transparent)]
pub(crate) struct PtrConst<T> {
	ptr: <Self as Input>::Type,
	_ty: PhantomData<T>,
}

impl<T> PtrConst<T> {
	pub(crate) fn new(value: &[T]) -> Self {
		let ptr = value.as_ptr();

		#[cfg(target_arch = "wasm64")]
		#[expect(
			clippy::cast_precision_loss,
			reason = "can't be larger than `MAX_SAFE_INTEGER`"
		)]
		let ptr = ptr.addr() as <Self as Input>::Type;

		Self {
			ptr,
			_ty: PhantomData,
		}
	}
}

// SAFETY: Delegated to already implemented types.
unsafe impl<T> Input for PtrConst<T> {
	const WAT_TYPE: &str = WatUsizeType::WAT_TYPE;
	const WAT_CONV: Option<InputWatConv> = WatUsizeType::WAT_CONV;
	const JS_CONV: Option<InputJsConv> = WatUsizeType::JS_CONV;

	#[cfg(target_arch = "wasm32")]
	type Type = *const T;
	#[cfg(target_arch = "wasm64")]
	type Type = f64;

	fn into_raw(self) -> Self::Type {
		self.ptr
	}
}

#[repr(transparent)]
pub(crate) struct PtrMut<T> {
	ptr: <Self as Input>::Type,
	_ty: PhantomData<T>,
}

impl<T> PtrMut<T> {
	pub(crate) fn new(value: &mut [T]) -> Self {
		Self::internal(value.as_mut_ptr())
	}

	pub(crate) fn from_uninit_array<const N: usize>(value: &mut MaybeUninit<[T; N]>) -> Self {
		Self::internal(value.as_mut_ptr().cast())
	}

	pub(crate) fn from_uninit_slice(value: &mut [MaybeUninit<T>]) -> Self {
		Self::internal(value.as_mut_ptr().cast())
	}

	fn internal(ptr: *mut T) -> Self {
		#[cfg(target_arch = "wasm64")]
		#[expect(
			clippy::cast_precision_loss,
			reason = "can't be larger than `MAX_SAFE_INTEGER`"
		)]
		let ptr = ptr.addr() as <Self as Input>::Type;

		Self {
			ptr,
			_ty: PhantomData,
		}
	}
}

// SAFETY: Delegated to already implemented types.
unsafe impl<T> Input for PtrMut<T> {
	const WAT_TYPE: &str = WatUsizeType::WAT_TYPE;
	const WAT_CONV: Option<InputWatConv> = WatUsizeType::WAT_CONV;
	const JS_CONV: Option<InputJsConv> = WatUsizeType::JS_CONV;

	#[cfg(target_arch = "wasm32")]
	type Type = *mut T;
	#[cfg(target_arch = "wasm64")]
	type Type = f64;

	fn into_raw(self) -> Self::Type {
		self.ptr
	}
}

#[repr(transparent)]
pub(crate) struct PtrLength<T> {
	len: <Self as Input>::Type,
	_ty: PhantomData<T>,
}

impl<T> PtrLength<T> {
	pub(crate) fn new(value: &[T]) -> Self {
		Self::internal(value.len())
	}

	pub(crate) fn from_uninit_array<const N: usize>(_: &MaybeUninit<[T; N]>) -> Self {
		Self::internal(N)
	}

	pub(crate) fn from_uninit_slice(value: &[MaybeUninit<T>]) -> Self {
		Self::internal(value.len())
	}

	fn internal(len: usize) -> Self {
		#[cfg(target_arch = "wasm64")]
		#[expect(
			clippy::cast_precision_loss,
			reason = "can't be larger than `MAX_SAFE_INTEGER`"
		)]
		let len = len as <Self as Input>::Type;

		Self {
			len,
			_ty: PhantomData,
		}
	}
}

// SAFETY: Delegated to already implemented types.
unsafe impl<T> Input for PtrLength<T> {
	const WAT_TYPE: &str = WatUsizeType::WAT_TYPE;
	const WAT_CONV: Option<InputWatConv> = WatUsizeType::WAT_CONV;
	const JS_CONV: Option<InputJsConv> = WatUsizeType::JS_CONV;

	#[cfg(target_arch = "wasm32")]
	type Type = usize;
	#[cfg(target_arch = "wasm64")]
	type Type = f64;

	fn into_raw(self) -> Self::Type {
		self.len
	}
}

#[cfg(not(any(js_sys_assume_endianness = "little", js_sys_assume_endianness = "big")))]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "isLittleEndian",
	"(() => {{",
	"	const buffer = new ArrayBuffer(2)",
	"	new DataView(buffer).setInt16(0, 256, true)",
	"	return new Int16Array(buffer)[0] === 256;",
	"}})()",
);

#[cfg(all(
	js_sys_target_feature = "unstable-rab",
	not(js_sys_assume_endianness = "little")
))]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.DataView",
	"new DataView(this.#memory.toResizableBuffer())",
);

macro_rules! buffer {
	($type:literal, $size:literal) => {
		#[cfg(all(js_sys_target_feature = "unstable-rab", not(js_sys_assume_endianness = "big")))]
		js_bindgen::embed_js!(
			module = "js_sys",
			name = concat!("view.", $type),
			"new {}Array(this.#memory.toResizableBuffer())",
			interpolate $type,
		);

		#[cfg(not(any(js_sys_assume_endianness = "little", js_sys_assume_endianness = "big")))]
		js_bindgen::embed_js!(
			module = "js_sys",
			name = concat!("view.get", $type),
			required_embeds = [
				("js_sys", "isLittleEndian"),
				#[cfg(js_sys_target_feature = "unstable-rab")]
				("js_sys", concat!("view.", $type)),
				#[cfg(js_sys_target_feature = "unstable-rab")]
				("js_sys", "view.DataView")
			],
			"(ptr, count) => {{",
			#[cfg(debug_assertions)]
			"	if (ptr % {size} !== 0)",
			"		throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
			"",
			"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
			#[cfg(js_sys_target_feature = "unstable-rab")]
			"		const base = ptr / {size}",
			"		const view = {buffer}",
			"		return Array.from(view)",
			"	}} else {{",
			"		const out = new Array(count)",
			"		const view = {data}",
			"		for (let index = 0; index < count; index++) {{",
			"			out[index] = view.get{type}(ptr + index * {size}, true)",
			"		}}",
			"		return out",
			"	}}",
			"}}",
			size = const $size,
			#[cfg(js_sys_target_feature = "unstable-rab")]
			buffer = interpolate concat!("this.#jsEmbed.js_sys['view.", $type, "'].subarray(base, base + count)"),
			#[cfg(not(js_sys_target_feature = "unstable-rab"))]
			buffer = interpolate concat!("new ", $type, "Array(this.#memory.buffer, ptr, count)"),
			#[cfg(js_sys_target_feature = "unstable-rab")]
			data = interpolate "this.#jsEmbed.js_sys['view.DataView']",
			#[cfg(not(js_sys_target_feature = "unstable-rab"))]
			data = interpolate "new DataView(this.#memory.buffer)",
			type = interpolate $type,
		);

		#[cfg(js_sys_assume_endianness = "little")]
		js_bindgen::embed_js!(
			module = "js_sys",
			name = concat!("view.get", $type),
			required_embeds = [
				#[cfg(js_sys_target_feature = "unstable-rab")]
				("js_sys", concat!("view.", $type)),
			],
			"(ptr, count) => {{",
			#[cfg(debug_assertions)]
			"	if (ptr % {size} !== 0)",
			"		throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
			"",
			#[cfg(js_sys_target_feature = "unstable-rab")]
			"	const base = ptr / {size}",
			"	const view = {buffer}",
			"	return Array.from(view)",
			"}}",
			size = const $size,
			#[cfg(js_sys_target_feature = "unstable-rab")]
			buffer = interpolate concat!("this.#jsEmbed.js_sys['view.", $type, "'].subarray(base, base + count)"),
			#[cfg(not(js_sys_target_feature = "unstable-rab"))]
			buffer = interpolate concat!("new ", $type, "Array(this.#memory.buffer, ptr, count)"),
		);

		#[cfg(js_sys_assume_endianness = "big")]
		js_bindgen::embed_js!(
			module = "js_sys",
			name = concat!("view.get", $type),
			required_embeds = [
				#[cfg(js_sys_target_feature = "unstable-rab")]
				("js_sys", "view.DataView")
			],
			"(ptr, count) => {{",
			#[cfg(debug_assertions)]
			"	if (ptr % {size} !== 0)",
			"		throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
			"",
			"	const out = new Array(count)",
			"	const view = {data}",
			"	for (let index = 0; index < count; index++) {{",
			"		out[index] = view.get{type}(ptr + index * {size}, true)",
			"	}}",
			"	return out",
			"}}",
			size = const $size,
			#[cfg(js_sys_target_feature = "unstable-rab")]
			data = interpolate "this.#jsEmbed.js_sys['view.DataView']",
			#[cfg(not(js_sys_target_feature = "unstable-rab"))]
			data = interpolate "new DataView(this.#memory.buffer)",
			type = interpolate $type,
		);

		#[cfg(not(any(js_sys_assume_endianness = "little", js_sys_assume_endianness = "big")))]
		js_bindgen::embed_js!(
			module = "js_sys",
			name = concat!("view.set", $type),
			required_embeds = [
				("js_sys", "isLittleEndian"),
				#[cfg(js_sys_target_feature = "unstable-rab")]
				("js_sys", concat!("view.", $type)),
				#[cfg(js_sys_target_feature = "unstable-rab")]
				("js_sys", "view.DataView")
			],
			"(ptr, array) => {{",
			#[cfg(debug_assertions)]
			"	if (ptr % {size} !== 0)",
			"		throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
			"",
			"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
			"		{buffer}.set(array, ptr / {size})",
			"	}} else {{",
			"		const view = {data}",
			"		for (let index = 0; index < array.length; index++) {{",
			"			view.set{type}(ptr + index * {size}, array[index], true)",
			"		}}",
			"	}}",
			"}}",
			size = const $size,
			#[cfg(js_sys_target_feature = "unstable-rab")]
			buffer = interpolate concat!("this.#jsEmbed.js_sys['view.", $type, "']"),
			#[cfg(not(js_sys_target_feature = "unstable-rab"))]
			buffer = interpolate concat!("new ", $type, "Array(this.#memory.buffer)"),
			#[cfg(js_sys_target_feature = "unstable-rab")]
			data = interpolate "this.#jsEmbed.js_sys['view.DataView']",
			#[cfg(not(js_sys_target_feature = "unstable-rab"))]
			data = interpolate "new DataView(this.#memory.buffer)",
			type = interpolate $type,
		);

		#[cfg(js_sys_assume_endianness = "little")]
		js_bindgen::embed_js!(
			module = "js_sys",
			name = concat!("view.set", $type),
			required_embeds = [
				#[cfg(js_sys_target_feature = "unstable-rab")]
				("js_sys", concat!("view.", $type)),
			],
			"(ptr, array) => {{",
			#[cfg(debug_assertions)]
			"	if (ptr % {size} !== 0)",
			"		throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
			"",
			"	{buffer}.set(array, ptr / {size})",
			"}}",
			size = const $size,
			#[cfg(js_sys_target_feature = "unstable-rab")]
			buffer = interpolate concat!("this.#jsEmbed.js_sys['view.", $type, "']"),
			#[cfg(not(js_sys_target_feature = "unstable-rab"))]
			buffer = interpolate concat!("new ", $type, "Array(this.#memory.buffer)"),
		);

		#[cfg(js_sys_assume_endianness = "big")]
		js_bindgen::embed_js!(
			module = "js_sys",
			name = concat!("view.set", $type),
			required_embeds = [
				#[cfg(js_sys_target_feature = "unstable-rab")]
				("js_sys", "view.DataView")
			],
			"(ptr, array) => {{",
			#[cfg(debug_assertions)]
			"	if (ptr % {size} !== 0)",
			"		throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
			"",
			"	const view = {data}",
			"	for (let index = 0; index < array.length; index++) {{",
			"		view.set{type}(ptr + index * {size}, array[index], true)",
			"	}}",
			"}}",
			size = const $size,
			#[cfg(js_sys_target_feature = "unstable-rab")]
			data = interpolate "this.#jsEmbed.js_sys['view.DataView']",
			#[cfg(not(js_sys_target_feature = "unstable-rab"))]
			data = interpolate "new DataView(this.#memory.buffer)",
			type = interpolate $type,
		);
	};
}

buffer!("Uint32", 4_usize);
buffer!("Int32", 4_usize);
buffer!("Float64", 8_usize);
buffer!("BigUint64", 8_usize);
buffer!("BigInt64", 8_usize);
