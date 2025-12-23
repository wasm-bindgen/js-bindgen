#![no_std]

pub use js_sys;
use js_sys::JsValue;

pub mod console {
	use super::*;

	pub fn log(par1: &JsValue) {
		js_bindgen::embed_asm!(
			".functype js_sys.externref.get (i32) -> (externref)",
			"",
			".import_module web_sys.import.console.log, web_sys",
			".import_name web_sys.import.console.log, console.log",
			".functype web_sys.import.console.log (externref) -> ()",
			"",
			".globl web_sys.console.log",
			"web_sys.console.log:",
			"    .functype web_sys.console.log (i32) -> ()",
			"    local.get 0",
			"    call js_sys.externref.get",
			"    call web_sys.import.console.log",
			"    end_function",
		);

		extern "C" {
			#[link_name = "web_sys.console.log"]
			fn log(par1: i32);
		}

		unsafe { log(par1.as_raw()) };
	}
}
