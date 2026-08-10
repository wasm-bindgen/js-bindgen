use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use crate::hazard::{
	EmptySlot, FromJS, FromJsConv, IntoJS, IntoJsConv, JsCast, ReturnAbi, ReturnMode, Sret, WasmAbi,
};
use crate::util::{JS_PTR_LEN_ARGS, PtrConst, PtrLength};
use crate::{JsString, JsValue};

js_bindgen::embed_js!(
	module = "js_sys",
	name = "vec.js_value.take",
	required_embeds = [("js_sys", "array.js_value.decode")],
	"(ptr, len) => {{",
	"    try {{",
	"        return this.#jsEmbed.js_sys['array.js_value.decode'](ptr, len)",
	"    }} finally {{",
	#[cfg(target_arch = "wasm32")]
	"        this.#jsExports['js_sys.externref.recycle_slice'](ptr, len)",
	#[cfg(target_arch = "wasm64")]
	"        this.#jsExports['js_sys.externref.recycle_slice'](BigInt(ptr), BigInt(len))",
	"    }}",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "vec.js_value.from_js",
	required_embeds = [("js_sys", "externref.table")],
	"(() => {{",
	"    const memory = this.#memory",
	"    let buffer = memory.buffer",
	"    let view = new DataView(buffer)",
	"    const store = (ptr, len, out) => {{",
	"        if (buffer !== memory.buffer) {{",
	"            buffer = memory.buffer",
	"            view = new DataView(buffer)",
	"        }}",
	#[cfg(target_arch = "wasm32")]
	"        view.setUint32(out, ptr, true)",
	#[cfg(target_arch = "wasm32")]
	"        view.setUint32(out + 4, len, true)",
	#[cfg(target_arch = "wasm64")]
	"        view.setBigUint64(out, ptr, true)",
	#[cfg(target_arch = "wasm64")]
	"        view.setBigUint64(out + 8, len, true)",
	"    }}",
	"    const fromJs = (value, out) => {{",
	"        if (!Array.isArray(value))",
	"            throw new TypeError('expected an Array')",
	"        const rawLength = value.length",
	"        const length = rawLength >>> 0",
	"        if (rawLength !== length)",
	"            throw new TypeError('invalid array length')",
	"",
	#[cfg(target_arch = "wasm32")]
	"        const ptr = this.#jsExports['js_sys.externref.reserve_slice'](length)",
	#[cfg(target_arch = "wasm64")]
	"        const ptr = this.#jsExports['js_sys.externref.reserve_slice'](BigInt(length))",
	"        let transferred = false",
	"        try {{",
	#[cfg(target_arch = "wasm32")]
	"            const address = ptr >>> 0",
	#[cfg(target_arch = "wasm64")]
	"            const address = Number(ptr)",
	"            const table = this.#jsEmbed.js_sys['externref.table']",
	"            for (let index = 0; index < length; index++) {{",
	"                if (buffer !== memory.buffer) {{",
	"                    buffer = memory.buffer",
	"                    view = new DataView(buffer)",
	"                }}",
	"                // Read the slot before an element getter can grow memory.",
	"                const slot = view.getUint32(address + index * 4, true)",
	"                const element = value[index]",
	"                table.set(slot, element)",
	"            }}",
	"",
	"            if (out === undefined) {{",
	"                transferred = true",
	#[cfg(target_arch = "wasm32")]
	"                return [ptr, length]",
	#[cfg(target_arch = "wasm64")]
	"                return [ptr, BigInt(length)]",
	"            }}",
	#[cfg(target_arch = "wasm32")]
	"            store(ptr, length, out)",
	#[cfg(target_arch = "wasm64")]
	"            store(ptr, BigInt(length), out)",
	"            transferred = true",
	"        }} finally {{",
	"            if (!transferred) {{",
	#[cfg(target_arch = "wasm32")]
	"                this.#jsExports['js_sys.externref.recycle_slice'](ptr, length)",
	#[cfg(target_arch = "wasm64")]
	"                this.#jsExports['js_sys.externref.recycle_slice'](ptr, BigInt(length))",
	"            }}",
	"        }}",
	"    }}",
	"    return {{",
	"        slots: fromJs,",
	"        sret: fromJs,",
	"    }}",
	"}})()",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "vec.string.from_js",
	required_embeds = [("js_sys", "vec.js_value.from_js")],
	"(() => {{",
	"    const validate = value => {{",
	"        if (!Array.isArray(value))",
	"            throw new TypeError('expected an Array')",
	"        const rawLength = value.length",
	"        const length = rawLength >>> 0",
	"        if (rawLength !== length)",
	"            throw new TypeError('invalid array length')",
	"        const strings = new Array(length)",
	"        for (let index = 0; index < length; index++) {{",
	"            const element = value[index]",
	"            if (typeof element !== 'string')",
	"                throw new TypeError('expected an Array of strings')",
	"            strings[index] = element",
	"        }}",
	"        return strings",
	"    }}",
	"    return {{",
	"        slots: value => {{",
	"            return this.#jsEmbed.js_sys['vec.js_value.from_js'].slots(validate(value))",
	"        }},",
	"        sret: (value, out) => {{",
	"            return this.#jsEmbed.js_sys['vec.js_value.from_js'].sret(validate(value), out)",
	"        }},",
	"    }}",
	"}})()",
);

#[doc(hidden)]
pub struct VecAbi<T> {
	ptr: PtrConst<T>,
	len: PtrLength<T>,
}

impl<T> VecAbi<T> {
	fn from_boxed_slice(value: Box<[T]>) -> Self {
		let len = value.len();
		let ptr = Box::into_raw(value).cast::<T>();

		Self {
			ptr: PtrConst::from_raw(ptr),
			len: PtrLength::from_len(len),
		}
	}

	unsafe fn into_boxed_slice(self) -> Box<[T]> {
		let ptr = self.ptr.as_ptr().cast_mut();
		let len = self.len.get();
		// SAFETY: The caller guarantees that the carrier owns an allocation for
		// exactly `len` initialized `T` values.
		unsafe { Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr, len)) }
	}
}

// SAFETY: `VecAbi` is represented by its element pointer and length, in that
// order.
unsafe impl<T> WasmAbi for VecAbi<T> {
	type Slot1 = PtrConst<T>;
	type Slot2 = PtrLength<T>;
	type Slot3 = EmptySlot;
	type Slot4 = EmptySlot;

	fn split(self) -> (Self::Slot1, Self::Slot2, Self::Slot3, Self::Slot4) {
		(self.ptr, self.len, EmptySlot::new(), EmptySlot::new())
	}

	fn join(ptr: Self::Slot1, len: Self::Slot2, _: Self::Slot3, _: Self::Slot4) -> Self {
		Self { ptr, len }
	}
}

// SAFETY: The two-word aggregate uses a hidden return pointer under Rust's
// `extern "C"` calling convention.
unsafe impl<T> ReturnAbi for VecAbi<T> {
	const MODE: ReturnMode = ReturnMode::Indirect;
}

const JS_VALUE_VEC_TO_JS: IntoJsConv = IntoJsConv::new(crate::const_concat!(
	"this.#jsEmbed.js_sys['vec.js_value.take'](",
	JS_PTR_LEN_ARGS,
	")"
))
.with_embed(("js_sys", "vec.js_value.take"));

/// Element-level policy for moving an owned vector from Rust to JavaScript.
///
/// # Safety
///
/// `Abi`, `vector_into_abi`, and `JS_CONV` must describe one ownership-
/// transferring conversion for a boxed slice of `Self`.
#[doc(hidden)]
pub unsafe trait VectorIntoJS: Sized {
	const JS_CONV: IntoJsConv;

	type Abi: WasmAbi;

	fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi;
}

/// Element-level policy for moving an owned vector from JavaScript to Rust.
///
/// # Safety
///
/// `Abi`, `vector_from_abi`, and `JS_CONV` must describe one ownership-
/// transferring conversion for a boxed slice of `Self`.
#[doc(hidden)]
pub unsafe trait VectorFromJS: Sized {
	const JS_CONV: FromJsConv;

	type Abi: WasmAbi;

	unsafe fn vector_from_abi(raw: Self::Abi) -> Box<[Self]>;
}

// SAFETY: Delegated to the element's vector conversion policy.
unsafe impl<T: VectorIntoJS> IntoJS for Vec<T> {
	const JS_CONV: Option<IntoJsConv> = Some(T::JS_CONV);

	type Abi = T::Abi;

	fn into_abi(self) -> Self::Abi {
		T::vector_into_abi(self.into_boxed_slice())
	}
}

// SAFETY: Delegated to the element's vector conversion policy.
unsafe impl<T: VectorFromJS> FromJS for Vec<T> {
	const JS_CONV: Option<FromJsConv> = Some(T::JS_CONV);

	type Abi = T::Abi;

	fn from_abi(raw: Self::Abi) -> Self {
		// SAFETY: `FromJS` guarantees that `raw` was produced by `T::JS_CONV`.
		unsafe { T::vector_from_abi(raw) }.into_vec()
	}
}

// SAFETY: Every element is moved into its owned `JsValue` representation, and
// the JavaScript helper consumes the resulting table-index allocation.
unsafe impl<T> VectorIntoJS for T
where
	T: JsCast + Into<JsValue>,
{
	const JS_CONV: IntoJsConv = JS_VALUE_VEC_TO_JS;

	type Abi = VecAbi<JsValue>;

	fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
		let values = vector
			.into_vec()
			.into_iter()
			.map(Into::into)
			.collect::<Vec<_>>()
			.into_boxed_slice();
		VecAbi::from_boxed_slice(values)
	}
}

// SAFETY: The helper creates an owned slice of valid `JsValue` table indices;
// `JsCast` transfers each value into the requested transparent wrapper.
unsafe impl<T: JsCast> VectorFromJS for T {
	const JS_CONV: FromJsConv = FromJsConv::slot1("$prepared[0]")
		.slot2("$prepared[1]")
		.prepare("this.#jsEmbed.js_sys['vec.js_value.from_js'].slots($value)")
		.sret(Sret::Value(
			"this.#jsEmbed.js_sys['vec.js_value.from_js'].sret",
		))
		.with_embed(("js_sys", "vec.js_value.from_js"));

	type Abi = VecAbi<JsValue>;

	unsafe fn vector_from_abi(raw: Self::Abi) -> Box<[Self]> {
		// SAFETY: `vec.js_value.from_js` initialized every table index and
		// transferred the exact-size allocation to this carrier.
		unsafe { raw.into_boxed_slice() }
			.into_vec()
			.into_iter()
			.map(T::unchecked_from)
			.collect()
	}
}

// SAFETY: Strings are converted to owned JavaScript string values before the
// table-index allocation is transferred to JavaScript.
unsafe impl VectorIntoJS for String {
	const JS_CONV: IntoJsConv = JS_VALUE_VEC_TO_JS;

	type Abi = VecAbi<JsValue>;

	fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
		let values = vector
			.into_vec()
			.into_iter()
			.map(|value| JsValue::from(JsString::from(value)))
			.collect::<Vec<_>>()
			.into_boxed_slice();
		VecAbi::from_boxed_slice(values)
	}
}

// SAFETY: The JavaScript helper validates every array element as a string
// before transferring its owned table index to Rust.
unsafe impl VectorFromJS for String {
	const JS_CONV: FromJsConv = FromJsConv::slot1("$prepared[0]")
		.slot2("$prepared[1]")
		.prepare("this.#jsEmbed.js_sys['vec.string.from_js'].slots($value)")
		.sret(Sret::Value(
			"this.#jsEmbed.js_sys['vec.string.from_js'].sret",
		))
		.with_embed(("js_sys", "vec.string.from_js"));

	type Abi = VecAbi<JsValue>;

	unsafe fn vector_from_abi(raw: Self::Abi) -> Box<[Self]> {
		// SAFETY: `vec.string.from_js` validated each value and then delegated to
		// `vec.js_value.from_js`, which transferred the exact-size allocation.
		unsafe { raw.into_boxed_slice() }
			.into_vec()
			.into_iter()
			.map(|value| Self::from(JsString::unchecked_from(value)))
			.collect()
	}
}

#[rustfmt::skip]
macro_rules! typed_vector {
	(
		$ty:ty,
		name = $name:literal,
		constructor = $constructor:literal,
		view = $view:literal $(,)?
	) => {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = concat!("vec.", $name, ".take"),
			required_embeds = [("js_sys", concat!("view.get", $view))],
			"(ptr, len) => {{",
			"    try {{",
			"        return new {constructor}(",
			"            this.#jsEmbed.js_sys['view.get{view}'](ptr, len),",
			"        )",
			"    }} finally {{",
			#[cfg(target_arch = "wasm32")]
			"        this.#jsExports['js_sys.memory.free'](ptr, len * {size}, {align})",
			#[cfg(target_arch = "wasm64")]
			"        this.#jsExports['js_sys.memory.free'](",
			#[cfg(target_arch = "wasm64")]
			"            BigInt(ptr), BigInt(len) * {size}n, {align}n,",
			#[cfg(target_arch = "wasm64")]
			"        )",
			"    }}",
			"}}",
			constructor = interpolate $constructor,
			view = interpolate $view,
			size = const core::mem::size_of::<$ty>(),
			align = const core::mem::align_of::<$ty>(),
		);

		js_bindgen::embed_js!(
			module = "js_sys",
			name = concat!("vec.", $name, ".from_js"),
			required_embeds = [("js_sys", concat!("view.set", $view))],
			"(() => {{",
			"    const memory = this.#memory",
			"    let buffer = memory.buffer",
			"    let view = new DataView(buffer)",
			"    const store = (ptr, len, out) => {{",
			"        if (buffer !== memory.buffer) {{",
			"            buffer = memory.buffer",
			"            view = new DataView(buffer)",
			"        }}",
			#[cfg(target_arch = "wasm32")]
			"        view.setUint32(out, ptr, true)",
			#[cfg(target_arch = "wasm32")]
			"        view.setUint32(out + 4, len, true)",
			#[cfg(target_arch = "wasm64")]
			"        view.setBigUint64(out, ptr, true)",
			#[cfg(target_arch = "wasm64")]
			"        view.setBigUint64(out + 8, len, true)",
			"    }}",
			"    const fromJs = (value, out) => {{",
			"        if (!(value instanceof {constructor}))",
			"            throw new TypeError('expected a {constructor}')",
			"        const length = value.length",
			#[cfg(target_arch = "wasm32")]
			"        const ptr = this.#jsExports['js_sys.memory.alloc'](length * {size}, {align})",
			#[cfg(target_arch = "wasm64")]
			"        const ptr = this.#jsExports['js_sys.memory.alloc'](",
			#[cfg(target_arch = "wasm64")]
			"            BigInt(length) * {size}n, {align}n,",
			#[cfg(target_arch = "wasm64")]
			"        )",
			"        let transferred = false",
			"        try {{",
			#[cfg(target_arch = "wasm32")]
			"            const address = ptr >>> 0",
			#[cfg(target_arch = "wasm64")]
			"            const address = Number(ptr)",
			"            this.#jsEmbed.js_sys['view.set{view}'](address, value, length)",
			"            if (out === undefined) {{",
			"                transferred = true",
			#[cfg(target_arch = "wasm32")]
			"                return [ptr, length]",
			#[cfg(target_arch = "wasm64")]
			"                return [ptr, BigInt(length)]",
			"            }}",
			#[cfg(target_arch = "wasm32")]
			"            store(ptr, length, out)",
			#[cfg(target_arch = "wasm64")]
			"            store(ptr, BigInt(length), out)",
			"            transferred = true",
			"        }} finally {{",
			"            if (!transferred) {{",
			#[cfg(target_arch = "wasm32")]
			"                this.#jsExports['js_sys.memory.free'](",
			#[cfg(target_arch = "wasm32")]
			"                    ptr, length * {size}, {align},",
			#[cfg(target_arch = "wasm32")]
			"                )",
			#[cfg(target_arch = "wasm64")]
			"                this.#jsExports['js_sys.memory.free'](",
			#[cfg(target_arch = "wasm64")]
			"                    ptr, BigInt(length) * {size}n, {align}n,",
			#[cfg(target_arch = "wasm64")]
			"                )",
			"            }}",
			"        }}",
			"    }}",
			"    return {{",
			"        slots: fromJs,",
			"        sret: fromJs,",
			"    }}",
			"}})()",
			constructor = interpolate $constructor,
			view = interpolate $view,
			size = const core::mem::size_of::<$ty>(),
			align = const core::mem::align_of::<$ty>(),
		);

		// SAFETY: The helper copies the allocation into an independent typed
		// array and releases the Rust buffer afterwards.
		unsafe impl VectorIntoJS for $ty {
			const JS_CONV: IntoJsConv = IntoJsConv::new(crate::const_concat!(
				"this.#jsEmbed.js_sys['vec.",
				$name,
				".take'](",
				JS_PTR_LEN_ARGS,
				")"
			))
			.with_embed(("js_sys", concat!("vec.", $name, ".take")));

			type Abi = VecAbi<Self>;

			fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
				VecAbi::from_boxed_slice(vector)
			}
		}

		// SAFETY: The helper allocates an exact-size buffer, initializes every
		// element from the matching typed array, and transfers it to Rust.
		unsafe impl VectorFromJS for $ty {
			const JS_CONV: FromJsConv = FromJsConv::slot1("$prepared[0]")
				.slot2("$prepared[1]")
				.prepare(concat!(
					"this.#jsEmbed.js_sys['vec.",
					$name,
					".from_js'].slots($value)"
				))
				.sret(Sret::Value(concat!(
					"this.#jsEmbed.js_sys['vec.",
					$name,
					".from_js'].sret"
				)))
				.with_embed(("js_sys", concat!("vec.", $name, ".from_js")));

			type Abi = VecAbi<Self>;

			unsafe fn vector_from_abi(raw: Self::Abi) -> Box<[Self]> {
				// SAFETY: The matching `from_js` helper initialized every element and
				// transferred the exact-size allocation to this carrier.
				unsafe { raw.into_boxed_slice() }
			}
		}
	};
}

typed_vector! {
	i8,
	name = "i8",
	constructor = "Int8Array",
	view = "Int8",
}

typed_vector! {
	u8,
	name = "u8",
	constructor = "Uint8Array",
	view = "Uint8",
}

typed_vector! {
	i16,
	name = "i16",
	constructor = "Int16Array",
	view = "Int16",
}

typed_vector! {
	u16,
	name = "u16",
	constructor = "Uint16Array",
	view = "Uint16",
}

typed_vector! {
	i32,
	name = "i32",
	constructor = "Int32Array",
	view = "Int32",
}

typed_vector! {
	u32,
	name = "u32",
	constructor = "Uint32Array",
	view = "Uint32",
}

typed_vector! {
	i64,
	name = "i64",
	constructor = "BigInt64Array",
	view = "BigInt64",
}

typed_vector! {
	u64,
	name = "u64",
	constructor = "BigUint64Array",
	view = "BigUint64",
}

typed_vector! {
	f32,
	name = "f32",
	constructor = "Float32Array",
	view = "Float32",
}

typed_vector! {
	f64,
	name = "f64",
	constructor = "Float64Array",
	view = "Float64",
}

#[cfg(target_arch = "wasm32")]
typed_vector! {
	isize,
	name = "isize",
	constructor = "Int32Array",
	view = "Int32",
}

#[cfg(target_arch = "wasm64")]
typed_vector! {
	isize,
	name = "isize",
	constructor = "BigInt64Array",
	view = "BigInt64",
}

#[cfg(target_arch = "wasm32")]
typed_vector! {
	usize,
	name = "usize",
	constructor = "Uint32Array",
	view = "Uint32",
}

#[cfg(target_arch = "wasm64")]
typed_vector! {
	usize,
	name = "usize",
	constructor = "BigUint64Array",
	view = "BigUint64",
}
