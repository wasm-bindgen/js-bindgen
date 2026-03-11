use quote::quote;

#[test]
fn basic() {
	let output = crate::import_js_internal(quote! {
		module = "foo", name = "bar", "baz", "qux",
	})
	.unwrap();

	test!(output, {
		const _: () = {
			const ARR_0: [::core::primitive::u8; 11] = *b"\x03\0foo\x03\0bar\0";
			const ARR_1: [::core::primitive::u8; 7] = *b"baz\nqux";
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 11;
				len += 7;
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				[::core::primitive::u8; 11],
				[::core::primitive::u8; 7],
			);

			#[unsafe(link_section = "js_bindgen.import")]
			static CUSTOM_SECTION: Layout =
				Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0, ARR_1);
		};
	});
}

#[test]
fn required_embeds() {
	let output = crate::import_js_internal(quote! {
		module = "foo", name = "bar", required_embeds = [("baz", "qux")], "",
	})
	.unwrap();

	test!(output, {
		const _: () = {
			const TUPLE_0: (&::core::primitive::str, &::core::primitive::str) = ("baz", "qux");
			const ARR_0: [::core::primitive::u8; 11] = *b"\x03\0foo\x03\0bar\x01";
			const VAL_1: &::core::primitive::str = TUPLE_0.0;
			const LEN_1: ::core::primitive::usize = ::core::primitive::str::len(VAL_1);
			const PTR_1: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_1);
			const ARR_1: [::core::primitive::u8; LEN_1] = unsafe { *(PTR_1 as *const _) };
			const VAL_1_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_1 as u16);
			const VAL_2: &::core::primitive::str = TUPLE_0.1;
			const LEN_2: ::core::primitive::usize = ::core::primitive::str::len(VAL_2);
			const PTR_2: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_2);
			const ARR_2: [::core::primitive::u8; LEN_2] = unsafe { *(PTR_2 as *const _) };
			const VAL_2_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_2 as u16);
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 11;
				len += LEN_1;
				len += 2;
				len += LEN_2;
				len += 2;
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				[::core::primitive::u8; 11],
				[::core::primitive::u8; 2],
				[::core::primitive::u8; LEN_1],
				[::core::primitive::u8; 2],
				[::core::primitive::u8; LEN_2],
			);

			#[unsafe(link_section = "js_bindgen.import")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				ARR_0,
				VAL_1_LEN,
				ARR_1,
				VAL_2_LEN,
				ARR_2,
			);
		};
	});
}

#[test]
fn required_embeds_expr() {
	let output = crate::import_js_internal(quote! {
		module = "foo", name = "bar", required_embeds = [123 + 456], "",
	})
	.unwrap();

	test!(output, {
		const _: () = {
			const TUPLE_0: (&::core::primitive::str, &::core::primitive::str) = 123 + 456;
			const ARR_0: [::core::primitive::u8; 11] = *b"\x03\0foo\x03\0bar\x01";
			const VAL_1: &::core::primitive::str = TUPLE_0.0;
			const LEN_1: ::core::primitive::usize = ::core::primitive::str::len(VAL_1);
			const PTR_1: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_1);
			const ARR_1: [::core::primitive::u8; LEN_1] = unsafe { *(PTR_1 as *const _) };
			const VAL_1_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_1 as u16);
			const VAL_2: &::core::primitive::str = TUPLE_0.1;
			const LEN_2: ::core::primitive::usize = ::core::primitive::str::len(VAL_2);
			const PTR_2: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_2);
			const ARR_2: [::core::primitive::u8; LEN_2] = unsafe { *(PTR_2 as *const _) };
			const VAL_2_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_2 as u16);
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 11;
				len += LEN_1;
				len += 2;
				len += LEN_2;
				len += 2;
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				[::core::primitive::u8; 11],
				[::core::primitive::u8; 2],
				[::core::primitive::u8; LEN_1],
				[::core::primitive::u8; 2],
				[::core::primitive::u8; LEN_2],
			);

			#[unsafe(link_section = "js_bindgen.import")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				ARR_0,
				VAL_1_LEN,
				ARR_1,
				VAL_2_LEN,
				ARR_2,
			);
		};
	});
}

#[test]
fn required_embeds_cfg() {
	let output = crate::import_js_internal(quote! {
		module = "foo", name = "bar", required_embeds = [#[cfg(test)] ("baz", "qux")], "",
	})
	.unwrap();

	test!(output, {
		const _: () = {
			#[cfg(test)]
			const TUPLE_0: (&::core::primitive::str, &::core::primitive::str) = ("baz", "qux");
			const ARR_0: [::core::primitive::u8; 10] = *b"\x03\0foo\x03\0bar";
			#[cfg(test)]
			const VAL_1: &::core::primitive::str = TUPLE_0.0;
			#[cfg(test)]
			const LEN_1: ::core::primitive::usize = ::core::primitive::str::len(VAL_1);
			#[cfg(test)]
			const PTR_1: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_1);
			#[cfg(test)]
			const ARR_1: [::core::primitive::u8; LEN_1] = unsafe { *(PTR_1 as *const _) };
			#[cfg(test)]
			const VAL_1_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_1 as u16);
			#[cfg(test)]
			const VAL_2: &::core::primitive::str = TUPLE_0.1;
			#[cfg(test)]
			const LEN_2: ::core::primitive::usize = ::core::primitive::str::len(VAL_2);
			#[cfg(test)]
			const PTR_2: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_2);
			#[cfg(test)]
			const ARR_2: [::core::primitive::u8; LEN_2] = unsafe { *(PTR_2 as *const _) };
			#[cfg(test)]
			const VAL_2_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_2 as u16);
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 10;
				len += 1;
				#[cfg(test)]
				{
					len += LEN_1;
					len += 2;
				}
				#[cfg(test)]
				{
					len += LEN_2;
					len += 2;
				}
				len as _
			};
			const TUPLE_COUNT: ::core::primitive::u8 = {
				let mut len = 0;
				#[cfg(test)]
				{
					len += 1;
				}
				len
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				[::core::primitive::u8; 10],
				[::core::primitive::u8; 1],
				#[cfg(test)] [::core::primitive::u8; 2],
				#[cfg(test)] [::core::primitive::u8; LEN_1],
				#[cfg(test)] [::core::primitive::u8; 2],
				#[cfg(test)] [::core::primitive::u8; LEN_2],
			);

			#[unsafe(link_section = "js_bindgen.import")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				ARR_0,
				TUPLE_COUNT,
				#[cfg(test)]
				VAL_1_LEN,
				#[cfg(test)]
				ARR_1,
				#[cfg(test)]
				VAL_2_LEN,
				#[cfg(test)]
				ARR_2,
			);
		};
	});
}

#[test]
fn required_embeds_multiple() {
	let output = crate::import_js_internal(quote! {
		module = "foo", name = "bar", required_embeds = [("baz", "qux"), ("quux", "corge")], "",
	})
	.unwrap();

	test!(output, {
		const _: () = {
			const TUPLE_0: (&::core::primitive::str, &::core::primitive::str) = ("baz", "qux");
			const TUPLE_1: (&::core::primitive::str, &::core::primitive::str) = ("quux", "corge");
			const ARR_0: [::core::primitive::u8; 11] = *b"\x03\0foo\x03\0bar\x02";
			const VAL_1: &::core::primitive::str = TUPLE_0.0;
			const LEN_1: ::core::primitive::usize = ::core::primitive::str::len(VAL_1);
			const PTR_1: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_1);
			const ARR_1: [::core::primitive::u8; LEN_1] = unsafe { *(PTR_1 as *const _) };
			const VAL_1_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_1 as u16);
			const VAL_2: &::core::primitive::str = TUPLE_0.1;
			const LEN_2: ::core::primitive::usize = ::core::primitive::str::len(VAL_2);
			const PTR_2: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_2);
			const ARR_2: [::core::primitive::u8; LEN_2] = unsafe { *(PTR_2 as *const _) };
			const VAL_2_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_2 as u16);
			const VAL_3: &::core::primitive::str = TUPLE_1.0;
			const LEN_3: ::core::primitive::usize = ::core::primitive::str::len(VAL_3);
			const PTR_3: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_3);
			const ARR_3: [::core::primitive::u8; LEN_3] = unsafe { *(PTR_3 as *const _) };
			const VAL_3_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_3 as u16);
			const VAL_4: &::core::primitive::str = TUPLE_1.1;
			const LEN_4: ::core::primitive::usize = ::core::primitive::str::len(VAL_4);
			const PTR_4: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_4);
			const ARR_4: [::core::primitive::u8; LEN_4] = unsafe { *(PTR_4 as *const _) };
			const VAL_4_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_4 as u16);
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 11;
				len += LEN_1;
				len += 2;
				len += LEN_2;
				len += 2;
				len += LEN_3;
				len += 2;
				len += LEN_4;
				len += 2;
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				[::core::primitive::u8; 11],
				[::core::primitive::u8; 2],
				[::core::primitive::u8; LEN_1],
				[::core::primitive::u8; 2],
				[::core::primitive::u8; LEN_2],
				[::core::primitive::u8; 2],
				[::core::primitive::u8; LEN_3],
				[::core::primitive::u8; 2],
				[::core::primitive::u8; LEN_4],
			);

			#[unsafe(link_section = "js_bindgen.import")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				ARR_0,
				VAL_1_LEN,
				ARR_1,
				VAL_2_LEN,
				ARR_2,
				VAL_3_LEN,
				ARR_3,
				VAL_4_LEN,
				ARR_4,
			);
		};
	});
}

#[test]
fn required_embeds_empty() {
	let output = crate::import_js_internal(quote! {
		module = "foo", name = "bar", required_embeds = [], "",
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

			#[unsafe(link_section = "js_bindgen.import")]
			static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0);
		};
	});
}

#[test]
fn required_embeds_mixed() {
	let output = crate::import_js_internal(quote! {
		module = "foo",
		name = "bar",
		required_embeds = [
			("0", "0"),
			#[cfg(test)]
			("1", "1"),
			#[cfg(not(test))]
			("42", "42"),
			("2", "2"),
		],
		""
	})
	.unwrap();

	test!(output, {
		const _: () = {
			const TUPLE_0: (&::core::primitive::str, &::core::primitive::str) = ("0", "0");
			#[cfg(test)]
			const TUPLE_1: (&::core::primitive::str, &::core::primitive::str) = ("1", "1");
			#[cfg(not(test))]
			const TUPLE_2: (&::core::primitive::str, &::core::primitive::str) = ("42", "42");
			const TUPLE_3: (&::core::primitive::str, &::core::primitive::str) = ("2", "2");
			const ARR_0: [::core::primitive::u8; 10] = *b"\x03\0foo\x03\0bar";
			const VAL_1: &::core::primitive::str = TUPLE_0.0;
			const LEN_1: ::core::primitive::usize = ::core::primitive::str::len(VAL_1);
			const PTR_1: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_1);
			const ARR_1: [::core::primitive::u8; LEN_1] = unsafe { *(PTR_1 as *const _) };
			const VAL_1_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_1 as u16);
			const VAL_2: &::core::primitive::str = TUPLE_0.1;
			const LEN_2: ::core::primitive::usize = ::core::primitive::str::len(VAL_2);
			const PTR_2: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_2);
			const ARR_2: [::core::primitive::u8; LEN_2] = unsafe { *(PTR_2 as *const _) };
			const VAL_2_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_2 as u16);
			#[cfg(test)]
			const VAL_3: &::core::primitive::str = TUPLE_1.0;
			#[cfg(test)]
			const LEN_3: ::core::primitive::usize = ::core::primitive::str::len(VAL_3);
			#[cfg(test)]
			const PTR_3: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_3);
			#[cfg(test)]
			const ARR_3: [::core::primitive::u8; LEN_3] = unsafe { *(PTR_3 as *const _) };
			#[cfg(test)]
			const VAL_3_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_3 as u16);
			#[cfg(test)]
			const VAL_4: &::core::primitive::str = TUPLE_1.1;
			#[cfg(test)]
			const LEN_4: ::core::primitive::usize = ::core::primitive::str::len(VAL_4);
			#[cfg(test)]
			const PTR_4: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_4);
			#[cfg(test)]
			const ARR_4: [::core::primitive::u8; LEN_4] = unsafe { *(PTR_4 as *const _) };
			#[cfg(test)]
			const VAL_4_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_4 as u16);
			#[cfg(not(test))]
			const VAL_5: &::core::primitive::str = TUPLE_2.0;
			#[cfg(not(test))]
			const LEN_5: ::core::primitive::usize = ::core::primitive::str::len(VAL_5);
			#[cfg(not(test))]
			const PTR_5: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_5);
			#[cfg(not(test))]
			const ARR_5: [::core::primitive::u8; LEN_5] = unsafe { *(PTR_5 as *const _) };
			#[cfg(not(test))]
			const VAL_5_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_5 as u16);
			#[cfg(not(test))]
			const VAL_6: &::core::primitive::str = TUPLE_2.1;
			#[cfg(not(test))]
			const LEN_6: ::core::primitive::usize = ::core::primitive::str::len(VAL_6);
			#[cfg(not(test))]
			const PTR_6: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_6);
			#[cfg(not(test))]
			const ARR_6: [::core::primitive::u8; LEN_6] = unsafe { *(PTR_6 as *const _) };
			#[cfg(not(test))]
			const VAL_6_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_6 as u16);
			const VAL_7: &::core::primitive::str = TUPLE_3.0;
			const LEN_7: ::core::primitive::usize = ::core::primitive::str::len(VAL_7);
			const PTR_7: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_7);
			const ARR_7: [::core::primitive::u8; LEN_7] = unsafe { *(PTR_7 as *const _) };
			const VAL_7_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_7 as u16);
			const VAL_8: &::core::primitive::str = TUPLE_3.1;
			const LEN_8: ::core::primitive::usize = ::core::primitive::str::len(VAL_8);
			const PTR_8: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_8);
			const ARR_8: [::core::primitive::u8; LEN_8] = unsafe { *(PTR_8 as *const _) };
			const VAL_8_LEN: [::core::primitive::u8; 2] =
				::core::primitive::u16::to_le_bytes(LEN_8 as u16);
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 10;
				len += 1;
				len += LEN_1;
				len += 2;
				len += LEN_2;
				len += 2;
				#[cfg(test)]
				{
					len += LEN_3;
					len += 2;
				}
				#[cfg(test)]
				{
					len += LEN_4;
					len += 2;
				}
				#[cfg(not(test))]
				{
					len += LEN_5;
					len += 2;
				}
				#[cfg(not(test))]
				{
					len += LEN_6;
					len += 2;
				}
				len += LEN_7;
				len += 2;
				len += LEN_8;
				len += 2;
				len as _
			};
			const TUPLE_COUNT: ::core::primitive::u8 = {
				let mut len = 2;
				#[cfg(test)]
				{
					len += 1;
				}
				#[cfg(not(test))]
				{
					len += 1;
				}
				len
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				[::core::primitive::u8; 10],
				[::core::primitive::u8; 1],
				[::core::primitive::u8; 2],
				[::core::primitive::u8; LEN_1],
				[::core::primitive::u8; 2],
				[::core::primitive::u8; LEN_2],
				#[cfg(test)] [::core::primitive::u8; 2],
				#[cfg(test)] [::core::primitive::u8; LEN_3],
				#[cfg(test)] [::core::primitive::u8; 2],
				#[cfg(test)] [::core::primitive::u8; LEN_4],
				#[cfg(not(test))] [::core::primitive::u8; 2],
				#[cfg(not(test))] [::core::primitive::u8; LEN_5],
				#[cfg(not(test))] [::core::primitive::u8; 2],
				#[cfg(not(test))] [::core::primitive::u8; LEN_6],
				[::core::primitive::u8; 2],
				[::core::primitive::u8; LEN_7],
				[::core::primitive::u8; 2],
				[::core::primitive::u8; LEN_8],
			);

			#[unsafe(link_section = "js_bindgen.import")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				ARR_0,
				TUPLE_COUNT,
				VAL_1_LEN,
				ARR_1,
				VAL_2_LEN,
				ARR_2,
				#[cfg(test)]
				VAL_3_LEN,
				#[cfg(test)]
				ARR_3,
				#[cfg(test)]
				VAL_4_LEN,
				#[cfg(test)]
				ARR_4,
				#[cfg(not(test))]
				VAL_5_LEN,
				#[cfg(not(test))]
				ARR_5,
				#[cfg(not(test))]
				VAL_6_LEN,
				#[cfg(not(test))]
				ARR_6,
				VAL_7_LEN,
				ARR_7,
				VAL_8_LEN,
				ARR_8,
			);
		};
	});
}
