use core::cell::Cell;

use crate::JsValue;
#[cfg(not(target_feature = "exception-handling"))]
use crate::externref;

#[cfg(target_feature = "exception-handling")]
js_bindgen::import_js!(
	module = "js_sys",
	name = "exception.tag",
	"WebAssembly.JSTag",
);

thread_local! {
	static EXCEPTION: Cell<i32> = const { Cell::new(0) };
}

fn set(index: i32) {
	EXCEPTION.with(|exception| {
		debug_assert_eq!(exception.get(), 0);
		exception.set(index);
	});
}

/// Stores the `externref` table index for an exception caught by Wasm.
#[cfg(target_feature = "exception-handling")]
#[unsafe(export_name = "js_sys.exception.store")]
extern "C" fn store(index: i32) {
	set(index);
}

/// Reserves an `externref` table entry for an exception caught by JavaScript.
///
/// JavaScript fills the returned table entry before returning to Wasm.
#[cfg(not(target_feature = "exception-handling"))]
#[unsafe(export_name = "js_sys.exception.store")]
extern "C" fn store() -> i32 {
	let index = externref::reserve();
	set(index);
	index
}

pub(crate) fn take() -> Option<JsValue> {
	let index = EXCEPTION.with(Cell::take);
	(index != 0).then(|| JsValue::new(index))
}
