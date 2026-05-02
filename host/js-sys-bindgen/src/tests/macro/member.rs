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
						"(import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \"test_crate.import.test\")) (param {}))){}",
						"(func $test_crate.test (@sym) (param $self {})", "  local.get $self{}",
						"  call $test_crate.import.test (@reloc)", ")",
						interpolate::js_sys::r#macro::wat_input_import_type:: < & ::js_sys::JsValue > (),
						interpolate::js_sys::r#macro::wat_imports!((& ::js_sys::JsValue),), interpolate < &
						::js_sys::JsValue as ::js_sys::hazard::Input > ::WAT_TYPE,
						interpolate::js_sys::r#macro::wat_input!(& ::js_sys::JsValue),
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
		"(import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \
		 \"test_crate.import.test\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.test (@sym) (param $self i32)
		  local.get $self
		  call $js_sys.externref.get (@reloc)
		  call $test_crate.import.test (@reloc)
		)",
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
					::js_sys::js_bindgen::unsafe_global_wat! {
						"(import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \"test_crate.import.test\")) (param {} {} {}))){}",
						"(func $test_crate.test (@sym) (param $self {}) (param $par1 {}) (param $par2 {})",
						"  local.get $self{}", "  local.get $par1{}", "  local.get $par2{}",
						"  call $test_crate.import.test (@reloc)", ")",
						interpolate::js_sys::r#macro::wat_input_import_type:: < & ::js_sys::JsValue > (),
						interpolate::js_sys::r#macro::wat_input_import_type:: < & JsValue > (),
						interpolate::js_sys::r#macro::wat_input_import_type:: < & JsValue > (),
						interpolate::js_sys::r#macro::wat_imports!((& ::js_sys::JsValue, & JsValue),),
						interpolate < & ::js_sys::JsValue as ::js_sys::hazard::Input > ::WAT_TYPE, interpolate <
						& JsValue as ::js_sys::hazard::Input > ::WAT_TYPE, interpolate < & JsValue as
						::js_sys::hazard::Input > ::WAT_TYPE, interpolate::js_sys::r#macro::wat_input!(&
						::js_sys::JsValue), interpolate::js_sys::r#macro::wat_input!(& JsValue),
						interpolate::js_sys::r#macro::wat_input!(& JsValue),
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
		"(import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \
		 \"test_crate.import.test\")) (param externref externref externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.test (@sym) (param $self i32) (param $par1 i32) (param $par2 i32)
		  local.get $self
		  call $js_sys.externref.get (@reloc)
		  local.get $par1
		  call $js_sys.externref.get (@reloc)
		  local.get $par2
		  call $js_sys.externref.get (@reloc)
		  call $test_crate.import.test (@reloc)
		)",
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
					::js_sys::js_bindgen::unsafe_global_wat! {
						"(import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \"test_crate.import.test\")) (param {}) (result {}))){}",
						"(func $test_crate.test (@sym) (param {}) (param $self {}) (result {})",
						"  local.get $self{}", "  call $test_crate.import.test (@reloc){}", ")",
						interpolate::js_sys::r#macro::wat_input_import_type:: < & ::js_sys::JsValue > (),
						interpolate::js_sys::r#macro::wat_output_import_type:: < JsValue > (),
						interpolate::js_sys::r#macro::wat_imports!((& ::js_sys::JsValue), JsValue),
						interpolate::js_sys::r#macro::wat_indirect!(JsValue), interpolate < & ::js_sys::JsValue
						as ::js_sys::hazard::Input > ::WAT_TYPE, interpolate::js_sys::r#macro::wat_direct:: <
						JsValue > (), interpolate::js_sys::r#macro::wat_input!(& ::js_sys::JsValue),
						interpolate::js_sys::r#macro::wat_output!(JsValue),
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

					::js_sys::hazard::Output::from_raw(unsafe {
						test(::js_sys::hazard::Input::into_raw(self))
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
		(func $test_crate.test (@sym) (param ) (param $self i32) (result i32)
		  local.get $self
		  call $js_sys.externref.get (@reloc)
		  call $test_crate.import.test (@reloc)
		  call $js_sys.externref.insert (@reloc)
		)",
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
					::js_sys::js_bindgen::unsafe_global_wat! {
						"(import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \"test_crate.import.test\")) (param {} {}))){}",
						"(func $test_crate.test (@sym) (param $self {}) (param $value {})",
						"  local.get $self{}", "  local.get $value{}",
						"  call $test_crate.import.test (@reloc)", ")",
						interpolate::js_sys::r#macro::wat_input_import_type:: < & ::js_sys::JsValue > (),
						interpolate::js_sys::r#macro::wat_input_import_type:: < & JsValue > (),
						interpolate::js_sys::r#macro::wat_imports!((& ::js_sys::JsValue, & JsValue),),
						interpolate < & ::js_sys::JsValue as ::js_sys::hazard::Input > ::WAT_TYPE, interpolate <
						& JsValue as ::js_sys::hazard::Input > ::WAT_TYPE,
						interpolate::js_sys::r#macro::wat_input!(& ::js_sys::JsValue),
						interpolate::js_sys::r#macro::wat_input!(& JsValue),
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
						test(
							::js_sys::hazard::Input::into_raw(self),
							::js_sys::hazard::Input::into_raw(value),
						)
					};
				}
			}
		},
		"(import \"test_crate\" \"test\" (func $test_crate.import.test (@sym (name \
		 \"test_crate.import.test\")) (param externref externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.test (@sym) (param $self i32) (param $value i32)
		  local.get $self
		  call $js_sys.externref.get (@reloc)
		  local.get $value
		  call $js_sys.externref.get (@reloc)
		  call $test_crate.import.test (@reloc)
		)",
		"(self, value) => self.test = value",
	);
}
