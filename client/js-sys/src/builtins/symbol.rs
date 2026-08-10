use crate::{JsString, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol)
	#[derive(Clone, Debug, PartialEq)]
	pub type Symbol;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_name = "Symbol")]
	fn symbol() -> Symbol;

	#[js_sys(js_name = "Symbol")]
	fn symbol_with_description(description: &str) -> Symbol;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/asyncDispose)
	#[must_use]
	#[js_sys(static_of = Symbol, getter = "asyncDispose")]
	pub fn async_dispose() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/asyncIterator)
	#[must_use]
	#[js_sys(static_of = Symbol, getter = "asyncIterator")]
	pub fn async_iterator() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/dispose)
	#[must_use]
	#[js_sys(static_of = Symbol, getter)]
	pub fn dispose() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/hasInstance)
	#[must_use]
	#[js_sys(static_of = Symbol, getter = "hasInstance")]
	pub fn has_instance() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/isConcatSpreadable)
	#[must_use]
	#[js_sys(static_of = Symbol, getter = "isConcatSpreadable")]
	pub fn is_concat_spreadable() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/iterator)
	#[must_use]
	#[js_sys(static_of = Symbol, getter)]
	pub fn iterator() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/match)
	#[must_use]
	#[js_sys(static_of = Symbol, getter = "match")]
	pub fn match_() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/matchAll)
	#[must_use]
	#[js_sys(static_of = Symbol, getter = "matchAll")]
	pub fn match_all() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/replace)
	#[must_use]
	#[js_sys(static_of = Symbol, getter)]
	pub fn replace() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/search)
	#[must_use]
	#[js_sys(static_of = Symbol, getter)]
	pub fn search() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/species)
	#[must_use]
	#[js_sys(static_of = Symbol, getter)]
	pub fn species() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/split)
	#[must_use]
	#[js_sys(static_of = Symbol, getter)]
	pub fn split() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toPrimitive)
	#[must_use]
	#[js_sys(static_of = Symbol, getter = "toPrimitive")]
	pub fn to_primitive() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toStringTag)
	#[must_use]
	#[js_sys(static_of = Symbol, getter = "toStringTag")]
	pub fn to_string_tag() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/unscopables)
	#[must_use]
	#[js_sys(static_of = Symbol, getter)]
	pub fn unscopables() -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/for)
	#[must_use]
	#[js_sys(static_of = Symbol, js_name = "for")]
	pub fn for_(key: &str) -> Symbol;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/keyFor)
	#[must_use]
	#[js_sys(static_of = Symbol, js_name = "keyFor")]
	pub fn key_for(symbol: &Symbol) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/description)
	#[must_use]
	#[js_sys(getter)]
	pub fn description(self: &Symbol) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_js_string(self: &Symbol) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/valueOf)
	#[must_use]
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &Symbol) -> Symbol;
}

impl Eq for Symbol {}

impl Symbol {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/Symbol)
	#[must_use]
	pub fn new() -> Self {
		symbol()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/Symbol)
	#[must_use]
	pub fn new_with_description(description: &str) -> Self {
		symbol_with_description(description)
	}
}

impl Default for Symbol {
	fn default() -> Self {
		Self::new()
	}
}
