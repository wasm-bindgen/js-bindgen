use core::fmt::{self, Formatter};

use super::{AsyncGenerator, Function, Generator, Object, Promise};
use crate::hazard::JsCast;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/GeneratorFunction)
	#[js_sys(js_name = "GeneratorFunction", extends = Function, extends = Object)]
	pub type GeneratorFunction<Y = JsValue, R = JsValue, N = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/call)
	#[js_sys(variadic, return_abi = Result<Generator, JsValue>)]
	pub fn call<Y, R, N>(
		self: &GeneratorFunction<Y, R, N>,
		this_arg: &JsValue,
		args: &[JsValue],
	) -> Result<Generator<Y, R, N>, JsValue>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncGeneratorFunction)
	#[js_sys(
		js_name = "AsyncGeneratorFunction",
		extends = Function,
		extends = Object
	)]
	pub type AsyncGeneratorFunction<Y = JsValue, R = JsValue, N = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/call)
	#[js_sys(variadic, return_abi = Result<AsyncGenerator, JsValue>)]
	pub fn call<Y, R, N>(
		self: &AsyncGeneratorFunction<Y, R, N>,
		this_arg: &JsValue,
		args: &[JsValue],
	) -> Result<AsyncGenerator<Y, R, N>, JsValue>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncFunction)
	#[js_sys(js_name = "AsyncFunction", extends = Function, extends = Object)]
	pub type AsyncFunction<R = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/call)
	#[js_sys(variadic, return_abi = Result<Promise, JsValue>)]
	pub fn call<R>(
		self: &AsyncFunction<R>,
		this_arg: &JsValue,
		args: &[JsValue],
	) -> Result<Promise<R>, JsValue>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(
		js_embed = "generator_function.new",
		return_abi = Result<Function, JsValue>
	)]
	fn generator_function_new<Y, R, N>(body: &str) -> Result<GeneratorFunction<Y, R, N>, JsValue>;

	#[js_sys(
		js_embed = "generator_function.new",
		return_abi = Result<Function, JsValue>
	)]
	fn generator_function_new_with_args<Y, R, N>(
		args: &str,
		body: &str,
	) -> Result<GeneratorFunction<Y, R, N>, JsValue>;

	#[js_sys(
		js_embed = "async_generator_function.new",
		return_abi = Result<Function, JsValue>
	)]
	fn async_generator_function_new<Y, R, N>(
		body: &str,
	) -> Result<AsyncGeneratorFunction<Y, R, N>, JsValue>;

	#[js_sys(
		js_embed = "async_generator_function.new",
		return_abi = Result<Function, JsValue>
	)]
	fn async_generator_function_new_with_args<Y, R, N>(
		args: &str,
		body: &str,
	) -> Result<AsyncGeneratorFunction<Y, R, N>, JsValue>;

	#[js_sys(
		js_embed = "async_function.new",
		return_abi = Result<Function, JsValue>
	)]
	fn async_function_new<R>(body: &str) -> Result<AsyncFunction<R>, JsValue>;

	#[js_sys(
		js_embed = "async_function.new",
		return_abi = Result<Function, JsValue>
	)]
	fn async_function_new_with_args<R>(args: &str, body: &str)
	-> Result<AsyncFunction<R>, JsValue>;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "generator_function.new",
	"(...args) => new ((function* () {{}}).constructor)(...args)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "async_generator_function.new",
	"(...args) => new ((async function* () {{}}).constructor)(...args)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "async_function.new",
	"(...args) => new ((async function () {{}}).constructor)(...args)",
);

impl<Y, R, N> GeneratorFunction<Y, R, N> {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/GeneratorFunction/GeneratorFunction)
	pub fn new(body: &str) -> Result<Self, JsValue> {
		generator_function_new(body)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/GeneratorFunction/GeneratorFunction)
	pub fn new_with_args(args: &str, body: &str) -> Result<Self, JsValue> {
		generator_function_new_with_args(args, body)
	}
}

impl<Y, R, N> AsyncGeneratorFunction<Y, R, N> {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncGeneratorFunction/AsyncGeneratorFunction)
	pub fn new(body: &str) -> Result<Self, JsValue> {
		async_generator_function_new(body)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncGeneratorFunction/AsyncGeneratorFunction)
	pub fn new_with_args(args: &str, body: &str) -> Result<Self, JsValue> {
		async_generator_function_new_with_args(args, body)
	}
}

impl<R> AsyncFunction<R> {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncFunction/AsyncFunction)
	pub fn new(body: &str) -> Result<Self, JsValue> {
		async_function_new(body)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncFunction/AsyncFunction)
	pub fn new_with_args(args: &str, body: &str) -> Result<Self, JsValue> {
		async_function_new_with_args(args, body)
	}
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

impl_wrapper!(GeneratorFunction<Y, R, N>);
impl_wrapper!(AsyncGeneratorFunction<Y, R, N>);
impl_wrapper!(AsyncFunction<R>);
