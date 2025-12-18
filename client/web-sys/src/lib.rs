#![feature(asm_experimental_arch)]
#![no_std]

pub use js_sys;

use js_bindgen::global_asm;

use js_sys::JsValue;

pub mod console {

    use super::*;

    global_asm!(
        ".import_module web_sys.import.console.log, web_sys",
        ".import_name web_sys.import.console.log, console.log",
        ".functype web_sys.import.console.log (externref) -> ()",
    );

    global_asm!(
        ".functype js_sys.externref.get (i32) -> (externref)",
        "web_sys.console.log:",
        ".globl web_sys.console.log",
        #[cfg(target_pointer_width = "32")]
        ".functype web_sys.console.log (i32) -> ()",
        #[cfg(target_pointer_width = "64")]
        ".functype web_sys.console.log (i64) -> ()",
        "local.get 0",
        "call js_sys.externref.get",
        "call web_sys.import.console.log",
        "end_function",
    );

    pub fn log(par1: &JsValue) {
        extern "C" {
            #[link_name = "web_sys.console.log"]
            fn log(par1: isize);
        }

        unsafe { log(par1.as_raw()) };
    }
}
