use proc_macro2::TokenStream;
use quote::quote;

fn embed_asm(input: TokenStream, expected: TokenStream) {
	let output = crate::unsafe_embed_asm(input);

	let output = syn::parse2(output).unwrap();
	let output = prettyplease::unparse(&output);
	let expected = syn::parse2(expected).unwrap();
	let expected = prettyplease::unparse(&expected);

	similar_asserts::assert_eq!(expected, output);
}

fn js_import(input: TokenStream, expected: TokenStream) {
	let output = crate::js_import(input);

	let output = syn::parse2(output).unwrap();
	let output = prettyplease::unparse(&output);
	let expected = syn::parse2(expected).unwrap();
	let expected = prettyplease::unparse(&expected);

	similar_asserts::assert_eq!(expected, output);
}

#[test]
fn basic() {
	embed_asm(
		quote! { "foo", "bar" },
		quote! {
			const _: () = {
				const LEN0: usize = 7;
				const ARR0: [u8; LEN0] = *b"foo\nbar";

				const LEN: u32 = {
					let mut len = 0;
					{ len += LEN0; }
					len as u32
				};

				#[repr(C)]
				struct Layout([u8; 4], [u8; 7]);

				#[link_section = "js_bindgen.assembly"]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR0);
			};
		},
	);
}

#[test]
fn minimum() {
	embed_asm(
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
fn no_newline() {
	embed_asm(
		quote! { "foo" },
		quote! {
			const _: () = {
				const LEN0: usize = 3;
				const ARR0: [u8; LEN0] = *b"foo";

				const LEN: u32 = {
					let mut len = 0;
					{ len += LEN0; }
					len as u32
				};

				#[repr(C)]
				struct Layout([u8; 4], [u8; 3]);

				#[link_section = "js_bindgen.assembly"]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR0);
			};
		},
	);
}

#[test]
fn merge() {
	embed_asm(
		quote! {
			"foo",
			"bar",
			"baz",
			#[cfg(test)]
			"qux",
			"quux",
			"corge",
			"grault",
		},
		quote! {
			const _: () = {
				const LEN0: usize = 12;
				const ARR0: [u8; LEN0] = *b"foo\nbar\nbaz\n";
				#[cfg(test)]
				const LEN1: usize = 4;
				#[cfg(test)]
				const ARR1: [u8; LEN1] = *b"qux\n";
				const LEN2: usize = 17;
				const ARR2: [u8; LEN2] = *b"quux\ncorge\ngrault";

				const LEN: u32 = {
					let mut len = 0;
					{ len += LEN0; }
					#[cfg(test)]
					{ len += LEN1; }
					{ len += LEN2; }
					len as u32
				};

				#[repr(C)]
				struct Layout(
					[u8; 4],
					[u8; 12],
					#[cfg(test)]
					[u8; 4],
					[u8; 17],
				);

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

#[test]
fn cfg() {
	embed_asm(
		quote! {
		   "test1",
		   #[cfg(test)]
		   "test2",
		   "test3",
		},
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

#[test]
fn escape() {
	embed_asm(
		quote! { "\n\t{{}}" },
		quote! {
			const _: () = {
				const LEN0: usize = 4;
				const ARR0: [u8; LEN0] = *b"\n\t{}";

				const LEN: u32 = {
					let mut len = 0;
					{ len += LEN0; }
					len as u32
				};

				#[repr(C)]
				struct Layout([u8; 4], [u8; 4]);

				#[link_section = "js_bindgen.assembly"]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR0);
			};
		},
	);
}

#[test]
fn interpolate() {
	embed_asm(
		quote! { "{}", interpolate "test" },
		quote! {
			const _: () = {
				const VAL0: &str = "test";
				const LEN0: usize = ::core::primitive::str::len(VAL0);
				const PTR0: *const u8 = ::core::primitive::str::as_ptr(VAL0);
				const ARR0: [u8; LEN0] = unsafe { *(PTR0 as *const _) };

				const LEN: u32 = {
					let mut len = 0;
					{ len += LEN0; }
					len as u32
				};

				#[repr(C)]
				struct Layout([u8; 4], [u8; LEN0]);

				#[link_section = "js_bindgen.assembly"]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR0);
			};
		},
	);
}

#[test]
fn import() {
	js_import(
		quote! {
			name = "foo",
			"bar", "baz",
		},
		quote! {
			const _: () = {
				const LEN0: usize = 7;
				const ARR0: [u8; LEN0] = *b"bar\nbaz";

				const LEN: u32 = {
					let mut len = 0;
					{ len += LEN0; }
					len as u32
				};

				#[repr(C)]
				struct Layout([u8; 4], [u8; 7]);

				#[link_section = "js_bindgen.import.test_crate.foo"]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR0);
			};
		},
	);
}
