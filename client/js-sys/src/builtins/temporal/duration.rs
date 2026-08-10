use crate::{JsString, JsValue, Object, js_sys};

#[js_sys(js_sys = crate, namespace = "Temporal")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration)
	#[js_sys(js_name = "Duration", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Duration;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/Duration)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> Duration;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/Duration)
	#[js_sys(constructor)]
	pub fn new_with_years(years: f64) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/Duration)
	#[js_sys(constructor)]
	pub fn new_with_years_months(years: f64, months: f64) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/Duration)
	#[js_sys(constructor)]
	pub fn new_with_years_months_weeks(
		years: f64,
		months: f64,
		weeks: f64,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/Duration)
	#[js_sys(constructor)]
	pub fn new_with_years_months_weeks_days(
		years: f64,
		months: f64,
		weeks: f64,
		days: f64,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/Duration)
	#[js_sys(constructor)]
	pub fn new_with_years_months_weeks_days_hours(
		years: f64,
		months: f64,
		weeks: f64,
		days: f64,
		hours: f64,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/Duration)
	#[js_sys(constructor)]
	pub fn new_with_years_months_weeks_days_hours_minutes(
		years: f64,
		months: f64,
		weeks: f64,
		days: f64,
		hours: f64,
		minutes: f64,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/Duration)
	#[js_sys(constructor)]
	pub fn new_with_years_months_weeks_days_hours_minutes_seconds(
		years: f64,
		months: f64,
		weeks: f64,
		days: f64,
		hours: f64,
		minutes: f64,
		seconds: f64,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/Duration)
	#[expect(
		clippy::too_many_arguments,
		reason = "matches the JavaScript constructor"
	)]
	#[js_sys(constructor)]
	pub fn new_with_years_months_weeks_days_hours_minutes_seconds_milliseconds(
		years: f64,
		months: f64,
		weeks: f64,
		days: f64,
		hours: f64,
		minutes: f64,
		seconds: f64,
		milliseconds: f64,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/Duration)
	#[expect(
		clippy::too_many_arguments,
		reason = "matches the JavaScript constructor"
	)]
	#[js_sys(constructor)]
	pub fn new_with_years_months_weeks_days_hours_minutes_seconds_milliseconds_microseconds(
		years: f64,
		months: f64,
		weeks: f64,
		days: f64,
		hours: f64,
		minutes: f64,
		seconds: f64,
		milliseconds: f64,
		microseconds: f64,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/Duration)
	#[expect(
		clippy::too_many_arguments,
		reason = "matches the JavaScript constructor"
	)]
	#[js_sys(constructor)]
	pub fn new_with_values(
		years: f64,
		months: f64,
		weeks: f64,
		days: f64,
		hours: f64,
		minutes: f64,
		seconds: f64,
		milliseconds: f64,
		microseconds: f64,
		nanoseconds: f64,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/from)
	#[js_sys(static_of = Duration)]
	pub fn from(item: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/compare)
	#[js_sys(static_of = Duration)]
	pub fn compare(one: &JsValue, two: &JsValue) -> Result<i32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/compare)
	#[js_sys(static_of = Duration, js_name = "compare")]
	pub fn compare_with_options(
		one: &JsValue,
		two: &JsValue,
		options: &JsValue,
	) -> Result<i32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/years)
	#[must_use]
	#[js_sys(getter)]
	pub fn years(self: &Duration) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/months)
	#[must_use]
	#[js_sys(getter)]
	pub fn months(self: &Duration) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/weeks)
	#[must_use]
	#[js_sys(getter)]
	pub fn weeks(self: &Duration) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/days)
	#[must_use]
	#[js_sys(getter)]
	pub fn days(self: &Duration) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/hours)
	#[must_use]
	#[js_sys(getter)]
	pub fn hours(self: &Duration) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/minutes)
	#[must_use]
	#[js_sys(getter)]
	pub fn minutes(self: &Duration) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/seconds)
	#[must_use]
	#[js_sys(getter)]
	pub fn seconds(self: &Duration) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/milliseconds)
	#[must_use]
	#[js_sys(getter)]
	pub fn milliseconds(self: &Duration) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/microseconds)
	#[must_use]
	#[js_sys(getter)]
	pub fn microseconds(self: &Duration) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/nanoseconds)
	#[must_use]
	#[js_sys(getter)]
	pub fn nanoseconds(self: &Duration) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/sign)
	#[must_use]
	#[js_sys(getter)]
	pub fn sign(self: &Duration) -> i32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/blank)
	#[must_use]
	#[js_sys(getter)]
	pub fn blank(self: &Duration) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/with)
	pub fn with(self: &Duration, duration_like: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/negated)
	#[must_use]
	pub fn negated(self: &Duration) -> Duration;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/abs)
	#[must_use]
	pub fn abs(self: &Duration) -> Duration;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/add)
	pub fn add(self: &Duration, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/subtract)
	pub fn subtract(self: &Duration, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/round)
	pub fn round(self: &Duration, round_to: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/total)
	pub fn total(self: &Duration, total_of: &JsValue) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &Duration) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/toString)
	#[js_sys(js_name = "toString")]
	pub fn to_string_with_options(self: &Duration, options: &JsValue) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string(self: &Duration) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales(
		self: &Duration,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales_and_options(
		self: &Duration,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/toJSON)
	#[must_use]
	#[js_sys(js_name = "toJSON")]
	pub fn to_json(self: &Duration) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Duration/valueOf)
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &Duration) -> Result<JsValue, JsValue>;
}

impl Default for Duration {
	fn default() -> Self {
		Self::new()
	}
}
