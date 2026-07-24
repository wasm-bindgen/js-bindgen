use crate::hazard::{IntoJS, IntoJsConv};
use crate::util::ExternSlice;

#[cfg(any(not(target_feature = "atomics"), js_sys_target_feature = "sab"))]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "string.decode",
	"(() => {{",
	"	const decoder = new TextDecoder('utf-8', {{",
	"		fatal: false,",
	"		ignoreBOM: false,",
	"	}})",
	"	return (ptr, len) => {{",
	"		const view = new Uint8Array(this.#memory.buffer, ptr, len)",
	"		return decoder.decode(view)",
	"	}}",
	"}})()",
);

#[cfg(all(target_feature = "atomics", not(js_sys_target_feature = "sab")))]
js_bindgen::embed_js!(
	module = "js_sys",
	name = "string.decode",
	required_embeds = [("js_sys", "string.sab")],
	"(() => {{",
	"	const decoder = new TextDecoder('utf-8', {{",
	"		fatal: false,",
	"		ignoreBOM: false,",
	"	}})",
	"	return (ptr, len) => {{",
	"		const view = new Uint8Array(this.#memory.buffer, ptr, len)",
	"		return decoder.decode(",
	"			this.#jsEmbed.js_sys['string.sab'] ? view : view.slice()",
	"		)",
	"	}}",
	"}})()",
);

// SAFETY: The UTF-8 byte slice is decoded into a JavaScript string before the
// import is called.
unsafe impl IntoJS for &str {
	const JS_CONV: Option<IntoJsConv> = Some(
		IntoJsConv::new("this.#jsEmbed.js_sys['string.decode']($slot1, $slot2)")
			.with_embed(("js_sys", "string.decode")),
	);

	type Abi = ExternSlice<u8>;

	fn into_abi(self) -> Self::Abi {
		ExternSlice::new(self.as_bytes())
	}
}
