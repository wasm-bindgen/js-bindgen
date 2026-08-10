use super::duration::Duration;
use super::zoned_date_time::ZonedDateTime;
use crate::{BigInt, JsString, JsValue, Object, js_sys};

#[js_sys(js_sys = crate, namespace = "Temporal")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant)
	#[js_sys(js_name = "Instant", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Instant;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/Instant)
	#[js_sys(constructor)]
	pub fn new(epoch_nanoseconds: &BigInt) -> Result<Instant, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/from)
	#[js_sys(static_of = Instant)]
	pub fn from(item: &JsValue) -> Result<Instant, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/fromEpochMilliseconds)
	#[js_sys(static_of = Instant, js_name = "fromEpochMilliseconds")]
	pub fn from_epoch_milliseconds(epoch_milliseconds: f64) -> Result<Instant, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/fromEpochNanoseconds)
	#[js_sys(static_of = Instant, js_name = "fromEpochNanoseconds")]
	pub fn from_epoch_nanoseconds(epoch_nanoseconds: &BigInt) -> Result<Instant, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/compare)
	#[js_sys(static_of = Instant)]
	pub fn compare(one: &JsValue, two: &JsValue) -> Result<i32, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/epochMilliseconds)
	#[must_use]
	#[js_sys(getter = "epochMilliseconds")]
	pub fn epoch_milliseconds(self: &Instant) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/epochNanoseconds)
	#[must_use]
	#[js_sys(getter = "epochNanoseconds")]
	pub fn epoch_nanoseconds(self: &Instant) -> BigInt;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/add)
	pub fn add(self: &Instant, duration: &JsValue) -> Result<Instant, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/subtract)
	pub fn subtract(self: &Instant, duration: &JsValue) -> Result<Instant, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/until)
	pub fn until(self: &Instant, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/until)
	#[js_sys(js_name = "until")]
	pub fn until_with_options(
		self: &Instant,
		other: &JsValue,
		options: &JsValue,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/since)
	pub fn since(self: &Instant, other: &JsValue) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/since)
	#[js_sys(js_name = "since")]
	pub fn since_with_options(
		self: &Instant,
		other: &JsValue,
		options: &JsValue,
	) -> Result<Duration, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/round)
	pub fn round(self: &Instant, round_to: &JsValue) -> Result<Instant, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/equals)
	pub fn equals(self: &Instant, other: &JsValue) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &Instant) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toString)
	#[js_sys(js_name = "toString")]
	pub fn to_string_with_options(self: &Instant, options: &JsValue) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string(self: &Instant) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales(
		self: &Instant,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toLocaleString)
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales_and_options(
		self: &Instant,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toJSON)
	#[must_use]
	#[js_sys(js_name = "toJSON")]
	pub fn to_json(self: &Instant) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/toZonedDateTimeISO)
	#[js_sys(js_name = "toZonedDateTimeISO")]
	pub fn to_zoned_date_time_iso(
		self: &Instant,
		time_zone: &JsValue,
	) -> Result<ZonedDateTime, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Instant/valueOf)
	#[js_sys(js_name = "valueOf")]
	pub fn value_of(self: &Instant) -> Result<JsValue, JsValue>;
}
