use crate::hazard::JsCast;
use crate::{Array, Function, JsString, JsValue, Symbol, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object)
	#[derive(Clone, Debug)]
	pub type Object<T = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#description)
	#[derive(Clone, Debug)]
	pub type PropertyDescriptor;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/Object)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> Object;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/Object)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_value(value: &JsValue) -> Object;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/assign)
	#[js_sys(static_of = Object)]
	pub fn assign(target: &Object, source: &Object) -> Result<Object, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/assign)
	#[js_sys(static_of = Object, js_name = "assign", variadic)]
	pub fn assign_many(
		target: &Object,
		#[js_sys(type = &[JsValue])] sources: &[Object],
	) -> Result<Object, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/create)
	#[js_sys(static_of = Object)]
	pub fn create(prototype: &JsValue) -> Result<Object, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/create)
	#[js_sys(static_of = Object, js_name = "create")]
	pub fn create_with_properties(
		prototype: &JsValue,
		properties: &Object<PropertyDescriptor>,
	) -> Result<Object, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperties)
	#[js_sys(static_of = Object, js_name = "defineProperties")]
	pub fn define_properties(
		object: &Object,
		properties: &Object<PropertyDescriptor>,
	) -> Result<Object, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty)
	#[js_sys(static_of = Object, js_name = "defineProperty")]
	pub fn define_property(
		object: &Object,
		property: &JsValue,
		descriptor: &PropertyDescriptor,
	) -> Result<Object, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/entries)
	#[js_sys(static_of = Object)]
	pub fn entries(object: &Object) -> Result<Array<Array>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/freeze)
	#[js_sys(static_of = Object)]
	pub fn freeze(object: &Object) -> Result<Object, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/fromEntries)
	#[js_sys(static_of = Object, js_name = "fromEntries")]
	pub fn from_entries(entries: &JsValue) -> Result<Object, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/getOwnPropertyDescriptor)
	#[js_sys(static_of = Object, js_name = "getOwnPropertyDescriptor")]
	pub fn get_own_property_descriptor(
		object: &Object,
		property: &JsValue,
	) -> Result<Option<PropertyDescriptor>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/getOwnPropertyDescriptors)
	#[js_sys(static_of = Object, js_name = "getOwnPropertyDescriptors")]
	pub fn get_own_property_descriptors(
		object: &Object,
	) -> Result<Object<PropertyDescriptor>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/getOwnPropertyNames)
	#[js_sys(
		static_of = Object,
		js_name = "getOwnPropertyNames"
	)]
	pub fn get_own_property_names(object: &Object) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/getOwnPropertySymbols)
	#[js_sys(
		static_of = Object,
		js_name = "getOwnPropertySymbols"
	)]
	pub fn get_own_property_symbols(object: &Object) -> Result<Array<Symbol>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/getPrototypeOf)
	#[js_sys(static_of = Object, js_name = "getPrototypeOf")]
	pub fn get_prototype_of(object: &Object) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/groupBy)
	#[js_sys(static_of = Object, js_name = "groupBy")]
	pub fn group_by(items: &JsValue, callback: &Function) -> Result<Object<Array>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwn)
	#[js_sys(static_of = Object, js_name = "hasOwn")]
	pub fn has_own(object: &Object, property: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/is)
	#[must_use]
	#[js_sys(static_of = Object)]
	pub fn is(left: &JsValue, right: &JsValue) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isExtensible)
	#[js_sys(static_of = Object, js_name = "isExtensible")]
	pub fn is_extensible(object: &Object) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isFrozen)
	#[js_sys(static_of = Object, js_name = "isFrozen")]
	pub fn is_frozen(object: &Object) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isSealed)
	#[js_sys(static_of = Object, js_name = "isSealed")]
	pub fn is_sealed(object: &Object) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/keys)
	#[js_sys(static_of = Object)]
	pub fn keys(object: &Object) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/preventExtensions)
	#[js_sys(static_of = Object, js_name = "preventExtensions")]
	pub fn prevent_extensions(object: &Object) -> Result<Object, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/seal)
	#[js_sys(static_of = Object)]
	pub fn seal(object: &Object) -> Result<Object, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/setPrototypeOf)
	#[js_sys(static_of = Object, js_name = "setPrototypeOf")]
	pub fn set_prototype_of(object: &Object, prototype: &JsValue) -> Result<Object, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/values)
	#[js_sys(static_of = Object)]
	pub fn values(object: &Object) -> Result<Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwnProperty)
	#[js_sys(js_name = "hasOwnProperty")]
	pub fn has_own_property<T>(self: &Object<T>, property: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/isPrototypeOf)
	#[js_sys(js_name = "isPrototypeOf")]
	pub fn is_prototype_of<T>(self: &Object<T>, value: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/propertyIsEnumerable)
	#[js_sys(js_name = "propertyIsEnumerable")]
	pub fn property_is_enumerable<T>(self: &Object<T>, property: &JsValue)
	-> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string<T>(self: &Object<T>) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/toString)
	#[js_sys(js_name = "toString")]
	pub fn to_string<T>(self: &Object<T>) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/valueOf)
	#[js_sys(js_name = "valueOf")]
	pub fn value_of<T>(self: &Object<T>) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#configurable)
	#[must_use]
	#[js_sys(getter)]
	pub fn configurable(self: &PropertyDescriptor) -> Option<bool>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#configurable)
	#[js_sys(setter)]
	pub fn set_configurable(self: &PropertyDescriptor, value: bool);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#enumerable)
	#[must_use]
	#[js_sys(getter)]
	pub fn enumerable(self: &PropertyDescriptor) -> Option<bool>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#enumerable)
	#[js_sys(setter)]
	pub fn set_enumerable(self: &PropertyDescriptor, value: bool);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#get)
	#[must_use]
	#[js_sys(getter = "get")]
	pub fn get(self: &PropertyDescriptor) -> Option<Function>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#get)
	#[js_sys(setter = "get")]
	pub fn set_get(self: &PropertyDescriptor, value: &Function);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#set)
	#[must_use]
	#[js_sys(getter = "set")]
	pub fn set(self: &PropertyDescriptor) -> Option<Function>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#set)
	#[js_sys(setter = "set")]
	pub fn set_set(self: &PropertyDescriptor, value: &Function);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#value)
	#[must_use]
	#[js_sys(getter)]
	pub fn value(self: &PropertyDescriptor) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#value)
	#[js_sys(setter)]
	pub fn set_value(self: &PropertyDescriptor, value: &JsValue);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#writable)
	#[must_use]
	#[js_sys(getter)]
	pub fn writable(self: &PropertyDescriptor) -> Option<bool>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#writable)
	#[js_sys(setter)]
	pub fn set_writable(self: &PropertyDescriptor, value: bool);
}

impl Default for Object {
	fn default() -> Self {
		Self::new()
	}
}

impl PropertyDescriptor {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/defineProperty#description)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}
}

impl Default for PropertyDescriptor {
	fn default() -> Self {
		Self::new()
	}
}
