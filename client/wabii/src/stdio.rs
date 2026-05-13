#[cfg(feature = "import")]
#[link(wasm_import_module = "wabii")]
unsafe extern "C" {
	#[link_name = "stdio.stdout"]
	pub fn stdout(ptr: *const u8, len: usize);
	#[link_name = "stdio.stderr"]
	pub fn stderr(ptr: *const u8, len: usize);
}

#[cfg(feature = "impl")]
mod import {
	js_bindgen::embed_js! {
		module = "wabii",
		name = "stdio.writer",
		"(memory, write) => {{",
		"  const decoder = new TextDecoder('utf-8', {{",
		"      fatal: false,",
		"      ignoreBOM: false,",
		"  }})",
		"  let buffer = ''",
		"  return (ptr, len) => {{",
        #[cfg(target_arch = "wasm64")]
		"      ptr = Number(ptr)",
		#[cfg(target_arch = "wasm64")]
		"      len = Number(len)",
		"      const view = new Uint8Array(memory.buffer, ptr, len)",
		"      buffer += decoder.decode(view, {{ stream: true }})",
		"      for (;;) {{",
		"          const newline = buffer.indexOf('\\n')",
		"          if (newline === -1) {{",
		"              break",
		"          }}",
		"          write(buffer.slice(0, newline))",
		"          buffer = buffer.slice(newline + 1)",
		"      }}",
		"  }}",
		"}}",
	}

	js_bindgen::import_js! {
		module = "wabii",
		name = "stdio.stdout",
		required_embeds = [("wabii", "stdio.writer")],
		"this.#jsEmbed.wabii['stdio.writer'](this.#memory, (line) => console.log(line))",
	}

	js_bindgen::import_js! {
		module = "wabii",
		name = "stdio.stderr",
		required_embeds = [("wabii", "stdio.writer")],
		"this.#jsEmbed.wabii['stdio.writer'](this.#memory, (line) => console.error(line))",
	}
}

#[cfg(test)]
mod tests {
	use js_bindgen_test::test;

	use super::{stderr, stdout};

	#[test]
	pub fn test_stdio() {
		let text = b"hello world\n";
		#[expect(clippy::undocumented_unsafe_blocks, reason = "just test")]
		unsafe {
			stdout(text.as_ptr(), text.len());
			stderr(text.as_ptr(), text.len());
		}
	}
}
