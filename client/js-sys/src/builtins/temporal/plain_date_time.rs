use super::duration::Duration;
use super::plain_date::PlainDate;
use super::plain_time::PlainTime;
use super::zoned_date_time::ZonedDateTime;
use crate::{JsString, JsValue, Object, js_sys};

#[js_sys(js_sys = crate, namespace = "Temporal")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime)
	#[js_sys(js_name = "PlainDateTime", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type PlainDateTime;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/PlainDateTime)
	#[js_sys(constructor)]
	pub fn new(iso_year: i32, iso_month: u32, iso_day: u32) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/PlainDateTime)
	#[js_sys(constructor)]
	pub fn new_with_year_month_day_hour(
		iso_year: i32,
		iso_month: u32,
		iso_day: u32,
		hour: u32,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/PlainDateTime)
	#[js_sys(constructor)]
	pub fn new_with_year_month_day_hour_minute(
		iso_year: i32,
		iso_month: u32,
		iso_day: u32,
		hour: u32,
		minute: u32,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/PlainDateTime)
	#[js_sys(constructor)]
	pub fn new_with_year_month_day_hour_minute_second(
		iso_year: i32,
		iso_month: u32,
		iso_day: u32,
		hour: u32,
		minute: u32,
		second: u32,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/PlainDateTime)
	#[js_sys(constructor)]
	pub fn new_with_year_month_day_hour_minute_second_millisecond(
		iso_year: i32,
		iso_month: u32,
		iso_day: u32,
		hour: u32,
		minute: u32,
		second: u32,
		millisecond: u32,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/PlainDateTime)
	#[expect(
		clippy::too_many_arguments,
		reason = "matches the JavaScript constructor"
	)]
	#[js_sys(constructor)]
	pub fn new_with_year_month_day_hour_minute_second_millisecond_microsecond(
		iso_year: i32,
		iso_month: u32,
		iso_day: u32,
		hour: u32,
		minute: u32,
		second: u32,
		millisecond: u32,
		microsecond: u32,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/PlainDateTime)
	#[expect(
		clippy::too_many_arguments,
		reason = "matches the JavaScript constructor"
	)]
	#[js_sys(constructor)]
	pub fn new_with_year_month_day_hour_minute_second_millisecond_microsecond_nanosecond(
		iso_year: i32,
		iso_month: u32,
		iso_day: u32,
		hour: u32,
		minute: u32,
		second: u32,
		millisecond: u32,
		microsecond: u32,
		nanosecond: u32,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/PlainDateTime)
	#[expect(
		clippy::too_many_arguments,
		reason = "matches the JavaScript constructor"
	)]
	#[js_sys(constructor)]
	pub fn new_with_values(
		iso_year: i32,
		iso_month: u32,
		iso_day: u32,
		hour: u32,
		minute: u32,
		second: u32,
		millisecond: u32,
		microsecond: u32,
		nanosecond: u32,
		calendar: &str,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/from)
	#[js_sys(static_of = PlainDateTime)]
	pub fn from(item: &JsValue) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/from)
	#[js_sys(static_of = PlainDateTime, js_name = "from")]
	pub fn from_with_options(item: &JsValue, options: &JsValue) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/compare)
	#[js_sys(static_of = PlainDateTime)]
	pub fn compare(one: &JsValue, two: &JsValue) -> Result<i32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/calendarId)
	#[must_use]
	#[js_sys(getter = "calendarId")]
	pub fn calendar_id(self: &PlainDateTime) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/era)
	#[must_use]
	#[js_sys(getter)]
	pub fn era(self: &PlainDateTime) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/eraYear)
	#[must_use]
	#[js_sys(getter = "eraYear")]
	pub fn era_year(self: &PlainDateTime) -> Option<i32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/year)
	#[must_use]
	#[js_sys(getter)]
	pub fn year(self: &PlainDateTime) -> i32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/month)
	#[must_use]
	#[js_sys(getter)]
	pub fn month(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/monthCode)
	#[must_use]
	#[js_sys(getter = "monthCode")]
	pub fn month_code(self: &PlainDateTime) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/day)
	#[must_use]
	#[js_sys(getter)]
	pub fn day(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/hour)
	#[must_use]
	#[js_sys(getter)]
	pub fn hour(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/minute)
	#[must_use]
	#[js_sys(getter)]
	pub fn minute(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/second)
	#[must_use]
	#[js_sys(getter)]
	pub fn second(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/millisecond)
	#[must_use]
	#[js_sys(getter)]
	pub fn millisecond(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/microsecond)
	#[must_use]
	#[js_sys(getter)]
	pub fn microsecond(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/nanosecond)
	#[must_use]
	#[js_sys(getter)]
	pub fn nanosecond(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/dayOfWeek)
	#[must_use]
	#[js_sys(getter = "dayOfWeek")]
	pub fn day_of_week(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/dayOfYear)
	#[must_use]
	#[js_sys(getter = "dayOfYear")]
	pub fn day_of_year(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/weekOfYear)
	#[must_use]
	#[js_sys(getter = "weekOfYear")]
	pub fn week_of_year(self: &PlainDateTime) -> Option<u32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/yearOfWeek)
	#[must_use]
	#[js_sys(getter = "yearOfWeek")]
	pub fn year_of_week(self: &PlainDateTime) -> Option<i32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/daysInWeek)
	#[must_use]
	#[js_sys(getter = "daysInWeek")]
	pub fn days_in_week(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/daysInMonth)
	#[must_use]
	#[js_sys(getter = "daysInMonth")]
	pub fn days_in_month(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/daysInYear)
	#[must_use]
	#[js_sys(getter = "daysInYear")]
	pub fn days_in_year(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/monthsInYear)
	#[must_use]
	#[js_sys(getter = "monthsInYear")]
	pub fn months_in_year(self: &PlainDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/inLeapYear)
	#[must_use]
	#[js_sys(getter = "inLeapYear")]
	pub fn in_leap_year(self: &PlainDateTime) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/with)
	pub fn with(self: &PlainDateTime, date_time_like: &JsValue) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/with)
	#[js_sys(js_name = "with")]
	pub fn with_options(
		self: &PlainDateTime,
		date_time_like: &JsValue,
		options: &JsValue,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/withPlainTime)
	#[must_use]
	#[js_sys(js_name = "withPlainTime")]
	pub fn with_plain_time(self: &PlainDateTime) -> PlainDateTime;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/withPlainTime)
	#[js_sys(js_name = "withPlainTime")]
	pub fn with_plain_time_value(
		self: &PlainDateTime,
		plain_time: &JsValue,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/withCalendar)
	#[js_sys(js_name = "withCalendar")]
	pub fn with_calendar(
		self: &PlainDateTime,
		calendar: &JsValue,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/add)
	pub fn add(self: &PlainDateTime, duration: &JsValue) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/add)
	#[js_sys(js_name = "add")]
	pub fn add_with_options(
		self: &PlainDateTime,
		duration: &JsValue,
		options: &JsValue,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/subtract)
	pub fn subtract(self: &PlainDateTime, duration: &JsValue) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/subtract)
	#[js_sys(js_name = "subtract")]
	pub fn subtract_with_options(
		self: &PlainDateTime,
		duration: &JsValue,
		options: &JsValue,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/until)
	pub fn until(self: &PlainDateTime, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/until)
	#[js_sys(js_name = "until")]
	pub fn until_with_options(
		self: &PlainDateTime,
		other: &JsValue,
		options: &JsValue,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/since)
	pub fn since(self: &PlainDateTime, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/since)
	#[js_sys(js_name = "since")]
	pub fn since_with_options(
		self: &PlainDateTime,
		other: &JsValue,
		options: &JsValue,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/round)
	pub fn round(self: &PlainDateTime, round_to: &JsValue) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/equals)
	pub fn equals(self: &PlainDateTime, other: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toZonedDateTime)
	#[js_sys(js_name = "toZonedDateTime")]
	pub fn to_zoned_date_time(
		self: &PlainDateTime,
		time_zone: &JsValue,
	) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toZonedDateTime)
	#[js_sys(js_name = "toZonedDateTime")]
	pub fn to_zoned_date_time_with_options(
		self: &PlainDateTime,
		time_zone: &JsValue,
		options: &JsValue,
	) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toPlainDate)
	#[must_use]
	#[js_sys(js_name = "toPlainDate")]
	pub fn to_plain_date(self: &PlainDateTime) -> PlainDate;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toPlainTime)
	#[must_use]
	#[js_sys(js_name = "toPlainTime")]
	pub fn to_plain_time(self: &PlainDateTime) -> PlainTime;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &PlainDateTime) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toString)
	#[js_sys(js_name = "toString")]
	pub fn to_string_with_options(
		self: &PlainDateTime,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string(self: &PlainDateTime) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales(
		self: &PlainDateTime,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales_and_options(
		self: &PlainDateTime,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/toJSON)
	#[must_use]
	#[js_sys(js_name = "toJSON")]
	pub fn to_json(self: &PlainDateTime) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDateTime/valueOf)
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &PlainDateTime) -> Result<JsValue, JsValue>;
}
