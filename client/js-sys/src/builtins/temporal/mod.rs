mod duration;
mod instant;
mod now;
mod plain_date;
mod plain_date_time;
mod plain_month_day;
mod plain_time;
mod plain_year_month;
mod zoned_date_time;

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Temporal)
#[expect(non_snake_case, reason = "matches the JavaScript global name")]
pub mod Temporal {
	pub use super::duration::Duration;
	pub use super::instant::Instant;
	pub use super::now::Now;
	pub use super::plain_date::PlainDate;
	pub use super::plain_date_time::PlainDateTime;
	pub use super::plain_month_day::PlainMonthDay;
	pub use super::plain_time::PlainTime;
	pub use super::plain_year_month::PlainYearMonth;
	pub use super::zoned_date_time::ZonedDateTime;
}
