use core::future::{Future, IntoFuture};
use core::task::{Context, Poll};

#[cfg(target_feature = "atomics")]
mod atomic;
#[cfg(not(target_feature = "atomics"))]
mod single;

#[cfg(target_feature = "atomics")]
use atomic::Signal;
#[cfg(not(target_feature = "atomics"))]
use single::Signal;

const POLLING: i32 = 0;
const WAITING: i32 = 1;
const AWAKE: i32 = 2;

/// Runs a future to completion by suspending the current Wasm stack with
/// `JSPI`.
///
/// The dynamic call into Wasm must enter through an export using
/// `#[js_sys(promising)]`. The `js-bindgen` runner marks binary entry points
/// automatically.
pub fn block_on<F: IntoFuture>(future: F) -> F::Output {
	let mut future = core::pin::pin!(future.into_future());
	let signal = Signal::new();
	let waker = signal.waker();
	let mut context = Context::from_waker(&waker);

	loop {
		signal.begin_poll();

		if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
			return output;
		}

		if signal.begin_wait() {
			signal.suspend();
		}
	}
}
