#[cfg(feature = "web-idl")]
macro_rules! test {
	($output:tt, $expected:tt $(,)?) => {
		let output = syn::parse_quote! $output;
		let output = prettyplease::unparse(&output);

		inline_snap::inline_snap!(output, $expected);
	};
}

mod closure;
mod r#macro;
#[cfg(feature = "web-idl")]
mod web_idl;
