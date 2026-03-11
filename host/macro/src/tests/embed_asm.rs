use quote::quote;

#[test]
fn basic() {
	let output = crate::embed_asm_internal(quote! { "foo", "bar" }).unwrap();

	test!(output, {
		const _: () = {
			const ARR_0: [::core::primitive::u8; 7] = *b"foo\nbar";
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 7;
				len as _
			};

			#[repr(C)]
			struct Layout([::core::primitive::u8; 4], [::core::primitive::u8; 7]);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0);
		};
	});
}

#[test]
fn minimum() {
	let output = crate::embed_asm_internal(quote! { "" }).unwrap();

	test!(output, {
		const _: () = {
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len as _
			};
			#[repr(C)]
			struct Layout([::core::primitive::u8; 4]);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN));
		};
	});
}

#[test]
fn no_newline() {
	let output = crate::embed_asm_internal(quote! { "foo" }).unwrap();

	test!(output, {
		const _: () = {
			const ARR_0: [::core::primitive::u8; 3] = *b"foo";
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 3;
				len as _
			};

			#[repr(C)]
			struct Layout([::core::primitive::u8; 4], [::core::primitive::u8; 3]);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0);
		};
	});
}

#[test]
fn merge_strings() {
	let output = crate::embed_asm_internal(quote! {
		"foo",
		"bar",
		"baz",
		#[cfg(test)]
		"qux",
		"quux",
		"corge",
		"grault",
	})
	.unwrap();

	test!(output, {
		const _: () = {
			const ARR_0: [::core::primitive::u8; 12] = *b"foo\nbar\nbaz\n";
			#[cfg(test)]
			const ARR_1: [::core::primitive::u8; 4] = *b"qux\n";
			const ARR_2: [::core::primitive::u8; 17] = *b"quux\ncorge\ngrault";
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 12;
				#[cfg(test)]
				{
					len += 4;
				}
				len += 17;
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				[::core::primitive::u8; 12],
				#[cfg(test)] [::core::primitive::u8; 4],
				[::core::primitive::u8; 17],
			);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				ARR_0,
				#[cfg(test)]
				ARR_1,
				ARR_2,
			);
		};
	});
}

#[test]
fn merge_edge_1() {
	let output = crate::embed_asm_internal(quote! {
		"",
		#[cfg(test)]
		"",
		"foo",
		"{}",
		interpolate &Bar
	})
	.unwrap();

	test!(output, {
		const _: () = {
			const ARR_0: [::core::primitive::u8; 1] = *b"\n";
			#[cfg(test)]
			const ARR_1: [::core::primitive::u8; 1] = *b"\n";
			const ARR_2: [::core::primitive::u8; 4] = *b"foo\n";
			const VAL_3: &::core::primitive::str = &Bar;
			const LEN_3: ::core::primitive::usize = ::core::primitive::str::len(VAL_3);
			const PTR_3: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_3);
			const ARR_3: [::core::primitive::u8; LEN_3] = unsafe { *(PTR_3 as *const _) };
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 1;
				#[cfg(test)]
				{
					len += 1;
				}
				len += 4;
				len += LEN_3;
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				[::core::primitive::u8; 1],
				#[cfg(test)] [::core::primitive::u8; 1],
				[::core::primitive::u8; 4],
				[::core::primitive::u8; LEN_3],
			);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				ARR_0,
				#[cfg(test)]
				ARR_1,
				ARR_2,
				ARR_3,
			);
		};
	});
}

#[test]
fn merge_edge_2() {
	let output = crate::embed_asm_internal(quote! {
		#[cfg(test)]
		"",
		#[cfg(test)]
		"",
	})
	.unwrap();

	test!(output, {
		const _: () = {
			#[cfg(test)]
			const ARR_0: [::core::primitive::u8; 1] = *b"\n";
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				#[cfg(test)]
				{
					len += 1;
				}
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				#[cfg(test)] [::core::primitive::u8; 1],
			);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				#[cfg(test)]
				ARR_0,
			);
		};
	});
}

#[test]
fn cfg() {
	let output = crate::embed_asm_internal(quote! {
	   "test1",
	   #[cfg(test)]
	   "test2",
	   "test3",
	})
	.unwrap();

	test!(output, {
		const _: () = {
			const ARR_0: [::core::primitive::u8; 6] = *b"test1\n";
			#[cfg(test)]
			const ARR_1: [::core::primitive::u8; 6] = *b"test2\n";
			const ARR_2: [::core::primitive::u8; 5] = *b"test3";
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 6;
				#[cfg(test)]
				{
					len += 6;
				}
				len += 5;
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				[::core::primitive::u8; 6],
				#[cfg(test)] [::core::primitive::u8; 6],
				[::core::primitive::u8; 5],
			);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				ARR_0,
				#[cfg(test)]
				ARR_1,
				ARR_2,
			);
		};
	});
}

#[test]
fn escape() {
	let output = crate::embed_asm_internal(quote! { "\n\t\"\\{{}}" }).unwrap();

	test!(output, {
		const _: () = {
			const ARR_0: [::core::primitive::u8; 6] = *b"\n\t\"\\{}";
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += 6;
				len as _
			};

			#[repr(C)]
			struct Layout([::core::primitive::u8; 4], [::core::primitive::u8; 6]);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0);
		};
	});
}

#[test]
fn interpolate() {
	let output = crate::embed_asm_internal(quote! { "{}", interpolate "test" }).unwrap();

	test!(output, {
		const _: () = {
			const VAL_0: &::core::primitive::str = "test";
			const LEN_0: ::core::primitive::usize = ::core::primitive::str::len(VAL_0);
			const PTR_0: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_0);
			const ARR_0: [::core::primitive::u8; LEN_0] = unsafe { *(PTR_0 as *const _) };
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += LEN_0;
				len as _
			};

			#[repr(C)]
			struct Layout([::core::primitive::u8; 4], [::core::primitive::u8; LEN_0]);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0);
		};
	});
}

#[test]
fn r#const() {
	let output = crate::embed_asm_internal(quote! { "{}", const 42 }).unwrap();

	test!(output, {
		const _: () = {
			const LEN_0: ::core::primitive::usize =
				::js_bindgen::r#macro::ConstInteger(42).__jbg_len();
			const ARR_0: [::core::primitive::u8; LEN_0] =
				::js_bindgen::r#macro::ConstInteger(42).__jbg_to_le_bytes::<LEN_0>();
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += LEN_0;
				len as _
			};

			#[repr(C)]
			struct Layout([::core::primitive::u8; 4], [::core::primitive::u8; LEN_0]);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0);
		};
	});
}

#[test]
fn interpolate_macro() {
	let output = crate::embed_asm_internal(quote! {
		"{}",
		"{}",
		interpolate foo!(),
		interpolate <Foo<Bar::Baz> as Qux>::QUUX,
	})
	.unwrap();

	test!(output, {
		const _: () = {
			const VAL_0: &::core::primitive::str = foo!();
			const LEN_0: ::core::primitive::usize = ::core::primitive::str::len(VAL_0);
			const PTR_0: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_0);
			const ARR_0: [::core::primitive::u8; LEN_0] = unsafe { *(PTR_0 as *const _) };
			const ARR_1: [::core::primitive::u8; 1] = *b"\n";
			const VAL_2: &::core::primitive::str = <Foo<Bar::Baz> as Qux>::QUUX;
			const LEN_2: ::core::primitive::usize = ::core::primitive::str::len(VAL_2);
			const PTR_2: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_2);
			const ARR_2: [::core::primitive::u8; LEN_2] = unsafe { *(PTR_2 as *const _) };
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += LEN_0;
				len += 1;
				len += LEN_2;
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				[::core::primitive::u8; LEN_0],
				[::core::primitive::u8; 1],
				[::core::primitive::u8; LEN_2],
			);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				ARR_0,
				ARR_1,
				ARR_2,
			);
		};
	});
}

#[test]
fn named_const() {
	let output = crate::embed_asm_internal(quote! { "{par}", par = const 42 }).unwrap();

	test!(output, {
		const _: () = {
			const LEN_par: ::core::primitive::usize =
				::js_bindgen::r#macro::ConstInteger(42).__jbg_len();
			const ARR_par: [::core::primitive::u8; LEN_par] =
				::js_bindgen::r#macro::ConstInteger(42).__jbg_to_le_bytes::<LEN_par>();
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += LEN_par;
				len as _
			};

			#[repr(C)]
			struct Layout([::core::primitive::u8; 4], [::core::primitive::u8; LEN_par]);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout =
				Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_par);
		};
	});
}

#[test]
fn named_cfg() {
	let output = crate::embed_asm_internal(quote! {
		"{par}",
		#[cfg(test)]
		par = interpolate "test",
	})
	.unwrap();

	test!(output, {
		const _: () = {
			#[cfg(test)]
			const VAL_par: &::core::primitive::str = "test";
			#[cfg(test)]
			const LEN_par: ::core::primitive::usize = ::core::primitive::str::len(VAL_par);
			#[cfg(test)]
			const PTR_par: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_par);
			#[cfg(test)]
			const ARR_par: [::core::primitive::u8; LEN_par] = unsafe { *(PTR_par as *const _) };
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				#[cfg(test)]
				{
					len += LEN_par;
				}
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				#[cfg(test)] [::core::primitive::u8; LEN_par],
			);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				#[cfg(test)]
				ARR_par,
			);
		};
	});
}

#[test]
fn named_cfg_2() {
	let output = crate::embed_asm_internal(quote! {
		"{par_1}",
		"{par_2}",
		par_1 = interpolate "test",
		#[cfg(test)]
		par_2 = interpolate "test",
	})
	.unwrap();

	test!(output, {
		const _: () = {
			const VAL_par_1: &::core::primitive::str = "test";
			const LEN_par_1: ::core::primitive::usize = ::core::primitive::str::len(VAL_par_1);
			const PTR_par_1: *const ::core::primitive::u8 =
				::core::primitive::str::as_ptr(VAL_par_1);
			const ARR_par_1: [::core::primitive::u8; LEN_par_1] =
				unsafe { *(PTR_par_1 as *const _) };
			#[cfg(test)]
			const VAL_par_2: &::core::primitive::str = "test";
			#[cfg(test)]
			const LEN_par_2: ::core::primitive::usize = ::core::primitive::str::len(VAL_par_2);
			#[cfg(test)]
			const PTR_par_2: *const ::core::primitive::u8 =
				::core::primitive::str::as_ptr(VAL_par_2);
			#[cfg(test)]
			const ARR_par_2: [::core::primitive::u8; LEN_par_2] =
				unsafe { *(PTR_par_2 as *const _) };
			const ARR_0: [::core::primitive::u8; 1] = *b"\n";
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				len += LEN_par_1;
				len += 1;
				#[cfg(test)]
				{
					len += LEN_par_2;
				}
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				[::core::primitive::u8; LEN_par_1],
				[::core::primitive::u8; 1],
				#[cfg(test)] [::core::primitive::u8; LEN_par_2],
			);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				ARR_par_1,
				ARR_0,
				#[cfg(test)]
				ARR_par_2,
			);
		};
	});
}

#[test]
fn named_cfg_same() {
	let output = crate::embed_asm_internal(quote! {
		"{par}",
		#[cfg(test)]
		par = interpolate "test",
		#[cfg(not(test))]
		par = interpolate "not test",
	})
	.unwrap();

	test!(output, {
		const _: () = {
			#[cfg(test)]
			const VAL_par: &::core::primitive::str = "test";
			#[cfg(test)]
			const LEN_par: ::core::primitive::usize = ::core::primitive::str::len(VAL_par);
			#[cfg(test)]
			const PTR_par: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_par);
			#[cfg(test)]
			const ARR_par: [::core::primitive::u8; LEN_par] = unsafe { *(PTR_par as *const _) };
			#[cfg(not(test))]
			const VAL_par: &::core::primitive::str = "not test";
			#[cfg(not(test))]
			const LEN_par: ::core::primitive::usize = ::core::primitive::str::len(VAL_par);
			#[cfg(not(test))]
			const PTR_par: *const ::core::primitive::u8 = ::core::primitive::str::as_ptr(VAL_par);
			#[cfg(not(test))]
			const ARR_par: [::core::primitive::u8; LEN_par] = unsafe { *(PTR_par as *const _) };
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				#[cfg(test)]
				{
					len += LEN_par;
				}
				#[cfg(not(test))]
				{
					len += LEN_par;
				}
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				#[cfg(test)] [::core::primitive::u8; LEN_par],
				#[cfg(not(test))] [::core::primitive::u8; LEN_par],
			);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				#[cfg(test)]
				ARR_par,
				#[cfg(not(test))]
				ARR_par,
			);
		};
	});
}

#[test]
fn named_const_cfg_same() {
	let output = crate::embed_asm_internal(quote! {
		"{par}",
		#[cfg(test)]
		par = const 43,
		#[cfg(not(test))]
		par = const 0,
	})
	.unwrap();

	test!(output, {
		const _: () = {
			#[cfg(test)]
			const LEN_par: ::core::primitive::usize =
				::js_bindgen::r#macro::ConstInteger(43).__jbg_len();
			#[cfg(test)]
			const ARR_par: [::core::primitive::u8; LEN_par] =
				::js_bindgen::r#macro::ConstInteger(43).__jbg_to_le_bytes::<LEN_par>();
			#[cfg(not(test))]
			const LEN_par: ::core::primitive::usize =
				::js_bindgen::r#macro::ConstInteger(0).__jbg_len();
			#[cfg(not(test))]
			const ARR_par: [::core::primitive::u8; LEN_par] =
				::js_bindgen::r#macro::ConstInteger(0).__jbg_to_le_bytes::<LEN_par>();
			const LEN: ::core::primitive::u32 = {
				let mut len = 0;
				#[cfg(test)]
				{
					len += LEN_par;
				}
				#[cfg(not(test))]
				{
					len += LEN_par;
				}
				len as _
			};

			#[repr(C)]
			struct Layout(
				[::core::primitive::u8; 4],
				#[cfg(test)] [::core::primitive::u8; LEN_par],
				#[cfg(not(test))] [::core::primitive::u8; LEN_par],
			);

			#[unsafe(link_section = "js_bindgen.assembly")]
			static CUSTOM_SECTION: Layout = Layout(
				::core::primitive::u32::to_le_bytes(LEN),
				#[cfg(test)]
				ARR_par,
				#[cfg(not(test))]
				ARR_par,
			);
		};
	});
}
