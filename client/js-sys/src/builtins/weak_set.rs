use core::fmt::{self, Formatter};

use super::{Iterable, Object};
use crate::hazard::JsCast;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet)
	#[js_sys(extends = Object)]
	pub type WeakSet<T = Object>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/WeakSet)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> WeakSet;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/WeakSet)
	#[must_use]
	#[js_sys(constructor, return_abi = WeakSet)]
	pub fn new_typed<T>() -> WeakSet<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/WeakSet)
	#[js_sys(constructor, return_abi = Result<WeakSet, JsValue>)]
	pub fn new_from_iterable<T: JsCast, I: Iterable<Item = T>>(
		#[js_sys(type = &JsValue)] values: &I,
	) -> Result<WeakSet<T>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/add)
	#[js_sys(return_abi = Result<WeakSet, JsValue>)]
	pub fn add<T: JsCast>(
		self: &WeakSet<T>,
		#[js_sys(type = &JsValue)] value: &T,
	) -> Result<WeakSet<T>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/delete)
	#[must_use]
	pub fn delete<T: JsCast>(self: &WeakSet<T>, #[js_sys(type = &JsValue)] value: &T) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet/has)
	#[must_use]
	pub fn has<T: JsCast>(self: &WeakSet<T>, #[js_sys(type = &JsValue)] value: &T) -> bool;
}

impl<T> Clone for WeakSet<T> {
	fn clone(&self) -> Self {
		Self::unchecked_from(<Self as AsRef<JsValue>>::as_ref(self).clone())
	}
}

impl<T> fmt::Debug for WeakSet<T> {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(<Self as AsRef<JsValue>>::as_ref(self), formatter)
	}
}

impl<T> PartialEq for WeakSet<T> {
	fn eq(&self, other: &Self) -> bool {
		<Self as AsRef<JsValue>>::as_ref(self) == <Self as AsRef<JsValue>>::as_ref(other)
	}
}

impl<T> Default for WeakSet<T> {
	fn default() -> Self {
		Self::new_typed()
	}
}
