use core::fmt::{self, Formatter};

use super::{AsyncIterable, AsyncIterator, Iterable, IteratorResult, JsIterator, Object, Promise};
use crate::hazard::JsCast;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator)
	#[js_sys(extends = JsIterator<Y>, extends = Object)]
	pub type Generator<Y = JsValue, R = JsValue, N = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/next)
	pub fn next<Y, R, N>(self: &Generator<Y, R, N>) -> Result<IteratorResult, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/next)
	#[js_sys(js_name = "next")]
	pub fn next_with<Y, R, N: JsCast>(
		self: &Generator<Y, R, N>,
		#[js_sys(type = &JsValue)] value: &N,
	) -> Result<IteratorResult, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/return)
	#[js_sys(js_name = "return")]
	pub fn return_<Y, R, N>(self: &Generator<Y, R, N>) -> Result<IteratorResult, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/return)
	#[js_sys(js_name = "return")]
	pub fn return_with<Y, R: JsCast, N>(
		self: &Generator<Y, R, N>,
		#[js_sys(type = &JsValue)] value: &R,
	) -> Result<IteratorResult, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/throw)
	#[js_sys(js_name = "throw")]
	pub fn throw<Y, R, N>(self: &Generator<Y, R, N>) -> Result<IteratorResult, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator/throw)
	#[js_sys(js_name = "throw")]
	pub fn throw_with<Y, R, N>(
		self: &Generator<Y, R, N>,
		error: &JsValue,
	) -> Result<IteratorResult, JsValue>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator)
	#[js_sys(extends = AsyncIterator<Y>, extends = Object)]
	pub type AsyncGenerator<Y = JsValue, R = JsValue, N = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator/next)
	pub fn next<Y, R, N>(self: &AsyncGenerator<Y, R, N>) -> Promise<IteratorResult>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator/next)
	#[js_sys(js_name = "next")]
	pub fn next_with<Y, R, N: JsCast>(
		self: &AsyncGenerator<Y, R, N>,
		#[js_sys(type = &JsValue)] value: &N,
	) -> Promise<IteratorResult>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator/return)
	#[js_sys(js_name = "return")]
	pub fn return_<Y, R, N>(self: &AsyncGenerator<Y, R, N>) -> Promise<IteratorResult>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator/return)
	#[js_sys(js_name = "return")]
	pub fn return_with<Y, R: JsCast, N>(
		self: &AsyncGenerator<Y, R, N>,
		#[js_sys(type = &JsValue)] value: &R,
	) -> Promise<IteratorResult>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator/throw)
	#[js_sys(js_name = "throw")]
	pub fn throw<Y, R, N>(self: &AsyncGenerator<Y, R, N>) -> Promise<IteratorResult>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncGenerator/throw)
	#[js_sys(js_name = "throw")]
	pub fn throw_with<Y, R, N>(
		self: &AsyncGenerator<Y, R, N>,
		error: &JsValue,
	) -> Promise<IteratorResult>;
}

macro_rules! impl_wrapper {
	($type:ident<$($generic:ident),+>) => {
		impl<$($generic),+> Clone for $type<$($generic),+> {
			fn clone(&self) -> Self {
				Self::unchecked_from(<Self as AsRef<JsValue>>::as_ref(self).clone())
			}
		}

		impl<$($generic),+> fmt::Debug for $type<$($generic),+> {
			fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
				fmt::Debug::fmt(<Self as AsRef<JsValue>>::as_ref(self), formatter)
			}
		}
	};
}

impl_wrapper!(Generator<Y, R, N>);
impl_wrapper!(AsyncGenerator<Y, R, N>);

impl<Y: JsCast, R, N> Iterable for Generator<Y, R, N> {
	type Item = Y;
}

impl<Y: JsCast, R, N> AsyncIterable for AsyncGenerator<Y, R, N> {
	type Item = Y;
}
