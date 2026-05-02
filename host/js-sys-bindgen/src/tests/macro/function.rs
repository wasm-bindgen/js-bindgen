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
				::js_sys::js_bindgen::unsafe_embed_asm! {
					"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \"test_crate.import.log\")) (param {}))){}",
					"(func $test_crate.log (@sym) (param $data {})",
					"  local.get $data{}",
					"  call $test_crate.import.log (@reloc)",
					")",
					interpolate ::js_sys::r#macro::asm_input_import_type::<&JsValue>(),
					interpolate ::js_sys::r#macro::asm_imports!((&JsValue)),
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_input!(&JsValue),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					required_embeds = [::js_sys::r#macro::js_input_embed::<&JsValue>()],
					"{}{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "(data) => {\n", (&JsValue)),
					interpolate ::js_sys::r#macro::js_parameter!("data", &JsValue),
					interpolate ::js_sys::r#macro::js_select!(
						"globalThis.log",
						"globalThis.log(data)\n}",
						(&JsValue),
					),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(::js_sys::hazard::Input::into_raw(data)) };
			}
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.log (@sym) (param $data i32)
		  local.get $data
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
				::js_sys::js_bindgen::unsafe_embed_asm! {
					"(import \"test_crate\" \"console.log\" (func $test_crate.import.console.log (@sym (name \"test_crate.import.console.log\")) (param {}))){}",
					"(func $test_crate.console.log (@sym) (param $data {})",
					"  local.get $data{}",
					"  call $test_crate.import.console.log (@reloc)",
					")",
					interpolate ::js_sys::r#macro::asm_input_import_type::<&JsValue>(),
					interpolate ::js_sys::r#macro::asm_imports!((&JsValue)),
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_input!(&JsValue),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "console.log",
					required_embeds = [::js_sys::r#macro::js_input_embed::<&JsValue>()],
					"{}{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "(data) => {\n", (&JsValue)),
					interpolate ::js_sys::r#macro::js_parameter!("data", &JsValue),
					interpolate ::js_sys::r#macro::js_select!(
						"globalThis.console.log",
						"globalThis.console.log(data)\n}",
						(&JsValue),
					),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.console.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(::js_sys::hazard::Input::into_raw(data)) };
			}
		},
		"(import \"test_crate\" \"console.log\" (func $test_crate.import.console.log (@sym (name \
		 \"test_crate.import.console.log\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.console.log (@sym) (param $data i32)
		  local.get $data
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
				js_sys::js_bindgen::unsafe_embed_asm! {
					"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \"test_crate.import.log\")) (param {}))){}",
					"(func $test_crate.log (@sym) (param $data {})",
					"  local.get $data{}",
					"  call $test_crate.import.log (@reloc)",
					")",
					interpolate js_sys::r#macro::asm_input_import_type::<&JsValue>(),
					interpolate js_sys::r#macro::asm_imports!((&JsValue)),
					interpolate <&JsValue as js_sys::hazard::Input>::ASM_TYPE,
					interpolate js_sys::r#macro::asm_input!(&JsValue),
				}

				js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					required_embeds = [js_sys::r#macro::js_input_embed::<&JsValue>()],
					"{}{}{}",
					interpolate js_sys::r#macro::js_select!("", "(data) => {\n", (&JsValue)),
					interpolate js_sys::r#macro::js_parameter!("data", &JsValue),
					interpolate js_sys::r#macro::js_select!(
						"globalThis.log",
						"globalThis.log(data)\n}",
						(&JsValue),
					),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as js_sys::hazard::Input>::Type);
				}

				unsafe { log(js_sys::hazard::Input::into_raw(data)) };
			}
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.log (@sym) (param $data i32)
		  local.get $data
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
				::js_sys::js_bindgen::unsafe_embed_asm! {
					"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \"test_crate.import.log\")) (param {} {}))){}",
					"(func $test_crate.log (@sym) (param $data1 {}) (param $data2 {})",
					"  local.get $data1{}",
					"  local.get $data2{}",
					"  call $test_crate.import.log (@reloc)",
					")",
					interpolate ::js_sys::r#macro::asm_input_import_type::<&JsValue>(),
					interpolate ::js_sys::r#macro::asm_input_import_type::<&JsValue>(),
					interpolate ::js_sys::r#macro::asm_imports!((&JsValue)),
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_input!(&JsValue),
					interpolate ::js_sys::r#macro::asm_input!(&JsValue),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					required_embeds = [::js_sys::r#macro::js_input_embed::<&JsValue>()],
					"{}{}{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "(data1, data2) => {\n", (&JsValue)),
					interpolate ::js_sys::r#macro::js_parameter!("data1", &JsValue),
					interpolate ::js_sys::r#macro::js_parameter!("data2", &JsValue),
					interpolate ::js_sys::r#macro::js_select!(
						"globalThis.log",
						"globalThis.log(data1, data2)\n}",
						(&JsValue),
					),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(
						data1: <&JsValue as ::js_sys::hazard::Input>::Type,
						data2: <&JsValue as ::js_sys::hazard::Input>::Type,
					);
				}

				unsafe {
					log(
						::js_sys::hazard::Input::into_raw(data1),
						::js_sys::hazard::Input::into_raw(data2),
					)
				};
			}
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.log (@sym) (param $data1 i32) (param $data2 i32)
		  local.get $data1
		  call $js_sys.externref.get (@reloc)
		  local.get $data2
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
				::js_sys::js_bindgen::unsafe_embed_asm! {
					"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \"test_crate.import.log\"))))",
					"(func $test_crate.log (@sym)",
					"  call $test_crate.import.log (@reloc)",
					")",
				}

				::js_sys::js_bindgen::import_js!(
					module = "test_crate",
					name = "log",
					"globalThis.log"
				);

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log();
				}

				unsafe { log() };
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
				::js_sys::js_bindgen::unsafe_embed_asm! {
					"(import \"test_crate\" \"logx\" (func $test_crate.import.logx (@sym (name \"test_crate.import.logx\")) (param {}))){}",
					"(func $test_crate.logx (@sym) (param $data {})",
					"  local.get $data{}",
					"  call $test_crate.import.logx (@reloc)",
					")",
					interpolate ::js_sys::r#macro::asm_input_import_type::<&JsValue>(),
					interpolate ::js_sys::r#macro::asm_imports!((&JsValue)),
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_input!(&JsValue),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "logx",
					required_embeds = [::js_sys::r#macro::js_input_embed::<&JsValue>()],
					"{}{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "(data) => {\n", (&JsValue)),
					interpolate ::js_sys::r#macro::js_parameter!("data", &JsValue),
					interpolate ::js_sys::r#macro::js_select!(
						"globalThis.log",
						"globalThis.log(data)\n}",
						(&JsValue),
					),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.logx"]
					fn logx(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { logx(::js_sys::hazard::Input::into_raw(data)) };
			}
		},
		"(import \"test_crate\" \"logx\" (func $test_crate.import.logx (@sym (name \
		 \"test_crate.import.logx\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.logx (@sym) (param $data i32)
		  local.get $data
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
				::js_sys::js_bindgen::unsafe_embed_asm! {
					"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \"test_crate.import.log\")) (param {}))){}",
					"(func $test_crate.log (@sym) (param $data {})",
					"  local.get $data{}",
					"  call $test_crate.import.log (@reloc)",
					")",
					interpolate ::js_sys::r#macro::asm_input_import_type::<&JsValue>(),
					interpolate ::js_sys::r#macro::asm_imports!((&JsValue)),
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_input!(&JsValue),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(::js_sys::hazard::Input::into_raw(data)) };
			}
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.log (@sym) (param $data i32)
		  local.get $data
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
				::js_sys::js_bindgen::unsafe_embed_asm! {
					"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \"test_crate.import.log\")) (param {}))){}",
					"(func $test_crate.log (@sym) (param $data {})",
					"  local.get $data{}",
					"  call $test_crate.import.log (@reloc)",
					")",
					interpolate ::js_sys::r#macro::asm_input_import_type::<&JsValue>(),
					interpolate ::js_sys::r#macro::asm_imports!((&JsValue)),
					interpolate <&JsValue as ::js_sys::hazard::Input>::ASM_TYPE,
					interpolate ::js_sys::r#macro::asm_input!(&JsValue),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "log",
					required_embeds = [
						("test_crate", "embed"),
						::js_sys::r#macro::js_input_embed::<&JsValue>(),
					],
					"{}{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "(data) => {\n", (&JsValue)),
					interpolate ::js_sys::r#macro::js_parameter!("data", &JsValue),
					interpolate ::js_sys::r#macro::js_select!(
						"this.#jsEmbed.test_crate['embed']",
						"this.#jsEmbed.test_crate['embed'](data)\n}",
						(&JsValue),
					),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log(data: <&JsValue as ::js_sys::hazard::Input>::Type);
				}

				unsafe { log(::js_sys::hazard::Input::into_raw(data)) };
			}
		},
		"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \
		 \"test_crate.import.log\")) (param externref)))
		(import \"env\" \"js_sys.externref.get\" (func $js_sys.externref.get (@sym) (param i32) (result \
		 externref)))
		(func $test_crate.log (@sym) (param $data i32)
		  local.get $data
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
				::js_sys::js_bindgen::unsafe_embed_asm! {
					"(import \"test_crate\" \"is_nan\" (func $test_crate.import.is_nan (@sym (name \"test_crate.import.is_nan\")) (result {}))){}",
					"(func $test_crate.is_nan (@sym) (param {}) (result {})",
					"  call $test_crate.import.is_nan (@reloc){}",
					")",
					interpolate ::js_sys::r#macro::asm_output_import_type::<JsValue>(),
					interpolate ::js_sys::r#macro::asm_imports!((), JsValue),
					interpolate ::js_sys::r#macro::asm_indirect!(JsValue),
					interpolate ::js_sys::r#macro::asm_direct::<JsValue>(),
					interpolate ::js_sys::r#macro::asm_output!(JsValue),
				}

				::js_sys::js_bindgen::import_js! {
					module = "test_crate",
					name = "is_nan",
					required_embeds = [::js_sys::r#macro::js_output_embed::<JsValue>()],
					"{}{}",
					interpolate ::js_sys::r#macro::js_select!("", "() => {\n\treturn ", (), JsValue),
					interpolate ::js_sys::r#macro::js_output!(
						"",
						"globalThis.is_nan",
						"globalThis.is_nan()",
						JsValue,
					),
				}

				unsafe extern "C" {
					#[link_name = "test_crate.is_nan"]
					fn is_nan() -> <JsValue as ::js_sys::hazard::Output>::Type;
				}

				::js_sys::hazard::Output::from_raw(unsafe { is_nan() })
			}
		},
		"(import \"test_crate\" \"is_nan\" (func $test_crate.import.is_nan (@sym (name \
		 \"test_crate.import.is_nan\")) (result externref)))
		(import \"env\" \"js_sys.externref.insert\" (func $js_sys.externref.insert (@sym) (param \
		 externref) (result i32)))
		(func $test_crate.is_nan (@sym) (param ) (result i32)
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
				::js_sys::js_bindgen::unsafe_embed_asm! {
					"(import \"test_crate\" \"log\" (func $test_crate.import.log (@sym (name \"test_crate.import.log\"))))",
					"(func $test_crate.log (@sym)",
					"  call $test_crate.import.log (@reloc)",
					")",
				}

				::js_sys::js_bindgen::import_js!(
					module = "test_crate",
					name = "log",
					"globalThis.log"
				);

				unsafe extern "C" {
					#[link_name = "test_crate.log"]
					fn log();
				}

				unsafe { log() };
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
