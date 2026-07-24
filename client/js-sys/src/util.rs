use core::marker::PhantomData;
use core::mem::MaybeUninit;

use crate::hazard::{EmptySlot, IntoJS, IntoJsConv, Slot, WasmAbi, WatConv};

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

pub struct ExternSlice<T> {
	ptr: PtrConst<T>,
	len: PtrLength<T>,
}

// SAFETY: `ExternSlice` is represented by `PtrConst` and `PtrLength`.
unsafe impl<T> WasmAbi for ExternSlice<T> {
	type Slot1 = PtrConst<T>;
	type Slot2 = PtrLength<T>;
	type Slot3 = EmptySlot;
	type Slot4 = EmptySlot;

	fn split(self) -> (Self::Slot1, Self::Slot2, Self::Slot3, Self::Slot4) {
		(
			self.ptr,
			self.len,
			EmptySlot::default(),
			EmptySlot::default(),
		)
	}

	fn join(
		slot1: Self::Slot1,
		slot2: Self::Slot2,
		_slot3: Self::Slot3,
		_slot4: Self::Slot4,
	) -> Self {
		Self {
			ptr: slot1,
			len: slot2,
		}
	}
}

impl<T> ExternSlice<T> {
	pub(crate) fn new(value: &[T]) -> Self {
		Self {
			ptr: PtrConst::new(value),
			len: PtrLength::new(value),
		}
	}
}

#[cfg(target_arch = "wasm32")]
type JsPointerType = u32;
#[cfg(target_arch = "wasm64")]
type JsPointerType = f64;

pub(crate) const WAT_PTR_TYPE: &str = <usize as Slot>::WAT_TYPE;

#[cfg(target_arch = "wasm32")]
const PTR_INTO_JS_WAT_CONV: Option<WatConv> = None;

#[cfg(target_arch = "wasm64")]
const PTR_INTO_JS_WAT_CONV: Option<WatConv> = Some(WatConv {
	import: None,
	conv: "f64.convert_i64_u",
	r#type: "f64",
});

#[repr(transparent)]
pub struct PtrConst<T> {
	ptr: *const T,
}

impl<T> PtrConst<T> {
	pub(crate) fn new(value: &[T]) -> Self {
		Self {
			ptr: value.as_ptr(),
		}
	}
}

// SAFETY: `PtrConst` is transparent over a native Wasm pointer. On `wasm64`,
// the WAT adapter converts it to `f64` without losing precision.
unsafe impl<T> Slot for PtrConst<T> {
	const WAT_TYPE: &'static str = WAT_PTR_TYPE;
	const INTO_JS_WAT_CONV: Option<WatConv> = PTR_INTO_JS_WAT_CONV;
}

// SAFETY: The JavaScript conversion matches the WAT boundary representation.
unsafe impl<T> IntoJS for PtrConst<T> {
	const JS_CONV: Option<IntoJsConv> = JsPointerType::JS_CONV;

	type Abi = Self;

	fn into_abi(self) -> Self::Abi {
		self
	}
}

#[repr(transparent)]
pub(crate) struct PtrMut<T> {
	ptr: *mut T,
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
		Self { ptr }
	}
}

// SAFETY: `PtrMut` is transparent over a native Wasm pointer. On `wasm64`,
// the WAT adapter converts it to `f64` without losing precision.
unsafe impl<T> Slot for PtrMut<T> {
	const WAT_TYPE: &'static str = WAT_PTR_TYPE;
	const INTO_JS_WAT_CONV: Option<WatConv> = PTR_INTO_JS_WAT_CONV;
}

// SAFETY: The JavaScript conversion matches the WAT boundary representation.
unsafe impl<T> IntoJS for PtrMut<T> {
	const JS_CONV: Option<IntoJsConv> = JsPointerType::JS_CONV;

	type Abi = Self;

	fn into_abi(self) -> Self::Abi {
		self
	}
}

#[repr(transparent)]
pub struct PtrLength<T> {
	len: usize,
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
		Self {
			len,
			_ty: PhantomData,
		}
	}
}

// SAFETY: `PtrLength` is transparent over `usize`. On `wasm64`, the WAT
// adapter converts it to `f64` without losing precision.
unsafe impl<T> Slot for PtrLength<T> {
	const WAT_TYPE: &'static str = WAT_PTR_TYPE;
	const INTO_JS_WAT_CONV: Option<WatConv> = PTR_INTO_JS_WAT_CONV;
}

// SAFETY: The JavaScript conversion matches the WAT boundary representation.
unsafe impl<T> IntoJS for PtrLength<T> {
	const JS_CONV: Option<IntoJsConv> = JsPointerType::JS_CONV;

	type Abi = Self;

	fn into_abi(self) -> Self::Abi {
		self
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
			#[cfg(debug_assertions)]
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
			#[cfg(debug_assertions)]
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
			#[cfg(debug_assertions)]
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
			#[cfg(debug_assertions)]
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
			#[cfg(debug_assertions)]
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
			#[cfg(debug_assertions)]
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
