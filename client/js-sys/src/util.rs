use core::marker::PhantomData;
use core::mem;
use core::mem::MaybeUninit;

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
pub struct ExternValue<T>(T);

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
	pub(crate) const ASM_IMPORT_TYPE: &str = ExternValue::<()>::ASM_IMPORT_TYPE;
	pub(crate) const ASM_TYPE: &str = ExternValue::<()>::ASM_TYPE;
	pub(crate) const ASM_CONV: Option<&str> = ExternValue::<()>::ASM_CONV;

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

js_bindgen::embed_js!(
	module = "js_sys",
	name = "isLittleEndian",
	"(() => {{",
	"	const buffer = new ArrayBuffer(2)",
	"	new DataView(buffer).setInt16(0, 256, true)",
	"	return new Int16Array(buffer)[0] === 256;",
	"}})()",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.DataView",
	"new DataView(this.#memory.buffer)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.Uint32",
	"new Uint32Array(this.#memory.buffer)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.getUint32",
	required_embeds = [
		("js_sys", "isLittleEndian"),
		("js_sys", "view.Uint32"),
		("js_sys", "view.DataView")
	],
	"(ptr, count) => {{",
	#[cfg(debug_assertions)]
	"	if (ptr & 3) throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
	"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
	"		const base = ptr / 4",
	"		const view = this.#jsEmbed.js_sys['view.Uint32'].subarray(base, base + count)",
	"		return Array.from(view)",
	"	}} else {{",
	"		const out = new Array(count)",
	"		for (let index = 0; index < count; index++) {{",
	"			out[index] = this.#jsEmbed.js_sys['view.DataView'].getUint32(ptr + index * 4, true)",
	"		}}",
	"		return out",
	"	}}",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.Int32",
	"new Int32Array(this.#memory.buffer)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.getInt32",
	required_embeds = [
		("js_sys", "isLittleEndian"),
		("js_sys", "view.Int32"),
		("js_sys", "view.DataView")
	],
	"(ptr, count) => {{",
	#[cfg(debug_assertions)]
	"	if (ptr & 3) throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
	"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
	"		const base = ptr / 4",
	"		const view = this.#jsEmbed.js_sys['view.Int32'].subarray(base, base + count)",
	"		return Array.from(view)",
	"	}} else {{",
	"		const out = new Array(count)",
	"		for (let index = 0; index < count; index++) {{",
	"			out[index] = this.#jsEmbed.js_sys['view.DataView'].getInt32(ptr + index * 4, true)",
	"		}}",
	"		return out",
	"	}}",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.setInt32",
	required_embeds = [
		("js_sys", "isLittleEndian"),
		("js_sys", "view.Int32"),
		("js_sys", "view.DataView")
	],
	"(ptr, array) => {{",
	#[cfg(debug_assertions)]
	"	if (ptr & 3) throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
	"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
	"		this.#jsEmbed.js_sys['view.Int32'].set(array, ptr / 4)",
	"	}} else {{",
	"		for (let index = 0; index < array.length; index++) {{",
	"			this.#jsEmbed.js_sys['view.DataView'].setInt32(ptr + index * 4, array[index], true)",
	"		}}",
	"	}}",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.Float64",
	"new Float64Array(this.#memory.buffer)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.getFloat64",
	required_embeds = [
		("js_sys", "isLittleEndian"),
		("js_sys", "view.Float64"),
		("js_sys", "view.DataView")
	],
	"(ptr, count) => {{",
	#[cfg(debug_assertions)]
	"	if (ptr & 7) throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
	"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
	"		const base = ptr / 8",
	"		const view = this.#jsEmbed.js_sys['view.Float64'].subarray(base, base + count)",
	"		return Array.from(view)",
	"	}} else {{",
	"		const out = new Array(count)",
	"		for (let index = 0; index < count; index++) {{",
	"			out[index] = this.#jsEmbed.js_sys['view.DataView'].getFloat64(ptr + index * 8, true)",
	"		}}",
	"		return out",
	"	}}",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.BigUint64",
	"new BigUint64Array(this.#memory.buffer)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.getBigUint64",
	required_embeds = [
		("js_sys", "isLittleEndian"),
		("js_sys", "view.BigUint64"),
		("js_sys", "view.DataView")
	],
	"(ptr, count) => {{",
	#[cfg(debug_assertions)]
	"	if (ptr & 7) throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
	"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
	"		const base = ptr / 8",
	"		const view = this.#jsEmbed.js_sys['view.BigUint64'].subarray(base, base + count)",
	"		return Array.from(view)",
	"	}} else {{",
	"		const out = new Array(count)",
	"		for (let index = 0; index < count; index++) {{",
	"			out[index] = this.#jsEmbed.js_sys['view.DataView'].getBigUint64(ptr + index * 8, true)",
	"		}}",
	"		return out",
	"	}}",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.BigInt64",
	"new BigInt64Array(this.#memory.buffer)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "view.getBigInt64",
	required_embeds = [
		("js_sys", "isLittleEndian"),
		("js_sys", "view.BigInt64"),
		("js_sys", "view.DataView")
	],
	"(ptr, count) => {{",
	#[cfg(debug_assertions)]
	"	if (ptr & 7) throw new WebAssembly.RuntimeError(`non-aligned pointer: ${{ptr}}`)",
	"	if (this.#jsEmbed.js_sys.isLittleEndian) {{",
	"		const base = ptr / 8",
	"		const view = this.#jsEmbed.js_sys['view.BigInt64'].subarray(base, base + count)",
	"		return Array.from(view)",
	"	}} else {{",
	"		const out = new Array(count)",
	"		for (let index = 0; index < count; index++) {{",
	"			out[index] = this.#jsEmbed.js_sys['view.DataView'].getBigInt64(ptr + index * 8, true)",
	"		}}",
	"		return out",
	"	}}",
	"}}",
);
