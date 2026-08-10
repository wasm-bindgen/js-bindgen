use super::plain_date::PlainDate;
use crate::{JsString, JsValue, Object, js_sys};

#[js_sys(js_sys = crate, namespace = "Temporal")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay)
	#[js_sys(js_name = "PlainMonthDay", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type PlainMonthDay;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/PlainMonthDay)
	#[js_sys(constructor)]
	pub fn new(iso_month: u32, iso_day: u32) -> Result<PlainMonthDay, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/PlainMonthDay)
	#[js_sys(constructor)]
	pub fn new_with_calendar(
		iso_month: u32,
		iso_day: u32,
		calendar: &str,
	) -> Result<PlainMonthDay, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/PlainMonthDay)
	#[js_sys(constructor)]
	pub fn new_with_calendar_and_reference_year(
		iso_month: u32,
		iso_day: u32,
		calendar: &str,
		reference_iso_year: i32,
	) -> Result<PlainMonthDay, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/from)
	#[js_sys(static_of = PlainMonthDay)]
	pub fn from(item: &JsValue) -> Result<PlainMonthDay, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/from)
	#[js_sys(static_of = PlainMonthDay, js_name = "from")]
	pub fn from_with_options(item: &JsValue, options: &JsValue) -> Result<PlainMonthDay, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/calendarId)
	#[must_use]
	#[js_sys(getter = "calendarId")]
	pub fn calendar_id(self: &PlainMonthDay) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/monthCode)
	#[must_use]
	#[js_sys(getter = "monthCode")]
	pub fn month_code(self: &PlainMonthDay) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/day)
	#[must_use]
	#[js_sys(getter)]
	pub fn day(self: &PlainMonthDay) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/with)
	pub fn with(self: &PlainMonthDay, month_day_like: &JsValue) -> Result<PlainMonthDay, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/with)
	#[js_sys(js_name = "with")]
	pub fn with_options(
		self: &PlainMonthDay,
		month_day_like: &JsValue,
		options: &JsValue,
	) -> Result<PlainMonthDay, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/equals)
	pub fn equals(self: &PlainMonthDay, other: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toPlainDate)
	#[js_sys(js_name = "toPlainDate")]
	pub fn to_plain_date(self: &PlainMonthDay, item: &JsValue) -> Result<PlainDate, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &PlainMonthDay) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toString)
	#[js_sys(js_name = "toString")]
	pub fn to_string_with_options(
		self: &PlainMonthDay,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string(self: &PlainMonthDay) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales(
		self: &PlainMonthDay,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales_and_options(
		self: &PlainMonthDay,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/toJSON)
	#[must_use]
	#[js_sys(js_name = "toJSON")]
	pub fn to_json(self: &PlainMonthDay) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainMonthDay/valueOf)
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &PlainMonthDay) -> Result<JsValue, JsValue>;
}
