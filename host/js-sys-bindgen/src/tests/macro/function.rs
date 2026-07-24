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
				::js_sys::js_bindgen::unsafe_global_wat! {
					"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import = "log",
					shim = "test_crate.log", inputs = [("arg0", & JsValue)],),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					required_embeds = [::js_sys::r#macro::js_input_embed::<&JsValue>()],
					"{}",
					interpolate ::js_sys::r#macro::js_import!(
						direct_open = "", direct_call = "globalThis.log", indirect_call =
						"globalThis.log(arg0_0)", inputs = [("arg0", & JsValue)],
					),
				}

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
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.log (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  call $js_sys.externref.get (@reloc)
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
				::js_sys::js_bindgen::unsafe_global_wat! {
					"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import =
					"console.log", shim = "test_crate.console.log", inputs = [("arg0", & JsValue)],),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "console.log",
					required_embeds = [::js_sys::r#macro::js_input_embed::<&JsValue>()],
					"{}",
					interpolate ::js_sys::r#macro::js_import!(
						direct_open = "", direct_call = "globalThis.console.log", indirect_call =
						"globalThis.console.log(arg0_0)", inputs = [("arg0", & JsValue)],
					),
				}

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
		},
		"(import \"test_crate\" \"console.log\" (func $test_crate.import.console.log (@sym (name \
		 \"test_crate.import.console.log\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.console.log (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  call $js_sys.externref.get (@reloc)
		  call $test_crate.import.console.log (@reloc)
		)",
		"globalThis.console.log",
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
				js_sys::js_bindgen::unsafe_global_wat! {
					"{}", interpolate js_sys::r#macro::wat_import!(module = "test_crate", import = "log",
					shim = "test_crate.log", inputs = [("arg0", & JsValue)],),
				}

				js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					required_embeds = [js_sys::r#macro::js_input_embed::<&JsValue>()],
					"{}",
					interpolate js_sys::r#macro::js_import!(
						direct_open = "", direct_call = "globalThis.log", indirect_call =
						"globalThis.log(arg0_0)", inputs = [("arg0", & JsValue)],
					),
				}

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
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.log (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  call $js_sys.externref.get (@reloc)
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
				::js_sys::js_bindgen::unsafe_global_wat! {
					"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import = "log",
					shim = "test_crate.log", inputs = [("arg0", & JsValue), ("arg1", & JsValue)],),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					required_embeds = [::js_sys::r#macro::js_input_embed::<&JsValue>()],
					"{}",
					interpolate ::js_sys::r#macro::js_import!(
						direct_open = "", direct_call = "globalThis.log", indirect_call =
						"globalThis.log(arg0_0, arg1_0)", inputs = [("arg0", & JsValue), ("arg1", & JsValue)],
					),
				}

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
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.log (@sym) (param $arg0_0 i32) (param $arg1_0 i32)
		  local.get $arg0_0
		  call $js_sys.externref.get (@reloc)
		  local.get $arg1_0
		  call $js_sys.externref.get (@reloc)
		  call $test_crate.import.log (@reloc)
		)",
		"globalThis.log",
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
				::js_sys::js_bindgen::unsafe_global_wat! {
					"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import = "log",
					shim = "test_crate.log", inputs = [],),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					"{}",
					interpolate ::js_sys::r#macro::js_import!(
						direct_open = "", direct_call = "globalThis.log", indirect_call = "globalThis.log()",
						inputs = [],
					),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log();
				}

				{
					unsafe { log() }
				};
			}
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
				::js_sys::js_bindgen::unsafe_global_wat! {
					"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import = "logx",
					shim = "test_crate.logx", inputs = [("arg0", & JsValue)],),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "logx",
					required_embeds = [::js_sys::r#macro::js_input_embed::<&JsValue>()],
					"{}",
					interpolate ::js_sys::r#macro::js_import!(
						direct_open = "", direct_call = "globalThis.log", indirect_call =
						"globalThis.log(arg0_0)", inputs = [("arg0", & JsValue)],
					),
				}

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
		},
		"(import \"test_crate\" \"logx\" (func $test_crate.import.logx (@sym (name \
		 \"test_crate.import.logx\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.logx (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  call $js_sys.externref.get (@reloc)
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
				::js_sys::js_bindgen::unsafe_global_wat! {
					"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import = "log",
					shim = "test_crate.log", inputs = [("arg0", & JsValue)],),
				}

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
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.log (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  call $js_sys.externref.get (@reloc)
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
				::js_sys::js_bindgen::unsafe_global_wat! {
					"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import = "log",
					shim = "test_crate.log", inputs = [("arg0", & JsValue)],),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					required_embeds = [
						("test_crate", "embed"),
						::js_sys::r#macro::js_input_embed::<&JsValue>(),
					],
					"{}",
					interpolate ::js_sys::r#macro::js_import!(
						direct_open = "", direct_call = "this.#jsEmbed.test_crate['embed']", indirect_call =
						"this.#jsEmbed.test_crate['embed'](arg0_0)", inputs = [("arg0", & JsValue)],
					),
				}

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
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.log (@sym) (param $arg0_0 i32)
		  local.get $arg0_0
		  call $js_sys.externref.get (@reloc)
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
				::js_sys::js_bindgen::unsafe_global_wat! {
					"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import = "is_nan",
					shim = "test_crate.is_nan", inputs = [], output = JsValue,),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "is_nan",
					required_embeds = [
						::js_sys::r#macro::js_output_embed::<JsValue>(),
						::js_sys::r#macro::js_result_embed::<JsValue>(),
					],
					"{}",
					interpolate ::js_sys::r#macro::js_import!(
						direct_open = "", direct_call = "globalThis.is_nan", indirect_call =
						"globalThis.is_nan()", inputs = [], output = JsValue,
					),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.is_nan"]
					fn is_nan() -> ::js_sys::r#macro::OutputRet<JsValue>;
				}

				::js_sys::r#macro::join_output({ unsafe { is_nan() } })
			}
		},
		"(import \"test_crate\" \"is_nan\" (func $test_crate.import.is_nan (@sym (name \
		 \"test_crate.import.is_nan\")) (result externref)))
		(import \"env\" \"js_sys.externref.insert\" (func $js_sys.externref.insert (@sym) (param \
		 externref) (result i32)))
		(func $test_crate.is_nan (@sym) (result i32)
		  call $test_crate.import.is_nan (@reloc)
		  call $js_sys.externref.insert (@reloc)
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
				::js_sys::js_bindgen::unsafe_global_wat! {
					"{}", interpolate::js_sys::r#macro::wat_import!(module = "test_crate", import = "log",
					shim = "test_crate.log", inputs = [],),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					"{}",
					interpolate ::js_sys::r#macro::js_import!(
						direct_open = "", direct_call = "globalThis.log", indirect_call = "globalThis.log()",
						inputs = [],
					),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log();
				}

				{
					unsafe { log() }
				};
			}
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\"))))
		(func $test_crate.log (@sym)
		  call $test_crate.import.log (@reloc)
		)",
		"globalThis.log",
	);
}
