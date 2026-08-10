//! Bridges JavaScript promises and Rust futures.

mod queue;
mod task;

use alloc::rc::{Rc, Weak};
use core::cell::RefCell;
use core::future::{Future, IntoFuture};
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use core::{fmt, mem};

use crate::hazard::JsCast;
use crate::{Closure, JsValue, Promise, PromiseWithResolvers};

js_bindgen::embed_js!(
	module = "js_sys",
	name = "future.observe",
	"(promise, callback) => {{",
	"	promise.then(",
	"		value => {{",
	"			try {{ callback(true, value) }} finally {{ callback.unref() }}",
	"		}},",
	"		error => {{",
	"			try {{ callback(false, error) }} finally {{ callback.unref() }}",
	"		}},",
	"	)",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "future.settle",
	"(resolvers, resolved, value) => {{",
	"	resolvers[resolved ? 'resolve' : 'reject'](value)",
	"}}",
);

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "future.observe")]
	fn observe(promise: &JsValue, callback: Closure<dyn Fn(bool, JsValue)>);

	#[js_sys(js_embed = "future.settle")]
	fn settle(resolvers: PromiseWithResolvers, resolved: bool, value: JsValue);
}

enum State<T> {
	Pending {
		waker: Option<Waker>,
		// Keep the `Promise` and its reaction callbacks alive while Rust waits.
		_promise: JsValue,
	},
	Ready(Result<T, JsValue>),
	Done,
}

impl<T> State<T> {
	fn finish(state: &Weak<RefCell<Self>>, result: Result<T, JsValue>) {
		let Some(state) = state.upgrade() else {
			return;
		};

		let waker = {
			let mut state = state.borrow_mut();
			let Self::Pending { waker, .. } = &mut *state else {
				return;
			};
			let waker = waker.take();
			*state = Self::Ready(result);
			waker
		};

		if let Some(waker) = waker {
			waker.wake();
		}
	}
}

/// A Rust [`Future`] backed by a JavaScript [`Promise`].
///
/// Fulfillment produces `T`; rejection produces [`JsValue`].
#[must_use = "futures do nothing unless polled or awaited"]
pub struct JsFuture<T = JsValue> {
	state: Rc<RefCell<State<T>>>,
}

impl<T> fmt::Debug for JsFuture<T> {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
		formatter.write_str("JsFuture { .. }")
	}
}

impl<T: JsCast + 'static> From<Promise<T>> for JsFuture<T> {
	fn from(promise: Promise<T>) -> Self {
		let promise = <Promise<T> as AsRef<JsValue>>::as_ref(&promise);
		let state = Rc::new(RefCell::new(State::Pending {
			waker: None,
			_promise: promise.clone(),
		}));
		let callback_state = Rc::downgrade(&state);
		let callback = crate::closure!(
			js_sys = crate,
			dyn Fn(bool, JsValue),
			move |resolved, value| {
				let result = if resolved {
					Ok(T::unchecked_from(value))
				} else {
					Err(value)
				};
				State::finish(&callback_state, result);
			}
		);

		observe(promise, callback);

		Self { state }
	}
}

impl<T> Future for JsFuture<T> {
	type Output = Result<T, JsValue>;

	fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
		let mut state = self.state.borrow_mut();

		match &mut *state {
			State::Pending { waker, .. } => {
				if waker
					.as_ref()
					.is_none_or(|waker| !waker.will_wake(context.waker()))
				{
					*waker = Some(context.waker().clone());
				}

				Poll::Pending
			}
			State::Ready(_) => {
				let State::Ready(result) = mem::replace(&mut *state, State::Done) else {
					unreachable!();
				};
				Poll::Ready(result)
			}
			State::Done => panic!("`JsFuture` polled after completion"),
		}
	}
}

impl<T: JsCast + 'static> IntoFuture for Promise<T> {
	type Output = Result<T, JsValue>;
	type IntoFuture = JsFuture<T>;

	fn into_future(self) -> Self::IntoFuture {
		self.into()
	}
}

/// Runs a future on the current JavaScript thread.
///
/// The first poll always runs on the next `microtask`.
#[inline]
pub fn spawn_local(future: impl Future<Output = ()> + 'static) {
	task::spawn(future);
}

/// Converts a Rust future into a JavaScript [`Promise`].
///
/// `Ok` fulfills the promise and `Err` rejects it.
pub fn future_to_promise<T>(
	future: impl Future<Output = Result<T, JsValue>> + 'static,
) -> Promise<T>
where
	T: JsCast + Into<JsValue> + 'static,
{
	let resolvers = Promise::with_resolvers();
	// This function is the only producer of the `resolver` object's successful
	// value.
	let promise = Promise::<T>::unchecked_from(resolvers.promise().into());

	spawn_local(async move {
		let (resolved, value) = match future.await {
			Ok(value) => (true, value.into()),
			Err(error) => (false, error),
		};
		settle(resolvers, resolved, value);
	});

	promise
}
