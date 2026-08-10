use alloc::string::String;

use super::{LocaleMatcher, LocaleMatcherOptions};
use crate::hazard::JsCast;
use crate::{Array, Function, JsString, JsValue, Object, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#usage)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollatorUsage {
	Sort,
	Search,
}

impl CollatorUsage {
	pub(super) const fn as_str(self) -> &'static str {
		match self {
			Self::Sort => "sort",
			Self::Search => "search",
		}
	}

	pub(super) fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"sort" => Some(Self::Sort),
			"search" => Some(Self::Search),
			_ => None,
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#sensitivity)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollatorSensitivity {
	Base,
	Accent,
	Case,
	Variant,
}

impl CollatorSensitivity {
	pub(super) const fn as_str(self) -> &'static str {
		match self {
			Self::Base => "base",
			Self::Accent => "accent",
			Self::Case => "case",
			Self::Variant => "variant",
		}
	}

	pub(super) fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"base" => Some(Self::Base),
			"accent" => Some(Self::Accent),
			"case" => Some(Self::Case),
			"variant" => Some(Self::Variant),
			_ => None,
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#casefirst)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollatorCaseFirst {
	Upper,
	Lower,
	False,
}

impl CollatorCaseFirst {
	pub(super) const fn as_str(self) -> &'static str {
		match self {
			Self::Upper => "upper",
			Self::Lower => "lower",
			Self::False => "false",
		}
	}

	pub(super) fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"upper" => Some(Self::Upper),
			"lower" => Some(Self::Lower),
			"false" => Some(Self::False),
			_ => None,
		}
	}
}

#[js_sys(js_sys = crate, namespace = "Intl")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator)
	#[js_sys(js_name = "Collator", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Collator;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type CollatorOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/resolvedOptions#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type CollatorResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> Collator;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator)
	#[js_sys(constructor)]
	pub fn new_with_locales(locales: &JsValue) -> Result<Collator, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator)
	#[js_sys(constructor)]
	pub fn new_with_locales_and_options(
		locales: &JsValue,
		options: &CollatorOptions,
	) -> Result<Collator, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/supportedLocalesOf)
	#[js_sys(static_of = Collator, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of(locales: &JsValue) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/supportedLocalesOf)
	#[js_sys(static_of = Collator, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of_with_options(
		locales: &JsValue,
		options: &LocaleMatcherOptions,
	) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/compare)
	#[must_use]
	#[js_sys(getter)]
	pub fn compare(self: &Collator) -> Function;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/resolvedOptions)
	#[must_use]
	#[js_sys(js_name = "resolvedOptions")]
	pub fn resolved_options(self: &Collator) -> CollatorResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn locale(self: &CollatorResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn usage(self: &CollatorResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn sensitivity(self: &CollatorResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "ignorePunctuation")]
	pub fn ignore_punctuation(self: &CollatorResolvedOptions) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn collation(self: &CollatorResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn numeric(self: &CollatorResolvedOptions) -> Option<bool>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "caseFirst")]
	pub fn case_first(self: &CollatorResolvedOptions) -> Option<JsString>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(getter = "localeMatcher")]
	fn locale_matcher_raw(self: &CollatorOptions) -> Option<JsString>;

	#[js_sys(getter = "usage")]
	fn usage_raw(self: &CollatorOptions) -> Option<JsString>;

	#[js_sys(getter = "collation")]
	fn collation_raw(self: &CollatorOptions) -> Option<JsString>;

	#[js_sys(getter = "numeric")]
	fn numeric_raw(self: &CollatorOptions) -> Option<bool>;

	#[js_sys(getter = "caseFirst")]
	fn case_first_raw(self: &CollatorOptions) -> Option<JsString>;

	#[js_sys(getter = "sensitivity")]
	fn sensitivity_raw(self: &CollatorOptions) -> Option<JsString>;

	#[js_sys(getter = "ignorePunctuation")]
	fn ignore_punctuation_raw(self: &CollatorOptions) -> Option<bool>;

	#[js_sys(setter = "localeMatcher")]
	fn set_locale_matcher_raw(self: &CollatorOptions, value: &str);

	#[js_sys(setter = "usage")]
	fn set_usage_raw(self: &CollatorOptions, value: &str);

	#[js_sys(setter = "collation")]
	fn set_collation_raw(self: &CollatorOptions, value: &str);

	#[js_sys(setter = "numeric")]
	fn set_numeric_raw(self: &CollatorOptions, value: bool);

	#[js_sys(setter = "caseFirst")]
	fn set_case_first_raw(self: &CollatorOptions, value: &str);

	#[js_sys(setter = "sensitivity")]
	fn set_sensitivity_raw(self: &CollatorOptions, value: &str);

	#[js_sys(setter = "ignorePunctuation")]
	fn set_ignore_punctuation_raw(self: &CollatorOptions, value: bool);
}

impl CollatorOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#localematcher)
	#[must_use]
	pub fn locale_matcher(&self) -> Option<LocaleMatcher> {
		self.locale_matcher_raw()
			.as_ref()
			.and_then(LocaleMatcher::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#localematcher)
	pub fn set_locale_matcher(&self, value: LocaleMatcher) {
		self.set_locale_matcher_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#usage)
	#[must_use]
	pub fn usage(&self) -> Option<CollatorUsage> {
		self.usage_raw()
			.as_ref()
			.and_then(CollatorUsage::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#usage)
	pub fn set_usage(&self, value: CollatorUsage) {
		self.set_usage_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#collation)
	#[must_use]
	pub fn collation(&self) -> Option<JsString> {
		self.collation_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#collation)
	pub fn set_collation(&self, value: &str) {
		self.set_collation_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#numeric)
	#[must_use]
	pub fn numeric(&self) -> Option<bool> {
		self.numeric_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#numeric)
	pub fn set_numeric(&self, value: bool) {
		self.set_numeric_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#casefirst)
	#[must_use]
	pub fn case_first(&self) -> Option<CollatorCaseFirst> {
		self.case_first_raw()
			.as_ref()
			.and_then(CollatorCaseFirst::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#casefirst)
	pub fn set_case_first(&self, value: CollatorCaseFirst) {
		self.set_case_first_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#sensitivity)
	#[must_use]
	pub fn sensitivity(&self) -> Option<CollatorSensitivity> {
		self.sensitivity_raw()
			.as_ref()
			.and_then(CollatorSensitivity::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#sensitivity)
	pub fn set_sensitivity(&self, value: CollatorSensitivity) {
		self.set_sensitivity_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#ignorepunctuation)
	#[must_use]
	pub fn ignore_punctuation(&self) -> Option<bool> {
		self.ignore_punctuation_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/Collator#ignorepunctuation)
	pub fn set_ignore_punctuation(&self, value: bool) {
		self.set_ignore_punctuation_raw(value);
	}
}

impl Default for CollatorOptions {
	fn default() -> Self {
		Self::new()
	}
}

impl Default for Collator {
	fn default() -> Self {
		Self::new()
	}
}
