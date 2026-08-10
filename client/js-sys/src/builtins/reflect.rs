use crate::{Array, Function, JsValue, js_sys};

#[expect(non_snake_case, reason = "matches the JavaScript global name")]
pub mod Reflect {
	use super::*;

	#[js_sys(js_sys = crate, namespace = "Reflect")]
	extern "js-sys" {
		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/apply)
		pub fn apply(
			target: &Function,
			this_argument: &JsValue,
			arguments_list: &Array,
		) -> Result<JsValue, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/construct)
		pub fn construct(target: &Function, arguments_list: &Array) -> Result<JsValue, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/construct)
		#[js_sys(js_name = "construct")]
		pub fn construct_with_new_target(
			target: &Function,
			arguments_list: &Array,
			new_target: &Function,
		) -> Result<JsValue, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/defineProperty)
		#[js_sys(js_name = "defineProperty")]
		pub fn define_property(
			target: &JsValue,
			property_key: &JsValue,
			attributes: &JsValue,
		) -> Result<bool, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/defineProperty)
		#[js_sys(js_name = "defineProperty")]
		pub fn define_property_str(
			target: &JsValue,
			property_key: &str,
			attributes: &JsValue,
		) -> Result<bool, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/deleteProperty)
		#[js_sys(js_name = "deleteProperty")]
		pub fn delete_property(target: &JsValue, property_key: &JsValue) -> Result<bool, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/deleteProperty)
		#[js_sys(js_name = "deleteProperty")]
		pub fn delete_property_str(target: &JsValue, property_key: &str) -> Result<bool, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/get)
		pub fn get(target: &JsValue, property_key: &JsValue) -> Result<JsValue, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/get)
		#[js_sys(js_name = "get")]
		pub fn get_str(target: &JsValue, property_key: &str) -> Result<JsValue, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/get)
		#[js_sys(js_name = "get")]
		pub fn get_u32(target: &JsValue, property_key: u32) -> Result<JsValue, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/get)
		#[js_sys(js_name = "get")]
		pub fn get_with_receiver(
			target: &JsValue,
			property_key: &JsValue,
			receiver: &JsValue,
		) -> Result<JsValue, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getOwnPropertyDescriptor)
		#[js_sys(js_name = "getOwnPropertyDescriptor")]
		pub fn get_own_property_descriptor(
			target: &JsValue,
			property_key: &JsValue,
		) -> Result<JsValue, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getOwnPropertyDescriptor)
		#[js_sys(js_name = "getOwnPropertyDescriptor")]
		pub fn get_own_property_descriptor_str(
			target: &JsValue,
			property_key: &str,
		) -> Result<JsValue, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getPrototypeOf)
		#[js_sys(js_name = "getPrototypeOf")]
		pub fn get_prototype_of(target: &JsValue) -> Result<JsValue, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/has)
		pub fn has(target: &JsValue, property_key: &JsValue) -> Result<bool, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/has)
		#[js_sys(js_name = "has")]
		pub fn has_str(target: &JsValue, property_key: &str) -> Result<bool, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/isExtensible)
		#[js_sys(js_name = "isExtensible")]
		pub fn is_extensible(target: &JsValue) -> Result<bool, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/ownKeys)
		#[js_sys(js_name = "ownKeys")]
		pub fn own_keys(target: &JsValue) -> Result<Array, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/preventExtensions)
		#[js_sys(js_name = "preventExtensions")]
		pub fn prevent_extensions(target: &JsValue) -> Result<bool, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/set)
		pub fn set(
			target: &JsValue,
			property_key: &JsValue,
			value: &JsValue,
		) -> Result<bool, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/set)
		#[js_sys(js_name = "set")]
		pub fn set_str(
			target: &JsValue,
			property_key: &str,
			value: &JsValue,
		) -> Result<bool, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/set)
		#[js_sys(js_name = "set")]
		pub fn set_u32(
			target: &JsValue,
			property_key: u32,
			value: &JsValue,
		) -> Result<bool, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/set)
		#[js_sys(js_name = "set")]
		pub fn set_with_receiver(
			target: &JsValue,
			property_key: &JsValue,
			value: &JsValue,
			receiver: &JsValue,
		) -> Result<bool, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/setPrototypeOf)
		#[js_sys(js_name = "setPrototypeOf")]
		pub fn set_prototype_of(target: &JsValue, prototype: &JsValue) -> Result<bool, JsValue>;
	}
}
