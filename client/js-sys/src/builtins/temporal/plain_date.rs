use super::duration::Duration;
use super::plain_date_time::PlainDateTime;
use super::plain_month_day::PlainMonthDay;
use super::plain_year_month::PlainYearMonth;
use super::zoned_date_time::ZonedDateTime;
use crate::{JsString, JsValue, Object, js_sys};

#[js_sys(js_sys = crate, namespace = "Temporal")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate)
	#[js_sys(js_name = "PlainDate", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type PlainDate;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/PlainDate)
	#[js_sys(constructor)]
	pub fn new(iso_year: i32, iso_month: u32, iso_day: u32) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/PlainDate)
	#[js_sys(constructor)]
	pub fn new_with_calendar(
		iso_year: i32,
		iso_month: u32,
		iso_day: u32,
		calendar: &str,
	) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/from)
	#[js_sys(static_of = PlainDate)]
	pub fn from(item: &JsValue) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/from)
	#[js_sys(static_of = PlainDate, js_name = "from")]
	pub fn from_with_options(item: &JsValue, options: &JsValue) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/compare)
	#[js_sys(static_of = PlainDate)]
	pub fn compare(one: &JsValue, two: &JsValue) -> Result<i32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/calendarId)
	#[must_use]
	#[js_sys(getter = "calendarId")]
	pub fn calendar_id(self: &PlainDate) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/era)
	#[must_use]
	#[js_sys(getter)]
	pub fn era(self: &PlainDate) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/eraYear)
	#[must_use]
	#[js_sys(getter = "eraYear")]
	pub fn era_year(self: &PlainDate) -> Option<i32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/year)
	#[must_use]
	#[js_sys(getter)]
	pub fn year(self: &PlainDate) -> i32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/month)
	#[must_use]
	#[js_sys(getter)]
	pub fn month(self: &PlainDate) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/monthCode)
	#[must_use]
	#[js_sys(getter = "monthCode")]
	pub fn month_code(self: &PlainDate) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/day)
	#[must_use]
	#[js_sys(getter)]
	pub fn day(self: &PlainDate) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/dayOfWeek)
	#[must_use]
	#[js_sys(getter = "dayOfWeek")]
	pub fn day_of_week(self: &PlainDate) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/dayOfYear)
	#[must_use]
	#[js_sys(getter = "dayOfYear")]
	pub fn day_of_year(self: &PlainDate) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/weekOfYear)
	#[must_use]
	#[js_sys(getter = "weekOfYear")]
	pub fn week_of_year(self: &PlainDate) -> Option<u32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/yearOfWeek)
	#[must_use]
	#[js_sys(getter = "yearOfWeek")]
	pub fn year_of_week(self: &PlainDate) -> Option<i32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/daysInWeek)
	#[must_use]
	#[js_sys(getter = "daysInWeek")]
	pub fn days_in_week(self: &PlainDate) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/daysInMonth)
	#[must_use]
	#[js_sys(getter = "daysInMonth")]
	pub fn days_in_month(self: &PlainDate) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/daysInYear)
	#[must_use]
	#[js_sys(getter = "daysInYear")]
	pub fn days_in_year(self: &PlainDate) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/monthsInYear)
	#[must_use]
	#[js_sys(getter = "monthsInYear")]
	pub fn months_in_year(self: &PlainDate) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/inLeapYear)
	#[must_use]
	#[js_sys(getter = "inLeapYear")]
	pub fn in_leap_year(self: &PlainDate) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/add)
	pub fn add(self: &PlainDate, duration: &JsValue) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/add)
	#[js_sys(js_name = "add")]
	pub fn add_with_options(
		self: &PlainDate,
		duration: &JsValue,
		options: &JsValue,
	) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/subtract)
	pub fn subtract(self: &PlainDate, duration: &JsValue) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/subtract)
	#[js_sys(js_name = "subtract")]
	pub fn subtract_with_options(
		self: &PlainDate,
		duration: &JsValue,
		options: &JsValue,
	) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/with)
	pub fn with(self: &PlainDate, date_like: &JsValue) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/with)
	#[js_sys(js_name = "with")]
	pub fn with_options(
		self: &PlainDate,
		date_like: &JsValue,
		options: &JsValue,
	) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/withCalendar)
	#[js_sys(js_name = "withCalendar")]
	pub fn with_calendar(self: &PlainDate, calendar: &JsValue) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/until)
	pub fn until(self: &PlainDate, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/until)
	#[js_sys(js_name = "until")]
	pub fn until_with_options(
		self: &PlainDate,
		other: &JsValue,
		options: &JsValue,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/since)
	pub fn since(self: &PlainDate, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/since)
	#[js_sys(js_name = "since")]
	pub fn since_with_options(
		self: &PlainDate,
		other: &JsValue,
		options: &JsValue,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/equals)
	pub fn equals(self: &PlainDate, other: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toPlainDateTime)
	#[must_use]
	#[js_sys(js_name = "toPlainDateTime")]
	pub fn to_plain_date_time(self: &PlainDate) -> PlainDateTime;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toPlainDateTime)
	#[js_sys(js_name = "toPlainDateTime")]
	pub fn to_plain_date_time_with_time(
		self: &PlainDate,
		time: &JsValue,
	) -> Result<PlainDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toZonedDateTime)
	#[js_sys(js_name = "toZonedDateTime")]
	pub fn to_zoned_date_time(self: &PlainDate, item: &JsValue) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toPlainYearMonth)
	#[must_use]
	#[js_sys(js_name = "toPlainYearMonth")]
	pub fn to_plain_year_month(self: &PlainDate) -> PlainYearMonth;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toPlainMonthDay)
	#[must_use]
	#[js_sys(js_name = "toPlainMonthDay")]
	pub fn to_plain_month_day(self: &PlainDate) -> PlainMonthDay;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &PlainDate) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toString)
	#[js_sys(js_name = "toString")]
	pub fn to_string_with_options(self: &PlainDate, options: &JsValue)
	-> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string(self: &PlainDate) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales(
		self: &PlainDate,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales_and_options(
		self: &PlainDate,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/toJSON)
	#[must_use]
	#[js_sys(js_name = "toJSON")]
	pub fn to_json(self: &PlainDate) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainDate/valueOf)
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &PlainDate) -> Result<JsValue, JsValue>;
}
