use core::future::{Future, poll_fn};
use core::pin::Pin;
use core::task::{Context, Poll};

use super::iterator::{CapturedNext, async_iterator_next_cached, read_result};
use crate::builtins::iterator::async_iterator_from;
use crate::hazard::JsCast;
use crate::runtime::JsFuture;
use crate::{AsyncIterator, IteratorResult, JsValue};

/// A cancellation-safe asynchronous Rust iterator over the JavaScript `async`
/// iterator protocol.
pub struct AsyncIter<T = JsValue> {
	iterator: AsyncIterator<T>,
	next_method: CapturedNext,
	next: Option<JsFuture<IteratorResult>>,
	done: bool,
}

impl<T> From<AsyncIterator<T>> for AsyncIter<T> {
	fn from(iterator: AsyncIterator<T>) -> Self {
		Self::new(iterator)
	}
}

impl<T> AsyncIter<T> {
	fn new(iterator: AsyncIterator<T>) -> Self {
		let next_method = CapturedNext::new(iterator.as_ref());

		Self {
			iterator,
			next_method,
			next: None,
			done: false,
		}
	}

	fn try_new(iterator: AsyncIterator<T>) -> Result<Self, JsValue> {
		let next_method = CapturedNext::try_new(iterator.as_ref())?;

		Ok(Self {
			iterator,
			next_method,
			next: None,
			done: false,
		})
	}
}

impl<T: JsCast + 'static> AsyncIter<T> {
	/// Polls the next item without issuing concurrent JavaScript `next()`
	/// calls.
	pub fn poll_next(&mut self, context: &mut Context<'_>) -> Poll<Option<Result<T, JsValue>>> {
		if self.done {
			return Poll::Ready(None);
		}

		if self.next.is_none() {
			let method = match self.next_method.method() {
				Ok(method) => method,
				Err(error) => {
					self.done = true;
					return Poll::Ready(Some(Err(error)));
				}
			};
			let promise = match async_iterator_next_cached(&self.iterator, method) {
				Ok(promise) => promise,
				Err(error) => {
					self.done = true;
					return Poll::Ready(Some(Err(error)));
				}
			};
			self.next = Some(promise.into());
		}

		let Some(future) = self.next.as_mut() else {
			unreachable!();
		};
		let result = match Pin::new(future).poll(context) {
			Poll::Pending => return Poll::Pending,
			Poll::Ready(result) => result,
		};
		self.next = None;

		let result = match result {
			Ok(result) => result,
			Err(error) => {
				self.done = true;
				return Poll::Ready(Some(Err(error)));
			}
		};

		Poll::Ready(read_result(&result, &mut self.done))
	}

	/// Waits for the next item.
	///
	/// Dropping this future keeps an in-flight JavaScript `next()` operation in
	/// the iterator, so the following call resumes it instead of losing an
	/// item.
	#[must_use = "futures do nothing unless polled or awaited"]
	#[expect(
		clippy::should_implement_trait,
		reason = "there is no standard asynchronous Iterator trait"
	)]
	pub fn next(&mut self) -> impl Future<Output = Option<Result<T, JsValue>>> + '_ {
		poll_fn(|context| self.poll_next(context))
	}
}

impl<T: JsCast + 'static> AsyncIterator<T> {
	#[must_use]
	pub fn into_async_iter(self) -> AsyncIter<T> {
		self.into()
	}
}

/// Returns an asynchronous Rust iterator for an asynchronous or synchronous
/// JavaScript `iterable`.
///
/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for-await...of#description)
pub fn try_async_iter(value: &JsValue) -> Result<Option<AsyncIter>, JsValue> {
	async_iterator_from(value)?
		.map(AsyncIter::try_new)
		.transpose()
}
