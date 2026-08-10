use alloc::string::String;

use super::{LocaleMatcher, LocaleMatcherOptions};
use crate::hazard::JsCast;
use crate::{Array, JsString, JsValue, Object, js_sys};

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
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#style)
	pub enum DurationFormatStyle {
		Long => "long",
		Short => "short",
		Narrow => "narrow",
		Digital => "digital",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#years)
	pub enum DurationUnitStyle {
		Long => "long",
		Short => "short",
		Narrow => "narrow",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#hours)
	pub enum DurationTimeUnitStyle {
		Long => "long",
		Short => "short",
		Narrow => "narrow",
		Numeric => "numeric",
		TwoDigit => "2-digit",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#milliseconds)
	pub enum DurationSubsecondUnitStyle {
		Long => "long",
		Short => "short",
		Narrow => "narrow",
		Numeric => "numeric",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#yearsdisplay)
	pub enum DurationUnitDisplay {
		Always => "always",
		Auto => "auto",
	}
}

readonly_string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/formatToParts#return_value)
	pub enum DurationFormatPartType {
		Integer => "integer",
		Group => "group",
		Decimal => "decimal",
		Fraction => "fraction",
		Literal => "literal",
		Unit => "unit",
		MinusSign => "minusSign",
	}
}

readonly_string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/formatToParts#return_value)
	pub enum DurationUnit {
		Year => "year",
		Month => "month",
		Week => "week",
		Day => "day",
		Hour => "hour",
		Minute => "minute",
		Second => "second",
		Millisecond => "millisecond",
		Microsecond => "microsecond",
		Nanosecond => "nanosecond",
	}
}

#[js_sys(js_sys = crate, namespace = "Intl")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat)
	#[js_sys(js_name = "DurationFormat", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DurationFormat;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DurationFormatOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/resolvedOptions#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DurationFormatResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Duration;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/formatToParts#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type DurationFormatPart;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> DurationFormat;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat)
	#[js_sys(constructor)]
	pub fn new_with_locales(locales: &JsValue) -> Result<DurationFormat, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat)
	#[js_sys(constructor)]
	pub fn new_with_locales_and_options(
		locales: &JsValue,
		options: &DurationFormatOptions,
	) -> Result<DurationFormat, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/supportedLocalesOf)
	#[js_sys(static_of = DurationFormat, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of(locales: &JsValue) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/supportedLocalesOf)
	#[js_sys(static_of = DurationFormat, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of_with_options(
		locales: &JsValue,
		options: &LocaleMatcherOptions,
	) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format)
	pub fn format(self: &DurationFormat, duration: &Duration) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/formatToParts)
	#[js_sys(js_name = "formatToParts")]
	pub fn format_to_parts(
		self: &DurationFormat,
		duration: &Duration,
	) -> Result<Array<DurationFormatPart>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/resolvedOptions)
	#[must_use]
	#[js_sys(js_name = "resolvedOptions")]
	pub fn resolved_options(self: &DurationFormat) -> DurationFormatResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#numberingsystem)
	#[must_use]
	#[js_sys(getter = "numberingSystem")]
	pub fn numbering_system(self: &DurationFormatOptions) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#numberingsystem)
	#[js_sys(setter = "numberingSystem")]
	pub fn set_numbering_system(self: &DurationFormatOptions, value: &str);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#fractionaldigits)
	#[must_use]
	#[js_sys(getter = "fractionalDigits")]
	pub fn fractional_digits(self: &DurationFormatOptions) -> Option<u8>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#fractionaldigits)
	#[js_sys(setter = "fractionalDigits")]
	pub fn set_fractional_digits(self: &DurationFormatOptions, value: u8);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn locale(self: &DurationFormatResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "numberingSystem")]
	pub fn numbering_system(self: &DurationFormatResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "fractionalDigits")]
	pub fn fractional_digits(self: &DurationFormatResolvedOptions) -> Option<u8>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[must_use]
	#[js_sys(getter)]
	pub fn years(self: &Duration) -> Option<f64>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[js_sys(setter)]
	pub fn set_years(self: &Duration, value: f64);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[must_use]
	#[js_sys(getter)]
	pub fn months(self: &Duration) -> Option<f64>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[js_sys(setter)]
	pub fn set_months(self: &Duration, value: f64);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[must_use]
	#[js_sys(getter)]
	pub fn weeks(self: &Duration) -> Option<f64>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[js_sys(setter)]
	pub fn set_weeks(self: &Duration, value: f64);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[must_use]
	#[js_sys(getter)]
	pub fn days(self: &Duration) -> Option<f64>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[js_sys(setter)]
	pub fn set_days(self: &Duration, value: f64);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[must_use]
	#[js_sys(getter)]
	pub fn hours(self: &Duration) -> Option<f64>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[js_sys(setter)]
	pub fn set_hours(self: &Duration, value: f64);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[must_use]
	#[js_sys(getter)]
	pub fn minutes(self: &Duration) -> Option<f64>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[js_sys(setter)]
	pub fn set_minutes(self: &Duration, value: f64);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[must_use]
	#[js_sys(getter)]
	pub fn seconds(self: &Duration) -> Option<f64>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[js_sys(setter)]
	pub fn set_seconds(self: &Duration, value: f64);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[must_use]
	#[js_sys(getter)]
	pub fn milliseconds(self: &Duration) -> Option<f64>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[js_sys(setter)]
	pub fn set_milliseconds(self: &Duration, value: f64);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[must_use]
	#[js_sys(getter)]
	pub fn microseconds(self: &Duration) -> Option<f64>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[js_sys(setter)]
	pub fn set_microseconds(self: &Duration, value: f64);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[must_use]
	#[js_sys(getter)]
	pub fn nanoseconds(self: &Duration) -> Option<f64>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[js_sys(setter)]
	pub fn set_nanoseconds(self: &Duration, value: f64);
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(getter = "localeMatcher")]
	fn locale_matcher_raw(self: &DurationFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "localeMatcher")]
	fn set_locale_matcher_raw(self: &DurationFormatOptions, value: &str);

	#[js_sys(getter = "style")]
	fn style_raw(self: &DurationFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "style")]
	fn set_style_raw(self: &DurationFormatOptions, value: &str);

	#[js_sys(getter = "style")]
	fn resolved_style_raw(self: &DurationFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "type")]
	fn part_type_raw(self: &DurationFormatPart) -> JsString;

	#[js_sys(getter = "value")]
	fn part_value_raw(self: &DurationFormatPart) -> JsString;

	#[js_sys(getter = "unit")]
	fn part_unit_raw(self: &DurationFormatPart) -> Option<JsString>;
}

fn parse_string_option<T>(
	value: Option<JsString>,
	parse: impl FnOnce(&str) -> Option<T>,
) -> Option<T> {
	let value = String::from(value?);
	parse(&value)
}

macro_rules! duration_unit_options {
	(
		$(
			$(#[$meta:meta])*
			$get:ident, $set:ident, $get_raw:ident, $set_raw:ident, $resolved_raw:ident:
				$ty:ty = $js_name:literal;
		)+
	) => {
		#[js_sys(js_sys = crate)]
		extern "js-sys" {
			$(
				#[js_sys(getter = $js_name)]
				fn $get_raw(self: &DurationFormatOptions) -> Option<JsString>;

				#[js_sys(setter = $js_name)]
				fn $set_raw(self: &DurationFormatOptions, value: &str);

				#[js_sys(getter = $js_name)]
				fn $resolved_raw(self: &DurationFormatResolvedOptions) -> JsString;
			)+
		}

		impl DurationFormatOptions {
			$(
				$(#[$meta])*
				#[must_use]
				pub fn $get(&self) -> Option<$ty> {
					parse_string_option(self.$get_raw(), <$ty>::from_str)
				}

				$(#[$meta])*
				pub fn $set(&self, value: $ty) {
					self.$set_raw(value.as_str());
				}
			)+
		}

		impl DurationFormatResolvedOptions {
			$(
				$(#[$meta])*
				#[must_use]
				pub fn $get(&self) -> Option<$ty> {
					<$ty>::from_str(&String::from(self.$resolved_raw()))
				}
			)+
		}
	};
}

duration_unit_options! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#years)
	years, set_years, years_raw, set_years_raw, resolved_years_raw:
		DurationUnitStyle = "years";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#yearsdisplay)
	years_display, set_years_display, years_display_raw, set_years_display_raw, resolved_years_display_raw:
		DurationUnitDisplay = "yearsDisplay";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#months)
	months, set_months, months_raw, set_months_raw, resolved_months_raw:
		DurationUnitStyle = "months";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#monthsdisplay)
	months_display, set_months_display, months_display_raw, set_months_display_raw, resolved_months_display_raw:
		DurationUnitDisplay = "monthsDisplay";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#weeks)
	weeks, set_weeks, weeks_raw, set_weeks_raw, resolved_weeks_raw:
		DurationUnitStyle = "weeks";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#weeksdisplay)
	weeks_display, set_weeks_display, weeks_display_raw, set_weeks_display_raw, resolved_weeks_display_raw:
		DurationUnitDisplay = "weeksDisplay";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#days)
	days, set_days, days_raw, set_days_raw, resolved_days_raw:
		DurationUnitStyle = "days";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#daysdisplay)
	days_display, set_days_display, days_display_raw, set_days_display_raw, resolved_days_display_raw:
		DurationUnitDisplay = "daysDisplay";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#hours)
	hours, set_hours, hours_raw, set_hours_raw, resolved_hours_raw:
		DurationTimeUnitStyle = "hours";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#hoursdisplay)
	hours_display, set_hours_display, hours_display_raw, set_hours_display_raw, resolved_hours_display_raw:
		DurationUnitDisplay = "hoursDisplay";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#minutes)
	minutes, set_minutes, minutes_raw, set_minutes_raw, resolved_minutes_raw:
		DurationTimeUnitStyle = "minutes";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#minutesdisplay)
	minutes_display, set_minutes_display, minutes_display_raw, set_minutes_display_raw, resolved_minutes_display_raw:
		DurationUnitDisplay = "minutesDisplay";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#seconds)
	seconds, set_seconds, seconds_raw, set_seconds_raw, resolved_seconds_raw:
		DurationTimeUnitStyle = "seconds";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#secondsdisplay)
	seconds_display, set_seconds_display, seconds_display_raw, set_seconds_display_raw, resolved_seconds_display_raw:
		DurationUnitDisplay = "secondsDisplay";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#milliseconds)
	milliseconds, set_milliseconds, milliseconds_raw, set_milliseconds_raw, resolved_milliseconds_raw:
		DurationSubsecondUnitStyle = "milliseconds";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#millisecondsdisplay)
	milliseconds_display, set_milliseconds_display, milliseconds_display_raw, set_milliseconds_display_raw, resolved_milliseconds_display_raw:
		DurationUnitDisplay = "millisecondsDisplay";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#microseconds)
	microseconds, set_microseconds, microseconds_raw, set_microseconds_raw, resolved_microseconds_raw:
		DurationSubsecondUnitStyle = "microseconds";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#microsecondsdisplay)
	microseconds_display, set_microseconds_display, microseconds_display_raw, set_microseconds_display_raw, resolved_microseconds_display_raw:
		DurationUnitDisplay = "microsecondsDisplay";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#nanoseconds)
	nanoseconds, set_nanoseconds, nanoseconds_raw, set_nanoseconds_raw, resolved_nanoseconds_raw:
		DurationSubsecondUnitStyle = "nanoseconds";
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#nanosecondsdisplay)
	nanoseconds_display, set_nanoseconds_display, nanoseconds_display_raw, set_nanoseconds_display_raw, resolved_nanoseconds_display_raw:
		DurationUnitDisplay = "nanosecondsDisplay";
}

impl DurationFormatOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#localematcher)
	#[must_use]
	pub fn locale_matcher(&self) -> Option<LocaleMatcher> {
		self.locale_matcher_raw()
			.as_ref()
			.and_then(LocaleMatcher::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#localematcher)
	pub fn set_locale_matcher(&self, value: LocaleMatcher) {
		self.set_locale_matcher_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#style)
	#[must_use]
	pub fn style(&self) -> Option<DurationFormatStyle> {
		parse_string_option(self.style_raw(), DurationFormatStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/DurationFormat#style)
	pub fn set_style(&self, value: DurationFormatStyle) {
		self.set_style_raw(value.as_str());
	}
}

impl DurationFormatResolvedOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn style(&self) -> Option<DurationFormatStyle> {
		DurationFormatStyle::from_str(&String::from(self.resolved_style_raw()))
	}
}

impl Duration {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/format#duration)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}
}

impl DurationFormatPart {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/formatToParts#return_value)
	#[must_use]
	pub fn type_(&self) -> Option<DurationFormatPartType> {
		DurationFormatPartType::from_str(&String::from(self.part_type_raw()))
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/formatToParts#return_value)
	#[must_use]
	pub fn value(&self) -> JsString {
		self.part_value_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DurationFormat/formatToParts#return_value)
	#[must_use]
	pub fn unit(&self) -> Option<DurationUnit> {
		parse_string_option(self.part_unit_raw(), DurationUnit::from_str)
	}
}

impl Default for DurationFormatOptions {
	fn default() -> Self {
		Self::new()
	}
}

impl Default for Duration {
	fn default() -> Self {
		Self::new()
	}
}

impl Default for DurationFormat {
	fn default() -> Self {
		Self::new()
	}
}
