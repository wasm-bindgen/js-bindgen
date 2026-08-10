use crate::builtins::Intl::{SegmentData, Segments};
use crate::builtins::{
	Array, AsyncIterator, Function, IteratorResult, JsIterator, JsString, Map, Promise, Set,
	iterator_from,
};
use crate::hazard::JsCast;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "iterator.next.method")]
	fn iterator_next_method(iterator: &JsValue) -> Result<Function, JsValue>;

	#[js_sys(js_embed = "iterator.next.cached")]
	fn iterator_next_cached<T>(
		#[js_sys(type = &JsValue)] iterator: &JsIterator<T>,
		next: &Function,
	) -> Result<IteratorResult, JsValue>;

	#[js_sys(js_embed = "async_iterator.next.cached")]
	pub(super) fn async_iterator_next_cached<T>(
		#[js_sys(type = &JsValue)] iterator: &AsyncIterator<T>,
		next: &Function,
	) -> Result<Promise<IteratorResult>, JsValue>;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "iterator.next.method",
	"(iterator) => {{",
	"    const method = iterator.next",
	"    if (typeof method !== 'function')",
	"        throw new TypeError('iterator does not provide a next method')",
	"    return method",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "iterator.next.cached",
	"(iterator, next) => {{",
	"    const result = next.call(iterator)",
	"    if (result == null || (typeof result !== 'object' && typeof result !== 'function'))",
	"        throw new TypeError('iterator next method returned a non-object value')",
	"    return result",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "async_iterator.next.cached",
	"(iterator, next) => Promise.resolve(next.call(iterator)).then(result => {{",
	"    if (result == null || (typeof result !== 'object' && typeof result !== 'function'))",
	"        throw new TypeError('async iterator next method returned a non-object value')",
	"    return result",
	"}})",
);

pub(super) enum CapturedNext {
	Method(Function),
	Error(Option<JsValue>),
}

impl CapturedNext {
	pub(super) fn new(iterator: &JsValue) -> Self {
		match iterator_next_method(iterator) {
			Ok(method) => Self::Method(method),
			Err(error) => Self::Error(Some(error)),
		}
	}

	pub(super) fn try_new(iterator: &JsValue) -> Result<Self, JsValue> {
		Ok(Self::Method(iterator_next_method(iterator)?))
	}

	pub(super) fn method(&mut self) -> Result<&Function, JsValue> {
		match self {
			Self::Method(method) => Ok(method),
			Self::Error(error) => Err(error
				.take()
				.expect("captured iterator next error must only be observed once")),
		}
	}
}

pub(super) fn read_result<T: JsCast>(
	result: &IteratorResult,
	done: &mut bool,
) -> Option<Result<T, JsValue>> {
	match result.done() {
		Ok(true) => {
			*done = true;
			None
		}
		Ok(false) => {
			let value = result.value().map(T::unchecked_from);
			if value.is_err() {
				*done = true;
			}
			Some(value)
		}
		Err(error) => {
			*done = true;
			Some(Err(error))
		}
	}
}

/// A borrowed Rust iterator over the JavaScript iterator protocol.
pub struct JsIter<'a, T = JsValue> {
	iterator: &'a JsIterator<T>,
	next: CapturedNext,
	done: bool,
}

/// An owned Rust iterator over the JavaScript iterator protocol.
pub struct JsIntoIter<T = JsValue> {
	iterator: JsIterator<T>,
	next: CapturedNext,
	done: bool,
}

fn next<T: JsCast>(
	iterator: &JsIterator<T>,
	next: &mut CapturedNext,
	done: &mut bool,
) -> Option<Result<T, JsValue>> {
	if *done {
		return None;
	}

	let method = match next.method() {
		Ok(method) => method,
		Err(error) => {
			*done = true;
			return Some(Err(error));
		}
	};
	let result = match iterator_next_cached(iterator, method) {
		Ok(result) => result,
		Err(error) => {
			*done = true;
			return Some(Err(error));
		}
	};

	read_result(&result, done)
}

impl<T: JsCast> JsIterator<T> {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols)
	#[must_use]
	pub fn iter(&self) -> JsIter<'_, T> {
		JsIter {
			iterator: self,
			next: CapturedNext::new(self.as_ref()),
			done: false,
		}
	}
}

impl<'a, T: JsCast> IntoIterator for &'a JsIterator<T> {
	type Item = Result<T, JsValue>;
	type IntoIter = JsIter<'a, T>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<T: JsCast> core::iter::Iterator for JsIter<'_, T> {
	type Item = Result<T, JsValue>;

	fn next(&mut self) -> Option<Self::Item> {
		next(self.iterator, &mut self.next, &mut self.done)
	}
}

impl<T: JsCast> core::iter::FusedIterator for JsIter<'_, T> {}

impl<T: JsCast> IntoIterator for JsIterator<T> {
	type Item = Result<T, JsValue>;
	type IntoIter = JsIntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		JsIntoIter::new(self)
	}
}

impl<T> JsIntoIter<T> {
	fn new(iterator: JsIterator<T>) -> Self {
		let next = CapturedNext::new(iterator.as_ref());

		Self {
			iterator,
			next,
			done: false,
		}
	}

	fn try_new(iterator: JsIterator<T>) -> Result<Self, JsValue> {
		let next = CapturedNext::try_new(iterator.as_ref())?;

		Ok(Self {
			iterator,
			next,
			done: false,
		})
	}
}

impl<T: JsCast> core::iter::Iterator for JsIntoIter<T> {
	type Item = Result<T, JsValue>;

	fn next(&mut self) -> Option<Self::Item> {
		next(&self.iterator, &mut self.next, &mut self.done)
	}
}

impl<T: JsCast> core::iter::FusedIterator for JsIntoIter<T> {}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols)
pub fn try_iter(value: &JsValue) -> Result<Option<JsIntoIter>, JsValue> {
	iterator_from(value)?.map(JsIntoIter::try_new).transpose()
}

impl JsString {
	#[must_use]
	pub fn iter(&self) -> JsIntoIter<Self> {
		self.iterator().into_iter()
	}
}

impl IntoIterator for &JsString {
	type Item = Result<JsString, JsValue>;
	type IntoIter = JsIntoIter<JsString>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl IntoIterator for JsString {
	type Item = Result<Self, JsValue>;
	type IntoIter = JsIntoIter<Self>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<K, V> Map<K, V> {
	#[must_use]
	pub fn iter(&self) -> JsIntoIter<Array> {
		self.symbol_iterator().into_iter()
	}
}

impl<K, V> IntoIterator for &Map<K, V> {
	type Item = Result<Array, JsValue>;
	type IntoIter = JsIntoIter<Array>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<K, V> IntoIterator for Map<K, V> {
	type Item = Result<Array, JsValue>;
	type IntoIter = JsIntoIter<Array>;

	fn into_iter(self) -> Self::IntoIter {
		self.symbol_iterator().into_iter()
	}
}

impl<T: JsCast> Set<T> {
	#[must_use]
	pub fn iter(&self) -> JsIntoIter<T> {
		self.symbol_iterator().into_iter()
	}
}

impl<T: JsCast> IntoIterator for &Set<T> {
	type Item = Result<T, JsValue>;
	type IntoIter = JsIntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<T: JsCast> IntoIterator for Set<T> {
	type Item = Result<T, JsValue>;
	type IntoIter = JsIntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		self.symbol_iterator().into_iter()
	}
}

impl Segments {
	#[must_use]
	pub fn iter(&self) -> JsIntoIter<SegmentData> {
		self.symbol_iterator().into_iter()
	}
}

impl IntoIterator for &Segments {
	type Item = Result<SegmentData, JsValue>;
	type IntoIter = JsIntoIter<SegmentData>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl IntoIterator for Segments {
	type Item = Result<SegmentData, JsValue>;
	type IntoIter = JsIntoIter<SegmentData>;

	fn into_iter(self) -> Self::IntoIter {
		self.symbol_iterator().into_iter()
	}
}
