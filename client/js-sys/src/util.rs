use core::marker::PhantomData;
use core::mem;
use core::mem::MaybeUninit;

use crate::hazard::{Input, InputAsmConv, InputJsConv};

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
	pub(crate) const ASM_TYPE: &str = ASM_PTR_TYPE;
	#[cfg(target_arch = "wasm32")]
	pub(crate) const ASM_CONV: Option<InputAsmConv> = None;
	#[cfg(target_arch = "wasm64")]
	pub(crate) const ASM_CONV: Option<InputAsmConv> = Some(InputAsmConv {
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
	ptr: <*const T as Input>::Type,
	len: PtrLength<T>,
}

#[expect(dead_code, reason = "custom sections are considered dead-code")]
impl<T> ExternSlice<T> {
	pub(crate) const ASM_TYPE: &str = ExternValue::<()>::ASM_TYPE;
	pub(crate) const ASM_CONV: Option<InputAsmConv> = ExternValue::<()>::ASM_CONV;

	#[cfg(target_arch = "wasm32")]
	const VIEW_FN: &str = "view.getUint32";
	#[cfg(target_arch = "wasm64")]
	const VIEW_FN: &str = "view.getFloat64";

	pub(crate) fn new(value: &[T]) -> Self {
		Self {
			ptr: <*const T>::into_raw(value.as_ptr()),
			len: PtrLength::new(value),
		}
	}
}

// Verify that we can access `ExternSlice` via a `TypedArray` with two elements.
const _: () = {
	debug_assert!(
		mem::align_of::<ExternSlice<()>>() == mem::size_of::<<*const () as Input>::Type>()
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
pub(crate) const ASM_PTR_TYPE: &str = "i32";
#[cfg(target_arch = "wasm64")]
pub(crate) const ASM_PTR_TYPE: &str = "i64";

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
		#[expect(clippy::cast_precision_loss, reason = "checked")]
		let len = len as f64;

		Self {
			len,
			_ty: PhantomData,
		}
	}
}

// SAFETY: Delegated to already implemented types.
unsafe impl<T> Input for PtrLength<T> {
	const ASM_TYPE: &str = Self::Type::ASM_TYPE;
	const JS_CONV: Option<InputJsConv> = Self::Type::JS_CONV;

	#[cfg(target_arch = "wasm32")]
	type Type = usize;
	#[cfg(target_arch = "wasm64")]
	type Type = f64;

	fn into_raw(self) -> Self::Type {
		self.len
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "isLittleEndian",
	"(() => {{",
	"	const buffer = new ArrayBuffer(2)",
	"	new DataView(buffer).setInt16(0, 256, true)",
	"	return new Int16Array(buffer)[0] === 256;",
	"}})()",
);

#[cfg(js_sys_target_feature = "unstable-rab")]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.DataView",
	"new DataView(this.#memory.toResizableBuffer())",
);

#[cfg(js_sys_target_feature = "unstable-rab")]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.Uint32",
	"new Uint32Array(this.#memory.toResizableBuffer())",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.getUint32",
	required_embeds = [
		("js_sys", "isLittleEndian"),
		#[cfg(js_sys_target_feature = "unstable-rab")]
		("js_sys", "view.Uint32"),
		#[cfg(js_sys_target_feature = "unstable-rab")]
		("js_sys", "view.DataView")
	],
	"(ptr, count) => {{",
	#[cfg(debug_assertions)]
	"	if (ptr % 4 !== 0) throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
	"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	"		const base = ptr / 4",
	"		const view = {uint32}",
	"		return Array.from(view)",
	"	}} else {{",
	"		const out = new Array(count)",
	"		const view = {data}",
	"		for (let index = 0; index < count; index++) {{",
	"			out[index] = view.getUint32(ptr + index * 4, true)",
	"		}}",
	"		return out",
	"	}}",
	"}}",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	uint32 = interpolate "this.#jsEmbed.js_sys['view.Uint32'].subarray(base, base + count)",
	#[cfg(not(js_sys_target_feature = "unstable-rab"))]
	uint32 = interpolate "new Uint32Array(this.#memory.buffer, ptr, count)",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	data = interpolate "this.#jsEmbed.js_sys['view.DataView']",
	#[cfg(not(js_sys_target_feature = "unstable-rab"))]
	data = interpolate "new DataView(this.#memory.buffer)",
);

#[cfg(js_sys_target_feature = "unstable-rab")]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.Int32",
	"new Int32Array(this.#memory.toResizableBuffer())",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.getInt32",
	required_embeds = [
		("js_sys", "isLittleEndian"),
		#[cfg(js_sys_target_feature = "unstable-rab")]
		("js_sys", "view.Int32"),
		#[cfg(js_sys_target_feature = "unstable-rab")]
		("js_sys", "view.DataView")
	],
	"(ptr, count) => {{",
	#[cfg(debug_assertions)]
	"	if (ptr % 4 !== 0) throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
	"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	"		const base = ptr / 4",
	"		const view = {int32}",
	"		return Array.from(view)",
	"	}} else {{",
	"		const out = new Array(count)",
	"		const view = {data}",
	"		for (let index = 0; index < count; index++) {{",
	"			out[index] = view.getInt32(ptr + index * 4, true)",
	"		}}",
	"		return out",
	"	}}",
	"}}",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	int32 = interpolate "this.#jsEmbed.js_sys['view.Int32'].subarray(base, base + count)",
	#[cfg(not(js_sys_target_feature = "unstable-rab"))]
	int32 = interpolate "new Int32Array(this.#memory.buffer, ptr, count)",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	data = interpolate "this.#jsEmbed.js_sys['view.DataView']",
	#[cfg(not(js_sys_target_feature = "unstable-rab"))]
	data = interpolate "new DataView(this.#memory.buffer)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.setInt32",
	required_embeds = [
		("js_sys", "isLittleEndian"),
		#[cfg(js_sys_target_feature = "unstable-rab")]
		("js_sys", "view.Int32"),
		#[cfg(js_sys_target_feature = "unstable-rab")]
		("js_sys", "view.DataView")
	],
	"(ptr, array) => {{",
	#[cfg(debug_assertions)]
	"	if (ptr % 4 !== 0) throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
	"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
	"		{int32}.set(array, ptr / 4)",
	"	}} else {{",
	"		const view = {data}",
	"		for (let index = 0; index < array.length; index++) {{",
	"			view.setInt32(ptr + index * 4, array[index], true)",
	"		}}",
	"	}}",
	"}}",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	int32 = interpolate "this.#jsEmbed.js_sys['view.Int32']",
	#[cfg(not(js_sys_target_feature = "unstable-rab"))]
	int32 = interpolate "new Int32Array(this.#memory.buffer)",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	data = interpolate "this.#jsEmbed.js_sys['view.DataView']",
	#[cfg(not(js_sys_target_feature = "unstable-rab"))]
	data = interpolate "new DataView(this.#memory.buffer)",
);

#[cfg(js_sys_target_feature = "unstable-rab")]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.Float64",
	"new Float64Array(this.#memory.toResizableBuffer())",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.getFloat64",
	required_embeds = [
		("js_sys", "isLittleEndian"),
		#[cfg(js_sys_target_feature = "unstable-rab")]
		("js_sys", "view.Float64"),
		#[cfg(js_sys_target_feature = "unstable-rab")]
		("js_sys", "view.DataView")
	],
	"(ptr, count) => {{",
	#[cfg(debug_assertions)]
	"	if (ptr % 8 !== 0) throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
	"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	"		const base = ptr / 8",
	"		const view = {float64}",
	"		return Array.from(view)",
	"	}} else {{",
	"		const out = new Array(count)",
	"		const view = {data}",
	"		for (let index = 0; index < count; index++) {{",
	"			out[index] = view.getFloat64(ptr + index * 8, true)",
	"		}}",
	"		return out",
	"	}}",
	"}}",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	float64 = interpolate "this.#jsEmbed.js_sys['view.Float64'].subarray(base, base + count)",
	#[cfg(not(js_sys_target_feature = "unstable-rab"))]
	float64 = interpolate "new Float64Array(this.#memory.buffer, ptr, count)",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	data = interpolate "this.#jsEmbed.js_sys['view.DataView']",
	#[cfg(not(js_sys_target_feature = "unstable-rab"))]
	data = interpolate "new DataView(this.#memory.buffer)",
);

#[cfg(js_sys_target_feature = "unstable-rab")]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.BigUint64",
	"new BigUint64Array(this.#memory.toResizableBuffer())",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.getBigUint64",
	required_embeds = [
		("js_sys", "isLittleEndian"),
		#[cfg(js_sys_target_feature = "unstable-rab")]
		("js_sys", "view.BigUint64"),
		#[cfg(js_sys_target_feature = "unstable-rab")]
		("js_sys", "view.DataView")
	],
	"(ptr, count) => {{",
	#[cfg(debug_assertions)]
	"	if (ptr % 8 !== 0) throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
	"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	"		const base = ptr / 8",
	"		const view = {biguint64}",
	"		return Array.from(view)",
	"	}} else {{",
	"		const out = new Array(count)",
	"		const view = {data}",
	"		for (let index = 0; index < count; index++) {{",
	"			out[index] = view.getBigUint64(ptr + index * 8, true)",
	"		}}",
	"		return out",
	"	}}",
	"}}",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	biguint64 = interpolate "this.#jsEmbed.js_sys['view.BigUint64'].subarray(base, base + count)",
	#[cfg(not(js_sys_target_feature = "unstable-rab"))]
	biguint64 = interpolate "new BigUint64Array(this.#memory.buffer, ptr, count)",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	data = interpolate "this.#jsEmbed.js_sys['view.DataView']",
	#[cfg(not(js_sys_target_feature = "unstable-rab"))]
	data = interpolate "new DataView(this.#memory.buffer)",
);

#[cfg(js_sys_target_feature = "unstable-rab")]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.BigInt64",
	"new BigInt64Array(this.#memory.toResizableBuffer())",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.getBigInt64",
	required_embeds = [
		("js_sys", "isLittleEndian"),
		#[cfg(js_sys_target_feature = "unstable-rab")]
		("js_sys", "view.BigInt64"),
		#[cfg(js_sys_target_feature = "unstable-rab")]
		("js_sys", "view.DataView")
	],
	"(ptr, count) => {{",
	#[cfg(debug_assertions)]
	"	if (ptr % 8 !== 0) throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
	"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	"		const base = ptr / 8",
	"		const view = {bigint64}",
	"		return Array.from(view)",
	"	}} else {{",
	"		const out = new Array(count)",
	"		const view = {data}",
	"		for (let index = 0; index < count; index++) {{",
	"			out[index] = view.getBigInt64(ptr + index * 8, true)",
	"		}}",
	"		return out",
	"	}}",
	"}}",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	bigint64 = interpolate "this.#jsEmbed.js_sys['view.BigInt64'].subarray(base, base + count)",
	#[cfg(not(js_sys_target_feature = "unstable-rab"))]
	bigint64 = interpolate "new BigInt64Array(this.#memory.buffer, ptr, count)",
	#[cfg(js_sys_target_feature = "unstable-rab")]
	data = interpolate "this.#jsEmbed.js_sys['view.DataView']",
	#[cfg(not(js_sys_target_feature = "unstable-rab"))]
	data = interpolate "new DataView(this.#memory.buffer)",
);
