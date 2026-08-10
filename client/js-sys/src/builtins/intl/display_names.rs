use alloc::string::String;

use super::{LocaleMatcher, LocaleMatcherOptions};
use crate::hazard::JsCast;
use crate::{Array, JsString, JsValue, Object, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#type)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayNamesType {
	Language,
	Region,
	Script,
	Currency,
	Calendar,
	DateTimeField,
}

impl DisplayNamesType {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Language => "language",
			Self::Region => "region",
			Self::Script => "script",
			Self::Currency => "currency",
			Self::Calendar => "calendar",
			Self::DateTimeField => "dateTimeField",
		}
	}

	fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"language" => Some(Self::Language),
			"region" => Some(Self::Region),
			"script" => Some(Self::Script),
			"currency" => Some(Self::Currency),
			"calendar" => Some(Self::Calendar),
			"dateTimeField" => Some(Self::DateTimeField),
			_ => None,
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#style)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayNamesStyle {
	Long,
	Short,
	Narrow,
}

impl DisplayNamesStyle {
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

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#fallback)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayNamesFallback {
	Code,
	None,
}

impl DisplayNamesFallback {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Code => "code",
			Self::None => "none",
		}
	}

	fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"code" => Some(Self::Code),
			"none" => Some(Self::None),
			_ => None,
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#languagedisplay)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayNamesLanguageDisplay {
	Dialect,
	Standard,
}

impl DisplayNamesLanguageDisplay {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Dialect => "dialect",
			Self::Standard => "standard",
		}
	}

	fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"dialect" => Some(Self::Dialect),
			"standard" => Some(Self::Standard),
			_ => None,
		}
	}
}

#[js_sys(js_sys = crate, namespace = "Intl")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames)
	#[js_sys(js_name = "DisplayNames", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DisplayNames;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DisplayNamesOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/resolvedOptions#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DisplayNamesResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames)
	#[js_sys(constructor)]
	pub fn new(locales: &JsValue, options: &DisplayNamesOptions) -> Result<DisplayNames, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/supportedLocalesOf)
	#[js_sys(static_of = DisplayNames, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of(locales: &JsValue) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/supportedLocalesOf)
	#[js_sys(static_of = DisplayNames, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of_with_options(
		locales: &JsValue,
		options: &LocaleMatcherOptions,
	) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/of)
	pub fn of(self: &DisplayNames, code: &str) -> Result<Option<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/resolvedOptions)
	#[must_use]
	#[js_sys(js_name = "resolvedOptions")]
	pub fn resolved_options(self: &DisplayNames) -> DisplayNamesResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn locale(self: &DisplayNamesResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn style(self: &DisplayNamesResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "type")]
	pub fn display_names_type(self: &DisplayNamesResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn fallback(self: &DisplayNamesResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "languageDisplay")]
	pub fn language_display(self: &DisplayNamesResolvedOptions) -> Option<JsString>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(getter = "localeMatcher")]
	fn locale_matcher_raw(self: &DisplayNamesOptions) -> Option<JsString>;

	#[js_sys(getter = "style")]
	fn style_raw(self: &DisplayNamesOptions) -> Option<JsString>;

	#[js_sys(getter = "type")]
	fn type_raw(self: &DisplayNamesOptions) -> Option<JsString>;

	#[js_sys(getter = "fallback")]
	fn fallback_raw(self: &DisplayNamesOptions) -> Option<JsString>;

	#[js_sys(getter = "languageDisplay")]
	fn language_display_raw(self: &DisplayNamesOptions) -> Option<JsString>;

	#[js_sys(setter = "localeMatcher")]
	fn set_locale_matcher_raw(self: &DisplayNamesOptions, value: &str);

	#[js_sys(setter = "style")]
	fn set_style_raw(self: &DisplayNamesOptions, value: &str);

	#[js_sys(setter = "type")]
	fn set_type_raw(self: &DisplayNamesOptions, value: &str);

	#[js_sys(setter = "fallback")]
	fn set_fallback_raw(self: &DisplayNamesOptions, value: &str);

	#[js_sys(setter = "languageDisplay")]
	fn set_language_display_raw(self: &DisplayNamesOptions, value: &str);
}

impl DisplayNamesOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#options)
	#[must_use]
	pub fn new(display_names_type: DisplayNamesType) -> Self {
		let options = Self::unchecked_from(Object::new().into());
		options.set_type(display_names_type);
		options
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#localematcher)
	#[must_use]
	pub fn locale_matcher(&self) -> Option<LocaleMatcher> {
		self.locale_matcher_raw()
			.as_ref()
			.and_then(LocaleMatcher::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#localematcher)
	pub fn set_locale_matcher(&self, value: LocaleMatcher) {
		self.set_locale_matcher_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#style)
	#[must_use]
	pub fn style(&self) -> Option<DisplayNamesStyle> {
		self.style_raw()
			.as_ref()
			.and_then(DisplayNamesStyle::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#style)
	pub fn set_style(&self, value: DisplayNamesStyle) {
		self.set_style_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#type)
	#[must_use]
	pub fn display_names_type(&self) -> Option<DisplayNamesType> {
		self.type_raw()
			.as_ref()
			.and_then(DisplayNamesType::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#type)
	pub fn set_type(&self, value: DisplayNamesType) {
		self.set_type_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#fallback)
	#[must_use]
	pub fn fallback(&self) -> Option<DisplayNamesFallback> {
		self.fallback_raw()
			.as_ref()
			.and_then(DisplayNamesFallback::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#fallback)
	pub fn set_fallback(&self, value: DisplayNamesFallback) {
		self.set_fallback_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#languagedisplay)
	#[must_use]
	pub fn language_display(&self) -> Option<DisplayNamesLanguageDisplay> {
		self.language_display_raw()
			.as_ref()
			.and_then(DisplayNamesLanguageDisplay::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DisplayNames/DisplayNames#languagedisplay)
	pub fn set_language_display(&self, value: DisplayNamesLanguageDisplay) {
		self.set_language_display_raw(value.as_str());
	}
}
