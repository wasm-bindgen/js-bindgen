use proc_macro2::TokenStream;
use quote::quote;
use syn::File;

fn expand(function: &TokenStream) -> (String, String) {
	expand_with_attr(&TokenStream::new(), function)
}

fn expand_with_attr(attr: &TokenStream, function: &TokenStream) -> (String, String) {
	let function = syn::parse2(quote! { #function }).unwrap();
	let output = crate::export::r#macro(attr.clone(), &function, Some("test_crate")).unwrap();
	let output = prettyplease::unparse(&syn::parse2::<File>(output).unwrap());
	let dir = tempfile::tempdir().unwrap();
	let (wat, js_import, js_export) = super::inner(dir.path(), &output).unwrap();

	assert_eq!(js_import, None);
	(wat.unwrap(), js_export.unwrap())
}

#[test]
fn js_name_expression() {
	let (wat, js) = expand_with_attr(
		&quote!(js_name = concat!("module", "::answer")),
		&quote! {
			fn answer() -> u32 {
				42
			}
		},
	);

	inline_snap::inline_snap!(
		wat,
		r#"
(import "env" "raw" (func $raw (@sym (name "__export_module::answer")) (result i32)))
(func $export (@sym (name "module::answer")) (result i32)
  call $raw (@reloc)
)"#
	);
	assert_eq!(
		js,
		r"() => {
    const ret = wasmExports['module::answer']()
    return ret >>> 0
}"
	);
}

#[test]
fn borrowed_return_is_rejected() {
	let function = syn::parse2(quote! {
		fn echo(value: &JsString) -> &JsString {
			value
		}
	})
	.unwrap();
	let error =
		crate::export::r#macro(TokenStream::new(), &function, Some("test_crate")).unwrap_err();

	assert_eq!(error.to_string(), "cannot return a borrowed reference");
}

#[test]
fn direct() {
	let (wat, js) = expand(&quote! {
		fn echo(value: u32) -> u32 {
			value
		}
	});

	inline_snap::inline_snap!(
		wat,
		r#"
(import "env" "raw" (func $raw (@sym (name "__export_echo")) (param i32) (result i32)))
(func $export (@sym (name "echo")) (param $arg0_0 i32) (result i32)
  local.get $arg0_0
  call $raw (@reloc)
)"#
	);
	assert_eq!(
		js,
		r"(arg0) => {
    const ret = wasmExports['echo'](arg0)
    return ret >>> 0
}"
	);
}

#[test]
fn wat_slot_conversions() {
	let (wat, js) = expand(&quote! {
		pub fn drop_value(value: JsValue) {
			let _ = value;
		}
	});

	inline_snap::inline_snap!(
		wat,
		"
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(import \"env\" \"js_sys.externref.next\" (func $js_sys.externref.next (@sym) (result i32)))
		(import \"env\" \"raw\" (func $raw (@sym (name \"__export_drop_value\")) (param i32)))
		(func $export (@sym (name \"drop_value\")) (param $arg0_0 externref)
		  (local $js_sys.externref.value externref)
		  (local $js_sys.externref.index i32)
		  local.get $arg0_0
		  local.set $js_sys.externref.value
		  call $js_sys.externref.next (@reloc)
		  local.tee $js_sys.externref.index
		  local.get $js_sys.externref.value
		  table.set $js_sys.import.externref.table (@reloc)
		  local.get $js_sys.externref.index
		  call $raw (@reloc)
		)"
	);
	assert_eq!(
		js,
		r"(arg0) => {
    wasmExports['drop_value'](arg0)
}"
	);

	let (wat, js) = expand(&quote! {
		pub fn undefined() -> Option<JsValue> {
			None
		}
	});

	inline_snap::inline_snap!(
		wat,
		"
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(import \"env\" \"js_sys.externref.release\" (func $js_sys.externref.release (@sym) (param i32)))
		(import \"env\" \"raw\" (func $raw (@sym (name \"__export_undefined\")) (result i32)))
		(func $export (@sym (name \"undefined\")) (result externref)
		  (local $js_sys.externref.index i32)
		  call $raw (@reloc)
		  local.tee $js_sys.externref.index
		  table.get $js_sys.import.externref.table (@reloc)
		  local.get $js_sys.externref.index
		  i32.const 2
		  i32.ge_u
		  if
		    local.get $js_sys.externref.index
		    call $js_sys.externref.release (@reloc)
		  end
		)"
	);
	assert_eq!(
		js,
		r"() => {
    const ret = wasmExports['undefined']()
    return ret
}"
	);
}

#[test]
fn indirect_and_multiple_parameters() {
	let (wat, js) = expand(&quote! {
		pub fn add(value: u32, delta: u128) -> u128 {
			u128::from(value) + delta
		}
	});

	inline_snap::inline_snap!(
		wat,
		r#"
(import "env" "raw" (func $raw (@sym (name "__export_add")) (param i32) (param i32) (param i64 i64)))
(import "env" "__stack_pointer" (global $__stack_pointer (mut i32)))
(func $export (@sym (name "add")) (param $arg0_0 i32) (param $arg1_0 i64) (param $arg1_1 i64) (result i64 i64)
  (local $retptr i32)
  global.get $__stack_pointer
  i32.const 16
  i32.sub
  local.tee $retptr
  global.set $__stack_pointer
  local.get $retptr
  local.get $arg0_0
  local.get $arg1_0
  local.get $arg1_1
  call $raw (@reloc)
  local.get $retptr
  i64.load offset=0
  local.get $retptr
  i64.load offset=8
  local.get $retptr
  i32.const 16
  i32.add
  global.set $__stack_pointer
)"#
	);
	assert_eq!(
		js,
		r"(arg0, arg1) => {
    const ret = wasmExports['add'](arg0, arg1, arg1 >> 64n)
    return this.#jsEmbed.js_sys['numeric.u128.decode'](ret[0], ret[1])
}"
	);
}

#[test]
fn result() {
	let (wat, js) = expand(&quote! {
		pub fn checked_add(value: u128, delta: u128) -> Result<u128, JsValue> {
			value.checked_add(delta).ok_or(JsValue::UNDEFINED)
		}
	});

	inline_snap::inline_snap!(
		wat,
		"
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(import \"env\" \"js_sys.externref.release\" (func $js_sys.externref.release (@sym) (param i32)))
		(import \"env\" \"raw\" (func $raw (@sym (name \"__export_checked_add\")) (param i32) (param i64 \
		 i64) (param i64 i64)))
		(import \"env\" \"__stack_pointer\" (global $__stack_pointer (mut i32)))
		(func $export (@sym (name \"checked_add\")) (param $arg0_0 i64) (param $arg0_1 i64) (param \
		 $arg1_0 i64) (param $arg1_1 i64) (result i64 i64 i32 externref)
		  (local $retptr i32)
		  (local $js_sys.result.discriminant i32)
		  (local $js_sys.externref.index i32)
		  global.get $__stack_pointer
		  i32.const 32
		  i32.sub
		  local.tee $retptr
		  global.set $__stack_pointer
		  local.get $retptr
		  local.get $arg0_0
		  local.get $arg0_1
		  local.get $arg1_0
		  local.get $arg1_1
		  call $raw (@reloc)
		  local.get $retptr
		  i64.load offset=0
		  local.get $retptr
		  i64.load offset=8
		  local.get $retptr
		  i32.load offset=16
		  local.tee $js_sys.result.discriminant
		  local.get $retptr
		  i32.load offset=20
		  local.set $js_sys.externref.index
		  local.get $js_sys.result.discriminant
		  if (result externref)
		    local.get $js_sys.externref.index
		    table.get $js_sys.import.externref.table (@reloc)
		    local.get $js_sys.externref.index
		    i32.const 2
		    i32.ge_u
		    if
		      local.get $js_sys.externref.index
		      call $js_sys.externref.release (@reloc)
		    end
		  else
		    ref.null extern
		  end
		  local.get $retptr
		  i32.const 32
		  i32.add
		  global.set $__stack_pointer
		)"
	);
	assert_eq!(
		js,
		r"(arg0, arg1) => {
    const ret = wasmExports['checked_add'](arg0, arg0 >> 64n, arg1, arg1 >> 64n)
    if (ret[2] !== 0) throw ret[3]
    return this.#jsEmbed.js_sys['numeric.u128.decode'](ret[0], ret[1])
}"
	);
}

#[test]
fn single_slot_result() {
	let (wat, js) = expand(&quote! {
		pub fn checked_add(value: i32, delta: i32) -> Result<i32, JsValue> {
			value.checked_add(delta).ok_or(JsValue::UNDEFINED)
		}
	});

	inline_snap::inline_snap!(
		wat,
		"
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(import \"env\" \"js_sys.externref.release\" (func $js_sys.externref.release (@sym) (param i32)))
		(import \"env\" \"raw\" (func $raw (@sym (name \"__export_checked_add\")) (param i32) (param \
		 i32) (param i32)))
		(import \"env\" \"__stack_pointer\" (global $__stack_pointer (mut i32)))
		(func $export (@sym (name \"checked_add\")) (param $arg0_0 i32) (param $arg1_0 i32) (result i32 \
		 i32 externref)
		  (local $retptr i32)
		  (local $js_sys.result.discriminant i32)
		  (local $js_sys.externref.index i32)
		  global.get $__stack_pointer
		  i32.const 16
		  i32.sub
		  local.tee $retptr
		  global.set $__stack_pointer
		  local.get $retptr
		  local.get $arg0_0
		  local.get $arg1_0
		  call $raw (@reloc)
		  local.get $retptr
		  i32.load offset=0
		  local.get $retptr
		  i32.load offset=4
		  local.tee $js_sys.result.discriminant
		  local.get $retptr
		  i32.load offset=8
		  local.set $js_sys.externref.index
		  local.get $js_sys.result.discriminant
		  if (result externref)
		    local.get $js_sys.externref.index
		    table.get $js_sys.import.externref.table (@reloc)
		    local.get $js_sys.externref.index
		    i32.const 2
		    i32.ge_u
		    if
		      local.get $js_sys.externref.index
		      call $js_sys.externref.release (@reloc)
		    end
		  else
		    ref.null extern
		  end
		  local.get $retptr
		  i32.const 16
		  i32.add
		  global.set $__stack_pointer
		)"
	);
	assert_eq!(
		js,
		r"(arg0, arg1) => {
    const ret = wasmExports['checked_add'](arg0, arg1)
    if (ret[1] !== 0) throw ret[2]
    return ret[0]
}"
	);
}

#[test]
fn unit_result() {
	let (wat, js) = expand(&quote! {
		pub fn succeeds() -> Result<(), JsValue> {
			Ok(())
		}
	});

	inline_snap::inline_snap!(
		wat,
		"
		(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
		 \"js_sys.externref.table\")) 2 externref))
		(import \"env\" \"js_sys.externref.release\" (func $js_sys.externref.release (@sym) (param i32)))
		(import \"env\" \"raw\" (func $raw (@sym (name \"__export_succeeds\")) (param i32)))
		(import \"env\" \"__stack_pointer\" (global $__stack_pointer (mut i32)))
		(func $export (@sym (name \"succeeds\")) (result i32 externref)
		  (local $retptr i32)
		  (local $js_sys.result.discriminant i32)
		  (local $js_sys.externref.index i32)
		  global.get $__stack_pointer
		  i32.const 16
		  i32.sub
		  local.tee $retptr
		  global.set $__stack_pointer
		  local.get $retptr
		  call $raw (@reloc)
		  local.get $retptr
		  i32.load offset=0
		  local.tee $js_sys.result.discriminant
		  local.get $retptr
		  i32.load offset=4
		  local.set $js_sys.externref.index
		  local.get $js_sys.result.discriminant
		  if (result externref)
		    local.get $js_sys.externref.index
		    table.get $js_sys.import.externref.table (@reloc)
		    local.get $js_sys.externref.index
		    i32.const 2
		    i32.ge_u
		    if
		      local.get $js_sys.externref.index
		      call $js_sys.externref.release (@reloc)
		    end
		  else
		    ref.null extern
		  end
		  local.get $retptr
		  i32.const 16
		  i32.add
		  global.set $__stack_pointer
		)"
	);
	assert_eq!(
		js,
		r"() => {
    const ret = wasmExports['succeeds']()
    if (ret[0] !== 0) throw ret[1]
    return undefined
}"
	);
}

#[test]
fn no_parameters() {
	let (wat, js) = expand(&quote! {
		pub fn answer() -> u32 {
			42
		}
	});

	inline_snap::inline_snap!(
		wat,
		r#"
(import "env" "raw" (func $raw (@sym (name "__export_answer")) (result i32)))
(func $export (@sym (name "answer")) (result i32)
  call $raw (@reloc)
)"#
	);
	assert_eq!(
		js,
		r"() => {
    const ret = wasmExports['answer']()
    return ret >>> 0
}"
	);
}

#[test]
fn no_return_value() {
	let (wat, js) = expand(&quote! {
		pub fn nothing(value: u32) -> () {
			let _ = value;
		}
	});

	inline_snap::inline_snap!(
		wat,
		r#"
(import "env" "raw" (func $raw (@sym (name "__export_nothing")) (param i32)))
(func $export (@sym (name "nothing")) (param $arg0_0 i32)
  local.get $arg0_0
  call $raw (@reloc)
)"#
	);
	assert_eq!(
		js,
		r"(arg0) => {
    wasmExports['nothing'](arg0)
}"
	);
}
