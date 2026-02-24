use crate::hazard::Input;
use crate::util::ExternSlice;

// SAFETY: Implementation.
unsafe impl Input for &str {
	const IMPORT_TYPE: &'static str = Self::Type::IMPORT_TYPE;
	const TYPE: &'static str = Self::Type::TYPE;
	const CONV: &'static str = Self::Type::CONV;
	const JS_CONV_EMBED: (&'static str, &'static str) = ("js_sys", "string.rust.decode");
	const JS_CONV: &'static str = " = this.#jsEmbed.js_sys['string.rust.decode'](";
	const JS_CONV_POST: &'static str = ")";

	type Type = ExternSlice<u8>;

	fn into_raw(self) -> Self::Type {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = "string.rust.decode",
			required_embeds = [("js_sys", "extern_slice"), ("js_sys", "string.decode")],
			"(dataPtr) => {{",
			"	const {{ ptr, len }} = this.#jsEmbed.js_sys['extern_slice'](dataPtr)",
			"	return this.#jsEmbed.js_sys['string.decode'](ptr, len)",
			"}}",
		);

		ExternSlice::new(self.as_bytes())
	}
}
