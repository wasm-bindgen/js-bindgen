#![feature(asm_experimental_arch)]
#![no_std]

use core::marker::PhantomData;

use js_bindgen::global_asm;

global_asm!(
    "js_sys.externref_table:",
    ".tabletype js_sys.externref_table, externref",
    ".export_name js_sys.externref_table, js_sys.externref_table"
);

global_asm!(
    ".import_module js_bindgen.externref.undefined, js_bindgen",
    ".import_name js_bindgen.externref.undefined, undefined",
    ".globaltype js_bindgen.externref.undefined, externref, immutable"
);

global_asm!(
    "js_sys.externref_table.grow:",
    ".globl js_sys.externref_table.grow",
    #[cfg(target_pointer_width = "32")]
    ".functype js_sys.externref_table.grow (externref, i32) -> (i32)",
    #[cfg(target_pointer_width = "64")]
    ".functype js_sys.externref_table.grow (externref, i64) -> (i64)",
    "local.get 0",
    "local.get 1",
    "table.grow js_sys.externref_table",
    "end_function",
);

global_asm!(
    "js_sys.externref_table.set:",
    ".globl js_sys.externref_table.set",
    #[cfg(target_pointer_width = "32")]
    ".functype js_sys.externref_table.set (i32, externref) -> ()",
    #[cfg(target_pointer_width = "64")]
    ".functype js_sys.externref_table.set (i64, externref) -> ()",
    "local.get 0",
    "local.get 1",
    "table.set js_sys.externref_table",
    "end_function",
);

global_asm!(
    "js_sys.externref_table.get:",
    ".globl js_sys.externref_table.get",
    #[cfg(target_pointer_width = "32")]
    ".functype js_sys.externref_table.get (i32) -> (externref)",
    #[cfg(target_pointer_width = "64")]
    ".functype js_sys.externref_table.get (i64) -> (externref)",
    "local.get 0",
    "table.get js_sys.externref_table",
    "end_function",
);

pub struct JsValue {
    index: isize,
    _local: PhantomData<*const ()>,
}

impl JsValue {
    const UNDEFINED: Self = Self::new(-1);

    const fn new(index: isize) -> Self {
        Self {
            index,
            _local: PhantomData,
        }
    }
}

impl Drop for JsValue {
    fn drop(&mut self) {
        if self.index >= 0 {

        }
    }
}

pub mod console {
    use crate::JsValue;

    pub fn log(par1: &JsValue) {}
}
