#![cfg_attr(target_feature = "atomics", feature(thread_local))]

extern crate alloc;

mod criterion;
mod time;

/// TODO: `no_std` support
use std::panic::{self, PanicHookInfo};
use std::sync::Once;

pub use criterion::{Criterion, Measurement, WallTime};
pub use js_bindgen_test_macro::test;
use js_sys::{JsString, js_sys};
pub use time::{Instant, SystemTime, UNIX_EPOCH};

#[macro_export]
macro_rules! criterion_group {
	($name:ident, $($target:path),+ $(,)?) => {
		fn $name(c: &mut $crate::Criterion) {
			$(
				$target(c);
			)+
		}
	};
}

#[macro_export]
macro_rules! criterion_main {
	($($group:path),+ $(,)?) => {
		fn main() {
			$crate::set_panic_hook();
			let mut bencher = $crate::Criterion::default().with_location(file!(), module_path!());
			$(
				$group(&mut bencher);
			)+
		}
	};
}

#[js_sys]
extern "js-sys" {
	#[js_sys(js_import)]
	fn set_message(message: &JsString);

	#[js_sys(js_import)]
	fn set_payload(payload: &JsString);
}

mod context {
	use js_sys::{JsString, js_sys};

	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_import)]
		pub(super) fn ctx() -> JsString;
	}
}

mod fs {
	use js_sys::{JsString, js_sys};

	#[js_sys]
	extern "js-sys" {
		#[js_sys(js_import)]
		pub(super) fn write_file(path: &JsString, data: &JsString);
	}
}

#[doc(hidden)]
pub fn set_panic_hook() {
	// MSRV: Stable on v1.91.
	fn payload_as_str<'a>(info: &'a PanicHookInfo) -> Option<&'a str> {
		if let Some(s) = info.payload().downcast_ref::<&str>() {
			Some(s)
		} else if let Some(s) = info.payload().downcast_ref::<String>() {
			Some(s)
		} else {
			None
		}
	}

	static HOOK: Once = Once::new();

	HOOK.call_once(|| {
		panic::set_hook(Box::new(|info| {
			let message = info.to_string();
			set_message(&JsString::from(message.as_str()));

			if let Some(payload) = payload_as_str(info) {
				set_payload(&JsString::from(payload));
			}
		}));
	});
}

pub mod console {
	use js_sys::{JsString, js_sys};

	#[js_sys(namespace = "console")]
	extern "js-sys" {
		pub fn log(data: &JsString);
		pub fn error(data: &JsString);
	}

	#[macro_export]
	macro_rules! console_log {
        ($($t:tt)*) => (
            $crate::console::log(
                &format_args!($($t)*).to_string().as_str().into()
            )
        )
    }

	#[macro_export]
	macro_rules! console_error {
        ($($t:tt)*) => (
            $crate::console::error(
                &format_args!($($t)*).to_string().as_str().into()
            )
        )
    }
}

// TODO: Move the following code into `xxx-shared` crate if needed.
pub(crate) mod utils {
	use core::ops::Deref;

	use once_cell::unsync::Lazy;

	pub(crate) struct ThreadLocalWrapper<T>(pub(crate) T);

	#[cfg(not(target_feature = "atomics"))]
	// SAFETY: In wasm targets without atomics there is no cross-thread access, so
	// treating this wrapper as `Sync` is equivalent to thread-local usage.
	unsafe impl<T> Sync for ThreadLocalWrapper<T> {}

	#[cfg(not(target_feature = "atomics"))]
	// SAFETY: In wasm targets without atomics there is no cross-thread transfer, so
	// treating this wrapper as `Send` is equivalent to thread-local usage.
	unsafe impl<T> Send for ThreadLocalWrapper<T> {}

	/// Wrapper around [`Lazy`] adding `Send + Sync` when `atomics` is not
	/// enabled.
	pub(crate) struct LazyCell<T, F = fn() -> T>(ThreadLocalWrapper<Lazy<T, F>>);

	impl<T, F> LazyCell<T, F> {
		pub const fn new(init: F) -> Self {
			Self(ThreadLocalWrapper(Lazy::new(init)))
		}
	}

	impl<T> Deref for LazyCell<T> {
		type Target = T;

		fn deref(&self) -> &T {
			Lazy::force(&self.0.0)
		}
	}
}
