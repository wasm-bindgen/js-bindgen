use alloc::string::String;

use super::{LocaleMatcher, LocaleMatcherOptions};
use crate::hazard::JsCast;
use crate::{Array, JsString, JsValue, Object, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat#style)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListFormatStyle {
	Long,
	Short,
	Narrow,
}

impl ListFormatStyle {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Long => "long",
			Self::Short => "short",
			Self::Narrow => "narrow",
		}
	}

	fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"long" => Some(Self::Long),
			"short" => Some(Self::Short),
			"narrow" => Some(Self::Narrow),
			_ => None,
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat#type)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListFormatType {
	Conjunction,
	Disjunction,
	Unit,
}

impl ListFormatType {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Conjunction => "conjunction",
			Self::Disjunction => "disjunction",
			Self::Unit => "unit",
		}
	}

	fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"conjunction" => Some(Self::Conjunction),
			"disjunction" => Some(Self::Disjunction),
			"unit" => Some(Self::Unit),
			_ => None,
		}
	}
}

#[js_sys(js_sys = crate, namespace = "Intl")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat)
	#[js_sys(js_name = "ListFormat", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type ListFormat;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type ListFormatOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/resolvedOptions#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type ListFormatResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/formatToParts#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type ListFormatPart;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> ListFormat;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat)
	#[js_sys(constructor)]
	pub fn new_with_locales(locales: &JsValue) -> Result<ListFormat, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat)
	#[js_sys(constructor)]
	pub fn new_with_locales_and_options(
		locales: &JsValue,
		options: &ListFormatOptions,
	) -> Result<ListFormat, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/supportedLocalesOf)
	#[js_sys(static_of = ListFormat, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of(locales: &JsValue) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/supportedLocalesOf)
	#[js_sys(static_of = ListFormat, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of_with_options(
		locales: &JsValue,
		options: &LocaleMatcherOptions,
	) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/format)
	pub fn format(self: &ListFormat, list: &JsValue) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/formatToParts)
	#[js_sys(js_name = "formatToParts")]
	pub fn format_to_parts(
		self: &ListFormat,
		list: &JsValue,
	) -> Result<Array<ListFormatPart>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/resolvedOptions)
	#[must_use]
	#[js_sys(js_name = "resolvedOptions")]
	pub fn resolved_options(self: &ListFormat) -> ListFormatResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn locale(self: &ListFormatResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "type")]
	pub fn type_(self: &ListFormatResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn style(self: &ListFormatResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/formatToParts#return_value)
	#[must_use]
	#[js_sys(getter = "type")]
	pub fn type_(self: &ListFormatPart) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/formatToParts#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn value(self: &ListFormatPart) -> JsString;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(getter = "localeMatcher")]
	fn locale_matcher_raw(self: &ListFormatOptions) -> Option<JsString>;

	#[js_sys(getter = "type")]
	fn type_raw(self: &ListFormatOptions) -> Option<JsString>;

	#[js_sys(getter = "style")]
	fn style_raw(self: &ListFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "localeMatcher")]
	fn set_locale_matcher_raw(self: &ListFormatOptions, value: &str);

	#[js_sys(setter = "type")]
	fn set_type_raw(self: &ListFormatOptions, value: &str);

	#[js_sys(setter = "style")]
	fn set_style_raw(self: &ListFormatOptions, value: &str);
}

impl ListFormatOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat#localematcher)
	#[must_use]
	pub fn locale_matcher(&self) -> Option<LocaleMatcher> {
		self.locale_matcher_raw()
			.as_ref()
			.and_then(LocaleMatcher::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat#localematcher)
	pub fn set_locale_matcher(&self, value: LocaleMatcher) {
		self.set_locale_matcher_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat#type)
	#[must_use]
	pub fn type_(&self) -> Option<ListFormatType> {
		self.type_raw()
			.as_ref()
			.and_then(ListFormatType::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat#type)
	pub fn set_type(&self, value: ListFormatType) {
		self.set_type_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat#style)
	#[must_use]
	pub fn style(&self) -> Option<ListFormatStyle> {
		self.style_raw()
			.as_ref()
			.and_then(ListFormatStyle::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/ListFormat/ListFormat#style)
	pub fn set_style(&self, value: ListFormatStyle) {
		self.set_style_raw(value.as_str());
	}
}

impl Default for ListFormatOptions {
	fn default() -> Self {
		Self::new()
	}
}

impl Default for ListFormat {
	fn default() -> Self {
		Self::new()
	}
}
