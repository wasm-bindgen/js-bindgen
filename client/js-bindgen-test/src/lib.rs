pub use js_bindgen_test_macro::test;
use js_sys::JsString;
use std::{cell::RefCell, panic::PanicHookInfo};

struct LastPanic {
	payload: Option<String>,
	info: Option<String>,
}

thread_local! {
	static LAST_PANIC: RefCell<LastPanic> = const { RefCell::new(LastPanic {
		payload: None,
		info: None
	}) };
}

pub fn set_panic_hook() {
	// TODO: Bump rustc to 1.91.0 and remove this func
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
					payload: payload_as_str(info).map(|s| s.to_owned()),
					info: Some(info.to_string()),
				}
			});
		}));
	});
}

#[unsafe(no_mangle)]
pub extern "C" fn last_panic_message() -> JsString {
	match LAST_PANIC.with(|cell| cell.borrow_mut().info.take()) {
		Some(info) => JsString::from_str(&info),
		None => JsString::from_str(""),
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn last_panic_payload() -> JsString {
	match LAST_PANIC.with(|cell| cell.borrow_mut().payload.take()) {
		Some(payload) => JsString::from_str(&payload),
		None => JsString::from_str(""),
	}
}
