use super::{Function, Object};
use crate::hazard::JsCast;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DisposableStack)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DisposableStack;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DisposableStack/DisposableStack)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> DisposableStack;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DisposableStack/adopt)
	#[js_sys(return_abi = Result<JsValue, JsValue>)]
	pub fn adopt<T: JsCast>(
		self: &DisposableStack,
		#[js_sys(type = &JsValue)] value: &T,
		on_dispose: &Function,
	) -> Result<T, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DisposableStack/defer)
	pub fn defer(self: &DisposableStack, on_dispose: &Function) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DisposableStack/dispose)
	pub fn dispose(self: &DisposableStack) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DisposableStack/disposed)
	#[must_use]
	#[js_sys(getter)]
	pub fn disposed(self: &DisposableStack) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DisposableStack/move)
	#[js_sys(js_name = "move")]
	pub fn move_(self: &DisposableStack) -> Result<DisposableStack, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DisposableStack/use)
	#[js_sys(js_name = "use", return_abi = Result<JsValue, JsValue>)]
	pub fn use_<T: JsCast>(
		self: &DisposableStack,
		#[js_sys(type = &JsValue)] value: &T,
	) -> Result<T, JsValue>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "disposable_stack.symbol_dispose")]
	fn disposable_stack_symbol_dispose(stack: &DisposableStack) -> Result<(), JsValue>;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "disposable_stack.symbol_dispose",
	"(stack) => stack[Symbol.dispose]()",
);

impl DisposableStack {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DisposableStack/Symbol.dispose)
	pub fn symbol_dispose(&self) -> Result<(), JsValue> {
		disposable_stack_symbol_dispose(self)
	}
}

impl Default for DisposableStack {
	fn default() -> Self {
		Self::new()
	}
}
