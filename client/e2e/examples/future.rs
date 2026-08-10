#[rustfmt::skip]
fn main() {
	// ;; await (async () => { const value = {}; return await exports["future_to_promise_ok"](value) === value })()
	// ;; await (async () => { const value = {}; try { await exports["future_to_promise_err"](value); return false } catch (error) { return error === value } })()
	// ;; await (async () => { const value = {}; return await exports["promise_to_future"](Promise.resolve(value)) === value })()
	// ;; await exports["typed_promise_to_future"](Promise.resolve("typed")) === "typed"
	// ;; await (async () => { const value = {}; try { await exports["promise_to_future"](Promise.reject(value)); return false } catch (error) { return error === value } })()
	// ;; await (async () => { const value = {}; return await exports["shared_promise"](Promise.resolve(value)) === value })()
	// ;; await (async () => { exports["spawn_local_start"](); if (exports["spawn_local_done"]()) return false; await Promise.resolve(); return exports["spawn_local_done"]() })()
	// ;; await (async () => { const value = {}; return await exports["self_wake"](value) === value })()
	// ;; await (async () => { const { promise, resolve } = Promise.withResolvers(); exports["drop_js_future"](promise); resolve(); await Promise.resolve(); return true })()
}

use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{Context, Poll};

use js_sys::{JsFuture, JsString, JsValue, Promise, future_to_promise, js_sys, spawn_local};

static SPAWN_LOCAL_DONE: AtomicBool = AtomicBool::new(false);

struct YieldOnce(bool);

impl Future for YieldOnce {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
		if self.0 {
			Poll::Ready(())
		} else {
			self.0 = true;
			context.waker().wake_by_ref();
			Poll::Pending
		}
	}
}

#[js_sys]
fn future_to_promise_ok(value: JsValue) -> Promise {
	future_to_promise(async move { Ok(value) })
}

#[js_sys]
fn future_to_promise_err(error: JsValue) -> Promise {
	future_to_promise(async move { Err(error) })
}

#[js_sys]
fn promise_to_future(promise: Promise) -> Promise {
	future_to_promise(async move { promise.await })
}

#[js_sys]
fn typed_promise_to_future(promise: Promise<JsString>) -> Promise<JsString> {
	future_to_promise(async move { promise.await })
}

#[js_sys]
fn shared_promise(promise: Promise) -> Promise {
	let first = JsFuture::from(promise.clone());
	let second = JsFuture::from(promise);

	future_to_promise(async move {
		let first = first.await?;
		let second = second.await?;

		if first == second {
			Ok(first)
		} else {
			Err(JsValue::NULL)
		}
	})
}

#[js_sys]
fn spawn_local_start() {
	SPAWN_LOCAL_DONE.store(false, Ordering::Relaxed);
	spawn_local(async {
		SPAWN_LOCAL_DONE.store(true, Ordering::Relaxed);
	});
}

#[js_sys]
fn spawn_local_done() -> bool {
	SPAWN_LOCAL_DONE.load(Ordering::Relaxed)
}

#[js_sys]
fn self_wake(value: JsValue) -> Promise {
	future_to_promise(async move {
		YieldOnce(false).await;
		Ok(value)
	})
}

#[js_sys]
fn drop_js_future(promise: Promise) {
	drop(JsFuture::from(promise));
}
