use super::{Function, Object};
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type FinalizationRegistry;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry/FinalizationRegistry)
	#[js_sys(constructor)]
	pub fn new(cleanup_callback: &Function) -> Result<FinalizationRegistry, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry/register)
	pub fn register(
		self: &FinalizationRegistry,
		target: &JsValue,
		held_value: &JsValue,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry/register)
	#[js_sys(js_name = "register")]
	pub fn register_with_token(
		self: &FinalizationRegistry,
		target: &JsValue,
		held_value: &JsValue,
		unregister_token: &JsValue,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry/unregister)
	pub fn unregister(
		self: &FinalizationRegistry,
		unregister_token: &JsValue,
	) -> Result<bool, JsValue>;
}
