use super::function::Function;
use super::object::Object;
use crate::{Closure, JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise)
	#[must_use]
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug)]
	pub type Promise<T = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/withResolvers)
	#[must_use]
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug)]
	pub type PromiseWithResolvers;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/Promise)
	#[js_sys(constructor)]
	pub fn new(executor: &Closure<dyn FnMut(Function, Function)>) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/all)
	#[js_sys(static_of = Promise)]
	pub fn all(iterable: &JsValue) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/allSettled)
	#[js_sys(static_of = Promise, js_name = "allSettled")]
	pub fn all_settled(iterable: &JsValue) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/any)
	#[js_sys(static_of = Promise)]
	pub fn any(iterable: &JsValue) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/race)
	#[js_sys(static_of = Promise)]
	pub fn race(iterable: &JsValue) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/reject)
	#[js_sys(static_of = Promise)]
	pub fn reject(reason: &JsValue) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/resolve)
	#[js_sys(static_of = Promise)]
	pub fn resolve(value: &JsValue) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/try)
	#[js_sys(static_of = Promise, js_name = "try", variadic)]
	pub fn try_(callback: &Function, args: &[JsValue]) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/withResolvers)
	#[js_sys(static_of = Promise, js_name = "withResolvers")]
	pub fn with_resolvers() -> PromiseWithResolvers;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/catch)
	pub fn catch<T>(self: &Promise<T>, handler: Closure<dyn FnMut(JsValue) -> JsValue>) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/finally)
	pub fn finally<T>(self: &Promise<T>, callback: Closure<dyn FnMut()>) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/then)
	pub fn then<T>(self: &Promise<T>, callback: Closure<dyn FnMut(JsValue) -> JsValue>) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/then)
	#[js_sys(js_name = "then")]
	pub fn then_with_reject<T>(
		self: &Promise<T>,
		resolve: Closure<dyn FnMut(JsValue) -> JsValue>,
		reject: Closure<dyn FnMut(JsValue) -> JsValue>,
	) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/withResolvers)
	#[js_sys(getter)]
	pub fn promise(self: &PromiseWithResolvers) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/withResolvers)
	#[must_use]
	#[js_sys(getter)]
	pub fn resolve(self: &PromiseWithResolvers) -> Function;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/withResolvers)
	#[must_use]
	#[js_sys(getter)]
	pub fn reject(self: &PromiseWithResolvers) -> Function;
}
