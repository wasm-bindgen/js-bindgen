#![no_std]
#![cfg_attr(target_arch = "wasm64", feature(simd_wasm64))]

extern crate alloc;

use mini_alloc::MiniAlloc;
use web_sys::console;
use web_sys::js_sys::JsString;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
	#[cfg(target_arch = "wasm32")]
	core::arch::wasm32::unreachable();
	#[cfg(target_arch = "wasm64")]
	core::arch::wasm64::unreachable();
}

#[global_allocator]
static ALLOC: MiniAlloc = MiniAlloc::INIT;

#[unsafe(no_mangle)]
extern "C" fn foo() {
	console::log(&JsString::from_str("Hello, World!"));
}
