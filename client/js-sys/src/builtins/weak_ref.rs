use core::fmt::{self, Formatter};

use super::Object;
use crate::hazard::JsCast;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakRef)
	#[js_sys(extends = Object)]
	pub type WeakRef<T = Object>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakRef/WeakRef)
	#[js_sys(constructor, return_abi = Result<WeakRef, JsValue>)]
	pub fn new<T: JsCast>(#[js_sys(type = &JsValue)] target: &T) -> Result<WeakRef<T>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakRef/deref)
	#[must_use]
	#[js_sys(return_abi = Option<JsValue>)]
	pub fn deref<T: JsCast>(self: &WeakRef<T>) -> Option<T>;
}

impl<T> Clone for WeakRef<T> {
	fn clone(&self) -> Self {
		Self::unchecked_from(<Self as AsRef<JsValue>>::as_ref(self).clone())
	}
}

impl<T> fmt::Debug for WeakRef<T> {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(<Self as AsRef<JsValue>>::as_ref(self), formatter)
	}
}

impl<T> PartialEq for WeakRef<T> {
	fn eq(&self, other: &Self) -> bool {
		<Self as AsRef<JsValue>>::as_ref(self) == <Self as AsRef<JsValue>>::as_ref(other)
	}
}
