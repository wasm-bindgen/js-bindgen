use super::{Array, Function, JsIterator, JsString, Number, Object};
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp)
	#[js_sys(js_name = "RegExp", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type RegExp;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/RegExp)
	#[js_sys(constructor)]
	pub fn new(pattern: &str) -> Result<RegExp, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/RegExp)
	#[js_sys(constructor)]
	pub fn new_with_flags(pattern: &str, flags: &str) -> Result<RegExp, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/RegExp)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_from_regexp(pattern: &RegExp) -> RegExp;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/RegExp)
	#[js_sys(constructor)]
	pub fn new_from_regexp_with_flags(pattern: &RegExp, flags: &str) -> Result<RegExp, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/escape)
	#[must_use]
	#[js_sys(static_of = RegExp)]
	pub fn escape(input: &str) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/dotAll)
	#[must_use]
	#[js_sys(getter = "dotAll")]
	pub fn dot_all(self: &RegExp) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec)
	#[must_use]
	pub fn exec(self: &RegExp, input: &str) -> Option<RegExpMatchArray>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/flags)
	#[must_use]
	#[js_sys(getter)]
	pub fn flags(self: &RegExp) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/global)
	#[must_use]
	#[js_sys(getter)]
	pub fn global(self: &RegExp) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/hasIndices)
	#[must_use]
	#[js_sys(getter = "hasIndices")]
	pub fn has_indices(self: &RegExp) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/ignoreCase)
	#[must_use]
	#[js_sys(getter = "ignoreCase")]
	pub fn ignore_case(self: &RegExp) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/lastIndex)
	#[must_use]
	#[js_sys(getter = "lastIndex")]
	pub fn last_index(self: &RegExp) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/lastIndex)
	#[js_sys(setter = "lastIndex")]
	pub fn set_last_index(self: &RegExp, index: f64);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/multiline)
	#[must_use]
	#[js_sys(getter)]
	pub fn multiline(self: &RegExp) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/source)
	#[must_use]
	#[js_sys(getter)]
	pub fn source(self: &RegExp) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/sticky)
	#[must_use]
	#[js_sys(getter)]
	pub fn sticky(self: &RegExp) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/test)
	#[must_use]
	pub fn test(self: &RegExp, input: &str) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &RegExp) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/unicode)
	#[must_use]
	#[js_sys(getter)]
	pub fn unicode(self: &RegExp) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/unicodeSets)
	#[must_use]
	#[js_sys(getter = "unicodeSets")]
	pub fn unicode_sets(self: &RegExp) -> bool;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec#return_value)
	#[js_sys(extends = Array<JsValue>, extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type RegExpMatchArray;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn index(self: &RegExpMatchArray) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn input(self: &RegExpMatchArray) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec#return_value)
	#[must_use]
	#[js_sys(getter = "groups")]
	pub fn groups(self: &RegExpMatchArray) -> Option<Object>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec#return_value)
	#[must_use]
	#[js_sys(getter = "indices")]
	pub fn indices(self: &RegExpMatchArray) -> Option<RegExpIndicesArray>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn length(self: &RegExpMatchArray) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec#return_value)
	#[must_use]
	#[js_sys(indexing_getter)]
	pub fn get(self: &RegExpMatchArray, index: u32) -> Option<JsString>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec#indices)
	#[js_sys(extends = Array<JsValue>, extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type RegExpIndicesArray;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec#indices)
	#[must_use]
	#[js_sys(getter = "groups")]
	pub fn groups(self: &RegExpIndicesArray) -> Option<Object>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec#indices)
	#[must_use]
	#[js_sys(getter)]
	pub fn length(self: &RegExpIndicesArray) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/exec#indices)
	#[must_use]
	#[js_sys(indexing_getter)]
	pub fn get(self: &RegExpIndicesArray, index: u32) -> Option<Array<Number>>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "regexp.match")]
	fn regexp_match(regexp: &RegExp, input: &str) -> Option<Array>;

	#[js_sys(js_embed = "regexp.match_all")]
	fn regexp_match_all(regexp: &RegExp, input: &str) -> JsIterator<RegExpMatchArray>;

	#[js_sys(js_embed = "regexp.replace")]
	fn regexp_replace(regexp: &RegExp, input: &str, replacement: &str) -> JsString;

	#[js_sys(js_embed = "regexp.replace")]
	fn regexp_replace_with_function(
		regexp: &RegExp,
		input: &str,
		replacement: &Function,
	) -> Result<JsString, JsValue>;

	#[js_sys(js_embed = "regexp.search")]
	fn regexp_search(regexp: &RegExp, input: &str) -> f64;

	#[js_sys(js_embed = "regexp.split")]
	fn regexp_split(regexp: &RegExp, input: &str) -> Array;

	#[js_sys(js_embed = "regexp.split")]
	fn regexp_split_with_limit(regexp: &RegExp, input: &str, limit: u32) -> Array;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "regexp.match",
	"(regexp, input) => regexp[Symbol.match](input)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "regexp.match_all",
	"(regexp, input) => regexp[Symbol.matchAll](input)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "regexp.replace",
	"(regexp, input, replacement) => regexp[Symbol.replace](input, replacement)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "regexp.search",
	"(regexp, input) => regexp[Symbol.search](input)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "regexp.split",
	"(regexp, input, limit) => regexp[Symbol.split](input, limit)",
);

impl RegExp {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.match)
	#[must_use]
	pub fn match_(&self, input: &str) -> Option<Array> {
		regexp_match(self, input)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.matchAll)
	#[must_use]
	pub fn match_all(&self, input: &str) -> JsIterator<RegExpMatchArray> {
		regexp_match_all(self, input)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.replace)
	#[must_use]
	pub fn replace(&self, input: &str, replacement: &str) -> JsString {
		regexp_replace(self, input, replacement)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.replace)
	pub fn replace_with_function(
		&self,
		input: &str,
		replacement: &Function,
	) -> Result<JsString, JsValue> {
		regexp_replace_with_function(self, input, replacement)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.search)
	#[must_use]
	pub fn search(&self, input: &str) -> f64 {
		regexp_search(self, input)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.split)
	#[must_use]
	pub fn split(&self, input: &str) -> Array {
		regexp_split(self, input)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/Symbol.split)
	#[must_use]
	pub fn split_with_limit(&self, input: &str, limit: u32) -> Array {
		regexp_split_with_limit(self, input, limit)
	}
}
