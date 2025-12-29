use proc_macro2::TokenStream;
use quote::quote;

fn test_embed_asm(input: TokenStream, expected: TokenStream) {
	let output = crate::unsafe_embed_asm(input);

	let output = syn::parse2(output).unwrap();
	let output = prettyplease::unparse(&output);
	let expected = syn::parse2(expected).unwrap();
	let expected = prettyplease::unparse(&expected);

	similar_asserts::assert_eq!(expected, output);
}

#[test]
fn minimum() {
	test_embed_asm(
		quote! { "" },
		quote! {
			const _: () = {
				const LEN: u32 = {
					let mut len = 0;
					len as u32
				};

				#[repr(C)]
				struct Layout([u8; 4]);

				#[link_section = "js_bindgen.assembly"]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN));
			};
		},
	);
}

#[test]
fn basic() {
	test_embed_asm(
		quote! { "test", "test" },
		quote! {
			const _: () = {
				const LEN0: usize = 9;
				const ARR0: [u8; LEN0] = *b"test\ntest";

				const LEN: u32 = {
					let mut len = 0;
					{ len += LEN0; }
					len as u32
				};

				#[repr(C)]
				struct Layout([u8; 4], [u8; 9]);

				#[link_section = "js_bindgen.assembly"]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR0);
			};
		},
	);
}

#[test]
fn cfg() {
	test_embed_asm(
		quote! { "test1", #[cfg(test)] "test2", "test3" },
		quote! {
			const _: () = {
				const LEN0: usize = 6;
				const ARR0: [u8; LEN0] = *b"test1\n";
				#[cfg(test)]
				const LEN1: usize = 6;
				#[cfg(test)]
				const ARR1: [u8; LEN1] = *b"test2\n";
				const LEN2: usize = 5;
				const ARR2: [u8; LEN2] = *b"test3";

				const LEN: u32 = {
					let mut len = 0;
					{ len += LEN0; }
					#[cfg(test)] { len += LEN1; }
					{ len += LEN2; }
					len as u32
				};

				#[repr(C)]
				struct Layout([u8; 4], [u8; 6], #[cfg(test)] [u8; 6], [u8; 5]);

				#[link_section = "js_bindgen.assembly"]
				static CUSTOM_SECTION: Layout = Layout(
					::core::primitive::u32::to_le_bytes(LEN),
					ARR0,
					#[cfg(test)]
					ARR1,
					ARR2,
				);
			};
		},
	);
}
