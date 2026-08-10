use alloc::string::String;

use super::collator::CollatorCaseFirst;
use crate::hazard::JsCast;
use crate::{Array, JsString, JsValue, Number, Object, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/hourCycle)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HourCycle {
	H11,
	H12,
	H23,
	H24,
}

impl HourCycle {
	pub(super) const fn as_str(self) -> &'static str {
		match self {
			Self::H11 => "h11",
			Self::H12 => "h12",
			Self::H23 => "h23",
			Self::H24 => "h24",
		}
	}

	pub(super) fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"h11" => Some(Self::H11),
			"h12" => Some(Self::H12),
			"h23" => Some(Self::H23),
			"h24" => Some(Self::H24),
			_ => None,
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getTextInfo)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextDirection {
	LeftToRight,
	RightToLeft,
}

/// [`ECMA-402` proposal](https://tc39.es/proposal-intl-locale-info/#sec-intl.locale.prototype.firstdayofweek)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FirstDayOfWeek {
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
	Sunday,
}

impl FirstDayOfWeek {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Monday => "mon",
			Self::Tuesday => "tue",
			Self::Wednesday => "wed",
			Self::Thursday => "thu",
			Self::Friday => "fri",
			Self::Saturday => "sat",
			Self::Sunday => "sun",
		}
	}

	fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"mon" => Some(Self::Monday),
			"tue" => Some(Self::Tuesday),
			"wed" => Some(Self::Wednesday),
			"thu" => Some(Self::Thursday),
			"fri" => Some(Self::Friday),
			"sat" => Some(Self::Saturday),
			"sun" => Some(Self::Sunday),
			_ => None,
		}
	}
}

impl TextDirection {
	fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"ltr" => Some(Self::LeftToRight),
			"rtl" => Some(Self::RightToLeft),
			_ => None,
		}
	}
}

#[js_sys(js_sys = crate, namespace = "Intl")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale)
	#[js_sys(js_name = "Locale", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Locale;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type LocaleOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getWeekInfo)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type WeekInfo;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getTextInfo)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type TextInfo;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale)
	#[js_sys(constructor)]
	pub fn new(tag: &str) -> Result<Locale, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale)
	#[js_sys(constructor)]
	pub fn new_with_options(tag: &str, options: &LocaleOptions) -> Result<Locale, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/baseName)
	#[must_use]
	#[js_sys(getter = "baseName")]
	pub fn base_name(self: &Locale) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/calendar)
	#[must_use]
	#[js_sys(getter)]
	pub fn calendar(self: &Locale) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/collation)
	#[must_use]
	#[js_sys(getter)]
	pub fn collation(self: &Locale) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/language)
	#[must_use]
	#[js_sys(getter)]
	pub fn language(self: &Locale) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/numberingSystem)
	#[must_use]
	#[js_sys(getter = "numberingSystem")]
	pub fn numbering_system(self: &Locale) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/numeric)
	#[must_use]
	#[js_sys(getter)]
	pub fn numeric(self: &Locale) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/region)
	#[must_use]
	#[js_sys(getter)]
	pub fn region(self: &Locale) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/script)
	#[must_use]
	#[js_sys(getter)]
	pub fn script(self: &Locale) -> Option<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/variants)
	#[must_use]
	#[js_sys(getter)]
	pub fn variants(self: &Locale) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getCalendars)
	#[must_use]
	#[js_sys(js_name = "getCalendars")]
	pub fn get_calendars(self: &Locale) -> Array<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getCollations)
	#[must_use]
	#[js_sys(js_name = "getCollations")]
	pub fn get_collations(self: &Locale) -> Array<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getHourCycles)
	#[must_use]
	#[js_sys(js_name = "getHourCycles")]
	pub fn get_hour_cycles(self: &Locale) -> Array<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getNumberingSystems)
	#[must_use]
	#[js_sys(js_name = "getNumberingSystems")]
	pub fn get_numbering_systems(self: &Locale) -> Array<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getTimeZones)
	#[must_use]
	#[js_sys(js_name = "getTimeZones")]
	pub fn get_time_zones(self: &Locale) -> Option<Array<JsString>>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getWeekInfo)
	#[js_sys(js_name = "getWeekInfo")]
	pub fn get_week_info(self: &Locale) -> Result<WeekInfo, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getTextInfo)
	#[js_sys(js_name = "getTextInfo")]
	pub fn get_text_info(self: &Locale) -> Result<TextInfo, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/maximize)
	#[must_use]
	pub fn maximize(self: &Locale) -> Locale;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/minimize)
	#[must_use]
	pub fn minimize(self: &Locale) -> Locale;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &Locale) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getWeekInfo)
	#[must_use]
	#[js_sys(getter = "firstDay")]
	pub fn first_day(self: &WeekInfo) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getWeekInfo)
	#[must_use]
	#[js_sys(getter)]
	pub fn weekend(self: &WeekInfo) -> Array<Number>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getWeekInfo)
	#[must_use]
	#[js_sys(getter = "minimalDays")]
	pub fn minimal_days(self: &WeekInfo) -> Option<u32>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(getter = "caseFirst")]
	fn case_first_raw(self: &Locale) -> Option<JsString>;

	#[js_sys(getter = "hourCycle")]
	fn hour_cycle_raw(self: &Locale) -> Option<JsString>;

	#[js_sys(getter = "direction")]
	fn direction_raw(self: &TextInfo) -> Option<JsString>;

	#[js_sys(getter = "language")]
	fn language_raw(self: &LocaleOptions) -> Option<JsString>;

	#[js_sys(getter = "script")]
	fn script_raw(self: &LocaleOptions) -> Option<JsString>;

	#[js_sys(getter = "region")]
	fn region_raw(self: &LocaleOptions) -> Option<JsString>;

	#[js_sys(getter = "variants")]
	fn variants_raw(self: &LocaleOptions) -> Option<JsString>;

	#[js_sys(getter = "calendar")]
	fn calendar_raw(self: &LocaleOptions) -> Option<JsString>;

	#[js_sys(getter = "collation")]
	fn collation_raw(self: &LocaleOptions) -> Option<JsString>;

	#[js_sys(getter = "numberingSystem")]
	fn numbering_system_raw(self: &LocaleOptions) -> Option<JsString>;

	#[js_sys(getter = "caseFirst")]
	fn option_case_first_raw(self: &LocaleOptions) -> Option<JsString>;

	#[js_sys(getter = "hourCycle")]
	fn option_hour_cycle_raw(self: &LocaleOptions) -> Option<JsString>;

	#[js_sys(getter = "numeric")]
	fn numeric_raw(self: &LocaleOptions) -> Option<bool>;

	#[js_sys(setter = "language")]
	fn set_language_raw(self: &LocaleOptions, value: &str);

	#[js_sys(setter = "script")]
	fn set_script_raw(self: &LocaleOptions, value: &str);

	#[js_sys(setter = "region")]
	fn set_region_raw(self: &LocaleOptions, value: &str);

	#[js_sys(setter = "variants")]
	fn set_variants_raw(self: &LocaleOptions, value: &str);

	#[js_sys(setter = "calendar")]
	fn set_calendar_raw(self: &LocaleOptions, value: &str);

	#[js_sys(setter = "collation")]
	fn set_collation_raw(self: &LocaleOptions, value: &str);

	#[js_sys(setter = "numberingSystem")]
	fn set_numbering_system_raw(self: &LocaleOptions, value: &str);

	#[js_sys(setter = "caseFirst")]
	fn set_case_first_raw(self: &LocaleOptions, value: &str);

	#[js_sys(setter = "hourCycle")]
	fn set_hour_cycle_raw(self: &LocaleOptions, value: &str);

	#[js_sys(setter = "numeric")]
	fn set_numeric_raw(self: &LocaleOptions, value: bool);
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(getter = "firstDayOfWeek")]
	fn first_day_of_week_raw(self: &Locale) -> Option<JsString>;

	#[js_sys(getter = "firstDayOfWeek")]
	fn option_first_day_of_week_raw(self: &LocaleOptions) -> Option<JsString>;

	#[js_sys(setter = "firstDayOfWeek")]
	fn set_first_day_of_week_raw(self: &LocaleOptions, value: &str);
}

impl Locale {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/caseFirst)
	#[must_use]
	pub fn case_first(&self) -> Option<CollatorCaseFirst> {
		self.case_first_raw()
			.as_ref()
			.and_then(CollatorCaseFirst::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/hourCycle)
	#[must_use]
	pub fn hour_cycle(&self) -> Option<HourCycle> {
		self.hour_cycle_raw()
			.as_ref()
			.and_then(HourCycle::from_js_string)
	}

	/// [`ECMA-402` proposal](https://tc39.es/proposal-intl-locale-info/#sec-intl.locale.prototype.firstdayofweek)
	#[must_use]
	pub fn first_day_of_week(&self) -> Option<FirstDayOfWeek> {
		self.first_day_of_week_raw()
			.as_ref()
			.and_then(FirstDayOfWeek::from_js_string)
	}
}

impl TextInfo {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/getTextInfo)
	#[must_use]
	pub fn direction(&self) -> Option<TextDirection> {
		self.direction_raw()
			.as_ref()
			.and_then(TextDirection::from_js_string)
	}
}

impl LocaleOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#language)
	#[must_use]
	pub fn language(&self) -> Option<JsString> {
		self.language_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#language)
	pub fn set_language(&self, value: &str) {
		self.set_language_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#script)
	#[must_use]
	pub fn script(&self) -> Option<JsString> {
		self.script_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#script)
	pub fn set_script(&self, value: &str) {
		self.set_script_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#region)
	#[must_use]
	pub fn region(&self) -> Option<JsString> {
		self.region_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#region)
	pub fn set_region(&self, value: &str) {
		self.set_region_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#variants)
	#[must_use]
	pub fn variants(&self) -> Option<JsString> {
		self.variants_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#variants)
	pub fn set_variants(&self, value: &str) {
		self.set_variants_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#calendar)
	#[must_use]
	pub fn calendar(&self) -> Option<JsString> {
		self.calendar_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#calendar)
	pub fn set_calendar(&self, value: &str) {
		self.set_calendar_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#collation)
	#[must_use]
	pub fn collation(&self) -> Option<JsString> {
		self.collation_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#collation)
	pub fn set_collation(&self, value: &str) {
		self.set_collation_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#numberingsystem)
	#[must_use]
	pub fn numbering_system(&self) -> Option<JsString> {
		self.numbering_system_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#numberingsystem)
	pub fn set_numbering_system(&self, value: &str) {
		self.set_numbering_system_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#casefirst)
	#[must_use]
	pub fn case_first(&self) -> Option<CollatorCaseFirst> {
		self.option_case_first_raw()
			.as_ref()
			.and_then(CollatorCaseFirst::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#hourcycle)
	#[must_use]
	pub fn hour_cycle(&self) -> Option<HourCycle> {
		self.option_hour_cycle_raw()
			.as_ref()
			.and_then(HourCycle::from_js_string)
	}

	/// [`ECMA-402` proposal](https://tc39.es/proposal-intl-locale-info/#sec-intl-locale-constructor)
	#[must_use]
	pub fn first_day_of_week(&self) -> Option<FirstDayOfWeek> {
		self.option_first_day_of_week_raw()
			.as_ref()
			.and_then(FirstDayOfWeek::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#numeric)
	#[must_use]
	pub fn numeric(&self) -> Option<bool> {
		self.numeric_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#numeric)
	pub fn set_numeric(&self, value: bool) {
		self.set_numeric_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#casefirst)
	pub fn set_case_first(&self, value: CollatorCaseFirst) {
		self.set_case_first_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale/Locale#hourcycle)
	pub fn set_hour_cycle(&self, value: HourCycle) {
		self.set_hour_cycle_raw(value.as_str());
	}

	/// [`ECMA-402` proposal](https://tc39.es/proposal-intl-locale-info/#sec-intl-locale-constructor)
	pub fn set_first_day_of_week(&self, value: FirstDayOfWeek) {
		self.set_first_day_of_week_raw(value.as_str());
	}
}

impl Default for LocaleOptions {
	fn default() -> Self {
		Self::new()
	}
}
