fn generated_items(input: syn::ItemForeignMod) -> Vec<syn::Item> {
	crate::r#macro::expand_for_test(proc_macro2::TokenStream::new(), input, "test_crate")
		.unwrap()
		.into_items()
		.unwrap()
}

fn generated_rust(input: syn::ItemForeignMod) -> String {
	let output = generated_items(input);
	prettyplease::unparse(&syn::File {
		shebang: None,
		attrs: Vec::new(),
		items: output,
	})
}

fn generated_js(input: syn::ItemForeignMod) -> String {
	let output = generated_rust(input);
	let dir = tempfile::tempdir().unwrap();
	let (_, js, _) = super::inner(dir.path(), &output).unwrap();
	js.unwrap()
}

#[test]
fn method() {
	test!(
		{},
		{
			extern "js-sys" {
				pub fn test(self: &JsTest);
			}
		},
		{
			impl JsTest {
				pub fn test(self: &JsTest) {
					unsafe extern "C" {
						#[link_name = "test_crate.JsTest.test"]
						fn test(
							arg0_0: ::js_sys::r#macro::InputSlot1<&::js_sys::JsValue>,
							arg0_1: ::js_sys::r#macro::InputSlot2<&::js_sys::JsValue>,
							arg0_2: ::js_sys::r#macro::InputSlot3<&::js_sys::JsValue>,
							arg0_3: ::js_sys::r#macro::InputSlot4<&::js_sys::JsValue>,
						);
					}

					{
						let (arg0_0, arg0_1, arg0_2, arg0_3) = unsafe {
							::js_sys::r#macro::split_input_as::<&::js_sys::JsValue>(self)
						};
						unsafe { test(arg0_0, arg0_1, arg0_2, arg0_3) }
					};
				}
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"JsTest.test",
						"test_crate.JsTest.test",
						&[::js_sys::r#macro::import_input::<&::js_sys::JsValue>(
							"arg0",
						)],
						::core::option::Option::None,
						::core::option::Option::Some(::js_sys::r#macro::ImportJs {
							direct_wrapper: true,
							direct_call: "arg0_0.test()",
							indirect_call: "arg0_0.test()",
							required_embeds: &[::js_sys::r#macro::js_input_embed::<
								&::js_sys::JsValue,
							>()],
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
		"(import \"test_crate\" \"JsTest.test\" (func $test_crate.import.JsTest.test (@sym (name \
		 \"test_crate.import.JsTest.test\")) (param externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(func $test_crate.JsTest.test (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.JsTest.test (@reloc)
		)",
		"(arg0_0) => arg0_0.test()",
	);
}

#[test]
fn method_par() {
	test!(
		{},
		{
			extern "js-sys" {
				pub fn test(self: &JsTest, par1: &JsValue, par2: &JsValue);
			}
		},
		{
			impl JsTest {
				pub fn test(self: &JsTest, par1: &JsValue, par2: &JsValue) {
					unsafe extern "C" {
						#[link_name = "test_crate.JsTest.test"]
						fn test(
							arg0_0: ::js_sys::r#macro::InputSlot1<&::js_sys::JsValue>,
							arg0_1: ::js_sys::r#macro::InputSlot2<&::js_sys::JsValue>,
							arg0_2: ::js_sys::r#macro::InputSlot3<&::js_sys::JsValue>,
							arg0_3: ::js_sys::r#macro::InputSlot4<&::js_sys::JsValue>,
							arg1_0: ::js_sys::r#macro::InputSlot1<&JsValue>,
							arg1_1: ::js_sys::r#macro::InputSlot2<&JsValue>,
							arg1_2: ::js_sys::r#macro::InputSlot3<&JsValue>,
							arg1_3: ::js_sys::r#macro::InputSlot4<&JsValue>,
							arg2_0: ::js_sys::r#macro::InputSlot1<&JsValue>,
							arg2_1: ::js_sys::r#macro::InputSlot2<&JsValue>,
							arg2_2: ::js_sys::r#macro::InputSlot3<&JsValue>,
							arg2_3: ::js_sys::r#macro::InputSlot4<&JsValue>,
						);
					}

					{
						let (arg0_0, arg0_1, arg0_2, arg0_3) = unsafe {
							::js_sys::r#macro::split_input_as::<&::js_sys::JsValue>(self)
						};
						let (arg1_0, arg1_1, arg1_2, arg1_3) =
							::js_sys::r#macro::split_input::<&JsValue>(par1);
						let (arg2_0, arg2_1, arg2_2, arg2_3) =
							::js_sys::r#macro::split_input::<&JsValue>(par2);
						unsafe {
							test(
								arg0_0, arg0_1, arg0_2, arg0_3, arg1_0, arg1_1, arg1_2, arg1_3,
								arg2_0, arg2_1, arg2_2, arg2_3,
							)
						}
					};
				}
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"JsTest.test",
						"test_crate.JsTest.test",
						&[
							::js_sys::r#macro::import_input::<&::js_sys::JsValue>("arg0"),
							::js_sys::r#macro::import_input::<&JsValue>("arg1"),
							::js_sys::r#macro::import_input::<&JsValue>("arg2"),
						],
						::core::option::Option::None,
						::core::option::Option::Some(::js_sys::r#macro::ImportJs {
							direct_wrapper: true,
							direct_call: "arg0_0.test(arg1_0, arg2_0)",
							indirect_call: "arg0_0.test(arg1_0, arg2_0)",
							required_embeds: &[
								::js_sys::r#macro::js_input_embed::<&::js_sys::JsValue>(),
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
		"(import \"test_crate\" \"JsTest.test\" (func $test_crate.import.JsTest.test (@sym (name \
		 \"test_crate.import.JsTest.test\")) (param externref externref externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(func $test_crate.JsTest.test (@sym) (param $arg0_0 i32) (param $arg1_0 i32) (param $arg2_0 i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  local.get $arg1_0
		  table.get $js_sys.import.externref.table (@reloc)
		  local.get $arg2_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.JsTest.test (@reloc)
		)",
		"(arg0_0, arg1_0, arg2_0) => arg0_0.test(arg1_0, arg2_0)",
	);
}

#[test]
fn variadic() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(variadic)]
			pub fn push(self: &JsTest, first: &JsValue, rest: &[JsValue]);
		}
	};

	assert_eq!(
		generated_js(input),
		"(arg0_0, arg1_0, arg2_0, arg2_1) => {\n    arg2_0 = \
		 this.#jsEmbed.js_sys['array.js_value.decode'](arg2_0, arg2_1)\narg0_0.push(arg1_0, \
		 ...arg2_0)\n}"
	);
}

#[test]
fn global_variadic() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(variadic)]
			pub fn call(values: &JsArray);
		}
	};

	assert_eq!(
		generated_js(input),
		"(arg0_0) => globalThis.call(...arg0_0)"
	);
}

#[test]
fn type_js_name() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(constructor)]
			pub fn new() -> RustType;

			#[js_sys(static_of = RustType, js_name = "create")]
			pub fn create();

			#[js_sys(js_name = "JavaScriptType")]
			pub type RustType;
		}
	};

	let output = generated_rust(input);
	assert!(output.contains(r#"direct_call: "new globalThis.JavaScriptType()""#));
	assert!(output.contains(r#"direct_call: "globalThis.JavaScriptType.create()""#));
}

#[test]
fn constructor_uses_return_owner() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(constructor)]
			pub fn new() -> Result<RustType, JsValue>;

			#[js_sys(js_name = "JavaScriptType")]
			pub type RustType;
		}
	};

	let output = generated_items(input);
	let output = prettyplease::unparse(&syn::File {
		shebang: None,
		attrs: Vec::new(),
		items: output,
	});
	assert!(output.contains(r#"direct_call: "new globalThis.JavaScriptType()""#));
}

#[test]
fn static_properties_use_the_type_js_name() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(static_of = RustType, getter = "value")]
			pub fn value() -> i32;

			#[js_sys(static_of = RustType, setter = "value")]
			pub fn set_value(value: i32);

			#[js_sys(js_name = "JavaScriptType")]
			pub type RustType;
		}
	};

	let output = generated_rust(input);
	assert!(output.contains(r#"direct_call: "globalThis.JavaScriptType.value""#));
	assert!(output.contains(r#"direct_call: "globalThis.JavaScriptType.value = arg0_0""#));
}

#[test]
fn variadic_requires_an_argument() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(variadic)]
			pub fn call();
		}
	};

	assert_eq!(
		super::macro_error(input),
		"`variadic` requires at least one argument"
	);
}

#[test]
fn named_getter() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(getter = "value")]
			pub fn renamed(self: &JsTest) -> i32;
		}
	};

	let output = generated_rust(input);
	assert!(output.contains(r#"direct_call: "arg0_0.value""#));
	assert!(output.contains(r#"indirect_call: "arg0_0.value""#));
}

#[test]
fn named_setter() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(setter = "value")]
			pub fn renamed(self: &JsTest, value: i32);
		}
	};

	let output = generated_rust(input);
	assert!(output.contains(r#"direct_call: "arg0_0.value = arg1_0""#));
	assert!(output.contains(r#"indirect_call: "arg0_0.value = arg1_0""#));
}

#[test]
fn inferred_setter() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(setter)]
			pub fn set_value(self: &JsTest, value: i32);
		}
	};

	let output = generated_rust(input);
	assert!(output.contains(r#"direct_call: "arg0_0.value = arg1_0""#));
	assert!(output.contains(r#"indirect_call: "arg0_0.value = arg1_0""#));
}

#[test]
fn setter_requires_a_field_name() {
	let input = syn::parse_quote! {
		extern "js-sys" {
			#[js_sys(setter)]
			pub fn update(self: &JsTest, value: i32);
		}
	};
	let (_, error) =
		crate::r#macro::expand_for_test(proc_macro2::TokenStream::new(), input, "test_crate")
			.unwrap_err();

	assert_eq!(
		error.to_string(),
		"`setter` cannot infer a field name; use `setter = \"field\"`"
	);
}

#[test]
fn getter() {
	test!(
		{},
		{
			extern "js-sys" {
				#[js_sys(getter)]
				pub fn test(self: &JsTest) -> JsValue;
			}
		},
		{
			impl JsTest {
				pub fn test(self: &JsTest) -> JsValue {
					unsafe extern "C" {
						#[link_name = "test_crate.JsTest.test"]
						fn test(
							arg0_0: ::js_sys::r#macro::InputSlot1<&::js_sys::JsValue>,
							arg0_1: ::js_sys::r#macro::InputSlot2<&::js_sys::JsValue>,
							arg0_2: ::js_sys::r#macro::InputSlot3<&::js_sys::JsValue>,
							arg0_3: ::js_sys::r#macro::InputSlot4<&::js_sys::JsValue>,
						) -> ::js_sys::r#macro::OutputRet<JsValue>;
					}

					::js_sys::r#macro::join_output({
						let (arg0_0, arg0_1, arg0_2, arg0_3) = unsafe {
							::js_sys::r#macro::split_input_as::<&::js_sys::JsValue>(self)
						};
						unsafe { test(arg0_0, arg0_1, arg0_2, arg0_3) }
					})
				}
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"JsTest.test",
						"test_crate.JsTest.test",
						&[::js_sys::r#macro::import_input::<&::js_sys::JsValue>(
							"arg0",
						)],
						::core::option::Option::Some(::js_sys::r#macro::import_output::<JsValue>()),
						::core::option::Option::Some(::js_sys::r#macro::ImportJs {
							direct_wrapper: true,
							direct_call: "arg0_0.test",
							indirect_call: "arg0_0.test",
							required_embeds: &[
								::js_sys::r#macro::js_input_embed::<&::js_sys::JsValue>(),
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
		"(import \"test_crate\" \"JsTest.test\" (func $test_crate.import.JsTest.test (@sym (name \
		 \"test_crate.import.JsTest.test\")) (param externref) (result externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(import \"env\" \"js_sys.externref.next\" (func $js_sys.externref.next (@sym) (result i32)))
		(func $test_crate.JsTest.test (@sym) (param $arg0_0 i32) (result i32)
		  (local $js_sys.externref.value externref)
		  (local $js_sys.externref.index i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.JsTest.test (@reloc)
		  local.set $js_sys.externref.value
		  call $js_sys.externref.next (@reloc)
		  local.tee $js_sys.externref.index
		  local.get $js_sys.externref.value
		  table.set $js_sys.import.externref.table (@reloc)
		  local.get $js_sys.externref.index
		)",
		"(arg0_0) => arg0_0.test",
	);
}

#[test]
fn setter() {
	test!(
		{},
		{
			extern "js-sys" {
				#[js_sys(setter = "test")]
				pub fn test(self: &JsTest, value: &JsValue);
			}
		},
		{
			impl JsTest {
				pub fn test(self: &JsTest, value: &JsValue) {
					unsafe extern "C" {
						#[link_name = "test_crate.JsTest.test"]
						fn test(
							arg0_0: ::js_sys::r#macro::InputSlot1<&::js_sys::JsValue>,
							arg0_1: ::js_sys::r#macro::InputSlot2<&::js_sys::JsValue>,
							arg0_2: ::js_sys::r#macro::InputSlot3<&::js_sys::JsValue>,
							arg0_3: ::js_sys::r#macro::InputSlot4<&::js_sys::JsValue>,
							arg1_0: ::js_sys::r#macro::InputSlot1<&JsValue>,
							arg1_1: ::js_sys::r#macro::InputSlot2<&JsValue>,
							arg1_2: ::js_sys::r#macro::InputSlot3<&JsValue>,
							arg1_3: ::js_sys::r#macro::InputSlot4<&JsValue>,
						);
					}

					{
						let (arg0_0, arg0_1, arg0_2, arg0_3) = unsafe {
							::js_sys::r#macro::split_input_as::<&::js_sys::JsValue>(self)
						};
						let (arg1_0, arg1_1, arg1_2, arg1_3) =
							::js_sys::r#macro::split_input::<&JsValue>(value);
						unsafe {
							test(
								arg0_0, arg0_1, arg0_2, arg0_3, arg1_0, arg1_1, arg1_2, arg1_3,
							)
						}
					};
				}
			}
			const _: () = {
				const IMPORTS: &[::js_sys::r#macro::ImportDescriptor] =
					&[::js_sys::r#macro::ImportDescriptor::new(
						"test_crate",
						"JsTest.test",
						"test_crate.JsTest.test",
						&[
							::js_sys::r#macro::import_input::<&::js_sys::JsValue>("arg0"),
							::js_sys::r#macro::import_input::<&JsValue>("arg1"),
						],
						::core::option::Option::None,
						::core::option::Option::Some(::js_sys::r#macro::ImportJs {
							direct_wrapper: true,
							direct_call: "arg0_0.test = arg1_0",
							indirect_call: "arg0_0.test = arg1_0",
							required_embeds: &[
								::js_sys::r#macro::js_input_embed::<&::js_sys::JsValue>(),
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
		"(import \"test_crate\" \"JsTest.test\" (func $test_crate.import.JsTest.test (@sym (name \
		 \"test_crate.import.JsTest.test\")) (param externref externref)))
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(func $test_crate.JsTest.test (@sym) (param $arg0_0 i32) (param $arg1_0 i32)
		  local.get $arg0_0
		  table.get $js_sys.import.externref.table (@reloc)
		  local.get $arg1_0
		  table.get $js_sys.import.externref.table (@reloc)
		  call $test_crate.import.JsTest.test (@reloc)
		)",
		"(arg0_0, arg1_0) => arg0_0.test = arg1_0",
	);
}
