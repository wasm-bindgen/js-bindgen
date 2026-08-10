#[test]
fn basic() {
	test!(
		{},
		{
			extern "js-sys" {
				pub fn log(data: &JsValue);
			}
		},
		{
			pub fn log(data: &JsValue) {
				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(
						arg0_0: ::js_sys::r#macro::InputSlot1<&JsValue>,
						arg0_1: ::js_sys::r#macro::InputSlot2<&JsValue>,
						arg0_2: ::js_sys::r#macro::InputSlot3<&JsValue>,
						arg0_3: ::js_sys::r#macro::InputSlot4<&JsValue>,
					);
				}

				{
					let (arg0_0, arg0_1, arg0_2, arg0_3) =
						::js_sys::r#macro::split_input::<&JsValue>(data);
					unsafe { log(arg0_0, arg0_1, arg0_2, arg0_3) }
				};
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"log",
						"test_crate.log",
						&[::js_sys::r#macro::import_input::<&JsValue>("arg0")],
						::core::option::Option::None,
						::core::option::Option::Some(::js_sys::r#macro::ImportJs {
							direct_wrapper: false,
							direct_call: "globalThis.log",
							indirect_call: "globalThis.log(arg0_0)",
							required_embeds: &[::js_sys::r#macro::js_input_embed::<&JsValue>()],
						}),
					)];
				const WAT_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_wat_capacity(IMPORTS);

				#[used]
				#[unsafe(link_section = "js_bindgen.wat")]
				static WAT_SECTION: ::js_sys::r#macro::ImportSection<WAT_CAPACITY> =
					::js_sys::r#macro::import_wat::<WAT_CAPACITY>(IMPORTS);
				const JS_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_js_capacity(IMPORTS);
				#[used]
				#[unsafe(link_section = "js_bindgen.import")]
				static JS_SECTION: ::js_sys::r#macro::ImportSection<JS_CAPACITY> =
					::js_sys::r#macro::import_js::<JS_CAPACITY>(IMPORTS);
			};
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(func $test_crate.log (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.log (@reloc)
		)",
		"globalThis.log",
	);
}

#[test]
fn namespace() {
	test!(
		{ namespace = "console" },
		{
			extern "js-sys" {
				pub fn log(data: &JsValue);
			}
		},
		{
			pub fn log(data: &JsValue) {
				unsafe extern "C" {
					#[link_name = "test_crate.console.log"]
					fn log(
						arg0_0: ::js_sys::r#macro::InputSlot1<&JsValue>,
						arg0_1: ::js_sys::r#macro::InputSlot2<&JsValue>,
						arg0_2: ::js_sys::r#macro::InputSlot3<&JsValue>,
						arg0_3: ::js_sys::r#macro::InputSlot4<&JsValue>,
					);
				}

				{
					let (arg0_0, arg0_1, arg0_2, arg0_3) =
						::js_sys::r#macro::split_input::<&JsValue>(data);
					unsafe { log(arg0_0, arg0_1, arg0_2, arg0_3) }
				};
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"console.log",
						"test_crate.console.log",
						&[::js_sys::r#macro::import_input::<&JsValue>("arg0")],
						::core::option::Option::None,
						::core::option::Option::Some(::js_sys::r#macro::ImportJs {
							direct_wrapper: true,
							direct_call: "globalThis.console.log(arg0_0)",
							indirect_call: "globalThis.console.log(arg0_0)",
							required_embeds: &[::js_sys::r#macro::js_input_embed::<&JsValue>()],
						}),
					)];
				const WAT_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_wat_capacity(IMPORTS);

				#[used]
				#[unsafe(link_section = "js_bindgen.wat")]
				static WAT_SECTION: ::js_sys::r#macro::ImportSection<WAT_CAPACITY> =
					::js_sys::r#macro::import_wat::<WAT_CAPACITY>(IMPORTS);
				const JS_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_js_capacity(IMPORTS);
				#[used]
				#[unsafe(link_section = "js_bindgen.import")]
				static JS_SECTION: ::js_sys::r#macro::ImportSection<JS_CAPACITY> =
					::js_sys::r#macro::import_js::<JS_CAPACITY>(IMPORTS);
			};
		},
		"(import \"test_crate\" \"console.log\" (func $test_crate.import.console.log (@sym (name \
		 \"test_crate.import.console.log\")) (param externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(func $test_crate.console.log (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.console.log (@reloc)
		)",
		"(arg0_0) => globalThis.console.log(arg0_0)",
	);
}

#[test]
fn js_sys() {
	test!(
		{ js_sys = js_sys },
		{
			extern "js-sys" {
				pub fn log(data: &JsValue);
			}
		},
		{
			pub fn log(data: &JsValue) {
				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(
						arg0_0: js_sys::r#macro::InputSlot1<&JsValue>,
						arg0_1: js_sys::r#macro::InputSlot2<&JsValue>,
						arg0_2: js_sys::r#macro::InputSlot3<&JsValue>,
						arg0_3: js_sys::r#macro::InputSlot4<&JsValue>,
					);
				}

				{
					let (arg0_0, arg0_1, arg0_2, arg0_3) =
						js_sys::r#macro::split_input::<&JsValue>(data);
					unsafe { log(arg0_0, arg0_1, arg0_2, arg0_3) }
				};
			}
			const _: () = {
				const IMPORTS: &[js_sys::r#macro::ImportDescriptor] =
					&[js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"log",
						"test_crate.log",
						&[js_sys::r#macro::import_input::<&JsValue>("arg0")],
						::core::option::Option::None,
						::core::option::Option::Some(js_sys::r#macro::ImportJs {
							direct_wrapper: false,
							direct_call: "globalThis.log",
							indirect_call: "globalThis.log(arg0_0)",
							required_embeds: &[js_sys::r#macro::js_input_embed::<&JsValue>()],
						}),
					)];
				const WAT_CAPACITY: ::core::primitive::usize =
					js_sys::r#macro::import_wat_capacity(IMPORTS);

				#[used]
				#[unsafe(link_section = "js_bindgen.wat")]
				static WAT_SECTION: js_sys::r#macro::ImportSection<WAT_CAPACITY> =
					js_sys::r#macro::import_wat::<WAT_CAPACITY>(IMPORTS);
				const JS_CAPACITY: ::core::primitive::usize =
					js_sys::r#macro::import_js_capacity(IMPORTS);
				#[used]
				#[unsafe(link_section = "js_bindgen.import")]
				static JS_SECTION: js_sys::r#macro::ImportSection<JS_CAPACITY> =
					js_sys::r#macro::import_js::<JS_CAPACITY>(IMPORTS);
			};
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(func $test_crate.log (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.log (@reloc)
		)",
		"globalThis.log",
	);
}

#[test]
fn two_parameters() {
	test!(
		{},
		{
			extern "js-sys" {
				pub fn log(data1: &JsValue, data2: &JsValue);
			}
		},
		{
			pub fn log(data1: &JsValue, data2: &JsValue) {
				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(
						arg0_0: ::js_sys::r#macro::InputSlot1<&JsValue>,
						arg0_1: ::js_sys::r#macro::InputSlot2<&JsValue>,
						arg0_2: ::js_sys::r#macro::InputSlot3<&JsValue>,
						arg0_3: ::js_sys::r#macro::InputSlot4<&JsValue>,
						arg1_0: ::js_sys::r#macro::InputSlot1<&JsValue>,
						arg1_1: ::js_sys::r#macro::InputSlot2<&JsValue>,
						arg1_2: ::js_sys::r#macro::InputSlot3<&JsValue>,
						arg1_3: ::js_sys::r#macro::InputSlot4<&JsValue>,
					);
				}

				{
					let (arg0_0, arg0_1, arg0_2, arg0_3) =
						::js_sys::r#macro::split_input::<&JsValue>(data1);
					let (arg1_0, arg1_1, arg1_2, arg1_3) =
						::js_sys::r#macro::split_input::<&JsValue>(data2);
					unsafe {
						log(
							arg0_0, arg0_1, arg0_2, arg0_3, arg1_0, arg1_1, arg1_2, arg1_3,
						)
					}
				};
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"log",
						"test_crate.log",
						&[
							::js_sys::r#macro::import_input::<&JsValue>("arg0"),
							::js_sys::r#macro::import_input::<&JsValue>("arg1"),
						],
						::core::option::Option::None,
						::core::option::Option::Some(::js_sys::r#macro::ImportJs {
							direct_wrapper: false,
							direct_call: "globalThis.log",
							indirect_call: "globalThis.log(arg0_0, arg1_0)",
							required_embeds: &[::js_sys::r#macro::js_input_embed::<&JsValue>()],
						}),
					)];
				const WAT_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_wat_capacity(IMPORTS);

				#[used]
				#[unsafe(link_section = "js_bindgen.wat")]
				static WAT_SECTION: ::js_sys::r#macro::ImportSection<WAT_CAPACITY> =
					::js_sys::r#macro::import_wat::<WAT_CAPACITY>(IMPORTS);
				const JS_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_js_capacity(IMPORTS);
				#[used]
				#[unsafe(link_section = "js_bindgen.import")]
				static JS_SECTION: ::js_sys::r#macro::ImportSection<JS_CAPACITY> =
					::js_sys::r#macro::import_js::<JS_CAPACITY>(IMPORTS);
			};
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(func $test_crate.log (@sym) (param $arg0_0 i32) (param $arg1_0 i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  local.get $arg1_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.log (@reloc)
		)",
		"globalThis.log",
	);
}

#[test]
fn groups_functions_and_shared_wat_imports() {
	test!(
		{},
		{
			extern "js-sys" {
				#[js_sys(js_import)]
				pub fn first(value: &JsValue);

				#[js_sys(js_import)]
				pub fn second(value: &JsValue);
			}
		},
		{
			pub fn first(value: &JsValue) {
				unsafe extern "C" {
					#[link_name = "test_crate.first"]
					fn first(
						arg0_0: ::js_sys::r#macro::InputSlot1<&JsValue>,
						arg0_1: ::js_sys::r#macro::InputSlot2<&JsValue>,
						arg0_2: ::js_sys::r#macro::InputSlot3<&JsValue>,
						arg0_3: ::js_sys::r#macro::InputSlot4<&JsValue>,
					);
				}

				{
					let (arg0_0, arg0_1, arg0_2, arg0_3) =
						::js_sys::r#macro::split_input::<&JsValue>(value);
					unsafe { first(arg0_0, arg0_1, arg0_2, arg0_3) }
				};
			}

			pub fn second(value: &JsValue) {
				unsafe extern "C" {
					#[link_name = "test_crate.second"]
					fn second(
						arg0_0: ::js_sys::r#macro::InputSlot1<&JsValue>,
						arg0_1: ::js_sys::r#macro::InputSlot2<&JsValue>,
						arg0_2: ::js_sys::r#macro::InputSlot3<&JsValue>,
						arg0_3: ::js_sys::r#macro::InputSlot4<&JsValue>,
					);
				}

				{
					let (arg0_0, arg0_1, arg0_2, arg0_3) =
						::js_sys::r#macro::split_input::<&JsValue>(value);
					unsafe { second(arg0_0, arg0_1, arg0_2, arg0_3) }
				};
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] = &[
					::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"first",
						"test_crate.first",
						&[::js_sys::r#macro::import_input::<&JsValue>("arg0")],
						::core::option::Option::None,
						::core::option::Option::None,
					),
					::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"second",
						"test_crate.second",
						&[::js_sys::r#macro::import_input::<&JsValue>("arg0")],
						::core::option::Option::None,
						::core::option::Option::None,
					),
				];
				const WAT_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_wat_capacity(IMPORTS);

				#[used]
				#[unsafe(link_section = "js_bindgen.wat")]
				static WAT_SECTION: ::js_sys::r#macro::ImportSection<WAT_CAPACITY> =
					::js_sys::r#macro::import_wat::<WAT_CAPACITY>(IMPORTS);
			};
		},
		"(import \"test_crate\" \"first\" (func $test_crate.import.first (@sym (name \
		 \"test_crate.import.first\")) (param externref)))
		(import \"test_crate\" \"second\" (func $test_crate.import.second (@sym (name \
		 \"test_crate.import.second\")) (param externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(func $test_crate.first (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.first (@reloc)
		)
		(func $test_crate.second (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.second (@reloc)
		)",
		None,
	);
}

#[test]
fn empty() {
	test!(
		{},
		{
			extern "js-sys" {
				pub fn log();
			}
		},
		{
			pub fn log() {
				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log();
				}

				{
					unsafe { log() }
				};
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"log",
						"test_crate.log",
						&[],
						::core::option::Option::None,
						::core::option::Option::Some(::js_sys::r#macro::ImportJs {
							direct_wrapper: false,
							direct_call: "globalThis.log",
							indirect_call: "globalThis.log()",
							required_embeds: &[],
						}),
					)];
				const WAT_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_wat_capacity(IMPORTS);

				#[used]
				#[unsafe(link_section = "js_bindgen.wat")]
				static WAT_SECTION: ::js_sys::r#macro::ImportSection<WAT_CAPACITY> =
					::js_sys::r#macro::import_wat::<WAT_CAPACITY>(IMPORTS);
				const JS_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_js_capacity(IMPORTS);
				#[used]
				#[unsafe(link_section = "js_bindgen.import")]
				static JS_SECTION: ::js_sys::r#macro::ImportSection<JS_CAPACITY> =
					::js_sys::r#macro::import_js::<JS_CAPACITY>(IMPORTS);
			};
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\"))))
		(func $test_crate.log (@sym)
		  call $test_crate.import.log (@reloc)
		)",
		"globalThis.log",
	);
}

#[test]
fn js_name() {
	test!(
		{},
		{
			extern "js-sys" {
				#[js_sys(js_name = "log")]
				pub fn logx(data: &JsValue);
			}
		},
		{
			pub fn logx(data: &JsValue) {
				unsafe extern "C" {
					#[link_name = "test_crate.logx"]
					fn logx(
						arg0_0: ::js_sys::r#macro::InputSlot1<&JsValue>,
						arg0_1: ::js_sys::r#macro::InputSlot2<&JsValue>,
						arg0_2: ::js_sys::r#macro::InputSlot3<&JsValue>,
						arg0_3: ::js_sys::r#macro::InputSlot4<&JsValue>,
					);
				}

				{
					let (arg0_0, arg0_1, arg0_2, arg0_3) =
						::js_sys::r#macro::split_input::<&JsValue>(data);
					unsafe { logx(arg0_0, arg0_1, arg0_2, arg0_3) }
				};
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"logx",
						"test_crate.logx",
						&[::js_sys::r#macro::import_input::<&JsValue>("arg0")],
						::core::option::Option::None,
						::core::option::Option::Some(::js_sys::r#macro::ImportJs {
							direct_wrapper: false,
							direct_call: "globalThis.log",
							indirect_call: "globalThis.log(arg0_0)",
							required_embeds: &[::js_sys::r#macro::js_input_embed::<&JsValue>()],
						}),
					)];
				const WAT_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_wat_capacity(IMPORTS);

				#[used]
				#[unsafe(link_section = "js_bindgen.wat")]
				static WAT_SECTION: ::js_sys::r#macro::ImportSection<WAT_CAPACITY> =
					::js_sys::r#macro::import_wat::<WAT_CAPACITY>(IMPORTS);
				const JS_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_js_capacity(IMPORTS);
				#[used]
				#[unsafe(link_section = "js_bindgen.import")]
				static JS_SECTION: ::js_sys::r#macro::ImportSection<JS_CAPACITY> =
					::js_sys::r#macro::import_js::<JS_CAPACITY>(IMPORTS);
			};
		},
		"(import \"test_crate\" \"logx\" (func $test_crate.import.logx (@sym (name \
		 \"test_crate.import.logx\")) (param externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(func $test_crate.logx (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.logx (@reloc)
		)",
		"globalThis.log",
	);
}

#[test]
fn js_import() {
	test!(
		{},
		{
			extern "js-sys" {
				#[js_sys(js_import)]
				pub fn log(data: &JsValue);
			}
		},
		{
			pub fn log(data: &JsValue) {
				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(
						arg0_0: ::js_sys::r#macro::InputSlot1<&JsValue>,
						arg0_1: ::js_sys::r#macro::InputSlot2<&JsValue>,
						arg0_2: ::js_sys::r#macro::InputSlot3<&JsValue>,
						arg0_3: ::js_sys::r#macro::InputSlot4<&JsValue>,
					);
				}

				{
					let (arg0_0, arg0_1, arg0_2, arg0_3) =
						::js_sys::r#macro::split_input::<&JsValue>(data);
					unsafe { log(arg0_0, arg0_1, arg0_2, arg0_3) }
				};
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"log",
						"test_crate.log",
						&[::js_sys::r#macro::import_input::<&JsValue>("arg0")],
						::core::option::Option::None,
						::core::option::Option::None,
					)];
				const WAT_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_wat_capacity(IMPORTS);

				#[used]
				#[unsafe(link_section = "js_bindgen.wat")]
				static WAT_SECTION: ::js_sys::r#macro::ImportSection<WAT_CAPACITY> =
					::js_sys::r#macro::import_wat::<WAT_CAPACITY>(IMPORTS);
			};
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(func $test_crate.log (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.log (@reloc)
		)",
		None,
	);
}

#[test]
fn js_embed() {
	test!(
		{},
		{
			extern "js-sys" {
				#[js_sys(js_embed = "embed")]
				pub fn log(data: &JsValue);
			}
		},
		{
			pub fn log(data: &JsValue) {
				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(
						arg0_0: ::js_sys::r#macro::InputSlot1<&JsValue>,
						arg0_1: ::js_sys::r#macro::InputSlot2<&JsValue>,
						arg0_2: ::js_sys::r#macro::InputSlot3<&JsValue>,
						arg0_3: ::js_sys::r#macro::InputSlot4<&JsValue>,
					);
				}

				{
					let (arg0_0, arg0_1, arg0_2, arg0_3) =
						::js_sys::r#macro::split_input::<&JsValue>(data);
					unsafe { log(arg0_0, arg0_1, arg0_2, arg0_3) }
				};
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"log",
						"test_crate.log",
						&[::js_sys::r#macro::import_input::<&JsValue>("arg0")],
						::core::option::Option::None,
						::core::option::Option::Some(::js_sys::r#macro::ImportJs {
							direct_wrapper: false,
							direct_call: "this.#jsEmbed.test_crate['embed']",
							indirect_call: "this.#jsEmbed.test_crate['embed'](arg0_0)",
							required_embeds: &[
								("test_crate", "embed"),
								::js_sys::r#macro::js_input_embed::<&JsValue>(),
							],
						}),
					)];
				const WAT_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_wat_capacity(IMPORTS);

				#[used]
				#[unsafe(link_section = "js_bindgen.wat")]
				static WAT_SECTION: ::js_sys::r#macro::ImportSection<WAT_CAPACITY> =
					::js_sys::r#macro::import_wat::<WAT_CAPACITY>(IMPORTS);
				const JS_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_js_capacity(IMPORTS);
				#[used]
				#[unsafe(link_section = "js_bindgen.import")]
				static JS_SECTION: ::js_sys::r#macro::ImportSection<JS_CAPACITY> =
					::js_sys::r#macro::import_js::<JS_CAPACITY>(IMPORTS);
			};
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(func $test_crate.log (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.log (@reloc)
		)",
		"this.#jsEmbed.test_crate['embed']",
	);
}

#[test]
fn r#return() {
	test!(
		{},
		{
			extern "js-sys" {
				pub fn is_nan() -> JsValue;
			}
		},
		{
			pub fn is_nan() -> JsValue {
				unsafe extern "C" {
					#[link_name = "test_crate.is_nan"]
					fn is_nan() -> ::js_sys::r#macro::OutputRet<JsValue>;
				}

				::js_sys::r#macro::join_output({ unsafe { is_nan() } })
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"is_nan",
						"test_crate.is_nan",
						&[],
						::core::option::Option::Some(::js_sys::r#macro::import_output::<JsValue>()),
						::core::option::Option::Some(::js_sys::r#macro::ImportJs {
							direct_wrapper: false,
							direct_call: "globalThis.is_nan",
							indirect_call: "globalThis.is_nan()",
							required_embeds: &[
								::js_sys::r#macro::js_output_embed::<JsValue>(),
								::js_sys::r#macro::js_result_embed::<JsValue>(),
							],
						}),
					)];
				const WAT_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_wat_capacity(IMPORTS);

				#[used]
				#[unsafe(link_section = "js_bindgen.wat")]
				static WAT_SECTION: ::js_sys::r#macro::ImportSection<WAT_CAPACITY> =
					::js_sys::r#macro::import_wat::<WAT_CAPACITY>(IMPORTS);
				const JS_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_js_capacity(IMPORTS);
				#[used]
				#[unsafe(link_section = "js_bindgen.import")]
				static JS_SECTION: ::js_sys::r#macro::ImportSection<JS_CAPACITY> =
					::js_sys::r#macro::import_js::<JS_CAPACITY>(IMPORTS);
			};
		},
		"(import \"test_crate\" \"is_nan\" (func $test_crate.import.is_nan (@sym (name \
		 \"test_crate.import.is_nan\")) (result externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(import \"env\" \"js_sys.externref.next\" (func $js_sys.externref.next (@sym) (result i32)))
		(func $test_crate.is_nan (@sym) (result i32)
		  (local $js_sys.externref.value externref)
		  (local $js_sys.externref.index i32)
		  call $test_crate.import.is_nan (@reloc)
		  local.set $js_sys.externref.value
		  call $js_sys.externref.next (@reloc)
		  local.tee $js_sys.externref.index
		  local.get $js_sys.externref.value
		  table.set $js_sys.import.externref.table (@reloc)
		  local.get $js_sys.externref.index
		)",
		"globalThis.is_nan",
	);
}

#[test]
fn cfg() {
	test!(
		{},
		{
			extern "js-sys" {
				#[cfg(all())]
				pub fn log();
			}
		},
		{
			#[cfg(all())]
			pub fn log() {
				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log();
				}

				{
					unsafe { log() }
				};
			}

			#[cfg(all())]
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"log",
						"test_crate.log",
						&[],
						::core::option::Option::None,
						::core::option::Option::Some(::js_sys::r#macro::ImportJs {
							direct_wrapper: false,
							direct_call: "globalThis.log",
							indirect_call: "globalThis.log()",
							required_embeds: &[],
						}),
					)];
				const WAT_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_wat_capacity(IMPORTS);
				#[used]
				#[unsafe(link_section = "js_bindgen.wat")]
				static WAT_SECTION: ::js_sys::r#macro::ImportSection<WAT_CAPACITY> =
					::js_sys::r#macro::import_wat::<WAT_CAPACITY>(IMPORTS);
				const JS_CAPACITY: ::core::primitive::usize =
					::js_sys::r#macro::import_js_capacity(IMPORTS);
				#[used]
				#[unsafe(link_section = "js_bindgen.import")]
				static JS_SECTION: ::js_sys::r#macro::ImportSection<JS_CAPACITY> =
					::js_sys::r#macro::import_js::<JS_CAPACITY>(IMPORTS);
			};
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\"))))
		(func $test_crate.log (@sym)
		  call $test_crate.import.log (@reloc)
		)",
		"globalThis.log",
	);
}

#[test]
fn preserves_successful_functions_after_an_error() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			pub fn good(value: i32) -> i32;
			pub async fn bad();
		}
	};
	let (Some(output), error) =
		crate::r#macro::expand_for_test(proc_macro2::TokenStream::new(), input, "test_crate")
			.unwrap_err()
	else {
		panic!("expected the successful function to be preserved");
	};

	let mut output = output.into_token_stream();
	output.extend(error.into_compile_error());
	let output = output.to_string();

	assert!(output.contains("pub fn good"));
	assert!(output.contains("\"test_crate.good\""));
	assert_eq!(output.matches("ImportDescriptor :: new").count(), 1);
	assert!(!output.contains("fn bad"));
	assert!(output.contains("compile_error"));
	assert!(output.contains("`async` functions are not supported"));
}

#[test]
fn requires_js_sys_abi() {
	let input = syn::parse_quote! {
		extern {
			pub fn log();
		}
	};

	assert_eq!(super::macro_error(input), "expected `js-sys` ABI");
}

#[test]
fn incompatible_binding_options_are_rejected() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(js_name = "renamed", js_import)]
			pub fn log();
		}
	};

	assert_eq!(
		super::macro_error(input),
		"`js_import` and `js_embed` cannot be combined with JavaScript binding options"
	);
}

#[test]
fn duplicate_parameter_abi_override_is_rejected() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			pub fn log(
				#[js_sys(type = i32)]
				#[js_sys(type = u32)]
				value: i32,
			);
		}
	};

	assert_eq!(super::macro_error(input), "duplicate attribute");
}
