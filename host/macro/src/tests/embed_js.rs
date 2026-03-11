use quote::quote;

#[test]
fn basic() {
	let output = crate::embed_js_internal(quote! {
		module = "foo", name = "bar", "",
	})
	.unwrap();

	test!(output, {
		const _: () = {
			const ARR_0: [::core::primitive::u8; 11] = *b"\x03\0foo\x03\0bar\0";
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 11;
				len as _
			};

			#[repr(C)]
			struct Layout([::core::primitive::u8; 4], [::core::primitive::u8; 11]);

			#[unsafe(link_section = "js_bindgen.embed")]
			static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0);
		};
	});
}
