use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};

use futures_util::task::AtomicWaker;

#[derive(Default)]
pub struct AtomicFlag {
	waker: AtomicWaker,
	set: AtomicBool,
}

impl AtomicFlag {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn signal(&self) {
		self.set.store(true, Ordering::Relaxed);
		self.waker.wake();
	}
}

impl Future for &AtomicFlag {
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		// Short-circuit.
		if self.set.load(Ordering::Relaxed) {
			return Poll::Ready(());
		}

		self.waker.register(cx.waker());

		// Need to check condition **after** `register()` to avoid a race condition that
		// would result in lost notifications.
		if self.set.load(Ordering::Relaxed) {
			Poll::Ready(())
		} else {
			Poll::Pending
		}
	}
}
