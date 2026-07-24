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
					::js_sys::js_bindgen::unsafe_global_wat! {
						"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import = "test",
						shim = "test_crate.test", inputs = [("arg0", & ::js_sys::JsValue)],),
					}

					::js_sys::js_bindgen::import_js! {
						module = "test_crate",
						name = "test",
						required_embeds = [::js_sys::r#macro::js_input_embed::<&::js_sys::JsValue>()],
						"{}",
						interpolate ::js_sys::r#macro::js_import!(
							direct_open = ::js_sys::r#macro::js_function!("(", ") => ", ("arg0", &
							::js_sys::JsValue)), direct_call = "arg0_0.test()", indirect_call = "arg0_0.test()",
							inputs = [("arg0", & ::js_sys::JsValue)],
						),
					}

					unsafe extern "C" {
						#[link_name = "test_crate.test"]
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
		},
		"(import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \
		 \"test_crate.import.test\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.test (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  call $js_sys.externref.get (@reloc)
		  call $test_crate.import.test (@reloc)
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
					::js_sys::js_bindgen::unsafe_global_wat! {
						"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import = "test",
						shim = "test_crate.test", inputs = [("arg0", & ::js_sys::JsValue), ("arg1", &
						JsValue), ("arg2", & JsValue)],),
					}

					::js_sys::js_bindgen::import_js! {
						module = "test_crate",
						name = "test",
						required_embeds = [
							::js_sys::r#macro::js_input_embed::<&::js_sys::JsValue>(),
							::js_sys::r#macro::js_input_embed::<&JsValue>(),
						],
						"{}",
						interpolate ::js_sys::r#macro::js_import!(
							direct_open = ::js_sys::r#macro::js_function!("(", ") => ", ("arg0", &
							::js_sys::JsValue), ("arg1", & JsValue), ("arg2", & JsValue)), direct_call =
							"arg0_0.test(arg1_0, arg2_0)", indirect_call = "arg0_0.test(arg1_0, arg2_0)", inputs
							= [("arg0", & ::js_sys::JsValue), ("arg1", & JsValue), ("arg2", & JsValue)],
						),
					}

					unsafe extern "C" {
						#[link_name = "test_crate.test"]
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
		},
		"(import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \
		 \"test_crate.import.test\")) (param externref externref externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.test (@sym) (param $arg0_0 i32) (param $arg1_0 i32) (param $arg2_0 i32)
		  local.get $arg0_0
		  call $js_sys.externref.get (@reloc)
		  local.get $arg1_0
		  call $js_sys.externref.get (@reloc)
		  local.get $arg2_0
		  call $js_sys.externref.get (@reloc)
		  call $test_crate.import.test (@reloc)
		)",
		"(arg0_0, arg1_0, arg2_0) => arg0_0.test(arg1_0, arg2_0)",
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
					::js_sys::js_bindgen::unsafe_global_wat! {
						"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import = "test",
						shim = "test_crate.test", inputs = [("arg0", & ::js_sys::JsValue)], output =
						JsValue,),
					}

					::js_sys::js_bindgen::import_js! {
						module = "test_crate",
						name = "test",
						required_embeds = [
							::js_sys::r#macro::js_input_embed::<&::js_sys::JsValue>(),
							::js_sys::r#macro::js_output_embed::<JsValue>(),
							::js_sys::r#macro::js_result_embed::<JsValue>(),
						],
						"{}",
						interpolate ::js_sys::r#macro::js_import!(
							direct_open = ::js_sys::r#macro::js_function!("(", ") => ", ("arg0", &
							::js_sys::JsValue)), direct_call = "arg0_0.test", indirect_call = "arg0_0.test",
							inputs = [("arg0", & ::js_sys::JsValue)], output = JsValue,
						),
					}

					unsafe extern "C" {
						#[link_name = "test_crate.test"]
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
		},
		"(import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \
		 \"test_crate.import.test\")) (param externref) (result externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(import \"env\" \"js_sys.externref.insert\" (func $js_sys.externref.insert (@sym) (param \
		 externref) (result i32)))
		(func $test_crate.test (@sym) (param $arg0_0 i32) (result i32)
		  local.get $arg0_0
		  call $js_sys.externref.get (@reloc)
		  call $test_crate.import.test (@reloc)
		  call $js_sys.externref.insert (@reloc)
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
				#[js_sys(property)]
				pub fn test(self: &JsTest, value: &JsValue);
			}
		},
		{
			impl JsTest {
				pub fn test(self: &JsTest, value: &JsValue) {
					::js_sys::js_bindgen::unsafe_global_wat! {
						"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import = "test",
						shim = "test_crate.test", inputs = [("arg0", & ::js_sys::JsValue), ("arg1", &
						JsValue)],),
					}

					::js_sys::js_bindgen::import_js! {
						module = "test_crate",
						name = "test",
						required_embeds = [
							::js_sys::r#macro::js_input_embed::<&::js_sys::JsValue>(),
							::js_sys::r#macro::js_input_embed::<&JsValue>(),
						],
						"{}",
						interpolate ::js_sys::r#macro::js_import!(
							direct_open = ::js_sys::r#macro::js_function!("(", ") => ", ("arg0", &
							::js_sys::JsValue), ("arg1", & JsValue)), direct_call = "arg0_0.test = arg1_0",
							indirect_call = "arg0_0.test = arg1_0", inputs = [("arg0", & ::js_sys::JsValue),
							("arg1", & JsValue)],
						),
					}

					unsafe extern "C" {
						#[link_name = "test_crate.test"]
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
		},
		"(import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \
		 \"test_crate.import.test\")) (param externref externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.test (@sym) (param $arg0_0 i32) (param $arg1_0 i32)
		  local.get $arg0_0
		  call $js_sys.externref.get (@reloc)
		  local.get $arg1_0
		  call $js_sys.externref.get (@reloc)
		  call $test_crate.import.test (@reloc)
		)",
		"(arg0_0, arg1_0) => arg0_0.test = arg1_0",
	);
}
