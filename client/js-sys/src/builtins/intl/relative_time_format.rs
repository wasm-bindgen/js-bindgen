use alloc::string::String;

use super::{LocaleMatcher, LocaleMatcherOptions};
use crate::hazard::JsCast;
use crate::{Array, JsString, JsValue, Object, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat#style)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelativeTimeFormatStyle {
	Long,
	Short,
	Narrow,
}

impl RelativeTimeFormatStyle {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Long => "long",
			Self::Short => "short",
			Self::Narrow => "narrow",
		}
	}

	fn parse(value: &str) -> Option<Self> {
		match value {
			"long" => Some(Self::Long),
			"short" => Some(Self::Short),
			"narrow" => Some(Self::Narrow),
			_ => None,
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat#numeric)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelativeTimeFormatNumeric {
	Always,
	Auto,
}

impl RelativeTimeFormatNumeric {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Always => "always",
			Self::Auto => "auto",
		}
	}

	fn parse(value: &str) -> Option<Self> {
		match value {
			"always" => Some(Self::Always),
			"auto" => Some(Self::Auto),
			_ => None,
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/format#unit)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelativeTimeUnit {
	Year,
	Quarter,
	Month,
	Week,
	Day,
	Hour,
	Minute,
	Second,
}

impl RelativeTimeUnit {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Year => "year",
			Self::Quarter => "quarter",
			Self::Month => "month",
			Self::Week => "week",
			Self::Day => "day",
			Self::Hour => "hour",
			Self::Minute => "minute",
			Self::Second => "second",
		}
	}
}

#[js_sys(js_sys = crate, namespace = "Intl")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat)
	#[js_sys(js_name = "RelativeTimeFormat", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type RelativeTimeFormat;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type RelativeTimeFormatOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/resolvedOptions#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type RelativeTimeFormatResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/formatToParts#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type RelativeTimeFormatPart;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> RelativeTimeFormat;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat)
	#[js_sys(constructor)]
	pub fn new_with_locales(locales: &JsValue) -> Result<RelativeTimeFormat, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat)
	#[js_sys(constructor)]
	pub fn new_with_locales_and_options(
		locales: &JsValue,
		options: &RelativeTimeFormatOptions,
	) -> Result<RelativeTimeFormat, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/supportedLocalesOf)
	#[js_sys(static_of = RelativeTimeFormat, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of(locales: &JsValue) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/supportedLocalesOf)
	#[js_sys(static_of = RelativeTimeFormat, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of_with_options(
		locales: &JsValue,
		options: &LocaleMatcherOptions,
	) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/resolvedOptions)
	#[must_use]
	#[js_sys(js_name = "resolvedOptions")]
	pub fn resolved_options(self: &RelativeTimeFormat) -> RelativeTimeFormatResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn locale(self: &RelativeTimeFormatResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn style(self: &RelativeTimeFormatResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn numeric(self: &RelativeTimeFormatResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "numberingSystem")]
	pub fn numbering_system(self: &RelativeTimeFormatResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/formatToParts#return_value)
	#[must_use]
	#[js_sys(getter = "type")]
	pub fn type_(self: &RelativeTimeFormatPart) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/formatToParts#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn value(self: &RelativeTimeFormatPart) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/formatToParts#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn unit(self: &RelativeTimeFormatPart) -> Option<JsString>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_name = "format")]
	fn format_raw(self: &RelativeTimeFormat, value: f64, unit: &str) -> Result<JsString, JsValue>;

	#[js_sys(js_name = "formatToParts")]
	fn format_to_parts_raw(
		self: &RelativeTimeFormat,
		value: f64,
		unit: &str,
	) -> Result<Array<RelativeTimeFormatPart>, JsValue>;

	#[js_sys(getter = "localeMatcher")]
	fn locale_matcher_raw(self: &RelativeTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "localeMatcher")]
	fn set_locale_matcher_raw(self: &RelativeTimeFormatOptions, value: &str);

	#[js_sys(getter = "numberingSystem")]
	fn numbering_system_raw(self: &RelativeTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "numberingSystem")]
	fn set_numbering_system_raw(self: &RelativeTimeFormatOptions, value: &str);

	#[js_sys(getter = "style")]
	fn style_raw(self: &RelativeTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "style")]
	fn set_style_raw(self: &RelativeTimeFormatOptions, value: &str);

	#[js_sys(getter = "numeric")]
	fn numeric_raw(self: &RelativeTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "numeric")]
	fn set_numeric_raw(self: &RelativeTimeFormatOptions, value: &str);
}

fn parse_string_option<T>(
	value: Option<JsString>,
	parse: impl FnOnce(&str) -> Option<T>,
) -> Option<T> {
	let value = String::from(value?);
	parse(&value)
}

impl RelativeTimeFormatOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat#localematcher)
	#[must_use]
	pub fn locale_matcher(&self) -> Option<LocaleMatcher> {
		self.locale_matcher_raw()
			.as_ref()
			.and_then(LocaleMatcher::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat#localematcher)
	pub fn set_locale_matcher(&self, value: LocaleMatcher) {
		self.set_locale_matcher_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat#numberingsystem)
	#[must_use]
	pub fn numbering_system(&self) -> Option<JsString> {
		self.numbering_system_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat#numberingsystem)
	pub fn set_numbering_system(&self, value: &str) {
		self.set_numbering_system_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat#style)
	#[must_use]
	pub fn style(&self) -> Option<RelativeTimeFormatStyle> {
		parse_string_option(self.style_raw(), RelativeTimeFormatStyle::parse)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat#style)
	pub fn set_style(&self, value: RelativeTimeFormatStyle) {
		self.set_style_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat#numeric)
	#[must_use]
	pub fn numeric(&self) -> Option<RelativeTimeFormatNumeric> {
		parse_string_option(self.numeric_raw(), RelativeTimeFormatNumeric::parse)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/RelativeTimeFormat#numeric)
	pub fn set_numeric(&self, value: RelativeTimeFormatNumeric) {
		self.set_numeric_raw(value.as_str());
	}
}

impl RelativeTimeFormat {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/format)
	pub fn format(&self, value: f64, unit: RelativeTimeUnit) -> Result<JsString, JsValue> {
		self.format_raw(value, unit.as_str())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/RelativeTimeFormat/formatToParts)
	pub fn format_to_parts(
		&self,
		value: f64,
		unit: RelativeTimeUnit,
	) -> Result<Array<RelativeTimeFormatPart>, JsValue> {
		self.format_to_parts_raw(value, unit.as_str())
	}
}

impl Default for RelativeTimeFormatOptions {
	fn default() -> Self {
		Self::new()
	}
}

impl Default for RelativeTimeFormat {
	fn default() -> Self {
		Self::new()
	}
}
