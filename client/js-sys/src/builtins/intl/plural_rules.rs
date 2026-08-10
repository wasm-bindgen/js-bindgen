use alloc::string::String;

use super::{LocaleMatcher, LocaleMatcherOptions};
use crate::hazard::JsCast;
use crate::{Array, JsString, JsValue, Object, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#type)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PluralRulesType {
	Cardinal,
	Ordinal,
}

impl PluralRulesType {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Cardinal => "cardinal",
			Self::Ordinal => "ordinal",
		}
	}

	fn parse(value: &str) -> Option<Self> {
		match value {
			"cardinal" => Some(Self::Cardinal),
			"ordinal" => Some(Self::Ordinal),
			_ => None,
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#roundingpriority)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PluralRulesRoundingPriority {
	Auto,
	MorePrecision,
	LessPrecision,
}

impl PluralRulesRoundingPriority {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Auto => "auto",
			Self::MorePrecision => "morePrecision",
			Self::LessPrecision => "lessPrecision",
		}
	}

	fn parse(value: &str) -> Option<Self> {
		match value {
			"auto" => Some(Self::Auto),
			"morePrecision" => Some(Self::MorePrecision),
			"lessPrecision" => Some(Self::LessPrecision),
			_ => None,
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#roundingincrement)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PluralRulesRoundingIncrement {
	One,
	Two,
	Five,
	Ten,
	Twenty,
	TwentyFive,
	Fifty,
	OneHundred,
	TwoHundred,
	TwoHundredFifty,
	FiveHundred,
	OneThousand,
	TwoThousand,
	TwoThousandFiveHundred,
	FiveThousand,
}

impl PluralRulesRoundingIncrement {
	const fn as_u32(self) -> u32 {
		match self {
			Self::One => 1,
			Self::Two => 2,
			Self::Five => 5,
			Self::Ten => 10,
			Self::Twenty => 20,
			Self::TwentyFive => 25,
			Self::Fifty => 50,
			Self::OneHundred => 100,
			Self::TwoHundred => 200,
			Self::TwoHundredFifty => 250,
			Self::FiveHundred => 500,
			Self::OneThousand => 1_000,
			Self::TwoThousand => 2_000,
			Self::TwoThousandFiveHundred => 2_500,
			Self::FiveThousand => 5_000,
		}
	}

	const fn from_u32(value: u32) -> Option<Self> {
		match value {
			1 => Some(Self::One),
			2 => Some(Self::Two),
			5 => Some(Self::Five),
			10 => Some(Self::Ten),
			20 => Some(Self::Twenty),
			25 => Some(Self::TwentyFive),
			50 => Some(Self::Fifty),
			100 => Some(Self::OneHundred),
			200 => Some(Self::TwoHundred),
			250 => Some(Self::TwoHundredFifty),
			500 => Some(Self::FiveHundred),
			1_000 => Some(Self::OneThousand),
			2_000 => Some(Self::TwoThousand),
			2_500 => Some(Self::TwoThousandFiveHundred),
			5_000 => Some(Self::FiveThousand),
			_ => None,
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#roundingmode)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PluralRulesRoundingMode {
	Ceil,
	Floor,
	Expand,
	Trunc,
	HalfCeil,
	HalfFloor,
	HalfExpand,
	HalfTrunc,
	HalfEven,
}

impl PluralRulesRoundingMode {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Ceil => "ceil",
			Self::Floor => "floor",
			Self::Expand => "expand",
			Self::Trunc => "trunc",
			Self::HalfCeil => "halfCeil",
			Self::HalfFloor => "halfFloor",
			Self::HalfExpand => "halfExpand",
			Self::HalfTrunc => "halfTrunc",
			Self::HalfEven => "halfEven",
		}
	}

	fn parse(value: &str) -> Option<Self> {
		match value {
			"ceil" => Some(Self::Ceil),
			"floor" => Some(Self::Floor),
			"expand" => Some(Self::Expand),
			"trunc" => Some(Self::Trunc),
			"halfCeil" => Some(Self::HalfCeil),
			"halfFloor" => Some(Self::HalfFloor),
			"halfExpand" => Some(Self::HalfExpand),
			"halfTrunc" => Some(Self::HalfTrunc),
			"halfEven" => Some(Self::HalfEven),
			_ => None,
		}
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#options)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PluralRulesTrailingZeroDisplay {
	Auto,
	StripIfInteger,
}

impl PluralRulesTrailingZeroDisplay {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Auto => "auto",
			Self::StripIfInteger => "stripIfInteger",
		}
	}

	fn parse(value: &str) -> Option<Self> {
		match value {
			"auto" => Some(Self::Auto),
			"stripIfInteger" => Some(Self::StripIfInteger),
			_ => None,
		}
	}
}

#[js_sys(js_sys = crate, namespace = "Intl")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules)
	#[js_sys(js_name = "PluralRules", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type PluralRules;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type PluralRulesOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type PluralRulesResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> PluralRules;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules)
	#[js_sys(constructor)]
	pub fn new_with_locales(locales: &JsValue) -> Result<PluralRules, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules)
	#[js_sys(constructor)]
	pub fn new_with_locales_and_options(
		locales: &JsValue,
		options: &PluralRulesOptions,
	) -> Result<PluralRules, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/supportedLocalesOf)
	#[js_sys(static_of = PluralRules, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of(locales: &JsValue) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/supportedLocalesOf)
	#[js_sys(static_of = PluralRules, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of_with_options(
		locales: &JsValue,
		options: &LocaleMatcherOptions,
	) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/select)
	#[must_use]
	pub fn select(self: &PluralRules, value: f64) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/selectRange)
	#[js_sys(js_name = "selectRange")]
	pub fn select_range(self: &PluralRules, start: f64, end: f64) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions)
	#[must_use]
	#[js_sys(js_name = "resolvedOptions")]
	pub fn resolved_options(self: &PluralRules) -> PluralRulesResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn locale(self: &PluralRulesResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "type")]
	pub fn type_(self: &PluralRulesResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "minimumIntegerDigits")]
	pub fn minimum_integer_digits(self: &PluralRulesResolvedOptions) -> Option<u32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "minimumFractionDigits")]
	pub fn minimum_fraction_digits(self: &PluralRulesResolvedOptions) -> Option<u32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "maximumFractionDigits")]
	pub fn maximum_fraction_digits(self: &PluralRulesResolvedOptions) -> Option<u32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "minimumSignificantDigits")]
	pub fn minimum_significant_digits(self: &PluralRulesResolvedOptions) -> Option<u32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "maximumSignificantDigits")]
	pub fn maximum_significant_digits(self: &PluralRulesResolvedOptions) -> Option<u32>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "pluralCategories")]
	pub fn plural_categories(self: &PluralRulesResolvedOptions) -> Array<JsString>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "roundingIncrement")]
	pub fn rounding_increment(self: &PluralRulesResolvedOptions) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "roundingMode")]
	pub fn rounding_mode(self: &PluralRulesResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "roundingPriority")]
	pub fn rounding_priority(self: &PluralRulesResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter = "trailingZeroDisplay")]
	pub fn trailing_zero_display(self: &PluralRulesResolvedOptions) -> JsString;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(getter = "localeMatcher")]
	fn locale_matcher_raw(self: &PluralRulesOptions) -> Option<JsString>;

	#[js_sys(setter = "localeMatcher")]
	fn set_locale_matcher_raw(self: &PluralRulesOptions, value: &str);

	#[js_sys(getter = "type")]
	fn type_raw(self: &PluralRulesOptions) -> Option<JsString>;

	#[js_sys(setter = "type")]
	fn set_type_raw(self: &PluralRulesOptions, value: &str);

	#[js_sys(getter = "minimumIntegerDigits")]
	fn minimum_integer_digits_raw(self: &PluralRulesOptions) -> Option<u32>;

	#[js_sys(setter = "minimumIntegerDigits")]
	fn set_minimum_integer_digits_raw(self: &PluralRulesOptions, value: u32);

	#[js_sys(getter = "minimumFractionDigits")]
	fn minimum_fraction_digits_raw(self: &PluralRulesOptions) -> Option<u32>;

	#[js_sys(setter = "minimumFractionDigits")]
	fn set_minimum_fraction_digits_raw(self: &PluralRulesOptions, value: u32);

	#[js_sys(getter = "maximumFractionDigits")]
	fn maximum_fraction_digits_raw(self: &PluralRulesOptions) -> Option<u32>;

	#[js_sys(setter = "maximumFractionDigits")]
	fn set_maximum_fraction_digits_raw(self: &PluralRulesOptions, value: u32);

	#[js_sys(getter = "minimumSignificantDigits")]
	fn minimum_significant_digits_raw(self: &PluralRulesOptions) -> Option<u32>;

	#[js_sys(setter = "minimumSignificantDigits")]
	fn set_minimum_significant_digits_raw(self: &PluralRulesOptions, value: u32);

	#[js_sys(getter = "maximumSignificantDigits")]
	fn maximum_significant_digits_raw(self: &PluralRulesOptions) -> Option<u32>;

	#[js_sys(setter = "maximumSignificantDigits")]
	fn set_maximum_significant_digits_raw(self: &PluralRulesOptions, value: u32);

	#[js_sys(getter = "roundingPriority")]
	fn rounding_priority_raw(self: &PluralRulesOptions) -> Option<JsString>;

	#[js_sys(setter = "roundingPriority")]
	fn set_rounding_priority_raw(self: &PluralRulesOptions, value: &str);

	#[js_sys(getter = "roundingIncrement")]
	fn rounding_increment_raw(self: &PluralRulesOptions) -> Option<u32>;

	#[js_sys(setter = "roundingIncrement")]
	fn set_rounding_increment_raw(self: &PluralRulesOptions, value: u32);

	#[js_sys(getter = "roundingMode")]
	fn rounding_mode_raw(self: &PluralRulesOptions) -> Option<JsString>;

	#[js_sys(setter = "roundingMode")]
	fn set_rounding_mode_raw(self: &PluralRulesOptions, value: &str);

	#[js_sys(getter = "trailingZeroDisplay")]
	fn trailing_zero_display_raw(self: &PluralRulesOptions) -> Option<JsString>;

	#[js_sys(setter = "trailingZeroDisplay")]
	fn set_trailing_zero_display_raw(self: &PluralRulesOptions, value: &str);
}

fn parse_string_option<T>(
	value: Option<JsString>,
	parse: impl FnOnce(&str) -> Option<T>,
) -> Option<T> {
	let value = String::from(value?);
	parse(&value)
}

impl PluralRulesOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#localematcher)
	#[must_use]
	pub fn locale_matcher(&self) -> Option<LocaleMatcher> {
		self.locale_matcher_raw()
			.as_ref()
			.and_then(LocaleMatcher::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#localematcher)
	pub fn set_locale_matcher(&self, value: LocaleMatcher) {
		self.set_locale_matcher_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#type)
	#[must_use]
	pub fn type_(&self) -> Option<PluralRulesType> {
		parse_string_option(self.type_raw(), PluralRulesType::parse)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#type)
	pub fn set_type(&self, value: PluralRulesType) {
		self.set_type_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#minimumintegerdigits)
	#[must_use]
	pub fn minimum_integer_digits(&self) -> Option<u32> {
		self.minimum_integer_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#minimumintegerdigits)
	pub fn set_minimum_integer_digits(&self, value: u32) {
		self.set_minimum_integer_digits_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#minimumfractiondigits)
	#[must_use]
	pub fn minimum_fraction_digits(&self) -> Option<u32> {
		self.minimum_fraction_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#minimumfractiondigits)
	pub fn set_minimum_fraction_digits(&self, value: u32) {
		self.set_minimum_fraction_digits_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#maximumfractiondigits)
	#[must_use]
	pub fn maximum_fraction_digits(&self) -> Option<u32> {
		self.maximum_fraction_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#maximumfractiondigits)
	pub fn set_maximum_fraction_digits(&self, value: u32) {
		self.set_maximum_fraction_digits_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#minimumsignificantdigits)
	#[must_use]
	pub fn minimum_significant_digits(&self) -> Option<u32> {
		self.minimum_significant_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#minimumsignificantdigits)
	pub fn set_minimum_significant_digits(&self, value: u32) {
		self.set_minimum_significant_digits_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#maximumsignificantdigits)
	#[must_use]
	pub fn maximum_significant_digits(&self) -> Option<u32> {
		self.maximum_significant_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#maximumsignificantdigits)
	pub fn set_maximum_significant_digits(&self, value: u32) {
		self.set_maximum_significant_digits_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#roundingpriority)
	#[must_use]
	pub fn rounding_priority(&self) -> Option<PluralRulesRoundingPriority> {
		parse_string_option(
			self.rounding_priority_raw(),
			PluralRulesRoundingPriority::parse,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#roundingpriority)
	pub fn set_rounding_priority(&self, value: PluralRulesRoundingPriority) {
		self.set_rounding_priority_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#roundingincrement)
	#[must_use]
	pub fn rounding_increment(&self) -> Option<PluralRulesRoundingIncrement> {
		self.rounding_increment_raw()
			.and_then(PluralRulesRoundingIncrement::from_u32)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#roundingincrement)
	pub fn set_rounding_increment(&self, value: PluralRulesRoundingIncrement) {
		self.set_rounding_increment_raw(value.as_u32());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#roundingmode)
	#[must_use]
	pub fn rounding_mode(&self) -> Option<PluralRulesRoundingMode> {
		parse_string_option(self.rounding_mode_raw(), PluralRulesRoundingMode::parse)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#roundingmode)
	pub fn set_rounding_mode(&self, value: PluralRulesRoundingMode) {
		self.set_rounding_mode_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#options)
	#[must_use]
	pub fn trailing_zero_display(&self) -> Option<PluralRulesTrailingZeroDisplay> {
		parse_string_option(
			self.trailing_zero_display_raw(),
			PluralRulesTrailingZeroDisplay::parse,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules/PluralRules#options)
	pub fn set_trailing_zero_display(&self, value: PluralRulesTrailingZeroDisplay) {
		self.set_trailing_zero_display_raw(value.as_str());
	}
}

impl Default for PluralRulesOptions {
	fn default() -> Self {
		Self::new()
	}
}

impl Default for PluralRules {
	fn default() -> Self {
		Self::new()
	}
}
