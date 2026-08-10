use alloc::boxed::Box;
use alloc::string::String;

use crate::hazard::{
	EmptySlot, FromJS, FromJsConv, IntoJS, IntoJsConv, OptionFromAbi, OptionIntoAbi, ReturnAbi,
	ReturnMode, Sret, WasmAbi,
};
use crate::util::{ExternSlice, JS_OPTION_PTR_LEN_ARGS, JS_PTR_LEN_ARGS, PtrConst, PtrLength};
use crate::{JsString, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "string.decode")]
	// SAFETY: The pointer and length must describe a valid UTF-8 byte slice.
	#[expect(
		clippy::allow_attributes,
		reason = "the macro emits an unsafe ABI call"
	)]
	#[allow(
		clippy::undocumented_unsafe_blocks,
		reason = "the safety requirement is documented on this declaration"
	)]
	unsafe fn string_decode(array: PtrConst<u8>, len: PtrLength<u8>) -> JsString;
}

pub(in crate::interop) fn js_string_from_str(value: &str) -> JsString {
	// SAFETY: A Rust string is valid UTF-8, and its pointer and length describe
	// the complete byte slice for the duration of the call.
	unsafe {
		string_decode(
			PtrConst::new(value.as_bytes()),
			PtrLength::new(value.as_bytes()),
		)
	}
}

#[cfg(any(not(target_feature = "atomics"), js_sys_target_feature = "sab"))]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "string.decode",
	"(() => {{",
	"    const decoder = new TextDecoder('utf-8', {{",
	"        fatal: true,",
	"        ignoreBOM: true,",
	"    }})",
	"    decoder.decode()",
	"    return (ptr, len) => {{",
	"        if (len === 0) return ''",
	"        const view = new Uint8Array(this.#memory.buffer, ptr, len)",
	"        return decoder.decode(view)",
	"    }}",
	"}})()",
);

#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "string.shared.decode",
	"(() => {{",
	"    if (this.#memory.buffer instanceof ArrayBuffer) return true",
	"    try {{",
	"        new TextDecoder().decode(new Uint8Array(this.#memory.buffer, 0, 0))",
	"        return true",
	"    }} catch {{",
	"        return false",
	"    }}",
	"}})()",
);

#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "string.decode",
	required_embeds = [("js_sys", "string.shared.decode")],
	"(() => {{",
	"    const decoder = new TextDecoder('utf-8', {{",
	"        fatal: true,",
	"        ignoreBOM: true,",
	"    }})",
	"    decoder.decode()",
	"    return (ptr, len) => {{",
	"        if (len === 0) return ''",
	"        const view = new Uint8Array(this.#memory.buffer, ptr, len)",
	"        return decoder.decode(",
	"            this.#jsEmbed.js_sys['string.shared.decode'] ? view : view.slice()",
	"        )",
	"    }}",
	"}})()",
);

#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "string.shared.encode",
	"(() => {{",
	"    if (this.#memory.buffer instanceof ArrayBuffer) return true",
	"    try {{",
	"        const view = new Uint8Array(this.#memory.buffer, 0, 0)",
	"        new TextEncoder().encodeInto('', view)",
	"        return true",
	"    }} catch {{",
	"        return false",
	"    }}",
	"}})()",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "string.take",
	required_embeds = [("js_sys", "string.decode")],
	"(ptr, len) => {{",
	"    try {{",
	"        return this.#jsEmbed.js_sys['string.decode'](ptr, len)",
	"    }} finally {{",
	"        if (len !== 0) {{",
	#[cfg(target_arch = "wasm32")]
	"            this.#jsExports['js_sys.memory.free'](ptr, len, 1)",
	#[cfg(target_arch = "wasm64")]
	"            this.#jsExports['js_sys.memory.free'](BigInt(ptr), BigInt(len), 1n)",
	"        }}",
	"    }}",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "string.from_js",
	required_embeds = [
		#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
		("js_sys", "string.shared.encode")
	],
	"(() => {{",
	"    const encoder = new TextEncoder()",
	"    const memory = this.#memory",
	"    let bytes = new Uint8Array(memory.buffer)",
	"    let buffer = memory.buffer",
	"    let view = new DataView(buffer)",
	"    const ascii = (value, ptr, capacity) => {{",
	#[cfg(target_arch = "wasm32")]
	"        const start = ptr >>> 0",
	#[cfg(target_arch = "wasm64")]
	"        const start = Number(ptr)",
	#[cfg(not(target_feature = "atomics"))]
	"        if (bytes.byteLength === 0) bytes = new Uint8Array(memory.buffer)",
	#[cfg(target_feature = "atomics")]
	"        if (bytes.buffer !== memory.buffer || bytes.byteLength !== memory.buffer.byteLength)",
	#[cfg(target_feature = "atomics")]
	"            bytes = new Uint8Array(memory.buffer)",
	"        let written = 0",
	"        for (; written < capacity; written++) {{",
	"            const code = value.charCodeAt(written)",
	"            if (code > 0x7f) break",
	"            bytes[start + written] = code",
	"        }}",
	"        return written",
	"    }}",
	"    const store = (ptr, written, out) => {{",
	"        if (buffer !== memory.buffer) {{",
	"            buffer = memory.buffer",
	"            view = new DataView(buffer)",
	"        }}",
	#[cfg(target_arch = "wasm32")]
	"        view.setUint32(out, ptr, true)",
	#[cfg(target_arch = "wasm64")]
	"        view.setBigUint64(out, ptr, true)",
	#[cfg(target_arch = "wasm32")]
	"        view.setUint32(out + 4, written, true)",
	#[cfg(target_arch = "wasm64")]
	"        view.setBigUint64(out + 8, BigInt(written), true)",
	"    }}",
	"    const unicode = (value, ptr, capacity, written, out) => {{",
	"        if (written !== capacity) {{",
	"            if (written !== 0) value = value.slice(written)",
	"            const nextCapacity = written + value.length * 3",
	"            if (!Number.isSafeInteger(nextCapacity))",
	"                throw new RangeError('string is too large')",
	"",
	#[cfg(target_arch = "wasm32")]
	"            ptr = this.#jsExports['js_sys.memory.realloc'](ptr, capacity, nextCapacity, 1)",
	#[cfg(target_arch = "wasm64")]
	"            ptr = this.#jsExports['js_sys.memory.realloc'](",
	#[cfg(target_arch = "wasm64")]
	"                ptr, BigInt(capacity), BigInt(nextCapacity), 1n,",
	#[cfg(target_arch = "wasm64")]
	"            )",
	"            capacity = nextCapacity",
	#[cfg(target_arch = "wasm32")]
	"            const start = ptr >>> 0",
	#[cfg(target_arch = "wasm64")]
	"            const start = Number(ptr)",
	#[cfg(not(target_feature = "atomics"))]
	"            if (bytes.byteLength === 0) bytes = new Uint8Array(memory.buffer)",
	#[cfg(target_feature = "atomics")]
	"            if (bytes.buffer !== memory.buffer || bytes.byteLength !== \
	 memory.buffer.byteLength)",
	#[cfg(target_feature = "atomics")]
	"                bytes = new Uint8Array(memory.buffer)",
	"            const target = bytes.subarray(start + written, start + capacity)",
	"",
	#[cfg(any(not(target_feature = "atomics"), js_sys_target_feature = "sab"))]
	"            const encoded = encoder.encodeInto(value, target)",
	#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
	"            let encoded",
	#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
	"            if (this.#jsEmbed.js_sys['string.shared.encode']) {{",
	#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
	"                encoded = encoder.encodeInto(value, target)",
	#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
	"            }} else {{",
	#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
	"                const bytes = encoder.encode(value)",
	#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
	"                target.set(bytes)",
	#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
	"                encoded = {{ read: value.length, written: bytes.length }}",
	#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
	"            }}",
	"            if (encoded.read !== value.length)",
	"                throw new RangeError('failed to encode the complete string')",
	"            written += encoded.written",
	"        }}",
	"",
	"        if (written !== capacity) {{",
	#[cfg(target_arch = "wasm32")]
	"            ptr = this.#jsExports['js_sys.memory.realloc'](ptr, capacity, written, 1)",
	#[cfg(target_arch = "wasm64")]
	"            ptr = this.#jsExports['js_sys.memory.realloc'](",
	#[cfg(target_arch = "wasm64")]
	"                ptr, BigInt(capacity), BigInt(written), 1n,",
	#[cfg(target_arch = "wasm64")]
	"            )",
	"        }}",
	"",
	"        if (out !== undefined) {{",
	"            store(ptr, written, out)",
	"            return",
	"        }}",
	#[cfg(target_arch = "wasm32")]
	"        return [ptr, written]",
	#[cfg(target_arch = "wasm64")]
	"        return [ptr, BigInt(written)]",
	"    }}",
	"    const slots = value => {{",
	"        if (typeof value !== 'string')",
	"            throw new TypeError(`expected a string, found ${{typeof value}}`)",
	"        const capacity = value.length",
	#[cfg(target_arch = "wasm32")]
	"        const ptr = this.#jsExports['js_sys.memory.alloc'](capacity, 1)",
	#[cfg(target_arch = "wasm64")]
	"        const ptr = this.#jsExports['js_sys.memory.alloc'](BigInt(capacity), 1n)",
	"        const written = ascii(value, ptr, capacity)",
	"        if (written !== capacity) return unicode(value, ptr, capacity, written)",
	#[cfg(target_arch = "wasm32")]
	"        return [ptr, written]",
	#[cfg(target_arch = "wasm64")]
	"        return [ptr, BigInt(written)]",
	"    }}",
	"    const sret = (value, out) => {{",
	"        if (typeof value !== 'string')",
	"            throw new TypeError(`expected a string, found ${{typeof value}}`)",
	"        const capacity = value.length",
	#[cfg(target_arch = "wasm32")]
	"        const ptr = this.#jsExports['js_sys.memory.alloc'](capacity, 1)",
	#[cfg(target_arch = "wasm64")]
	"        const ptr = this.#jsExports['js_sys.memory.alloc'](BigInt(capacity), 1n)",
	"        const written = ascii(value, ptr, capacity)",
	"        if (written !== capacity) return unicode(value, ptr, capacity, written, out)",
	"        store(ptr, written, out)",
	"    }}",
	"    return {{ slots, sret }}",
	"}})()",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "string.option.from_js",
	required_embeds = [("js_sys", "string.from_js")],
	"(() => {{",
	"    const memory = this.#memory",
	"    let buffer = memory.buffer",
	"    let view = new DataView(buffer)",
	"    const slots = value => {{",
	"        if (value == null) return [0, {zero}, {zero}]",
	"        const pair = this.#jsEmbed.js_sys['string.from_js'].slots(value)",
	"        return [1, pair[0], pair[1]]",
	"    }}",
	"    const sret = (value, out) => {{",
	"        if (value == null) {{",
	"            if (buffer !== memory.buffer) {{",
	"                buffer = memory.buffer",
	"                view = new DataView(buffer)",
	"            }}",
	"            view.setUint32(out, 0, true)",
	#[cfg(target_arch = "wasm32")]
	"            view.setUint32(out + 4, 0, true)",
	#[cfg(target_arch = "wasm32")]
	"            view.setUint32(out + 8, 0, true)",
	#[cfg(target_arch = "wasm64")]
	"            view.setBigUint64(out + 8, 0n, true)",
	#[cfg(target_arch = "wasm64")]
	"            view.setBigUint64(out + 16, 0n, true)",
	"            return",
	"        }}",
	#[cfg(target_arch = "wasm32")]
	"        this.#jsEmbed.js_sys['string.from_js'].sret(value, out + 4)",
	#[cfg(target_arch = "wasm64")]
	"        this.#jsEmbed.js_sys['string.from_js'].sret(value, out + 8)",
	"        if (buffer !== memory.buffer) {{",
	"            buffer = memory.buffer",
	"            view = new DataView(buffer)",
	"        }}",
	"        view.setUint32(out, 1, true)",
	"    }}",
	"    return {{ slots, sret }}",
	"}})()",
	#[cfg(target_arch = "wasm32")]
	zero = interpolate "0",
	#[cfg(target_arch = "wasm64")]
	zero = interpolate "0n",
);

#[doc(hidden)]
pub struct StringAbi {
	ptr: PtrConst<u8>,
	len: PtrLength<u8>,
}

// SAFETY: `StringAbi` is represented by its pointer and length, in that order.
unsafe impl WasmAbi for StringAbi {
	type Slot1 = PtrConst<u8>;
	type Slot2 = PtrLength<u8>;
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
unsafe impl ReturnAbi for StringAbi {
	const MODE: ReturnMode = ReturnMode::Indirect;
}

// SAFETY: The presence tag followed by the pointer and length forms a
// three-slot aggregate returned through a hidden pointer.
unsafe impl ReturnAbi for Option<StringAbi> {
	const MODE: ReturnMode = ReturnMode::Indirect;
}

// SAFETY: The UTF-8 byte slice is decoded before the JavaScript call.
unsafe impl IntoJS for &str {
	const JS_CONV: Option<IntoJsConv> = Some(
		IntoJsConv::new(crate::const_concat!(
			"this.#jsEmbed.js_sys['string.decode'](",
			JS_PTR_LEN_ARGS,
			")"
		))
		.with_embed("js_sys", "string.decode"),
	);

	type Abi = ExternSlice<u8>;

	fn into_abi(self) -> Self::Abi {
		ExternSlice::new(self.as_bytes())
	}
}

// SAFETY: The allocation is decoded as UTF-8 and freed before JavaScript
// observes the converted value.
unsafe impl IntoJS for String {
	const JS_CONV: Option<IntoJsConv> = Some(
		IntoJsConv::new(crate::const_concat!(
			"this.#jsEmbed.js_sys['string.take'](",
			JS_PTR_LEN_ARGS,
			")"
		))
		.with_embed("js_sys", "string.take"),
	);

	type Abi = StringAbi;

	fn into_abi(self) -> Self::Abi {
		let bytes = self.into_bytes().into_boxed_slice();
		let len = bytes.len();
		let ptr = Box::into_raw(bytes).cast::<u8>();

		StringAbi {
			ptr: PtrConst::from_raw(ptr),
			len: PtrLength::from_len(len),
		}
	}
}

// SAFETY: The presence tag distinguishes `None` from every string, including
// the empty string. Only a present allocation is decoded and released.
unsafe impl OptionIntoAbi<String> for StringAbi {
	const JS_CONV: Option<IntoJsConv> = Some(
		IntoJsConv::new(crate::const_concat!(
			"$slot1 === 0 ? undefined : this.#jsEmbed.js_sys['string.take'](",
			JS_OPTION_PTR_LEN_ARGS,
			")"
		))
		.with_embed("js_sys", "string.take"),
	);

	type Abi = Option<Self>;

	fn into_option_abi(value: Option<String>) -> Self::Abi {
		value.map(<String as IntoJS>::into_abi)
	}
}

// SAFETY: JavaScript allocates an exact-size byte buffer, fills it with valid
// UTF-8, and transfers ownership through the pointer and length slots.
unsafe impl FromJS for String {
	const JS_CONV: Option<FromJsConv> = Some(
		FromJsConv::slot1("$prepared[0]")
			.slot2("$prepared[1]")
			.prepare("this.#jsEmbed.js_sys['string.from_js'].slots($value)")
			.with_embed("js_sys", "string.from_js"),
	);
	const JS_SRET: Option<Sret> = Some(Sret::Value(
		"this.#jsEmbed.js_sys['string.from_js'].sret",
	));

	type Abi = StringAbi;

	fn from_abi(raw: Self::Abi) -> Self {
		let ptr = raw.ptr.as_ptr().cast_mut();
		let len = raw.len.get();
		// SAFETY: The conversion helper allocated exactly `len` bytes through the
		// shared allocator and initialized all of them with valid UTF-8.
		unsafe {
			let bytes = Box::from_raw(core::ptr::slice_from_raw_parts_mut(ptr, len));
			Self::from_utf8_unchecked(bytes.into_vec())
		}
	}
}

// SAFETY: JavaScript `null` and `undefined` become `None`; every other value is
// converted once to an owned UTF-8 allocation and tagged as `Some`.
unsafe impl OptionFromAbi<String> for StringAbi {
	const JS_CONV: Option<FromJsConv> = Some(
		FromJsConv::slot1("$prepared[0]")
			.slot2("$prepared[1]")
			.slot3("$prepared[2]")
			.prepare("this.#jsEmbed.js_sys['string.option.from_js'].slots($value)")
			.with_embed("js_sys", "string.option.from_js"),
	);
	const JS_SRET: Option<Sret> = Some(Sret::Value(
		"this.#jsEmbed.js_sys['string.option.from_js'].sret",
	));

	type Abi = Option<Self>;

	fn from_option_abi(raw: Self::Abi) -> Option<String> {
		raw.map(<String as FromJS>::from_abi)
	}
}
