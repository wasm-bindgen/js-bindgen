use super::duration::Duration;
use super::plain_date::PlainDate;
use crate::{JsString, JsValue, Object, js_sys};

#[js_sys(js_sys = crate, namespace = "Temporal")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth)
	#[js_sys(js_name = "PlainYearMonth", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type PlainYearMonth;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/PlainYearMonth)
	#[js_sys(constructor)]
	pub fn new(iso_year: i32, iso_month: u32) -> Result<PlainYearMonth, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/PlainYearMonth)
	#[js_sys(constructor)]
	pub fn new_with_calendar(
		iso_year: i32,
		iso_month: u32,
		calendar: &str,
	) -> Result<PlainYearMonth, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/PlainYearMonth)
	#[js_sys(constructor)]
	pub fn new_with_calendar_and_reference_day(
		iso_year: i32,
		iso_month: u32,
		calendar: &str,
		reference_iso_day: u32,
	) -> Result<PlainYearMonth, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/from)
	#[js_sys(static_of = PlainYearMonth)]
	pub fn from(item: &JsValue) -> Result<PlainYearMonth, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/from)
	#[js_sys(static_of = PlainYearMonth, js_name = "from")]
	pub fn from_with_options(item: &JsValue, options: &JsValue) -> Result<PlainYearMonth, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/compare)
	#[js_sys(static_of = PlainYearMonth)]
	pub fn compare(one: &JsValue, two: &JsValue) -> Result<i32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/calendarId)
	#[must_use]
	#[js_sys(getter = "calendarId")]
	pub fn calendar_id(self: &PlainYearMonth) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/era)
	#[must_use]
	#[js_sys(getter)]
	pub fn era(self: &PlainYearMonth) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/eraYear)
	#[must_use]
	#[js_sys(getter = "eraYear")]
	pub fn era_year(self: &PlainYearMonth) -> Option<i32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/year)
	#[must_use]
	#[js_sys(getter)]
	pub fn year(self: &PlainYearMonth) -> i32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/month)
	#[must_use]
	#[js_sys(getter)]
	pub fn month(self: &PlainYearMonth) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/monthCode)
	#[must_use]
	#[js_sys(getter = "monthCode")]
	pub fn month_code(self: &PlainYearMonth) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/daysInYear)
	#[must_use]
	#[js_sys(getter = "daysInYear")]
	pub fn days_in_year(self: &PlainYearMonth) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/daysInMonth)
	#[must_use]
	#[js_sys(getter = "daysInMonth")]
	pub fn days_in_month(self: &PlainYearMonth) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/monthsInYear)
	#[must_use]
	#[js_sys(getter = "monthsInYear")]
	pub fn months_in_year(self: &PlainYearMonth) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/inLeapYear)
	#[must_use]
	#[js_sys(getter = "inLeapYear")]
	pub fn in_leap_year(self: &PlainYearMonth) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/with)
	pub fn with(
		self: &PlainYearMonth,
		year_month_like: &JsValue,
	) -> Result<PlainYearMonth, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/with)
	#[js_sys(js_name = "with")]
	pub fn with_options(
		self: &PlainYearMonth,
		year_month_like: &JsValue,
		options: &JsValue,
	) -> Result<PlainYearMonth, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/add)
	pub fn add(self: &PlainYearMonth, duration: &JsValue) -> Result<PlainYearMonth, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/add)
	#[js_sys(js_name = "add")]
	pub fn add_with_options(
		self: &PlainYearMonth,
		duration: &JsValue,
		options: &JsValue,
	) -> Result<PlainYearMonth, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/subtract)
	pub fn subtract(self: &PlainYearMonth, duration: &JsValue) -> Result<PlainYearMonth, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/subtract)
	#[js_sys(js_name = "subtract")]
	pub fn subtract_with_options(
		self: &PlainYearMonth,
		duration: &JsValue,
		options: &JsValue,
	) -> Result<PlainYearMonth, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/until)
	pub fn until(self: &PlainYearMonth, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/until)
	#[js_sys(js_name = "until")]
	pub fn until_with_options(
		self: &PlainYearMonth,
		other: &JsValue,
		options: &JsValue,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/since)
	pub fn since(self: &PlainYearMonth, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/since)
	#[js_sys(js_name = "since")]
	pub fn since_with_options(
		self: &PlainYearMonth,
		other: &JsValue,
		options: &JsValue,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/equals)
	pub fn equals(self: &PlainYearMonth, other: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toPlainDate)
	#[js_sys(js_name = "toPlainDate")]
	pub fn to_plain_date(self: &PlainYearMonth, item: &JsValue) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &PlainYearMonth) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toString)
	#[js_sys(js_name = "toString")]
	pub fn to_string_with_options(
		self: &PlainYearMonth,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string(self: &PlainYearMonth) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales(
		self: &PlainYearMonth,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales_and_options(
		self: &PlainYearMonth,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/toJSON)
	#[must_use]
	#[js_sys(js_name = "toJSON")]
	pub fn to_json(self: &PlainYearMonth) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainYearMonth/valueOf)
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &PlainYearMonth) -> Result<JsValue, JsValue>;
}
