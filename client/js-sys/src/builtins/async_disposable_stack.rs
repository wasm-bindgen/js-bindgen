use super::{Function, Object, Promise};
use crate::hazard::JsCast;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncDisposableStack)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type AsyncDisposableStack;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncDisposableStack/AsyncDisposableStack)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> AsyncDisposableStack;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncDisposableStack/adopt)
	#[js_sys(return_abi = Result<JsValue, JsValue>)]
	pub fn adopt<T: JsCast>(
		self: &AsyncDisposableStack,
		#[js_sys(type = &JsValue)] value: &T,
		on_dispose_async: &Function,
	) -> Result<T, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncDisposableStack/defer)
	pub fn defer(self: &AsyncDisposableStack, on_dispose_async: &Function) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncDisposableStack/disposeAsync)
	#[js_sys(js_name = "disposeAsync")]
	pub fn dispose_async(self: &AsyncDisposableStack) -> Promise;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncDisposableStack/disposed)
	#[must_use]
	#[js_sys(getter)]
	pub fn disposed(self: &AsyncDisposableStack) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncDisposableStack/move)
	#[js_sys(js_name = "move")]
	pub fn move_(self: &AsyncDisposableStack) -> Result<AsyncDisposableStack, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncDisposableStack/use)
	#[js_sys(js_name = "use", return_abi = Result<JsValue, JsValue>)]
	pub fn use_<T: JsCast>(
		self: &AsyncDisposableStack,
		#[js_sys(type = &JsValue)] value: &T,
	) -> Result<T, JsValue>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "async_disposable_stack.symbol_async_dispose")]
	fn async_disposable_stack_symbol_async_dispose(stack: &AsyncDisposableStack) -> Promise;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "async_disposable_stack.symbol_async_dispose",
	"(stack) => stack[Symbol.asyncDispose]()",
);

impl AsyncDisposableStack {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncDisposableStack/Symbol.asyncDispose)
	pub fn symbol_async_dispose(&self) -> Promise {
		async_disposable_stack_symbol_async_dispose(self)
	}
}

impl Default for AsyncDisposableStack {
	fn default() -> Self {
		Self::new()
	}
}
