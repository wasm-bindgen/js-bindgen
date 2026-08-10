use alloc::alloc::{alloc, dealloc, handle_alloc_error, realloc};
use core::alloc::Layout;
use core::ptr::NonNull;

#[unsafe(export_name = "js_sys.memory.alloc")]
extern "C" fn allocate(size: usize, align: usize) -> *mut u8 {
	let layout = layout(size, align);
	if size == 0 {
		return core::ptr::without_provenance_mut(layout.align());
	}

	// SAFETY: `layout` is non-empty and valid.
	NonNull::new(unsafe { alloc(layout) })
		.unwrap_or_else(|| handle_alloc_error(layout))
		.as_ptr()
}

pub(super) fn allocate_slice<T>(len: usize) -> *mut T {
	let layout = slice_layout::<T>(len);
	allocate(layout.size(), layout.align()).cast()
}

#[unsafe(export_name = "js_sys.memory.realloc")]
unsafe extern "C" fn reallocate(
	ptr: *mut u8,
	old_size: usize,
	new_size: usize,
	align: usize,
) -> *mut u8 {
	if old_size == 0 {
		return allocate(new_size, align);
	}
	if new_size == 0 {
		// SAFETY: The caller transfers the allocation described by this layout.
		unsafe { dealloc(ptr, layout(old_size, align)) };
		return core::ptr::without_provenance_mut(align);
	}

	let old_layout = layout(old_size, align);
	let new_layout = layout(new_size, align);
	// SAFETY: The caller transfers the allocation described by `old_layout`;
	// the returned pointer owns the `resized` allocation.
	NonNull::new(unsafe { realloc(ptr, old_layout, new_size) })
		.unwrap_or_else(|| handle_alloc_error(new_layout))
		.as_ptr()
}

#[unsafe(export_name = "js_sys.memory.free")]
unsafe extern "C" fn release(ptr: *mut u8, size: usize, align: usize) {
	if size == 0 {
		return;
	}

	// SAFETY: The caller transfers the allocation described by this layout and
	// never uses it again.
	unsafe { dealloc(ptr, layout(size, align)) };
}

pub(super) unsafe fn release_slice<T>(ptr: *mut T, len: usize) {
	let layout = slice_layout::<T>(len);
	// SAFETY: The caller transfers the slice allocation described by `layout`.
	unsafe { release(ptr.cast(), layout.size(), layout.align()) };
}

#[inline]
fn layout(size: usize, align: usize) -> Layout {
	match Layout::from_size_align(size, align) {
		Ok(layout) => layout,
		Err(_) => handle_alloc_error(Layout::new::<u8>()),
	}
}

#[inline]
fn slice_layout<T>(len: usize) -> Layout {
	match Layout::array::<T>(len) {
		Ok(layout) => layout,
		Err(_) => handle_alloc_error(Layout::new::<T>()),
	}
}
