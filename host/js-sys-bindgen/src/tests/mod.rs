macro_rules! test {
	($output:tt, $expected:tt $(,)?) => {
		let output = syn::parse_quote! $output;
		let output = prettyplease::unparse(&output);

		inline_snap::inline_snap!(output, $expected);
	};
}

mod r#macro;
mod r#type;
#[cfg(feature = "web-idl")]
mod web_idl;
