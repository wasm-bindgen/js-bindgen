#![feature(asm_experimental_arch)]

use js_bindgen::global_asm;
use js_sys::JsValue;
use js_sys::console;

global_asm!(
    ".import_module js_bindgen.externref.undefined, js_bindgen",
    ".import_name js_bindgen.externref.undefined, undefined",
    ".globaltype js_bindgen.externref.undefined, externref, immutable",
    #[cfg(target_pointer_width = "32")]
    ".functype js_sys.externref_table.set (i32, externref) -> ()",
    #[cfg(target_pointer_width = "64")]
    ".functype js_sys.externref_table.set (i64, externref) -> ()",
    #[cfg(target_pointer_width = "32")]
    ".functype js_sys.externref_table.grow (externref, i32) -> (i32)",
    #[cfg(target_pointer_width = "64")]
    ".functype js_sys.externref_table.grow (externref, i64) -> (i64)",
);

global_asm!(
    "test:",
    ".globl test",
    ".export_name test, test",
    ".functype test (externref) -> ()",
    "global.get js_bindgen.externref.undefined",
    #[cfg(target_pointer_width = "32")]
    "i32.const 128",
    #[cfg(target_pointer_width = "64")]
    "i64.const 128",
    "call js_sys.externref_table.grow",
    "drop",
    #[cfg(target_pointer_width = "32")]
    "i32.const 1",
    #[cfg(target_pointer_width = "64")]
    "i64.const 1",
    "local.get 0",
    "call js_sys.externref_table.set",
    "end_function",
);

#[unsafe(no_mangle)]
extern "C" fn foo() {
    console::log(&JsValue::UNDEFINED);
}
