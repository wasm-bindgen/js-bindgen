use js_sys_macro::js_sys;

use super::{Array, Function, Intl, Iterable, JsIterator, Object, RegExp, RegExpMatchArray};
use crate::JsValue;

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String)
	#[js_sys(js_name = "String")]
	#[derive(Clone, PartialEq)]
	pub type JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/fromCharCode)
	#[must_use]
	#[js_sys(static_of = JsString, js_name = "fromCharCode", variadic)]
	pub fn from_char_code(char_codes: &[u32]) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/fromCodePoint)
	#[js_sys(static_of = JsString, js_name = "fromCodePoint", variadic)]
	pub fn from_code_point(code_points: &[u32]) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/raw)
	#[js_sys(static_of = JsString, variadic)]
	pub fn raw(call_site: &Object, substitutions: &[JsValue]) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/length)
	#[must_use]
	#[js_sys(getter)]
	pub fn length(self: &JsString) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/at)
	#[must_use]
	pub fn at(self: &JsString, index: f64) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/charAt)
	#[must_use]
	#[js_sys(js_name = "charAt")]
	pub fn char_at(self: &JsString, index: f64) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/charCodeAt)
	#[must_use]
	#[js_sys(js_name = "charCodeAt")]
	pub fn char_code_at(self: &JsString, index: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/codePointAt)
	#[must_use]
	#[js_sys(js_name = "codePointAt")]
	pub fn code_point_at(self: &JsString, index: f64) -> Option<u32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/concat)
	#[must_use]
	pub fn concat(self: &JsString, string: &str) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/concat)
	#[must_use]
	#[js_sys(js_name = "concat", variadic)]
	pub fn concat_many(
		self: &JsString,
		#[js_sys(type = &[JsValue])] strings: &[JsString],
	) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/endsWith)
	#[must_use]
	#[js_sys(js_name = "endsWith")]
	pub fn ends_with(self: &JsString, search: &str) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/endsWith)
	#[must_use]
	#[js_sys(js_name = "endsWith")]
	pub fn ends_with_at(self: &JsString, search: &str, end: f64) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/includes)
	#[must_use]
	pub fn includes(self: &JsString, search: &str) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/includes)
	#[must_use]
	#[js_sys(js_name = "includes")]
	pub fn includes_from(self: &JsString, search: &str, position: f64) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/indexOf)
	#[must_use]
	#[js_sys(js_name = "indexOf")]
	pub fn index_of(self: &JsString, search: &str) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/indexOf)
	#[must_use]
	#[js_sys(js_name = "indexOf")]
	pub fn index_of_from(self: &JsString, search: &str, position: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/isWellFormed)
	#[must_use]
	#[js_sys(js_name = "isWellFormed")]
	pub fn is_well_formed(self: &JsString) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/lastIndexOf)
	#[must_use]
	#[js_sys(js_name = "lastIndexOf")]
	pub fn last_index_of(self: &JsString, search: &str) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/lastIndexOf)
	#[must_use]
	#[js_sys(js_name = "lastIndexOf")]
	pub fn last_index_of_from(self: &JsString, search: &str, position: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/localeCompare)
	#[must_use]
	#[js_sys(js_name = "localeCompare")]
	pub fn locale_compare(self: &JsString, compare: &str) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/localeCompare)
	#[js_sys(js_name = "localeCompare")]
	pub fn locale_compare_with_locales(
		self: &JsString,
		compare: &str,
		locales: &JsValue,
	) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/localeCompare)
	#[js_sys(js_name = "localeCompare")]
	pub fn locale_compare_with_locales_and_options(
		self: &JsString,
		compare: &str,
		locales: &JsValue,
		options: &Intl::CollatorOptions,
	) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/match)
	#[must_use]
	#[js_sys(js_name = "match")]
	pub fn match_(self: &JsString, pattern: &RegExp) -> Option<Array>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/match)
	#[js_sys(js_name = "match")]
	pub fn match_str(self: &JsString, pattern: &str) -> Result<Option<RegExpMatchArray>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/matchAll)
	#[js_sys(js_name = "matchAll")]
	pub fn match_all(
		self: &JsString,
		pattern: &RegExp,
	) -> Result<JsIterator<RegExpMatchArray>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/matchAll)
	#[js_sys(js_name = "matchAll")]
	pub fn match_all_str(
		self: &JsString,
		pattern: &str,
	) -> Result<JsIterator<RegExpMatchArray>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/normalize)
	#[must_use]
	pub fn normalize(self: &JsString) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/normalize)
	#[js_sys(js_name = "normalize")]
	pub fn normalize_with_form(self: &JsString, form: &str) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/padEnd)
	#[must_use]
	#[js_sys(js_name = "padEnd")]
	pub fn pad_end(self: &JsString, target_length: f64) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/padEnd)
	#[must_use]
	#[js_sys(js_name = "padEnd")]
	pub fn pad_end_with_string(self: &JsString, target_length: f64, pad_string: &str) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/padStart)
	#[must_use]
	#[js_sys(js_name = "padStart")]
	pub fn pad_start(self: &JsString, target_length: f64) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/padStart)
	#[must_use]
	#[js_sys(js_name = "padStart")]
	pub fn pad_start_with_string(self: &JsString, target_length: f64, pad_string: &str)
	-> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/repeat)
	pub fn repeat(self: &JsString, count: f64) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replace)
	#[must_use]
	pub fn replace(self: &JsString, pattern: &str, replacement: &str) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replace)
	#[must_use]
	#[js_sys(js_name = "replace")]
	pub fn replace_regexp(self: &JsString, pattern: &RegExp, replacement: &str) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replace)
	#[js_sys(js_name = "replace")]
	pub fn replace_with_function(
		self: &JsString,
		pattern: &str,
		replacement: &Function,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replace)
	#[js_sys(js_name = "replace")]
	pub fn replace_regexp_with_function(
		self: &JsString,
		pattern: &RegExp,
		replacement: &Function,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replaceAll)
	#[must_use]
	#[js_sys(js_name = "replaceAll")]
	pub fn replace_all(self: &JsString, pattern: &str, replacement: &str) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replaceAll)
	#[js_sys(js_name = "replaceAll")]
	pub fn replace_all_regexp(
		self: &JsString,
		pattern: &RegExp,
		replacement: &str,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replaceAll)
	#[js_sys(js_name = "replaceAll")]
	pub fn replace_all_with_function(
		self: &JsString,
		pattern: &str,
		replacement: &Function,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replaceAll)
	#[js_sys(js_name = "replaceAll")]
	pub fn replace_all_regexp_with_function(
		self: &JsString,
		pattern: &RegExp,
		replacement: &Function,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/search)
	#[must_use]
	pub fn search(self: &JsString, pattern: &RegExp) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/search)
	#[js_sys(js_name = "search")]
	pub fn search_str(self: &JsString, pattern: &str) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/slice)
	#[must_use]
	pub fn slice(self: &JsString, start: f64) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/slice)
	#[must_use]
	#[js_sys(js_name = "slice")]
	pub fn slice_range(self: &JsString, start: f64, end: f64) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/split)
	#[must_use]
	pub fn split(self: &JsString) -> Array<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/split)
	#[must_use]
	#[js_sys(js_name = "split")]
	pub fn split_with_separator(self: &JsString, separator: &str) -> Array<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/split)
	#[must_use]
	#[js_sys(js_name = "split")]
	pub fn split_with_separator_and_limit(
		self: &JsString,
		separator: &str,
		limit: u32,
	) -> Array<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/split)
	#[must_use]
	#[js_sys(js_name = "split")]
	pub fn split_with_regexp(self: &JsString, separator: &RegExp) -> Array;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/split)
	#[must_use]
	#[js_sys(js_name = "split")]
	pub fn split_with_regexp_and_limit(self: &JsString, separator: &RegExp, limit: u32) -> Array;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/startsWith)
	#[must_use]
	#[js_sys(js_name = "startsWith")]
	pub fn starts_with(self: &JsString, search: &str) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/startsWith)
	#[must_use]
	#[js_sys(js_name = "startsWith")]
	pub fn starts_with_at(self: &JsString, search: &str, position: f64) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring)
	#[must_use]
	pub fn substring(self: &JsString, start: f64) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring)
	#[must_use]
	#[js_sys(js_name = "substring")]
	pub fn substring_range(self: &JsString, start: f64, end: f64) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toLocaleLowerCase)
	#[must_use]
	#[js_sys(js_name = "toLocaleLowerCase")]
	pub fn to_locale_lower_case(self: &JsString) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toLocaleLowerCase)
	#[js_sys(js_name = "toLocaleLowerCase")]
	pub fn to_locale_lower_case_with_locale(
		self: &JsString,
		locale: &str,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toLocaleLowerCase)
	#[js_sys(js_name = "toLocaleLowerCase")]
	pub fn to_locale_lower_case_with_locales(
		self: &JsString,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toLocaleUpperCase)
	#[must_use]
	#[js_sys(js_name = "toLocaleUpperCase")]
	pub fn to_locale_upper_case(self: &JsString) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toLocaleUpperCase)
	#[js_sys(js_name = "toLocaleUpperCase")]
	pub fn to_locale_upper_case_with_locale(
		self: &JsString,
		locale: &str,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toLocaleUpperCase)
	#[js_sys(js_name = "toLocaleUpperCase")]
	pub fn to_locale_upper_case_with_locales(
		self: &JsString,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toLowerCase)
	#[must_use]
	#[js_sys(js_name = "toLowerCase")]
	pub fn to_lower_case(self: &JsString) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &JsString) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toUpperCase)
	#[must_use]
	#[js_sys(js_name = "toUpperCase")]
	pub fn to_upper_case(self: &JsString) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/toWellFormed)
	#[must_use]
	#[js_sys(js_name = "toWellFormed")]
	pub fn to_well_formed(self: &JsString) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trim)
	#[must_use]
	pub fn trim(self: &JsString) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd)
	#[must_use]
	#[js_sys(js_name = "trimEnd")]
	pub fn trim_end(self: &JsString) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart)
	#[must_use]
	#[js_sys(js_name = "trimStart")]
	pub fn trim_start(self: &JsString) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/valueOf)
	#[must_use]
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &JsString) -> JsString;

}

impl Eq for JsString {}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_name = "String")]
	fn string_constructor(value: &JsValue) -> Result<JsString, JsValue>;

	#[js_sys(js_embed = "string.iterator")]
	fn string_iterator(value: &JsString) -> JsIterator<JsString>;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "string.iterator",
	"value => value[Symbol.iterator]()"
);

impl JsString {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/String)
	pub fn new(value: &JsValue) -> Result<Self, JsValue> {
		string_constructor(value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/Symbol.iterator)
	#[must_use]
	pub fn iterator(&self) -> JsIterator<Self> {
		string_iterator(self)
	}
}

impl Iterable for JsString {
	type Item = Self;
}
