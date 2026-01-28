use quote::quote;

#[test]
fn basic() {
	super::test(
		crate::unsafe_embed_asm(quote! { "foo", "bar" }),
		quote! {
			const _: () = {
				const ARR_0: [u8; 7] = *b"foo\nbar";

				const LEN: u32 = {
					let mut len: usize = 0;
					{ len += 7; }
					len as u32
				};

				const _: () = {
					#[repr(C)]
					struct Layout([u8; 4], [u8; 7]);

					#[unsafe(link_section = "js_bindgen.assembly")]
					static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0);
				};
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
					let mut len: usize = 0;
					len as u32
				};

				const _: () = {
					#[repr(C)]
					struct Layout([u8; 4]);

					#[unsafe(link_section = "js_bindgen.assembly")]
					static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN));
				};
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
				const ARR_0: [u8; 3] = *b"foo";

				const LEN: u32 = {
					let mut len: usize = 0;
					{ len += 3; }
					len as u32
				};

				const _: () = {
					#[repr(C)]
					struct Layout([u8; 4], [u8; 3]);

					#[unsafe(link_section = "js_bindgen.assembly")]
					static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0);
				};
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
				const ARR_0: [u8; 12] = *b"foo\nbar\nbaz\n";
				#[cfg(test)]
				const ARR_1: [u8; 4] = *b"qux\n";
				const ARR_2: [u8; 17] = *b"quux\ncorge\ngrault";

				const LEN: u32 = {
					let mut len: usize = 0;
					{ len += 12; }
					#[cfg(test)]
					{ len += 4; }
					{ len += 17; }
					len as u32
				};

				const _: () = {
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
						ARR_0,
						#[cfg(test)]
						ARR_1,
						ARR_2,
					);
				};
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
				const ARR_0: [u8; 1] = *b"\n";
				#[cfg(test)]
				const ARR_1: [u8; 1] = *b"\n";
				const ARR_2: [u8; 4] = *b"foo\n";
				const VAL_3: &str = &Bar;
				const LEN_3: usize = ::core::primitive::str::len(VAL_3);
				const PTR_3: *const u8 = ::core::primitive::str::as_ptr(VAL_3);
				const ARR_3: [u8; LEN_3] = unsafe { *(PTR_3 as *const _) };

				const LEN: u32 = {
					let mut len: usize = 0;
					{ len += 1; }
					#[cfg(test)]
					{ len += 1; }
					{ len += 4; }
					{ len += LEN_3; }
					len as u32
				};

				const _: () = {
					#[repr(C)]
					struct Layout(
						[u8; 4],
						[u8; 1],
						#[cfg(test)]
						[u8; 1],
						[u8; 4],
						[u8; LEN_3],
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
				const ARR_0: [u8; 1] = *b"\n";

				const LEN: u32 = {
					let mut len: usize = 0;
					#[cfg(test)]
					{ len += 1; }
					len as u32
				};

				const _: () = {
					#[repr(C)]
					struct Layout([u8; 4], #[cfg(test)] [u8; 1]);

					#[unsafe(link_section = "js_bindgen.assembly")]
					static CUSTOM_SECTION: Layout = Layout(
						::core::primitive::u32::to_le_bytes(LEN),
						#[cfg(test)]
						ARR_0,
					);
				};
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
				const ARR_0: [u8; 6] = *b"test1\n";
				#[cfg(test)]
				const ARR_1: [u8; 6] = *b"test2\n";
				const ARR_2: [u8; 5] = *b"test3";

				const LEN: u32 = {
					let mut len: usize = 0;
					{ len += 6; }
					#[cfg(test)] { len += 6; }
					{ len += 5; }
					len as u32
				};

				const _: () = {
					#[repr(C)]
					struct Layout([u8; 4], [u8; 6], #[cfg(test)] [u8; 6], [u8; 5]);

					#[unsafe(link_section = "js_bindgen.assembly")]
					static CUSTOM_SECTION: Layout = Layout(
						::core::primitive::u32::to_le_bytes(LEN),
						ARR_0,
						#[cfg(test)]
						ARR_1,
						ARR_2,
					);
				};
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
				const ARR_0: [u8; 6] = *b"\n\t\"\\{}";

				const LEN: u32 = {
					let mut len: usize = 0;
					{ len += 6; }
					len as u32
				};

				const _: () = {
					#[repr(C)]
					struct Layout([u8; 4], [u8; 6]);

					#[unsafe(link_section = "js_bindgen.assembly")]
					static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0);
				};
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
				const VAL_0: &str = "test";
				const LEN_0: usize = ::core::primitive::str::len(VAL_0);
				const PTR_0: *const u8 = ::core::primitive::str::as_ptr(VAL_0);
				const ARR_0: [u8; LEN_0] = unsafe { *(PTR_0 as *const _) };

				const LEN: u32 = {
					let mut len: usize = 0;
					{ len += LEN_0; }
					len as u32
				};

				const _: () = {
					#[repr(C)]
					struct Layout([u8; 4], [u8; LEN_0]);

					#[unsafe(link_section = "js_bindgen.assembly")]
					static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0);
				};
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
				const VAL_0: &str = foo!();
				const LEN_0: usize = ::core::primitive::str::len(VAL_0);
				const PTR_0: *const u8 = ::core::primitive::str::as_ptr(VAL_0);
				const ARR_0: [u8; LEN_0] = unsafe { *(PTR_0 as *const _) };
				const ARR_1: [u8; 1] = *b"\n";
				const VAL_2: &str = <Foo<Bar::Baz> as Qux>::QUUX;
				const LEN_2: usize = ::core::primitive::str::len(VAL_2);
				const PTR_2: *const u8 = ::core::primitive::str::as_ptr(VAL_2);
				const ARR_2: [u8; LEN_2] = unsafe { *(PTR_2 as *const _) };

				const LEN: u32 = {
					let mut len: usize = 0;
					{ len += LEN_0; }
					{ len += 1; }
					{ len += LEN_2; }
					len as u32
				};

				const _: () = {
					#[repr(C)]
					struct Layout([u8; 4], [u8; LEN_0], [u8; 1], [u8; LEN_2]);

					#[unsafe(link_section = "js_bindgen.assembly")]
					static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), ARR_0, ARR_1, ARR_2);
				};
			};
		},
	);
}

#[test]
fn import() {
	super::test(
		crate::import_js(quote! {
			name = "foo",
			"bar", "baz",
		}),
		quote! {
			const _: () = {
				const ARR_0: [u8; 7] = *b"bar\nbaz";

				const LEN: u32 = {
					let mut len: usize = 0;
					{ len += 7; }
					len as u32
				};

				const _: () = {
					#[repr(C)]
					struct Layout([u8; 4], [u8; 2], [u8; 7]);

					#[unsafe(link_section = "js_bindgen.import.test_crate.foo")]
					static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), [0, 0], ARR_0);
				};
			};
		},
	);
}

#[test]
fn required_embed() {
	super::test(
		crate::import_js(quote! {
			name = "foo", required_embed = "bar", "",
		}),
		quote! {
			const _: () = {
				const ARR_PREFIX: [u8; 3] = *b"bar";
				const LEN: u32 = {
					let mut len: usize = 0;
					len as u32
				};

				const _: () = {
					#[repr(C)]
					struct Layout([u8; 4], [u8; 2], [u8; 3]);

					#[unsafe(link_section = "js_bindgen.import.test_crate.foo")]
					static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN), [3, 0], ARR_PREFIX);
				};
			};
		},
	);
}

#[test]
fn embed() {
	super::test(
		crate::embed_js(quote! {
			name = "foo", "",
		}),
		quote! {
			const _: () = {
				const LEN: u32 = {
					let mut len: usize = 0;
					len as u32
				};

				const _: () = {
					#[repr(C)]
					struct Layout([u8; 4]);

					#[unsafe(link_section = "js_bindgen.js.test_crate.foo")]
					static CUSTOM_SECTION: Layout = Layout(::core::primitive::u32::to_le_bytes(LEN));
				};
			};
		},
	);
}
