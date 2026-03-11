macro_rules! test {
	($output:expr, $expected:tt $(,)?) => {
		let output = syn::parse2($output).unwrap();
		let output = prettyplease::unparse(&output);

		inline_snap::inline_snap!(output, $expected);
	};
}

mod embed_asm;
mod embed_js;
mod import_js;
