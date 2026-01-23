pub use js_bindgen_test_macro::test;
use js_sys::JsString;
use std::{cell::RefCell, panic::PanicHookInfo};

struct LastPanic {
	payload: Option<JsString>,
	info: Option<JsString>,
}

thread_local! {
	static LAST_PANIC: RefCell<LastPanic> = const { RefCell::new(LastPanic {
		payload: None,
		info: None
	}) };
}

js_sys::js_bindgen::embed_js!(
	name = "panic.stack",
	"(message) => {{",
	"	const error = new Error(message);",
	"	return error.stack || String(error);",
	"}}"
);

#[js_sys::js_sys(js_sys = js_sys)]
extern "C" {
	#[js_sys(js_embed = "panic.stack")]
	fn panic_stack(message: &JsString) -> JsString;
}

pub fn set_panic_hook() {
	fn payload_as_str<'a>(info: &'a PanicHookInfo) -> Option<&'a str> {
		if let Some(s) = info.payload().downcast_ref::<&str>() {
			Some(s)
		} else if let Some(s) = info.payload().downcast_ref::<String>() {
			Some(s)
		} else {
			None
		}
	}

	static HOOK: std::sync::Once = std::sync::Once::new();
	HOOK.call_once(|| {
		std::panic::set_hook(Box::new(|info| {
			LAST_PANIC.with(|cell| {
				*cell.borrow_mut() = LastPanic {
					payload: payload_as_str(info).map(JsString::from_str),
					info: Some(panic_stack(&JsString::from_str(&info.to_string()))),
				}
			});
		}));
	});
}

#[unsafe(no_mangle)]
pub extern "C" fn last_panic_message() -> JsString {
	match LAST_PANIC.with(|cell| cell.borrow_mut().info.take()) {
		Some(info) => info,
		None => JsString::from_str(""),
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn last_panic_payload() -> JsString {
	match LAST_PANIC.with(|cell| cell.borrow_mut().payload.take()) {
		Some(payload) => payload,
		None => JsString::from_str(""),
	}
}
