use alloc::string::String;

use super::locale::HourCycle;
use super::{LocaleMatcher, LocaleMatcherOptions};
use crate::hazard::JsCast;
use crate::{Array, Function, JsString, JsValue, Object, js_sys};

macro_rules! string_enum {
	(
		$(#[$meta:meta])*
		pub enum $name:ident {
			$($variant:ident => $value:literal),+ $(,)?
		}
	) => {
		$(#[$meta])*
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		pub enum $name {
			$($variant),+
		}

		impl $name {
			const fn as_str(self) -> &'static str {
				match self {
					$(Self::$variant => $value),+
				}
			}

			fn from_str(value: &str) -> Option<Self> {
				match value {
					$($value => Some(Self::$variant)),+,
					_ => None,
				}
			}
		}
	};
}

macro_rules! readonly_string_enum {
	(
		$(#[$meta:meta])*
		pub enum $name:ident {
			$($variant:ident => $value:literal),+ $(,)?
		}
	) => {
		$(#[$meta])*
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		pub enum $name {
			$($variant),+
		}

		impl $name {
			fn from_str(value: &str) -> Option<Self> {
				match value {
					$($value => Some(Self::$variant)),+,
					_ => None,
				}
			}
		}
	};
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#date-time_component_options)
	pub enum DateTimeFormatTextStyle {
		Long => "long",
		Short => "short",
		Narrow => "narrow",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#date-time_component_options)
	pub enum DateTimeFormatNumericStyle {
		Numeric => "numeric",
		TwoDigit => "2-digit",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#month)
	pub enum DateTimeFormatMonthStyle {
		Numeric => "numeric",
		TwoDigit => "2-digit",
		Long => "long",
		Short => "short",
		Narrow => "narrow",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#formatmatcher)
	pub enum DateTimeFormatMatcher {
		Basic => "basic",
		BestFit => "best fit",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#style_shortcuts)
	pub enum DateTimeFormatStyle {
		Full => "full",
		Long => "long",
		Medium => "medium",
		Short => "short",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#timezonename)
	pub enum DateTimeFormatTimeZoneName {
		Long => "long",
		Short => "short",
		ShortOffset => "shortOffset",
		LongOffset => "longOffset",
		ShortGeneric => "shortGeneric",
		LongGeneric => "longGeneric",
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#fractionalseconddigits)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DateTimeFormatFractionalSecondDigits {
	One,
	Two,
	Three,
}

impl DateTimeFormatFractionalSecondDigits {
	const fn as_u8(self) -> u8 {
		match self {
			Self::One => 1,
			Self::Two => 2,
			Self::Three => 3,
		}
	}

	const fn from_u8(value: u8) -> Option<Self> {
		match value {
			1 => Some(Self::One),
			2 => Some(Self::Two),
			3 => Some(Self::Three),
			_ => None,
		}
	}
}

readonly_string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/formatToParts#return_value)
	pub enum DateTimeFormatPartType {
		Weekday => "weekday",
		Era => "era",
		Year => "year",
		Month => "month",
		Day => "day",
		DayPeriod => "dayPeriod",
		Hour => "hour",
		Minute => "minute",
		Second => "second",
		FractionalSecond => "fractionalSecond",
		TimeZoneName => "timeZoneName",
		Literal => "literal",
		RelatedYear => "relatedYear",
		YearName => "yearName",
		Unknown => "unknown",
	}
}

readonly_string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/formatRangeToParts#return_value)
	pub enum DateTimeFormatRangeSource {
		StartRange => "startRange",
		EndRange => "endRange",
		Shared => "shared",
	}
}

#[js_sys(js_sys = crate, namespace = "Intl")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat)
	#[js_sys(js_name = "DateTimeFormat", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DateTimeFormat;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DateTimeFormatOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DateTimeFormatResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/formatToParts#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DateTimeFormatPart;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/formatRangeToParts#return_value)
	#[js_sys(extends = DateTimeFormatPart)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DateTimeRangeFormatPart;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> DateTimeFormat;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat)
	#[js_sys(constructor)]
	pub fn new_with_locales(locales: &JsValue) -> Result<DateTimeFormat, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat)
	#[js_sys(constructor)]
	pub fn new_with_locales_and_options(
		locales: &JsValue,
		options: &DateTimeFormatOptions,
	) -> Result<DateTimeFormat, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/supportedLocalesOf)
	#[js_sys(static_of = DateTimeFormat, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of(locales: &JsValue) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/supportedLocalesOf)
	#[js_sys(static_of = DateTimeFormat, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of_with_options(
		locales: &JsValue,
		options: &LocaleMatcherOptions,
	) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/format)
	#[must_use]
	#[js_sys(getter)]
	pub fn format(self: &DateTimeFormat) -> Function;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/formatToParts)
	#[must_use]
	#[js_sys(js_name = "formatToParts")]
	pub fn format_to_parts(self: &DateTimeFormat) -> Array<DateTimeFormatPart>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/formatToParts)
	#[js_sys(js_name = "formatToParts")]
	pub fn format_to_parts_with_date(
		self: &DateTimeFormat,
		date: &JsValue,
	) -> Result<Array<DateTimeFormatPart>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/formatRange)
	#[js_sys(js_name = "formatRange")]
	pub fn format_range(
		self: &DateTimeFormat,
		start_date: &JsValue,
		end_date: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/formatRangeToParts)
	#[js_sys(js_name = "formatRangeToParts")]
	pub fn format_range_to_parts(
		self: &DateTimeFormat,
		start_date: &JsValue,
		end_date: &JsValue,
	) -> Result<Array<DateTimeRangeFormatPart>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions)
	#[must_use]
	#[js_sys(js_name = "resolvedOptions")]
	pub fn resolved_options(self: &DateTimeFormat) -> DateTimeFormatResolvedOptions;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(getter = "localeMatcher")]
	fn locale_matcher_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "localeMatcher")]
	fn set_locale_matcher_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "calendar")]
	fn calendar_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "calendar")]
	fn set_calendar_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "numberingSystem")]
	fn numbering_system_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "numberingSystem")]
	fn set_numbering_system_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "hour12")]
	fn hour_12_raw(self: &DateTimeFormatOptions) -> Option<bool>;

	#[js_sys(setter = "hour12")]
	fn set_hour_12_raw(self: &DateTimeFormatOptions, value: bool);

	#[js_sys(getter = "hourCycle")]
	fn hour_cycle_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "hourCycle")]
	fn set_hour_cycle_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "timeZone")]
	fn time_zone_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "timeZone")]
	fn set_time_zone_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "weekday")]
	fn weekday_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "weekday")]
	fn set_weekday_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "era")]
	fn era_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "era")]
	fn set_era_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "year")]
	fn year_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "year")]
	fn set_year_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "month")]
	fn month_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "month")]
	fn set_month_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "day")]
	fn day_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "day")]
	fn set_day_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "dayPeriod")]
	fn day_period_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "dayPeriod")]
	fn set_day_period_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "hour")]
	fn hour_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "hour")]
	fn set_hour_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "minute")]
	fn minute_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "minute")]
	fn set_minute_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "second")]
	fn second_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "second")]
	fn set_second_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "fractionalSecondDigits")]
	fn fractional_second_digits_raw(self: &DateTimeFormatOptions) -> Option<u8>;

	#[js_sys(setter = "fractionalSecondDigits")]
	fn set_fractional_second_digits_raw(self: &DateTimeFormatOptions, value: u8);

	#[js_sys(getter = "timeZoneName")]
	fn time_zone_name_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "timeZoneName")]
	fn set_time_zone_name_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "formatMatcher")]
	fn format_matcher_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "formatMatcher")]
	fn set_format_matcher_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "dateStyle")]
	fn date_style_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "dateStyle")]
	fn set_date_style_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "timeStyle")]
	fn time_style_raw(self: &DateTimeFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "timeStyle")]
	fn set_time_style_raw(self: &DateTimeFormatOptions, value: &str);

	#[js_sys(getter = "locale")]
	fn resolved_locale_raw(self: &DateTimeFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "calendar")]
	fn resolved_calendar_raw(self: &DateTimeFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "numberingSystem")]
	fn resolved_numbering_system_raw(self: &DateTimeFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "timeZone")]
	fn resolved_time_zone_raw(self: &DateTimeFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "hourCycle")]
	fn resolved_hour_cycle_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "hour12")]
	fn resolved_hour_12_raw(self: &DateTimeFormatResolvedOptions) -> Option<bool>;

	#[js_sys(getter = "weekday")]
	fn resolved_weekday_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "era")]
	fn resolved_era_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "year")]
	fn resolved_year_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "month")]
	fn resolved_month_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "day")]
	fn resolved_day_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "dayPeriod")]
	fn resolved_day_period_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "hour")]
	fn resolved_hour_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "minute")]
	fn resolved_minute_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "second")]
	fn resolved_second_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "fractionalSecondDigits")]
	fn resolved_fractional_second_digits_raw(self: &DateTimeFormatResolvedOptions) -> Option<u8>;

	#[js_sys(getter = "timeZoneName")]
	fn resolved_time_zone_name_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "dateStyle")]
	fn resolved_date_style_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "timeStyle")]
	fn resolved_time_style_raw(self: &DateTimeFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "type")]
	fn part_type_raw(self: &DateTimeFormatPart) -> JsString;

	#[js_sys(getter = "value")]
	fn part_value_raw(self: &DateTimeFormatPart) -> JsString;

	#[js_sys(getter = "source")]
	fn range_source_raw(self: &DateTimeRangeFormatPart) -> JsString;
}

fn parse_string_option<T>(value: Option<JsString>, parse: fn(&str) -> Option<T>) -> Option<T> {
	value.and_then(|value| parse(&String::from(value)))
}

impl DateTimeFormatOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#localematcher)
	#[must_use]
	pub fn locale_matcher(&self) -> Option<LocaleMatcher> {
		self.locale_matcher_raw()
			.as_ref()
			.and_then(LocaleMatcher::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#localematcher)
	pub fn set_locale_matcher(&self, value: LocaleMatcher) {
		self.set_locale_matcher_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#calendar)
	#[must_use]
	pub fn calendar(&self) -> Option<JsString> {
		self.calendar_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#calendar)
	pub fn set_calendar(&self, value: &str) {
		self.set_calendar_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#numberingsystem)
	#[must_use]
	pub fn numbering_system(&self) -> Option<JsString> {
		self.numbering_system_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#numberingsystem)
	pub fn set_numbering_system(&self, value: &str) {
		self.set_numbering_system_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#hour12)
	#[must_use]
	pub fn hour_12(&self) -> Option<bool> {
		self.hour_12_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#hour12)
	pub fn set_hour_12(&self, value: bool) {
		self.set_hour_12_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#hourcycle)
	#[must_use]
	pub fn hour_cycle(&self) -> Option<HourCycle> {
		self.hour_cycle_raw()
			.as_ref()
			.and_then(HourCycle::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#hourcycle)
	pub fn set_hour_cycle(&self, value: HourCycle) {
		self.set_hour_cycle_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#timezone)
	#[must_use]
	pub fn time_zone(&self) -> Option<JsString> {
		self.time_zone_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#timezone)
	pub fn set_time_zone(&self, value: &str) {
		self.set_time_zone_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#weekday)
	#[must_use]
	pub fn weekday(&self) -> Option<DateTimeFormatTextStyle> {
		parse_string_option(self.weekday_raw(), DateTimeFormatTextStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#weekday)
	pub fn set_weekday(&self, value: DateTimeFormatTextStyle) {
		self.set_weekday_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#era)
	#[must_use]
	pub fn era(&self) -> Option<DateTimeFormatTextStyle> {
		parse_string_option(self.era_raw(), DateTimeFormatTextStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#era)
	pub fn set_era(&self, value: DateTimeFormatTextStyle) {
		self.set_era_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#year)
	#[must_use]
	pub fn year(&self) -> Option<DateTimeFormatNumericStyle> {
		parse_string_option(self.year_raw(), DateTimeFormatNumericStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#year)
	pub fn set_year(&self, value: DateTimeFormatNumericStyle) {
		self.set_year_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#month)
	#[must_use]
	pub fn month(&self) -> Option<DateTimeFormatMonthStyle> {
		parse_string_option(self.month_raw(), DateTimeFormatMonthStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#month)
	pub fn set_month(&self, value: DateTimeFormatMonthStyle) {
		self.set_month_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#day)
	#[must_use]
	pub fn day(&self) -> Option<DateTimeFormatNumericStyle> {
		parse_string_option(self.day_raw(), DateTimeFormatNumericStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#day)
	pub fn set_day(&self, value: DateTimeFormatNumericStyle) {
		self.set_day_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#dayperiod)
	#[must_use]
	pub fn day_period(&self) -> Option<DateTimeFormatTextStyle> {
		parse_string_option(self.day_period_raw(), DateTimeFormatTextStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#dayperiod)
	pub fn set_day_period(&self, value: DateTimeFormatTextStyle) {
		self.set_day_period_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#hour)
	#[must_use]
	pub fn hour(&self) -> Option<DateTimeFormatNumericStyle> {
		parse_string_option(self.hour_raw(), DateTimeFormatNumericStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#hour)
	pub fn set_hour(&self, value: DateTimeFormatNumericStyle) {
		self.set_hour_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#minute)
	#[must_use]
	pub fn minute(&self) -> Option<DateTimeFormatNumericStyle> {
		parse_string_option(self.minute_raw(), DateTimeFormatNumericStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#minute)
	pub fn set_minute(&self, value: DateTimeFormatNumericStyle) {
		self.set_minute_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#second)
	#[must_use]
	pub fn second(&self) -> Option<DateTimeFormatNumericStyle> {
		parse_string_option(self.second_raw(), DateTimeFormatNumericStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#second)
	pub fn set_second(&self, value: DateTimeFormatNumericStyle) {
		self.set_second_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#fractionalseconddigits)
	#[must_use]
	pub fn fractional_second_digits(&self) -> Option<DateTimeFormatFractionalSecondDigits> {
		self.fractional_second_digits_raw()
			.and_then(DateTimeFormatFractionalSecondDigits::from_u8)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#fractionalseconddigits)
	pub fn set_fractional_second_digits(&self, value: DateTimeFormatFractionalSecondDigits) {
		self.set_fractional_second_digits_raw(value.as_u8());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#timezonename)
	#[must_use]
	pub fn time_zone_name(&self) -> Option<DateTimeFormatTimeZoneName> {
		parse_string_option(
			self.time_zone_name_raw(),
			DateTimeFormatTimeZoneName::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#timezonename)
	pub fn set_time_zone_name(&self, value: DateTimeFormatTimeZoneName) {
		self.set_time_zone_name_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#formatmatcher)
	#[must_use]
	pub fn format_matcher(&self) -> Option<DateTimeFormatMatcher> {
		parse_string_option(self.format_matcher_raw(), DateTimeFormatMatcher::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#formatmatcher)
	pub fn set_format_matcher(&self, value: DateTimeFormatMatcher) {
		self.set_format_matcher_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#datestyle)
	#[must_use]
	pub fn date_style(&self) -> Option<DateTimeFormatStyle> {
		parse_string_option(self.date_style_raw(), DateTimeFormatStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#datestyle)
	pub fn set_date_style(&self, value: DateTimeFormatStyle) {
		self.set_date_style_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#timestyle)
	#[must_use]
	pub fn time_style(&self) -> Option<DateTimeFormatStyle> {
		parse_string_option(self.time_style_raw(), DateTimeFormatStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/DateTimeFormat#timestyle)
	pub fn set_time_style(&self, value: DateTimeFormatStyle) {
		self.set_time_style_raw(value.as_str());
	}
}

impl Default for DateTimeFormatOptions {
	fn default() -> Self {
		Self::new()
	}
}

impl Default for DateTimeFormat {
	fn default() -> Self {
		Self::new()
	}
}

impl DateTimeFormatResolvedOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn locale(&self) -> JsString {
		self.resolved_locale_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn calendar(&self) -> JsString {
		self.resolved_calendar_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn numbering_system(&self) -> JsString {
		self.resolved_numbering_system_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn time_zone(&self) -> JsString {
		self.resolved_time_zone_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn hour_cycle(&self) -> Option<HourCycle> {
		self.resolved_hour_cycle_raw()
			.as_ref()
			.and_then(HourCycle::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn hour_12(&self) -> Option<bool> {
		self.resolved_hour_12_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn weekday(&self) -> Option<DateTimeFormatTextStyle> {
		parse_string_option(
			self.resolved_weekday_raw(),
			DateTimeFormatTextStyle::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn era(&self) -> Option<DateTimeFormatTextStyle> {
		parse_string_option(self.resolved_era_raw(), DateTimeFormatTextStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn year(&self) -> Option<DateTimeFormatNumericStyle> {
		parse_string_option(
			self.resolved_year_raw(),
			DateTimeFormatNumericStyle::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn month(&self) -> Option<DateTimeFormatMonthStyle> {
		parse_string_option(
			self.resolved_month_raw(),
			DateTimeFormatMonthStyle::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn day(&self) -> Option<DateTimeFormatNumericStyle> {
		parse_string_option(
			self.resolved_day_raw(),
			DateTimeFormatNumericStyle::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn day_period(&self) -> Option<DateTimeFormatTextStyle> {
		parse_string_option(
			self.resolved_day_period_raw(),
			DateTimeFormatTextStyle::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn hour(&self) -> Option<DateTimeFormatNumericStyle> {
		parse_string_option(
			self.resolved_hour_raw(),
			DateTimeFormatNumericStyle::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn minute(&self) -> Option<DateTimeFormatNumericStyle> {
		parse_string_option(
			self.resolved_minute_raw(),
			DateTimeFormatNumericStyle::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn second(&self) -> Option<DateTimeFormatNumericStyle> {
		parse_string_option(
			self.resolved_second_raw(),
			DateTimeFormatNumericStyle::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn fractional_second_digits(&self) -> Option<DateTimeFormatFractionalSecondDigits> {
		self.resolved_fractional_second_digits_raw()
			.and_then(DateTimeFormatFractionalSecondDigits::from_u8)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn time_zone_name(&self) -> Option<DateTimeFormatTimeZoneName> {
		parse_string_option(
			self.resolved_time_zone_name_raw(),
			DateTimeFormatTimeZoneName::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn date_style(&self) -> Option<DateTimeFormatStyle> {
		parse_string_option(
			self.resolved_date_style_raw(),
			DateTimeFormatStyle::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn time_style(&self) -> Option<DateTimeFormatStyle> {
		parse_string_option(
			self.resolved_time_style_raw(),
			DateTimeFormatStyle::from_str,
		)
	}
}

impl DateTimeFormatPart {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/formatToParts#return_value)
	#[must_use]
	pub fn type_(&self) -> Option<DateTimeFormatPartType> {
		DateTimeFormatPartType::from_str(&String::from(self.part_type_raw()))
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/formatToParts#return_value)
	#[must_use]
	pub fn value(&self) -> JsString {
		self.part_value_raw()
	}
}

impl DateTimeRangeFormatPart {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/formatRangeToParts#return_value)
	#[must_use]
	pub fn source(&self) -> Option<DateTimeFormatRangeSource> {
		DateTimeFormatRangeSource::from_str(&String::from(self.range_source_raw()))
	}
}
