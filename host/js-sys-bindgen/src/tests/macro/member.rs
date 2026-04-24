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
					::js_sys::js_bindgen::unsafe_embed_asm! {
						"(module (@rwat)",
						#[cfg(target_arch = "wasm64")]
						"  (import \"env\" \"__linear_memory\" (memory i64 0))",
						"  (import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \"test_crate.import.test\")) (param {}) (result )))",
						"  {}",
						"",
						"  (func $test_crate.test (@sym) (param  {}) (result )",
						"    local.get {}",
						"    call $test_crate.import.test (@reloc)",
						"    ",
						"  )",
						")",
						interpolate ::js_sys::r#macro::asm_input_import_type::<&::js_sys::JsValue>(),
						interpolate ::js_sys::r#macro::asm_input_import::<&::js_sys::JsValue>(),
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate ::js_sys::r#macro::asm_input!("0", &::js_sys::JsValue),
					}

					::js_sys::js_bindgen::import_js! {
						module = "test_crate",
						name = "test",
						required_embeds = [::js_sys::r#macro::js_input_embed::<&::js_sys::JsValue>()],
						"{}{}{}",
						interpolate ::js_sys::r#macro::js_select!(
							"(self) => ",
							"(self) => {\n",
							(&::js_sys::JsValue),
						),
						interpolate ::js_sys::r#macro::js_parameter!("self", &::js_sys::JsValue),
						interpolate ::js_sys::r#macro::js_select!(
							"self.test()",
							"self.test()\n}",
							(&::js_sys::JsValue),
						),
					}

					unsafe extern "C" {
						#[link_name = "test_crate.test"]
						fn test(this: <&::js_sys::JsValue as ::js_sys::hazard::Input>::Type);
					}

					unsafe { test(::js_sys::hazard::Input::into_raw(self)) };
				}
			}
		},
		indoc::indoc!(
			"(module (@rwat)
			  (import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \"test_crate.import.test\")) (param externref) (result )))
			  (import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result externref)))

			  (func $test_crate.test (@sym) (param  i32) (result )
			    local.get 0
				call $js_sys.externref.get (@reloc)
			    call $test_crate.import.test (@reloc)
			    
			  )
			)"
		),
		"(self) => self.test()",
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
					::js_sys::js_bindgen::unsafe_embed_asm! {
						"(module (@rwat)",
						#[cfg(target_arch = "wasm64")]
						"  (import \"env\" \"__linear_memory\" (memory i64 0))",
						"  (import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \"test_crate.import.test\")) (param {} {} {}) (result )))",
						"  {}",
						"",
						"  {}",
						"",
						"  (func $test_crate.test (@sym) (param  {} {} {}) (result )",
						"    local.get {}",
						"    local.get {}",
						"    local.get {}",
						"    call $test_crate.import.test (@reloc)",
						"    ",
						"  )",
						")",
						interpolate ::js_sys::r#macro::asm_input_import_type::<&::js_sys::JsValue>(),
						interpolate ::js_sys::r#macro::asm_input_import_type::<&JsValue>(),
						interpolate ::js_sys::r#macro::asm_input_import_type::<&JsValue>(),
						interpolate ::js_sys::r#macro::asm_input_import::<&::js_sys::JsValue>(),
						interpolate ::js_sys::r#macro::asm_input_import::<&JsValue>(),
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate ::js_sys::r#macro::asm_input!("0", &::js_sys::JsValue),
						interpolate ::js_sys::r#macro::asm_input!("1", &JsValue),
						interpolate ::js_sys::r#macro::asm_input!("2", &JsValue),
					}

					::js_sys::js_bindgen::import_js! {
						module = "test_crate",
						name = "test",
						required_embeds = [
							::js_sys::r#macro::js_input_embed::<&::js_sys::JsValue>(),
							::js_sys::r#macro::js_input_embed::<&JsValue>(),
						],
						"{}{}{}{}{}",
						interpolate ::js_sys::r#macro::js_select!(
							"(self, par1, par2) => ",
							"(self, par1, par2) => {\n",
							(&::js_sys::JsValue, &JsValue),
						),
						interpolate ::js_sys::r#macro::js_parameter!("self", &::js_sys::JsValue),
						interpolate ::js_sys::r#macro::js_parameter!("par1", &JsValue),
						interpolate ::js_sys::r#macro::js_parameter!("par2", &JsValue),
						interpolate ::js_sys::r#macro::js_select!(
							"self.test(par1, par2)",
							"self.test(par1, par2)\n}",
							(&::js_sys::JsValue, &JsValue),
						),
					}

					unsafe extern "C" {
						#[link_name = "test_crate.test"]
						fn test(
							this: <&::js_sys::JsValue as ::js_sys::hazard::Input>::Type,
							par1: <&JsValue as ::js_sys::hazard::Input>::Type,
							par2: <&JsValue as ::js_sys::hazard::Input>::Type,
						);
					}

					unsafe {
						test(
							::js_sys::hazard::Input::into_raw(self),
							::js_sys::hazard::Input::into_raw(par1),
							::js_sys::hazard::Input::into_raw(par2),
						)
					};
				}
			}
		},
		indoc::indoc!(
			"(module (@rwat)
			  (import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \"test_crate.import.test\")) (param externref externref externref) (result )))
			  (import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result externref)))

			  (import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result externref)))

			  (func $test_crate.test (@sym) (param  i32 i32 i32) (result )
			    local.get 0
				call $js_sys.externref.get (@reloc)
			    local.get 1
				call $js_sys.externref.get (@reloc)
			    local.get 2
				call $js_sys.externref.get (@reloc)
			    call $test_crate.import.test (@reloc)
			    
			  )
			)"
		),
		"(self, par1, par2) => self.test(par1, par2)",
	);
}

#[test]
fn getter() {
	test!(
		{},
		{
			extern "js-sys" {
				#[js_sys(property)]
				pub fn test(self: &JsTest) -> JsValue;
			}
		},
		{
			impl JsTest {
				pub fn test(self: &JsTest) -> JsValue {
					::js_sys::js_bindgen::unsafe_embed_asm! {
						"(module (@rwat)",
						#[cfg(target_arch = "wasm64")]
						"  (import \"env\" \"__linear_memory\" (memory i64 0))",
						"  (import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \"test_crate.import.test\")) (param {}) (result {})))",
						"  {}",
						"",
						"  {}",
						"",
						"  (func $test_crate.test (@sym) (param {} {}) (result {})",
						"    local.get {}",
						"    call $test_crate.import.test (@reloc)",
						"    {}",
						"  )",
						")",
						interpolate ::js_sys::r#macro::asm_input_import_type::<&::js_sys::JsValue>(),
						interpolate ::js_sys::r#macro::asm_output_import_type::<JsValue>(),
						interpolate ::js_sys::r#macro::asm_input_import::<&::js_sys::JsValue>(),
						interpolate ::js_sys::r#macro::asm_output_import::<JsValue>(),
						interpolate ::js_sys::r#macro::asm_indirect!(JsValue),
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate ::js_sys::r#macro::asm_direct::<JsValue>(),
						interpolate ::js_sys::r#macro::asm_input!("0", "1", &::js_sys::JsValue, JsValue),
						interpolate ::js_sys::r#macro::asm_output!(JsValue),
					}

					::js_sys::js_bindgen::import_js! {
						module = "test_crate",
						name = "test",
						required_embeds = [
							::js_sys::r#macro::js_input_embed::<&::js_sys::JsValue>(),
							::js_sys::r#macro::js_output_embed::<JsValue>(),
						],
						"{}{}{}",
						interpolate ::js_sys::r#macro::js_select!(
							"(self) => ",
							"(self) => {\n",
							(&::js_sys::JsValue),
							JsValue,
						),
						interpolate ::js_sys::r#macro::js_parameter!("self", &::js_sys::JsValue),
						interpolate ::js_sys::r#macro::js_output!(
							"\treturn ",
							"self.test",
							"self.test",
							JsValue,
							&::js_sys::JsValue,
						),
					}

					unsafe extern "C" {
						#[link_name = "test_crate.test"]
						fn test(
							this: <&::js_sys::JsValue as ::js_sys::hazard::Input>::Type,
						) -> <JsValue as ::js_sys::hazard::Output>::Type;
					}

					::js_sys::hazard::Output::from_raw(unsafe { test(::js_sys::hazard::Input::into_raw(self)) })
				}
			}
		},
		indoc::indoc!(
			"(module (@rwat)
			  (import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \"test_crate.import.test\")) (param externref) (result externref)))
			  (import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result externref)))

			  (import \"env\" \"js_sys.externref.insert\" (func $js_sys.externref.insert (@sym) (param externref) (result i32)))

			  (func $test_crate.test (@sym) (param  i32) (result i32)
			    local.get 0
				call $js_sys.externref.get (@reloc)
			    call $test_crate.import.test (@reloc)
			    
				call $js_sys.externref.insert (@reloc)
			  )
			)"
		),
		"(self) => self.test",
	);
}

#[test]
fn setter() {
	test!(
		{},
		{
			extern "js-sys" {
				#[js_sys(property)]
				pub fn test(self: &JsTest, value: &JsValue);
			}
		},
		{
			impl JsTest {
				pub fn test(self: &JsTest, value: &JsValue) {
					::js_sys::js_bindgen::unsafe_embed_asm! {
						"(module (@rwat)",
						#[cfg(target_arch = "wasm64")]
						"  (import \"env\" \"__linear_memory\" (memory i64 0))",
						"  (import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \"test_crate.import.test\")) (param {} {}) (result )))",
						"  {}",
						"",
						"  {}",
						"",
						"  (func $test_crate.test (@sym) (param  {} {}) (result )",
						"    local.get {}",
						"    local.get {}",
						"    call $test_crate.import.test (@reloc)",
						"    ",
						"  )",
						")",
						interpolate ::js_sys::r#macro::asm_input_import_type::<&::js_sys::JsValue>(),
						interpolate ::js_sys::r#macro::asm_input_import_type::<&JsValue>(),
						interpolate ::js_sys::r#macro::asm_input_import::<&::js_sys::JsValue>(),
						interpolate ::js_sys::r#macro::asm_input_import::<&JsValue>(),
						interpolate <&::js_sys::JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
						interpolate ::js_sys::r#macro::asm_input!("0", &::js_sys::JsValue),
						interpolate ::js_sys::r#macro::asm_input!("1", &JsValue),
					}

					::js_sys::js_bindgen::import_js! {
						module = "test_crate",
						name = "test",
						required_embeds = [
							::js_sys::r#macro::js_input_embed::<&::js_sys::JsValue>(),
							::js_sys::r#macro::js_input_embed::<&JsValue>(),
						],
						"{}{}{}self.test = {}",
						interpolate ::js_sys::r#macro::js_select!(
							"(self, value) => ",
							"(self, value) => {\n",
							(&::js_sys::JsValue, &JsValue),
						),
						interpolate ::js_sys::r#macro::js_parameter!("self", &::js_sys::JsValue),
						interpolate ::js_sys::r#macro::js_parameter!("value", &JsValue),
						interpolate ::js_sys::r#macro::js_select!(
							"value",
							"value\n}",
							(&::js_sys::JsValue, &JsValue),
						),
					}

					unsafe extern "C" {
						#[link_name = "test_crate.test"]
						fn test(
							this: <&::js_sys::JsValue as ::js_sys::hazard::Input>::Type,
							value: <&JsValue as ::js_sys::hazard::Input>::Type,
						);
					}

					unsafe {
						test(::js_sys::hazard::Input::into_raw(self), ::js_sys::hazard::Input::into_raw(value))
					};
				}
			}
		},
		indoc::indoc!(
			"(module (@rwat)
			  (import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \"test_crate.import.test\")) (param externref externref) (result )))
			  (import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result externref)))

			  (import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result externref)))

			  (func $test_crate.test (@sym) (param  i32 i32) (result )
			    local.get 0
				call $js_sys.externref.get (@reloc)
			    local.get 1
				call $js_sys.externref.get (@reloc)
			    call $test_crate.import.test (@reloc)
			    
			  )
			)"
		),
		"(self, value) => self.test = value",
	);
}
