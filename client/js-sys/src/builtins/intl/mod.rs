mod collator;
mod date_time_format;
mod display_names;
mod duration_format;
mod list_format;
mod locale;
mod number_format;
mod plural_rules;
mod relative_time_format;
mod segmenter;

use alloc::string::String;

use crate::hazard::JsCast;
use crate::{JsString, Object, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl#locale_identification_and_negotiation)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocaleMatcher {
	Lookup,
	BestFit,
}

impl LocaleMatcher {
	pub(super) const fn as_str(self) -> &'static str {
		match self {
			Self::Lookup => "lookup",
			Self::BestFit => "best fit",
		}
	}

	pub(super) fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"lookup" => Some(Self::Lookup),
			"best fit" => Some(Self::BestFit),
			_ => None,
		}
	}
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/supportedLocalesOf#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type LocaleMatcherOptions;

	#[js_sys(getter = "localeMatcher")]
	fn locale_matcher_raw(self: &LocaleMatcherOptions) -> Option<JsString>;

	#[js_sys(setter = "localeMatcher")]
	fn set_locale_matcher_raw(self: &LocaleMatcherOptions, value: &str);
}

impl LocaleMatcherOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/supportedLocalesOf#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/supportedLocalesOf#options)
	#[must_use]
	pub fn locale_matcher(&self) -> Option<LocaleMatcher> {
		self.locale_matcher_raw()
			.as_ref()
			.and_then(LocaleMatcher::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator/supportedLocalesOf#options)
	pub fn set_locale_matcher(&self, value: LocaleMatcher) {
		self.set_locale_matcher_raw(value.as_str());
	}
}

impl Default for LocaleMatcherOptions {
	fn default() -> Self {
		Self::new()
	}
}

#[expect(non_snake_case, reason = "matches the JavaScript global name")]
pub mod Intl {
	pub use super::collator::{
		Collator, CollatorCaseFirst, CollatorOptions, CollatorResolvedOptions, CollatorSensitivity,
		CollatorUsage,
	};
	pub use super::date_time_format::{
		DateTimeFormat, DateTimeFormatFractionalSecondDigits, DateTimeFormatMatcher,
		DateTimeFormatMonthStyle, DateTimeFormatNumericStyle, DateTimeFormatOptions,
		DateTimeFormatPart, DateTimeFormatPartType, DateTimeFormatRangeSource,
		DateTimeFormatResolvedOptions, DateTimeFormatStyle, DateTimeFormatTextStyle,
		DateTimeFormatTimeZoneName, DateTimeRangeFormatPart,
	};
	pub use super::display_names::{
		DisplayNames, DisplayNamesFallback, DisplayNamesLanguageDisplay, DisplayNamesOptions,
		DisplayNamesResolvedOptions, DisplayNamesStyle, DisplayNamesType,
	};
	pub use super::duration_format::{
		Duration, DurationFormat, DurationFormatOptions, DurationFormatPart,
		DurationFormatPartType, DurationFormatResolvedOptions, DurationFormatStyle,
		DurationSubsecondUnitStyle, DurationTimeUnitStyle, DurationUnit, DurationUnitDisplay,
		DurationUnitStyle,
	};
	pub use super::list_format::{
		ListFormat, ListFormatOptions, ListFormatPart, ListFormatResolvedOptions, ListFormatStyle,
		ListFormatType,
	};
	pub use super::locale::{
		FirstDayOfWeek, HourCycle, Locale, LocaleOptions, TextDirection, TextInfo, WeekInfo,
	};
	pub use super::number_format::{
		NumberFormat, NumberFormatCompactDisplay, NumberFormatCurrencyDisplay,
		NumberFormatCurrencySign, NumberFormatNotation, NumberFormatOptions, NumberFormatPart,
		NumberFormatPartType, NumberFormatRangeSource, NumberFormatResolvedOptions,
		NumberFormatRoundingIncrement, NumberFormatRoundingMode, NumberFormatRoundingPriority,
		NumberFormatSignDisplay, NumberFormatStyle, NumberFormatTrailingZeroDisplay,
		NumberFormatUnitDisplay, NumberFormatUseGrouping, NumberRangeFormatPart,
	};
	pub use super::plural_rules::{
		PluralRules, PluralRulesOptions, PluralRulesResolvedOptions, PluralRulesRoundingIncrement,
		PluralRulesRoundingMode, PluralRulesRoundingPriority, PluralRulesTrailingZeroDisplay,
		PluralRulesType,
	};
	pub use super::relative_time_format::{
		RelativeTimeFormat, RelativeTimeFormatNumeric, RelativeTimeFormatOptions,
		RelativeTimeFormatPart, RelativeTimeFormatResolvedOptions, RelativeTimeFormatStyle,
		RelativeTimeUnit,
	};
	pub use super::segmenter::{
		SegmentData, Segmenter, SegmenterGranularity, SegmenterOptions, SegmenterResolvedOptions,
		Segments,
	};
	pub use super::{LocaleMatcher, LocaleMatcherOptions};
	use crate::{Array, JsString, JsValue, js_sys};

	#[js_sys(js_sys = crate, namespace = "Intl")]
	extern "js-sys" {
		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/getCanonicalLocales)
		#[js_sys(js_name = "getCanonicalLocales")]
		pub fn get_canonical_locales(locales: &JsValue) -> Result<Array<JsString>, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/supportedValuesOf)
		#[js_sys(js_name = "supportedValuesOf")]
		pub fn supported_values_of(key: &str) -> Result<Array<JsString>, JsValue>;
	}
}
