use core::fmt::{self, Formatter};

use super::{Array, Function, Iterable, JsIterator, Object};
use crate::hazard::JsCast;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map)
	#[js_sys(extends = Object)]
	pub type Map<K = JsValue, V = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/Map)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> Map;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/Map)
	#[must_use]
	#[js_sys(constructor, return_abi = Map)]
	pub fn new_typed<K, V>() -> Map<K, V>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/Map)
	#[js_sys(constructor, return_abi = Result<Map, JsValue>)]
	pub fn new_from_iterable<K, V, I: Iterable>(
		#[js_sys(type = &JsValue)] entries: &I,
	) -> Result<Map<K, V>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/groupBy)
	#[js_sys(static_of = Map, js_name = "groupBy")]
	pub fn group_by(items: &JsValue, callback: &Function) -> Result<Map, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/clear)
	pub fn clear<K, V>(self: &Map<K, V>);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/delete)
	#[must_use]
	pub fn delete<K: JsCast, V>(self: &Map<K, V>, #[js_sys(type = &JsValue)] key: &K) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/entries)
	#[must_use]
	pub fn entries<K, V>(self: &Map<K, V>) -> JsIterator<Array>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/forEach)
	#[js_sys(js_name = "forEach")]
	pub fn for_each<K, V>(self: &Map<K, V>, callback: &Function) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/forEach)
	#[js_sys(js_name = "forEach")]
	pub fn for_each_with_this<K, V>(
		self: &Map<K, V>,
		callback: &Function,
		this: &JsValue,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/get)
	#[must_use]
	pub fn get<K: JsCast, V>(self: &Map<K, V>, #[js_sys(type = &JsValue)] key: &K) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/get)
	#[must_use]
	#[js_sys(js_name = "get", return_abi = Option<JsValue>)]
	pub fn get_checked<K: JsCast, V: JsCast>(
		self: &Map<K, V>,
		#[js_sys(type = &JsValue)] key: &K,
	) -> Option<V>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/getOrInsert)
	#[must_use]
	#[js_sys(js_name = "getOrInsert", return_abi = JsValue)]
	pub fn get_or_insert<K: JsCast, V: JsCast>(
		self: &Map<K, V>,
		#[js_sys(type = &JsValue)] key: &K,
		#[js_sys(type = &JsValue)] default_value: &V,
	) -> V;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/getOrInsertComputed)
	#[js_sys(
		js_name = "getOrInsertComputed",
		return_abi = Result<JsValue, JsValue>
	)]
	pub fn get_or_insert_computed<K: JsCast, V: JsCast>(
		self: &Map<K, V>,
		#[js_sys(type = &JsValue)] key: &K,
		callback: &Function,
	) -> Result<V, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/has)
	#[must_use]
	pub fn has<K: JsCast, V>(self: &Map<K, V>, #[js_sys(type = &JsValue)] key: &K) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/keys)
	#[must_use]
	#[js_sys(return_abi = JsIterator)]
	pub fn keys<K: JsCast, V>(self: &Map<K, V>) -> JsIterator<K>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/set)
	#[must_use]
	#[js_sys(return_abi = Map)]
	pub fn set<K: JsCast, V: JsCast>(
		self: &Map<K, V>,
		#[js_sys(type = &JsValue)] key: &K,
		#[js_sys(type = &JsValue)] value: &V,
	) -> Map<K, V>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/size)
	#[must_use]
	#[js_sys(getter)]
	pub fn size<K, V>(self: &Map<K, V>) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/values)
	#[must_use]
	#[js_sys(return_abi = JsIterator)]
	pub fn values<K, V: JsCast>(self: &Map<K, V>) -> JsIterator<V>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "map.symbol_iterator")]
	fn map_symbol_iterator<K, V>(#[js_sys(type = &JsValue)] map: &Map<K, V>) -> JsIterator<Array>;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "map.symbol_iterator",
	"(map) => map[Symbol.iterator]()",
);

impl<K, V> Clone for Map<K, V> {
	fn clone(&self) -> Self {
		Self::unchecked_from(<Self as AsRef<JsValue>>::as_ref(self).clone())
	}
}

impl<K, V> fmt::Debug for Map<K, V> {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(<Self as AsRef<JsValue>>::as_ref(self), formatter)
	}
}

impl<K, V> PartialEq for Map<K, V> {
	fn eq(&self, other: &Self) -> bool {
		<Self as AsRef<JsValue>>::as_ref(self) == <Self as AsRef<JsValue>>::as_ref(other)
	}
}

impl<K, V> Default for Map<K, V> {
	fn default() -> Self {
		Self::new_typed()
	}
}

impl<K, V> Map<K, V> {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map/Symbol.iterator)
	#[must_use]
	pub fn symbol_iterator(&self) -> JsIterator<Array> {
		map_symbol_iterator(self)
	}
}

impl<K, V> Iterable for Map<K, V> {
	type Item = Array;
}
