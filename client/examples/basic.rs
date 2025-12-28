#![no_std]

extern crate alloc;

use core::arch::wasm32;

use mini_alloc::MiniAlloc;
use web_sys::console;
use web_sys::js_sys::JsString;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
	wasm32::unreachable()
}

#[global_allocator]
static ALLOC: MiniAlloc = MiniAlloc::INIT;

#[unsafe(no_mangle)]
extern "C" fn foo() {
	console::log(&JsString::from_str("Hello, World!"));
}
