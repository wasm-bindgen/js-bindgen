use super::temporal::Temporal::Instant;
use super::{JsString, Object};
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Date;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> Date;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date)
	#[js_sys(constructor)]
	pub fn new_with_value(value: &JsValue) -> Result<Date, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_milliseconds(milliseconds: f64) -> Date;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_string(value: &str) -> Date;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_date(value: &Date) -> Date;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_year_month(year: f64, month: f64) -> Date;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_year_month_day(year: f64, month: f64, day: f64) -> Date;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_year_month_day_hour(year: f64, month: f64, day: f64, hour: f64) -> Date;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_year_month_day_hour_minute(
		year: f64,
		month: f64,
		day: f64,
		hour: f64,
		minute: f64,
	) -> Date;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_year_month_day_hour_minute_second(
		year: f64,
		month: f64,
		day: f64,
		hour: f64,
		minute: f64,
		second: f64,
	) -> Date;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/Date)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_year_month_day_hour_minute_second_millisecond(
		year: f64,
		month: f64,
		day: f64,
		hour: f64,
		minute: f64,
		second: f64,
		millisecond: f64,
	) -> Date;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now)
	#[must_use]
	#[js_sys(static_of = Date)]
	pub fn now() -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/parse)
	#[must_use]
	#[js_sys(static_of = Date)]
	pub fn parse(date: &str) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/UTC)
	#[must_use]
	#[js_sys(static_of = Date, js_name = "UTC")]
	pub fn utc(year: f64, month: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/UTC)
	#[must_use]
	#[js_sys(static_of = Date, js_name = "UTC")]
	pub fn utc_with_day(year: f64, month: f64, day: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/UTC)
	#[must_use]
	#[js_sys(static_of = Date, js_name = "UTC")]
	pub fn utc_with_day_hour(year: f64, month: f64, day: f64, hour: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/UTC)
	#[must_use]
	#[js_sys(static_of = Date, js_name = "UTC")]
	pub fn utc_with_day_hour_minute(year: f64, month: f64, day: f64, hour: f64, minute: f64)
	-> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/UTC)
	#[must_use]
	#[js_sys(static_of = Date, js_name = "UTC")]
	pub fn utc_with_day_hour_minute_second(
		year: f64,
		month: f64,
		day: f64,
		hour: f64,
		minute: f64,
		second: f64,
	) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/UTC)
	#[must_use]
	#[js_sys(static_of = Date, js_name = "UTC")]
	pub fn utc_with_day_hour_minute_second_millisecond(
		year: f64,
		month: f64,
		day: f64,
		hour: f64,
		minute: f64,
		second: f64,
		millisecond: f64,
	) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDate)
	#[must_use]
	#[js_sys(js_name = "getDate")]
	pub fn get_date(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getDay)
	#[must_use]
	#[js_sys(js_name = "getDay")]
	pub fn get_day(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getFullYear)
	#[must_use]
	#[js_sys(js_name = "getFullYear")]
	pub fn get_full_year(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getHours)
	#[must_use]
	#[js_sys(js_name = "getHours")]
	pub fn get_hours(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMilliseconds)
	#[must_use]
	#[js_sys(js_name = "getMilliseconds")]
	pub fn get_milliseconds(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMinutes)
	#[must_use]
	#[js_sys(js_name = "getMinutes")]
	pub fn get_minutes(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getMonth)
	#[must_use]
	#[js_sys(js_name = "getMonth")]
	pub fn get_month(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getSeconds)
	#[must_use]
	#[js_sys(js_name = "getSeconds")]
	pub fn get_seconds(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTime)
	#[must_use]
	#[js_sys(js_name = "getTime")]
	pub fn get_time(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTimezoneOffset)
	#[must_use]
	#[js_sys(js_name = "getTimezoneOffset")]
	pub fn get_timezone_offset(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDate)
	#[must_use]
	#[js_sys(js_name = "getUTCDate")]
	pub fn get_utc_date(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCDay)
	#[must_use]
	#[js_sys(js_name = "getUTCDay")]
	pub fn get_utc_day(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCFullYear)
	#[must_use]
	#[js_sys(js_name = "getUTCFullYear")]
	pub fn get_utc_full_year(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCHours)
	#[must_use]
	#[js_sys(js_name = "getUTCHours")]
	pub fn get_utc_hours(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMilliseconds)
	#[must_use]
	#[js_sys(js_name = "getUTCMilliseconds")]
	pub fn get_utc_milliseconds(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMinutes)
	#[must_use]
	#[js_sys(js_name = "getUTCMinutes")]
	pub fn get_utc_minutes(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCMonth)
	#[must_use]
	#[js_sys(js_name = "getUTCMonth")]
	pub fn get_utc_month(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getUTCSeconds)
	#[must_use]
	#[js_sys(js_name = "getUTCSeconds")]
	pub fn get_utc_seconds(self: &Date) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setDate)
	#[must_use]
	#[js_sys(js_name = "setDate")]
	pub fn set_date(self: &Date, date: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setFullYear)
	#[must_use]
	#[js_sys(js_name = "setFullYear")]
	pub fn set_full_year(self: &Date, year: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setFullYear)
	#[must_use]
	#[js_sys(js_name = "setFullYear")]
	pub fn set_full_year_with_month(self: &Date, year: f64, month: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setFullYear)
	#[must_use]
	#[js_sys(js_name = "setFullYear")]
	pub fn set_full_year_with_month_date(self: &Date, year: f64, month: f64, date: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setHours)
	#[must_use]
	#[js_sys(js_name = "setHours")]
	pub fn set_hours(self: &Date, hours: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setHours)
	#[must_use]
	#[js_sys(js_name = "setHours")]
	pub fn set_hours_with_minutes(self: &Date, hours: f64, minutes: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setHours)
	#[must_use]
	#[js_sys(js_name = "setHours")]
	pub fn set_hours_with_minutes_seconds(
		self: &Date,
		hours: f64,
		minutes: f64,
		seconds: f64,
	) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setHours)
	#[must_use]
	#[js_sys(js_name = "setHours")]
	pub fn set_hours_with_minutes_seconds_milliseconds(
		self: &Date,
		hours: f64,
		minutes: f64,
		seconds: f64,
		milliseconds: f64,
	) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMilliseconds)
	#[must_use]
	#[js_sys(js_name = "setMilliseconds")]
	pub fn set_milliseconds(self: &Date, milliseconds: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMinutes)
	#[must_use]
	#[js_sys(js_name = "setMinutes")]
	pub fn set_minutes(self: &Date, minutes: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMinutes)
	#[must_use]
	#[js_sys(js_name = "setMinutes")]
	pub fn set_minutes_with_seconds(self: &Date, minutes: f64, seconds: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMinutes)
	#[must_use]
	#[js_sys(js_name = "setMinutes")]
	pub fn set_minutes_with_seconds_milliseconds(
		self: &Date,
		minutes: f64,
		seconds: f64,
		milliseconds: f64,
	) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMonth)
	#[must_use]
	#[js_sys(js_name = "setMonth")]
	pub fn set_month(self: &Date, month: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setMonth)
	#[must_use]
	#[js_sys(js_name = "setMonth")]
	pub fn set_month_with_date(self: &Date, month: f64, date: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setSeconds)
	#[must_use]
	#[js_sys(js_name = "setSeconds")]
	pub fn set_seconds(self: &Date, seconds: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setSeconds)
	#[must_use]
	#[js_sys(js_name = "setSeconds")]
	pub fn set_seconds_with_milliseconds(self: &Date, seconds: f64, milliseconds: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setTime)
	#[must_use]
	#[js_sys(js_name = "setTime")]
	pub fn set_time(self: &Date, time: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCDate)
	#[must_use]
	#[js_sys(js_name = "setUTCDate")]
	pub fn set_utc_date(self: &Date, date: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCFullYear)
	#[must_use]
	#[js_sys(js_name = "setUTCFullYear")]
	pub fn set_utc_full_year(self: &Date, year: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCFullYear)
	#[must_use]
	#[js_sys(js_name = "setUTCFullYear")]
	pub fn set_utc_full_year_with_month(self: &Date, year: f64, month: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCFullYear)
	#[must_use]
	#[js_sys(js_name = "setUTCFullYear")]
	pub fn set_utc_full_year_with_month_date(self: &Date, year: f64, month: f64, date: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCHours)
	#[must_use]
	#[js_sys(js_name = "setUTCHours")]
	pub fn set_utc_hours(self: &Date, hours: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCHours)
	#[must_use]
	#[js_sys(js_name = "setUTCHours")]
	pub fn set_utc_hours_with_minutes(self: &Date, hours: f64, minutes: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCHours)
	#[must_use]
	#[js_sys(js_name = "setUTCHours")]
	pub fn set_utc_hours_with_minutes_seconds(
		self: &Date,
		hours: f64,
		minutes: f64,
		seconds: f64,
	) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCHours)
	#[must_use]
	#[js_sys(js_name = "setUTCHours")]
	pub fn set_utc_hours_with_minutes_seconds_milliseconds(
		self: &Date,
		hours: f64,
		minutes: f64,
		seconds: f64,
		milliseconds: f64,
	) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMilliseconds)
	#[must_use]
	#[js_sys(js_name = "setUTCMilliseconds")]
	pub fn set_utc_milliseconds(self: &Date, milliseconds: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMinutes)
	#[must_use]
	#[js_sys(js_name = "setUTCMinutes")]
	pub fn set_utc_minutes(self: &Date, minutes: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMinutes)
	#[must_use]
	#[js_sys(js_name = "setUTCMinutes")]
	pub fn set_utc_minutes_with_seconds(self: &Date, minutes: f64, seconds: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMinutes)
	#[must_use]
	#[js_sys(js_name = "setUTCMinutes")]
	pub fn set_utc_minutes_with_seconds_milliseconds(
		self: &Date,
		minutes: f64,
		seconds: f64,
		milliseconds: f64,
	) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMonth)
	#[must_use]
	#[js_sys(js_name = "setUTCMonth")]
	pub fn set_utc_month(self: &Date, month: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCMonth)
	#[must_use]
	#[js_sys(js_name = "setUTCMonth")]
	pub fn set_utc_month_with_date(self: &Date, month: f64, date: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCSeconds)
	#[must_use]
	#[js_sys(js_name = "setUTCSeconds")]
	pub fn set_utc_seconds(self: &Date, seconds: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setUTCSeconds)
	#[must_use]
	#[js_sys(js_name = "setUTCSeconds")]
	pub fn set_utc_seconds_with_milliseconds(self: &Date, seconds: f64, milliseconds: f64) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toDateString)
	#[must_use]
	#[js_sys(js_name = "toDateString")]
	pub fn to_date_string(self: &Date) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toISOString)
	#[js_sys(js_name = "toISOString")]
	pub fn to_iso_string(self: &Date) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toJSON)
	#[must_use]
	#[js_sys(js_name = "toJSON")]
	pub fn to_json(self: &Date) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleDateString)
	#[js_sys(js_name = "toLocaleDateString")]
	pub fn to_locale_date_string(self: &Date) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleDateString)
	#[js_sys(js_name = "toLocaleDateString")]
	pub fn to_locale_date_string_with_locales(
		self: &Date,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleDateString)
	#[js_sys(js_name = "toLocaleDateString")]
	pub fn to_locale_date_string_with_locales_and_options(
		self: &Date,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string(self: &Date) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales(
		self: &Date,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales_and_options(
		self: &Date,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleTimeString)
	#[js_sys(js_name = "toLocaleTimeString")]
	pub fn to_locale_time_string(self: &Date) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleTimeString)
	#[js_sys(js_name = "toLocaleTimeString")]
	pub fn to_locale_time_string_with_locales(
		self: &Date,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toLocaleTimeString)
	#[js_sys(js_name = "toLocaleTimeString")]
	pub fn to_locale_time_string_with_locales_and_options(
		self: &Date,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &Date) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toTemporalInstant)
	#[js_sys(js_name = "toTemporalInstant")]
	pub fn to_temporal_instant(self: &Date) -> Result<Instant, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toTimeString)
	#[must_use]
	#[js_sys(js_name = "toTimeString")]
	pub fn to_time_string(self: &Date) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/toUTCString)
	#[must_use]
	#[js_sys(js_name = "toUTCString")]
	pub fn to_utc_string(self: &Date) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/valueOf)
	#[must_use]
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &Date) -> f64;
}

impl Default for Date {
	fn default() -> Self {
		Self::new()
	}
}
