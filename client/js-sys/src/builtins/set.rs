use core::fmt::{self, Formatter};

use super::{Array, Function, Iterable, JsIterator, Object};
use crate::hazard::JsCast;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set)
	#[js_sys(extends = Object)]
	pub type Set<T = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/Set)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> Set;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/Set)
	#[must_use]
	#[js_sys(constructor, return_abi = Set)]
	pub fn new_typed<T>() -> Set<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/Set)
	#[js_sys(constructor, return_abi = Result<Set, JsValue>)]
	pub fn new_from_iterable<T: JsCast, I: Iterable<Item = T>>(
		#[js_sys(type = &JsValue)] items: &I,
	) -> Result<Set<T>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/add)
	#[must_use]
	#[js_sys(return_abi = Set)]
	pub fn add<T: JsCast>(self: &Set<T>, #[js_sys(type = &JsValue)] value: &T) -> Set<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/clear)
	pub fn clear<T>(self: &Set<T>);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/delete)
	#[must_use]
	pub fn delete<T: JsCast>(self: &Set<T>, #[js_sys(type = &JsValue)] value: &T) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/difference)
	#[must_use]
	#[js_sys(return_abi = Set)]
	pub fn difference<T>(self: &Set<T>, #[js_sys(type = &JsValue)] other: &Set<T>) -> Set<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/entries)
	#[must_use]
	pub fn entries<T>(self: &Set<T>) -> JsIterator<Array>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/forEach)
	#[js_sys(js_name = "forEach")]
	pub fn for_each<T>(self: &Set<T>, callback: &Function) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/forEach)
	#[js_sys(js_name = "forEach")]
	pub fn for_each_with_this<T>(
		self: &Set<T>,
		callback: &Function,
		this: &JsValue,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/has)
	#[must_use]
	pub fn has<T: JsCast>(self: &Set<T>, #[js_sys(type = &JsValue)] value: &T) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/intersection)
	#[must_use]
	#[js_sys(return_abi = Set)]
	pub fn intersection<T>(self: &Set<T>, #[js_sys(type = &JsValue)] other: &Set<T>) -> Set<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/isDisjointFrom)
	#[must_use]
	#[js_sys(js_name = "isDisjointFrom")]
	pub fn is_disjoint_from<T>(self: &Set<T>, #[js_sys(type = &JsValue)] other: &Set<T>) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/isSubsetOf)
	#[must_use]
	#[js_sys(js_name = "isSubsetOf")]
	pub fn is_subset_of<T>(self: &Set<T>, #[js_sys(type = &JsValue)] other: &Set<T>) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/isSupersetOf)
	#[must_use]
	#[js_sys(js_name = "isSupersetOf")]
	pub fn is_superset_of<T>(self: &Set<T>, #[js_sys(type = &JsValue)] other: &Set<T>) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/keys)
	#[must_use]
	#[js_sys(return_abi = JsIterator)]
	pub fn keys<T: JsCast>(self: &Set<T>) -> JsIterator<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/size)
	#[must_use]
	#[js_sys(getter)]
	pub fn size<T>(self: &Set<T>) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/symmetricDifference)
	#[must_use]
	#[js_sys(js_name = "symmetricDifference", return_abi = Set)]
	pub fn symmetric_difference<T>(
		self: &Set<T>,
		#[js_sys(type = &JsValue)] other: &Set<T>,
	) -> Set<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/union)
	#[must_use]
	#[js_sys(return_abi = Set)]
	pub fn union<T>(self: &Set<T>, #[js_sys(type = &JsValue)] other: &Set<T>) -> Set<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/values)
	#[must_use]
	#[js_sys(return_abi = JsIterator)]
	pub fn values<T: JsCast>(self: &Set<T>) -> JsIterator<T>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "set.symbol_iterator", return_abi = JsIterator)]
	fn set_symbol_iterator<T: JsCast>(#[js_sys(type = &JsValue)] set: &Set<T>) -> JsIterator<T>;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "set.symbol_iterator",
	"(set) => set[Symbol.iterator]()",
);

impl<T> Clone for Set<T> {
	fn clone(&self) -> Self {
		Self::unchecked_from(<Self as AsRef<JsValue>>::as_ref(self).clone())
	}
}

impl<T> fmt::Debug for Set<T> {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(<Self as AsRef<JsValue>>::as_ref(self), formatter)
	}
}

impl<T> PartialEq for Set<T> {
	fn eq(&self, other: &Self) -> bool {
		<Self as AsRef<JsValue>>::as_ref(self) == <Self as AsRef<JsValue>>::as_ref(other)
	}
}

impl<T> Default for Set<T> {
	fn default() -> Self {
		Self::new_typed()
	}
}

impl<T: JsCast> Set<T> {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set/Symbol.iterator)
	#[must_use]
	pub fn symbol_iterator(&self) -> JsIterator<T> {
		set_symbol_iterator(self)
	}
}

impl<T: JsCast> Iterable for Set<T> {
	type Item = T;
}
