use super::duration::Duration;
use super::instant::Instant;
use super::plain_date::PlainDate;
use super::plain_date_time::PlainDateTime;
use super::plain_time::PlainTime;
use crate::{BigInt, JsString, JsValue, Object, js_sys};

#[js_sys(js_sys = crate, namespace = "Temporal")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime)
	#[js_sys(js_name = "ZonedDateTime", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type ZonedDateTime;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/ZonedDateTime)
	#[js_sys(constructor)]
	pub fn new(epoch_nanoseconds: &BigInt, time_zone: &str) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/ZonedDateTime)
	#[js_sys(constructor)]
	pub fn new_with_calendar(
		epoch_nanoseconds: &BigInt,
		time_zone: &str,
		calendar: &str,
	) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/from)
	#[js_sys(static_of = ZonedDateTime)]
	pub fn from(item: &JsValue) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/from)
	#[js_sys(static_of = ZonedDateTime, js_name = "from")]
	pub fn from_with_options(item: &JsValue, options: &JsValue) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/compare)
	#[js_sys(static_of = ZonedDateTime)]
	pub fn compare(one: &JsValue, two: &JsValue) -> Result<i32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/calendarId)
	#[must_use]
	#[js_sys(getter = "calendarId")]
	pub fn calendar_id(self: &ZonedDateTime) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/timeZoneId)
	#[must_use]
	#[js_sys(getter = "timeZoneId")]
	pub fn time_zone_id(self: &ZonedDateTime) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/era)
	#[must_use]
	#[js_sys(getter)]
	pub fn era(self: &ZonedDateTime) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/eraYear)
	#[must_use]
	#[js_sys(getter = "eraYear")]
	pub fn era_year(self: &ZonedDateTime) -> Option<i32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/year)
	#[must_use]
	#[js_sys(getter)]
	pub fn year(self: &ZonedDateTime) -> i32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/month)
	#[must_use]
	#[js_sys(getter)]
	pub fn month(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/monthCode)
	#[must_use]
	#[js_sys(getter = "monthCode")]
	pub fn month_code(self: &ZonedDateTime) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/day)
	#[must_use]
	#[js_sys(getter)]
	pub fn day(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/hour)
	#[must_use]
	#[js_sys(getter)]
	pub fn hour(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/minute)
	#[must_use]
	#[js_sys(getter)]
	pub fn minute(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/second)
	#[must_use]
	#[js_sys(getter)]
	pub fn second(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/millisecond)
	#[must_use]
	#[js_sys(getter)]
	pub fn millisecond(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/microsecond)
	#[must_use]
	#[js_sys(getter)]
	pub fn microsecond(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/nanosecond)
	#[must_use]
	#[js_sys(getter)]
	pub fn nanosecond(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/epochMilliseconds)
	#[must_use]
	#[js_sys(getter = "epochMilliseconds")]
	pub fn epoch_milliseconds(self: &ZonedDateTime) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/epochNanoseconds)
	#[must_use]
	#[js_sys(getter = "epochNanoseconds")]
	pub fn epoch_nanoseconds(self: &ZonedDateTime) -> BigInt;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/dayOfWeek)
	#[must_use]
	#[js_sys(getter = "dayOfWeek")]
	pub fn day_of_week(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/dayOfYear)
	#[must_use]
	#[js_sys(getter = "dayOfYear")]
	pub fn day_of_year(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/weekOfYear)
	#[must_use]
	#[js_sys(getter = "weekOfYear")]
	pub fn week_of_year(self: &ZonedDateTime) -> Option<u32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/yearOfWeek)
	#[must_use]
	#[js_sys(getter = "yearOfWeek")]
	pub fn year_of_week(self: &ZonedDateTime) -> Option<i32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/hoursInDay)
	#[js_sys(getter = "hoursInDay")]
	pub fn hours_in_day(self: &ZonedDateTime) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/daysInWeek)
	#[must_use]
	#[js_sys(getter = "daysInWeek")]
	pub fn days_in_week(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/daysInMonth)
	#[must_use]
	#[js_sys(getter = "daysInMonth")]
	pub fn days_in_month(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/daysInYear)
	#[must_use]
	#[js_sys(getter = "daysInYear")]
	pub fn days_in_year(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/monthsInYear)
	#[must_use]
	#[js_sys(getter = "monthsInYear")]
	pub fn months_in_year(self: &ZonedDateTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/inLeapYear)
	#[must_use]
	#[js_sys(getter = "inLeapYear")]
	pub fn in_leap_year(self: &ZonedDateTime) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/offsetNanoseconds)
	#[must_use]
	#[js_sys(getter = "offsetNanoseconds")]
	pub fn offset_nanoseconds(self: &ZonedDateTime) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/offset)
	#[must_use]
	#[js_sys(getter)]
	pub fn offset(self: &ZonedDateTime) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/with)
	pub fn with(
		self: &ZonedDateTime,
		zoned_date_time_like: &JsValue,
	) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/with)
	#[js_sys(js_name = "with")]
	pub fn with_options(
		self: &ZonedDateTime,
		zoned_date_time_like: &JsValue,
		options: &JsValue,
	) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/withPlainTime)
	#[js_sys(js_name = "withPlainTime")]
	pub fn with_plain_time(self: &ZonedDateTime) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/withPlainTime)
	#[js_sys(js_name = "withPlainTime")]
	pub fn with_plain_time_value(
		self: &ZonedDateTime,
		plain_time: &JsValue,
	) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/withTimeZone)
	#[js_sys(js_name = "withTimeZone")]
	pub fn with_time_zone(
		self: &ZonedDateTime,
		time_zone: &JsValue,
	) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/withCalendar)
	#[js_sys(js_name = "withCalendar")]
	pub fn with_calendar(
		self: &ZonedDateTime,
		calendar: &JsValue,
	) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/add)
	pub fn add(self: &ZonedDateTime, duration: &JsValue) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/add)
	#[js_sys(js_name = "add")]
	pub fn add_with_options(
		self: &ZonedDateTime,
		duration: &JsValue,
		options: &JsValue,
	) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/subtract)
	pub fn subtract(self: &ZonedDateTime, duration: &JsValue) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/subtract)
	#[js_sys(js_name = "subtract")]
	pub fn subtract_with_options(
		self: &ZonedDateTime,
		duration: &JsValue,
		options: &JsValue,
	) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/until)
	pub fn until(self: &ZonedDateTime, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/until)
	#[js_sys(js_name = "until")]
	pub fn until_with_options(
		self: &ZonedDateTime,
		other: &JsValue,
		options: &JsValue,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/since)
	pub fn since(self: &ZonedDateTime, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/since)
	#[js_sys(js_name = "since")]
	pub fn since_with_options(
		self: &ZonedDateTime,
		other: &JsValue,
		options: &JsValue,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/round)
	pub fn round(self: &ZonedDateTime, round_to: &JsValue) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/equals)
	pub fn equals(self: &ZonedDateTime, other: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/startOfDay)
	#[js_sys(js_name = "startOfDay")]
	pub fn start_of_day(self: &ZonedDateTime) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/getTimeZoneTransition)
	#[js_sys(js_name = "getTimeZoneTransition")]
	pub fn get_time_zone_transition(
		self: &ZonedDateTime,
		direction: &JsValue,
	) -> Result<Option<ZonedDateTime>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toInstant)
	#[must_use]
	#[js_sys(js_name = "toInstant")]
	pub fn to_instant(self: &ZonedDateTime) -> Instant;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toPlainDate)
	#[must_use]
	#[js_sys(js_name = "toPlainDate")]
	pub fn to_plain_date(self: &ZonedDateTime) -> PlainDate;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toPlainTime)
	#[must_use]
	#[js_sys(js_name = "toPlainTime")]
	pub fn to_plain_time(self: &ZonedDateTime) -> PlainTime;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toPlainDateTime)
	#[must_use]
	#[js_sys(js_name = "toPlainDateTime")]
	pub fn to_plain_date_time(self: &ZonedDateTime) -> PlainDateTime;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &ZonedDateTime) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toString)
	#[js_sys(js_name = "toString")]
	pub fn to_string_with_options(
		self: &ZonedDateTime,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string(self: &ZonedDateTime) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales(
		self: &ZonedDateTime,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales_and_options(
		self: &ZonedDateTime,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/toJSON)
	#[must_use]
	#[js_sys(js_name = "toJSON")]
	pub fn to_json(self: &ZonedDateTime) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/ZonedDateTime/valueOf)
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &ZonedDateTime) -> Result<JsValue, JsValue>;
}
