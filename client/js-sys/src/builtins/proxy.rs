use super::{Function, Object};
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy)
	#[js_sys(js_name = "Proxy", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Proxy;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/revocable)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type ProxyRevocable;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/Proxy)
	#[js_sys(constructor)]
	pub fn new(target: &JsValue, handler: &JsValue) -> Result<Proxy, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/revocable)
	#[js_sys(static_of = Proxy)]
	pub fn revocable(target: &JsValue, handler: &JsValue) -> Result<ProxyRevocable, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/revocable)
	#[must_use]
	#[js_sys(getter)]
	pub fn proxy(self: &ProxyRevocable) -> Proxy;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/revocable)
	#[must_use]
	#[js_sys(getter)]
	pub fn revoke(self: &ProxyRevocable) -> Function;
}
