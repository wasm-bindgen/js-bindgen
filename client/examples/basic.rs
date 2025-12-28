#![no_std]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::arch::wasm32;

use web_sys::{console, js_sys};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
	wasm32::unreachable()
}

#[global_allocator]
static ALLOCATOR: Global = Global;

struct Global;

unsafe impl GlobalAlloc for Global {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		alloc::alloc::alloc(layout)
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		alloc::alloc::dealloc(ptr, layout);
	}

	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
		alloc::alloc::alloc_zeroed(layout)
	}

	unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
		alloc::alloc::realloc(ptr, layout, new_size)
	}
}

#[unsafe(no_mangle)]
extern "C" fn foo() {
	console::log(&js_sys::is_nan());
}
