#![no_std]
#![cfg_attr(target_arch = "wasm64", feature(simd_wasm64))]

#[cfg(target_arch = "wasm32")]
use core::arch::wasm32 as wasm;
#[cfg(target_arch = "wasm64")]
use core::arch::wasm64 as wasm;

use mini_alloc::MiniAlloc;
use web_sys::console;
use web_sys::js_sys::JsString;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
	wasm::unreachable();
}

#[global_allocator]
static ALLOC: MiniAlloc = MiniAlloc::INIT;

#[unsafe(no_mangle)]
extern "C" fn foo() {
	console::log(&JsString::from_str("Hello, World!"));
}
