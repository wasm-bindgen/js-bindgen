use quote::quote;

#[test]
fn basic() {
	super::test(
		crate::unsafe_embed_asm(quote! { "foo", "bar" }),
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

				#[unsafe(link_section = "js_bindgen.assembly")]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR0);
			};
		},
	);
}

#[test]
fn minimum() {
	super::test(
		crate::unsafe_embed_asm(quote! { "" }),
		quote! {
			const _: () = {
				const LEN: u32 = {
					let mut len = 0;
					len as u32
				};

				#[repr(C)]
				struct Layout([u8; 4]);

				#[unsafe(link_section = "js_bindgen.assembly")]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN));
			};
		},
	);
}

#[test]
fn no_newline() {
	super::test(
		crate::unsafe_embed_asm(quote! { "foo" }),
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

				#[unsafe(link_section = "js_bindgen.assembly")]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR0);
			};
		},
	);
}

#[test]
fn merge_strings() {
	super::test(
		crate::unsafe_embed_asm(quote! {
			"foo",
			"bar",
			"baz",
			#[cfg(test)]
			"qux",
			"quux",
			"corge",
			"grault",
		}),
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

				#[unsafe(link_section = "js_bindgen.assembly")]
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
fn merge_edge_1() {
	super::test(
		crate::unsafe_embed_asm(quote! {
			"",
			#[cfg(test)]
			"",
			"foo",
			"{}",
			interpolate &Bar
		}),
		quote! {
			const _: () = {
				const LEN0: usize = 1;
				const ARR0: [u8; LEN0] = *b"\n";
				#[cfg(test)]
				const LEN1: usize = 1;
				#[cfg(test)]
				const ARR1: [u8; LEN1] = *b"\n";
				const LEN2: usize = 4;
				const ARR2: [u8; LEN2] = *b"foo\n";
				const VAL3: &str = &Bar;
				const LEN3: usize = ::core::primitive::str::len(VAL3);
				const PTR3: *const u8 = ::core::primitive::str::as_ptr(VAL3);
				const ARR3: [u8; LEN3] = unsafe { *(PTR3 as *const _) };

				const LEN: u32 = {
					let mut len = 0;
					{ len += LEN0; }
					#[cfg(test)]
					{ len += LEN1; }
					{ len += LEN2; }
					{ len += LEN3; }
					len as u32
				};

				#[repr(C)]
				struct Layout(
					[u8; 4],
					[u8; 1],
					#[cfg(test)]
					[u8; 1],
					[u8; 4],
					[u8; LEN3],
				);

				#[unsafe(link_section = "js_bindgen.assembly")]
				static CUSTOM_SECTION: Layout = Layout(
					::core::primitive::u32::to_le_bytes(LEN),
					ARR0,
					#[cfg(test)]
					ARR1,
					ARR2,
					ARR3,
				);
			};
		},
	);
}

#[test]
fn merge_edge_2() {
	super::test(
		crate::unsafe_embed_asm(quote! {
			#[cfg(test)]
			"",
			#[cfg(test)]
			"",
		}),
		quote! {
			const _: () = {
				#[cfg(test)]
				const LEN0: usize = 1;
				#[cfg(test)]
				const ARR0: [u8; LEN0] = *b"\n";

				const LEN: u32 = {
					let mut len = 0;
					#[cfg(test)]
					{ len += LEN0; }
					len as u32
				};

				#[repr(C)]
				struct Layout([u8; 4], #[cfg(test)] [u8; 1]);

				#[unsafe(link_section = "js_bindgen.assembly")]
				static CUSTOM_SECTION: Layout = Layout(
					::core::primitive::u32::to_le_bytes(LEN),
					#[cfg(test)]
					ARR0,
				);
			};
		},
	);
}

#[test]
fn cfg() {
	super::test(
		crate::unsafe_embed_asm(quote! {
		   "test1",
		   #[cfg(test)]
		   "test2",
		   "test3",
		}),
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

				#[unsafe(link_section = "js_bindgen.assembly")]
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
	super::test(
		crate::unsafe_embed_asm(quote! { "\n\t\"\\{{}}" }),
		quote! {
			const _: () = {
				const LEN0: usize = 6;
				const ARR0: [u8; LEN0] = *b"\n\t\"\\{}";

				const LEN: u32 = {
					let mut len = 0;
					{ len += LEN0; }
					len as u32
				};

				#[repr(C)]
				struct Layout([u8; 4], [u8; 6]);

				#[unsafe(link_section = "js_bindgen.assembly")]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR0);
			};
		},
	);
}

#[test]
fn interpolate() {
	super::test(
		crate::unsafe_embed_asm(quote! { "{}", interpolate "test" }),
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

				#[unsafe(link_section = "js_bindgen.assembly")]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR0);
			};
		},
	);
}

#[test]
fn interpolate_edge() {
	super::test(
		crate::unsafe_embed_asm(quote! {
			"{}",
			"{}",
			interpolate foo!(),
			interpolate <Foo<Bar::Baz> as Qux>::QUUX,
		}),
		quote! {
			const _: () = {
				const VAL0: &str = foo!();
				const LEN0: usize = ::core::primitive::str::len(VAL0);
				const PTR0: *const u8 = ::core::primitive::str::as_ptr(VAL0);
				const ARR0: [u8; LEN0] = unsafe { *(PTR0 as *const _) };
				const LEN1: usize = 1;
				const ARR1: [u8; LEN1] = *b"\n";
				const VAL2: &str = <Foo<Bar::Baz> as Qux>::QUUX;
				const LEN2: usize = ::core::primitive::str::len(VAL2);
				const PTR2: *const u8 = ::core::primitive::str::as_ptr(VAL2);
				const ARR2: [u8; LEN2] = unsafe { *(PTR2 as *const _) };

				const LEN: u32 = {
					let mut len = 0;
					{ len += LEN0; }
					{ len += LEN1; }
					{ len += LEN2; }
					len as u32
				};

				#[repr(C)]
				struct Layout([u8; 4], [u8; LEN0], [u8; 1], [u8; LEN2]);

				#[unsafe(link_section = "js_bindgen.assembly")]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR0, ARR1, ARR2);
			};
		},
	);
}

#[test]
fn import() {
	super::test(
		crate::js_import(quote! {
			name = "foo",
			"bar", "baz",
		}),
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

				#[unsafe(link_section = "js_bindgen.import.test_crate.foo")]
				static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR0);
			};
		},
	);
}
