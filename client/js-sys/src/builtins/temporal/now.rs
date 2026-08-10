/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now)
#[expect(non_snake_case, reason = "matches the JavaScript namespace name")]
pub mod Now {
	use super::super::instant::Instant;
	use super::super::plain_date::PlainDate;
	use super::super::plain_date_time::PlainDateTime;
	use super::super::plain_time::PlainTime;
	use super::super::zoned_date_time::ZonedDateTime;
	use crate::{JsString, JsValue, js_sys};

	#[js_sys(js_sys = crate, namespace = "Temporal.Now")]
	extern "js-sys" {
		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/instant)
		#[must_use]
		pub fn instant() -> Instant;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/plainDateISO)
		#[must_use]
		#[js_sys(js_name = "plainDateISO")]
		pub fn plain_date_iso() -> PlainDate;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/plainDateISO)
		#[js_sys(js_name = "plainDateISO")]
		pub fn plain_date_iso_with_time_zone(time_zone: &JsValue) -> Result<PlainDate, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/plainDateTimeISO)
		#[must_use]
		#[js_sys(js_name = "plainDateTimeISO")]
		pub fn plain_date_time_iso() -> PlainDateTime;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/plainDateTimeISO)
		#[js_sys(js_name = "plainDateTimeISO")]
		pub fn plain_date_time_iso_with_time_zone(
			time_zone: &JsValue,
		) -> Result<PlainDateTime, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/plainTimeISO)
		#[must_use]
		#[js_sys(js_name = "plainTimeISO")]
		pub fn plain_time_iso() -> PlainTime;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/plainTimeISO)
		#[js_sys(js_name = "plainTimeISO")]
		pub fn plain_time_iso_with_time_zone(time_zone: &JsValue) -> Result<PlainTime, JsValue>;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/timeZoneId)
		#[must_use]
		#[js_sys(js_name = "timeZoneId")]
		pub fn time_zone_id() -> JsString;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/zonedDateTimeISO)
		#[must_use]
		#[js_sys(js_name = "zonedDateTimeISO")]
		pub fn zoned_date_time_iso() -> ZonedDateTime;

		/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal/Now/zonedDateTimeISO)
		#[js_sys(js_name = "zonedDateTimeISO")]
		pub fn zoned_date_time_iso_with_time_zone(
			time_zone: &JsValue,
		) -> Result<ZonedDateTime, JsValue>;
	}
}
