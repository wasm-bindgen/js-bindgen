use std::panic::PanicHookInfo;

pub use js_bindgen_test_macro::test;
use js_sys::{JsString, js_sys};

#[js_sys]
extern "js-sys" {
	#[js_sys(js_import)]
	fn set_message(message: &JsString);

	#[js_sys(js_import)]
	fn set_payload(payload: &JsString);
}

pub fn set_panic_hook() {
	// TODO: Bump msrv rustc to 1.91.0 and remove this func
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
			let message = info.to_string();
			set_message(&JsString::from_str(&message));
			if let Some(payload) = payload_as_str(info) {
				set_payload(&JsString::from_str(payload));
			}
		}));
	});
}
