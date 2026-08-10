use core::fmt::{self, Formatter};

use super::{Function, Iterable, Object};
use crate::hazard::JsCast;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap)
	#[js_sys(extends = Object)]
	pub type WeakMap<K = Object, V = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/WeakMap)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> WeakMap;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/WeakMap)
	#[must_use]
	#[js_sys(constructor, return_abi = WeakMap)]
	pub fn new_typed<K, V>() -> WeakMap<K, V>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/WeakMap)
	#[js_sys(constructor, return_abi = Result<WeakMap, JsValue>)]
	pub fn new_from_iterable<K, V, I: Iterable>(
		#[js_sys(type = &JsValue)] entries: &I,
	) -> Result<WeakMap<K, V>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/delete)
	#[must_use]
	pub fn delete<K: JsCast, V>(self: &WeakMap<K, V>, #[js_sys(type = &JsValue)] key: &K) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/get)
	#[must_use]
	pub fn get<K: JsCast, V>(self: &WeakMap<K, V>, #[js_sys(type = &JsValue)] key: &K) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/get)
	#[must_use]
	#[js_sys(js_name = "get", return_abi = Option<JsValue>)]
	pub fn get_checked<K: JsCast, V: JsCast>(
		self: &WeakMap<K, V>,
		#[js_sys(type = &JsValue)] key: &K,
	) -> Option<V>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/getOrInsert)
	#[js_sys(js_name = "getOrInsert", return_abi = Result<JsValue, JsValue>)]
	pub fn get_or_insert<K: JsCast, V: JsCast>(
		self: &WeakMap<K, V>,
		#[js_sys(type = &JsValue)] key: &K,
		#[js_sys(type = &JsValue)] default_value: &V,
	) -> Result<V, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/getOrInsertComputed)
	#[js_sys(
		js_name = "getOrInsertComputed",
		return_abi = Result<JsValue, JsValue>
	)]
	pub fn get_or_insert_computed<K: JsCast, V: JsCast>(
		self: &WeakMap<K, V>,
		#[js_sys(type = &JsValue)] key: &K,
		callback: &Function,
	) -> Result<V, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/has)
	#[must_use]
	pub fn has<K: JsCast, V>(self: &WeakMap<K, V>, #[js_sys(type = &JsValue)] key: &K) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap/set)
	#[js_sys(return_abi = Result<WeakMap, JsValue>)]
	pub fn set<K: JsCast, V: JsCast>(
		self: &WeakMap<K, V>,
		#[js_sys(type = &JsValue)] key: &K,
		#[js_sys(type = &JsValue)] value: &V,
	) -> Result<WeakMap<K, V>, JsValue>;
}

impl<K, V> Clone for WeakMap<K, V> {
	fn clone(&self) -> Self {
		Self::unchecked_from(<Self as AsRef<JsValue>>::as_ref(self).clone())
	}
}

impl<K, V> fmt::Debug for WeakMap<K, V> {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(<Self as AsRef<JsValue>>::as_ref(self), formatter)
	}
}

impl<K, V> PartialEq for WeakMap<K, V> {
	fn eq(&self, other: &Self) -> bool {
		<Self as AsRef<JsValue>>::as_ref(self) == <Self as AsRef<JsValue>>::as_ref(other)
	}
}

impl<K, V> Default for WeakMap<K, V> {
	fn default() -> Self {
		Self::new_typed()
	}
}
