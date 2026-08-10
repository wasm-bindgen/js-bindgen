use super::duration::Duration;
use crate::{JsString, JsValue, Object, js_sys};

#[js_sys(js_sys = crate, namespace = "Temporal")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime)
	#[js_sys(js_name = "PlainTime", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type PlainTime;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/PlainTime)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> PlainTime;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/PlainTime)
	#[js_sys(constructor)]
	pub fn new_with_hour(hour: u32) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/PlainTime)
	#[js_sys(constructor)]
	pub fn new_with_hour_minute(hour: u32, minute: u32) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/PlainTime)
	#[js_sys(constructor)]
	pub fn new_with_hour_minute_second(
		hour: u32,
		minute: u32,
		second: u32,
	) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/PlainTime)
	#[js_sys(constructor)]
	pub fn new_with_hour_minute_second_millisecond(
		hour: u32,
		minute: u32,
		second: u32,
		millisecond: u32,
	) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/PlainTime)
	#[js_sys(constructor)]
	pub fn new_with_hour_minute_second_millisecond_microsecond(
		hour: u32,
		minute: u32,
		second: u32,
		millisecond: u32,
		microsecond: u32,
	) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/PlainTime)
	#[js_sys(constructor)]
	pub fn new_with_values(
		hour: u32,
		minute: u32,
		second: u32,
		millisecond: u32,
		microsecond: u32,
		nanosecond: u32,
	) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/from)
	#[js_sys(static_of = PlainTime)]
	pub fn from(item: &JsValue) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/from)
	#[js_sys(static_of = PlainTime, js_name = "from")]
	pub fn from_with_options(item: &JsValue, options: &JsValue) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/compare)
	#[js_sys(static_of = PlainTime)]
	pub fn compare(one: &JsValue, two: &JsValue) -> Result<i32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/hour)
	#[must_use]
	#[js_sys(getter)]
	pub fn hour(self: &PlainTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/minute)
	#[must_use]
	#[js_sys(getter)]
	pub fn minute(self: &PlainTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/second)
	#[must_use]
	#[js_sys(getter)]
	pub fn second(self: &PlainTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/millisecond)
	#[must_use]
	#[js_sys(getter)]
	pub fn millisecond(self: &PlainTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/microsecond)
	#[must_use]
	#[js_sys(getter)]
	pub fn microsecond(self: &PlainTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/nanosecond)
	#[must_use]
	#[js_sys(getter)]
	pub fn nanosecond(self: &PlainTime) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/add)
	pub fn add(self: &PlainTime, duration: &JsValue) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/subtract)
	pub fn subtract(self: &PlainTime, duration: &JsValue) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/with)
	pub fn with(self: &PlainTime, time_like: &JsValue) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/with)
	#[js_sys(js_name = "with")]
	pub fn with_options(
		self: &PlainTime,
		time_like: &JsValue,
		options: &JsValue,
	) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/until)
	pub fn until(self: &PlainTime, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/until)
	#[js_sys(js_name = "until")]
	pub fn until_with_options(
		self: &PlainTime,
		other: &JsValue,
		options: &JsValue,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/since)
	pub fn since(self: &PlainTime, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/since)
	#[js_sys(js_name = "since")]
	pub fn since_with_options(
		self: &PlainTime,
		other: &JsValue,
		options: &JsValue,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/equals)
	pub fn equals(self: &PlainTime, other: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/round)
	pub fn round(self: &PlainTime, round_to: &JsValue) -> Result<PlainTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &PlainTime) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/toString)
	#[js_sys(js_name = "toString")]
	pub fn to_string_with_options(self: &PlainTime, options: &JsValue)
	-> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string(self: &PlainTime) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales(
		self: &PlainTime,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales_and_options(
		self: &PlainTime,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/toJSON)
	#[must_use]
	#[js_sys(js_name = "toJSON")]
	pub fn to_json(self: &PlainTime) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/PlainTime/valueOf)
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &PlainTime) -> Result<JsValue, JsValue>;
}

impl Default for PlainTime {
	fn default() -> Self {
		Self::new()
	}
}
