#[doc(hidden)]
pub extern crate js_sys;

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::panic::{self, PanicHookInfo};
use std::sync::Once;

pub use js_bindgen_test_macro::test;
use js_sys::{Closure, JsString, JsValue, Promise, js_sys};

js_sys::js_bindgen::embed_js!(
	module = "js_bindgen_test",
	name = "call",
	"callback => {{",
	"	try {{",
	"		callback()",
	"	}} catch (error) {{",
	"		return error ?? new Error('nullish exception')",
	"	}}",
	"}}",
);

#[js_sys]
extern "js-sys" {
	#[js_sys(js_import)]
	fn set_message(message: &JsString);

	#[js_sys(js_import)]
	fn set_payload(payload: &JsString);

	#[js_sys(js_embed = "call")]
	fn call(callback: &Closure<dyn FnMut()>) -> Option<JsValue>;
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

struct AsyncTest<F> {
	future: F,
}

struct PollState<'future, 'context, F> {
	future: Pin<&'future mut F>,
	context: &'future mut Context<'context>,
	output: Option<Poll<()>>,
}

impl<F: Future<Output = ()> + 'static> Future for AsyncTest<F> {
	type Output = Result<JsValue, JsValue>;

	fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
		// SAFETY: `future` remains pinned with its containing `AsyncTest`.
		let future = unsafe { Pin::new_unchecked(&mut self.as_mut().get_unchecked_mut().future) };
		let mut state = PollState {
			future,
			context,
			output: None,
		};
		let state = (&raw mut state).cast::<()>();
		let callback = js_sys::closure!(dyn FnMut(), move || {
			// SAFETY: `call` calls this closure synchronously and does not retain
			// it, so `state` still points to the live stack allocation above.
			let state = unsafe { &mut *state.cast::<PollState<'_, '_, F>>() };
			state.output = Some(state.future.as_mut().poll(state.context));
		});
		let error = call(&callback);
		// SAFETY: `call` has returned without retaining `callback`, and the
		// stack allocation remains live until this function returns.
		let output = unsafe { &mut *state.cast::<PollState<'_, '_, F>>() }.output;

		match (error, output) {
			(None, Some(Poll::Ready(()))) => Poll::Ready(Ok(JsValue::UNDEFINED)),
			(None, Some(Poll::Pending)) => Poll::Pending,
			(Some(error), None) => Poll::Ready(Err(error)),
			_ => unreachable!("invalid async test poll state"),
		}
	}
}

#[doc(hidden)]
pub fn async_test(future: impl Future<Output = ()> + 'static) -> Promise {
	set_panic_hook();
	js_sys::future_to_promise(AsyncTest { future })
}
