use alloc::string::String;

use super::{LocaleMatcher, LocaleMatcherOptions};
use crate::hazard::JsCast;
use crate::{Array, Function, JsString, JsValue, Object, js_sys};

macro_rules! string_enum {
	(
		$(#[$meta:meta])*
		pub enum $name:ident {
			$($variant:ident => $value:literal),+ $(,)?
		}
	) => {
		$(#[$meta])*
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		pub enum $name {
			$($variant),+
		}

		impl $name {
			const fn as_str(self) -> &'static str {
				match self {
					$(Self::$variant => $value),+
				}
			}

			fn from_str(value: &str) -> Option<Self> {
				match value {
					$($value => Some(Self::$variant)),+,
					_ => None,
				}
			}
		}
	};
}

macro_rules! readonly_string_enum {
	(
		$(#[$meta:meta])*
		pub enum $name:ident {
			$($variant:ident => $value:literal),+ $(,)?
		}
	) => {
		$(#[$meta])*
		#[derive(Clone, Copy, Debug, Eq, PartialEq)]
		pub enum $name {
			$($variant),+
		}

		impl $name {
			fn from_str(value: &str) -> Option<Self> {
				match value {
					$($value => Some(Self::$variant)),+,
					_ => None,
				}
			}
		}
	};
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#style)
	pub enum NumberFormatStyle {
		Decimal => "decimal",
		Currency => "currency",
		Percent => "percent",
		Unit => "unit",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#currencydisplay)
	pub enum NumberFormatCurrencyDisplay {
		Code => "code",
		Symbol => "symbol",
		NarrowSymbol => "narrowSymbol",
		Name => "name",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#currencysign)
	pub enum NumberFormatCurrencySign {
		Standard => "standard",
		Accounting => "accounting",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#unitdisplay)
	pub enum NumberFormatUnitDisplay {
		Short => "short",
		Narrow => "narrow",
		Long => "long",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#notation)
	pub enum NumberFormatNotation {
		Standard => "standard",
		Scientific => "scientific",
		Engineering => "engineering",
		Compact => "compact",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#compactdisplay)
	pub enum NumberFormatCompactDisplay {
		Short => "short",
		Long => "long",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#signdisplay)
	pub enum NumberFormatSignDisplay {
		Auto => "auto",
		Always => "always",
		ExceptZero => "exceptZero",
		Negative => "negative",
		Never => "never",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#roundingmode)
	pub enum NumberFormatRoundingMode {
		Ceil => "ceil",
		Floor => "floor",
		Expand => "expand",
		Trunc => "trunc",
		HalfCeil => "halfCeil",
		HalfFloor => "halfFloor",
		HalfExpand => "halfExpand",
		HalfTrunc => "halfTrunc",
		HalfEven => "halfEven",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#roundingpriority)
	pub enum NumberFormatRoundingPriority {
		Auto => "auto",
		MorePrecision => "morePrecision",
		LessPrecision => "lessPrecision",
	}
}

string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#trailingzerodisplay)
	pub enum NumberFormatTrailingZeroDisplay {
		Auto => "auto",
		StripIfInteger => "stripIfInteger",
	}
}

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#roundingincrement)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumberFormatRoundingIncrement {
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

impl NumberFormatRoundingIncrement {
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

	fn from_u32(value: u32) -> Option<Self> {
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

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#usegrouping)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumberFormatUseGrouping {
	Always,
	Auto,
	Min2,
	True,
	False,
}

impl NumberFormatUseGrouping {
	fn from_str(value: &str) -> Option<Self> {
		match value {
			"always" => Some(Self::Always),
			"auto" => Some(Self::Auto),
			"min2" => Some(Self::Min2),
			"true" => Some(Self::True),
			"false" => Some(Self::False),
			_ => None,
		}
	}
}

readonly_string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/formatToParts#return_value)
	pub enum NumberFormatPartType {
		ApproximatelySign => "approximatelySign",
		Compact => "compact",
		Currency => "currency",
		Decimal => "decimal",
		ExponentInteger => "exponentInteger",
		ExponentMinusSign => "exponentMinusSign",
		ExponentSeparator => "exponentSeparator",
		Fraction => "fraction",
		Group => "group",
		Infinity => "infinity",
		Integer => "integer",
		Literal => "literal",
		MinusSign => "minusSign",
		Nan => "nan",
		PercentSign => "percentSign",
		PlusSign => "plusSign",
		Unit => "unit",
		Unknown => "unknown",
	}
}

readonly_string_enum! {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/formatRangeToParts#return_value)
	pub enum NumberFormatRangeSource {
		StartRange => "startRange",
		EndRange => "endRange",
		Shared => "shared",
	}
}

#[js_sys(js_sys = crate, namespace = "Intl")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat)
	#[js_sys(js_name = "NumberFormat", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type NumberFormat;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type NumberFormatOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type NumberFormatResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/formatToParts#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type NumberFormatPart;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/formatRangeToParts#return_value)
	#[js_sys(extends = NumberFormatPart)]
	#[derive(Clone, Debug, PartialEq)]
	pub type NumberRangeFormatPart;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> NumberFormat;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat)
	#[js_sys(constructor)]
	pub fn new_with_locales(locales: &JsValue) -> Result<NumberFormat, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat)
	#[js_sys(constructor)]
	pub fn new_with_locales_and_options(
		locales: &JsValue,
		options: &NumberFormatOptions,
	) -> Result<NumberFormat, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/supportedLocalesOf)
	#[js_sys(static_of = NumberFormat, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of(locales: &JsValue) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/supportedLocalesOf)
	#[js_sys(static_of = NumberFormat, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of_with_options(
		locales: &JsValue,
		options: &LocaleMatcherOptions,
	) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/format)
	#[must_use]
	#[js_sys(getter)]
	pub fn format(self: &NumberFormat) -> Function;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/formatToParts)
	#[must_use]
	#[js_sys(js_name = "formatToParts")]
	pub fn format_to_parts(self: &NumberFormat) -> Array<NumberFormatPart>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/formatToParts)
	#[js_sys(js_name = "formatToParts")]
	pub fn format_to_parts_with_value(
		self: &NumberFormat,
		value: &JsValue,
	) -> Result<Array<NumberFormatPart>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/formatRange)
	#[js_sys(js_name = "formatRange")]
	pub fn format_range(
		self: &NumberFormat,
		start: &JsValue,
		end: &JsValue,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/formatRangeToParts)
	#[js_sys(js_name = "formatRangeToParts")]
	pub fn format_range_to_parts(
		self: &NumberFormat,
		start: &JsValue,
		end: &JsValue,
	) -> Result<Array<NumberRangeFormatPart>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions)
	#[must_use]
	#[js_sys(js_name = "resolvedOptions")]
	pub fn resolved_options(self: &NumberFormat) -> NumberFormatResolvedOptions;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(getter = "localeMatcher")]
	fn locale_matcher_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "localeMatcher")]
	fn set_locale_matcher_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "numberingSystem")]
	fn numbering_system_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "numberingSystem")]
	fn set_numbering_system_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "style")]
	fn style_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "style")]
	fn set_style_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "currency")]
	fn currency_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "currency")]
	fn set_currency_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "currencyDisplay")]
	fn currency_display_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "currencyDisplay")]
	fn set_currency_display_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "currencySign")]
	fn currency_sign_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "currencySign")]
	fn set_currency_sign_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "unit")]
	fn unit_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "unit")]
	fn set_unit_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "unitDisplay")]
	fn unit_display_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "unitDisplay")]
	fn set_unit_display_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "minimumIntegerDigits")]
	fn minimum_integer_digits_raw(self: &NumberFormatOptions) -> Option<u32>;

	#[js_sys(setter = "minimumIntegerDigits")]
	fn set_minimum_integer_digits_raw(self: &NumberFormatOptions, value: u32);

	#[js_sys(getter = "minimumFractionDigits")]
	fn minimum_fraction_digits_raw(self: &NumberFormatOptions) -> Option<u32>;

	#[js_sys(setter = "minimumFractionDigits")]
	fn set_minimum_fraction_digits_raw(self: &NumberFormatOptions, value: u32);

	#[js_sys(getter = "maximumFractionDigits")]
	fn maximum_fraction_digits_raw(self: &NumberFormatOptions) -> Option<u32>;

	#[js_sys(setter = "maximumFractionDigits")]
	fn set_maximum_fraction_digits_raw(self: &NumberFormatOptions, value: u32);

	#[js_sys(getter = "minimumSignificantDigits")]
	fn minimum_significant_digits_raw(self: &NumberFormatOptions) -> Option<u32>;

	#[js_sys(setter = "minimumSignificantDigits")]
	fn set_minimum_significant_digits_raw(self: &NumberFormatOptions, value: u32);

	#[js_sys(getter = "maximumSignificantDigits")]
	fn maximum_significant_digits_raw(self: &NumberFormatOptions) -> Option<u32>;

	#[js_sys(setter = "maximumSignificantDigits")]
	fn set_maximum_significant_digits_raw(self: &NumberFormatOptions, value: u32);

	#[js_sys(getter = "roundingPriority")]
	fn rounding_priority_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "roundingPriority")]
	fn set_rounding_priority_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "roundingIncrement")]
	fn rounding_increment_raw(self: &NumberFormatOptions) -> Option<u32>;

	#[js_sys(setter = "roundingIncrement")]
	fn set_rounding_increment_raw(self: &NumberFormatOptions, value: u32);

	#[js_sys(getter = "roundingMode")]
	fn rounding_mode_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "roundingMode")]
	fn set_rounding_mode_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "trailingZeroDisplay")]
	fn trailing_zero_display_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "trailingZeroDisplay")]
	fn set_trailing_zero_display_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "notation")]
	fn notation_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "notation")]
	fn set_notation_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "compactDisplay")]
	fn compact_display_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "compactDisplay")]
	fn set_compact_display_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(js_embed = "intl.number_format.use_grouping")]
	fn use_grouping_raw(options: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "useGrouping")]
	fn set_use_grouping_string_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(setter = "useGrouping")]
	fn set_use_grouping_bool_raw(self: &NumberFormatOptions, value: bool);

	#[js_sys(getter = "signDisplay")]
	fn sign_display_raw(self: &NumberFormatOptions) -> Option<JsString>;

	#[js_sys(setter = "signDisplay")]
	fn set_sign_display_raw(self: &NumberFormatOptions, value: &str);

	#[js_sys(getter = "locale")]
	fn locale_raw(self: &NumberFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "numberingSystem")]
	fn resolved_numbering_system_raw(self: &NumberFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "style")]
	fn resolved_style_raw(self: &NumberFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "currency")]
	fn resolved_currency_raw(self: &NumberFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "currencyDisplay")]
	fn resolved_currency_display_raw(self: &NumberFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "currencySign")]
	fn resolved_currency_sign_raw(self: &NumberFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "unit")]
	fn resolved_unit_raw(self: &NumberFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "unitDisplay")]
	fn resolved_unit_display_raw(self: &NumberFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "minimumIntegerDigits")]
	fn resolved_minimum_integer_digits_raw(self: &NumberFormatResolvedOptions) -> u32;

	#[js_sys(getter = "minimumFractionDigits")]
	fn resolved_minimum_fraction_digits_raw(self: &NumberFormatResolvedOptions) -> Option<u32>;

	#[js_sys(getter = "maximumFractionDigits")]
	fn resolved_maximum_fraction_digits_raw(self: &NumberFormatResolvedOptions) -> Option<u32>;

	#[js_sys(getter = "minimumSignificantDigits")]
	fn resolved_minimum_significant_digits_raw(self: &NumberFormatResolvedOptions) -> Option<u32>;

	#[js_sys(getter = "maximumSignificantDigits")]
	fn resolved_maximum_significant_digits_raw(self: &NumberFormatResolvedOptions) -> Option<u32>;

	#[js_sys(js_embed = "intl.number_format.use_grouping")]
	fn resolved_use_grouping_raw(options: &NumberFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "notation")]
	fn resolved_notation_raw(self: &NumberFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "compactDisplay")]
	fn resolved_compact_display_raw(self: &NumberFormatResolvedOptions) -> Option<JsString>;

	#[js_sys(getter = "signDisplay")]
	fn resolved_sign_display_raw(self: &NumberFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "roundingIncrement")]
	fn resolved_rounding_increment_raw(self: &NumberFormatResolvedOptions) -> u32;

	#[js_sys(getter = "roundingMode")]
	fn resolved_rounding_mode_raw(self: &NumberFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "roundingPriority")]
	fn resolved_rounding_priority_raw(self: &NumberFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "trailingZeroDisplay")]
	fn resolved_trailing_zero_display_raw(self: &NumberFormatResolvedOptions) -> JsString;

	#[js_sys(getter = "type")]
	fn part_type_raw(self: &NumberFormatPart) -> JsString;

	#[js_sys(getter = "value")]
	fn part_value_raw(self: &NumberFormatPart) -> JsString;

	#[js_sys(getter = "source")]
	fn range_source_raw(self: &NumberRangeFormatPart) -> JsString;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "intl.number_format.use_grouping",
	"value => {{ const grouping = value.useGrouping; return grouping === true ? 'true' : grouping \
	 === false ? 'false' : typeof grouping === 'string' ? grouping : undefined }}",
);

fn parse_string_option<T>(
	value: Option<JsString>,
	parse: impl FnOnce(&str) -> Option<T>,
) -> Option<T> {
	let value = String::from(value?);
	parse(&value)
}

impl NumberFormatOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#localematcher)
	#[must_use]
	pub fn locale_matcher(&self) -> Option<LocaleMatcher> {
		parse_string_option(self.locale_matcher_raw(), |value| match value {
			"lookup" => Some(LocaleMatcher::Lookup),
			"best fit" => Some(LocaleMatcher::BestFit),
			_ => None,
		})
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#localematcher)
	pub fn set_locale_matcher(&self, value: LocaleMatcher) {
		self.set_locale_matcher_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#numberingsystem)
	#[must_use]
	pub fn numbering_system(&self) -> Option<JsString> {
		self.numbering_system_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#numberingsystem)
	pub fn set_numbering_system(&self, value: &str) {
		self.set_numbering_system_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#style)
	#[must_use]
	pub fn style(&self) -> Option<NumberFormatStyle> {
		parse_string_option(self.style_raw(), NumberFormatStyle::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#style)
	pub fn set_style(&self, value: NumberFormatStyle) {
		self.set_style_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#currency)
	#[must_use]
	pub fn currency(&self) -> Option<JsString> {
		self.currency_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#currency)
	pub fn set_currency(&self, value: &str) {
		self.set_currency_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#currencydisplay)
	#[must_use]
	pub fn currency_display(&self) -> Option<NumberFormatCurrencyDisplay> {
		parse_string_option(
			self.currency_display_raw(),
			NumberFormatCurrencyDisplay::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#currencydisplay)
	pub fn set_currency_display(&self, value: NumberFormatCurrencyDisplay) {
		self.set_currency_display_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#currencysign)
	#[must_use]
	pub fn currency_sign(&self) -> Option<NumberFormatCurrencySign> {
		parse_string_option(self.currency_sign_raw(), NumberFormatCurrencySign::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#currencysign)
	pub fn set_currency_sign(&self, value: NumberFormatCurrencySign) {
		self.set_currency_sign_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#unit)
	#[must_use]
	pub fn unit(&self) -> Option<JsString> {
		self.unit_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#unit)
	pub fn set_unit(&self, value: &str) {
		self.set_unit_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#unitdisplay)
	#[must_use]
	pub fn unit_display(&self) -> Option<NumberFormatUnitDisplay> {
		parse_string_option(self.unit_display_raw(), NumberFormatUnitDisplay::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#unitdisplay)
	pub fn set_unit_display(&self, value: NumberFormatUnitDisplay) {
		self.set_unit_display_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#minimumintegerdigits)
	#[must_use]
	pub fn minimum_integer_digits(&self) -> Option<u32> {
		self.minimum_integer_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#minimumintegerdigits)
	pub fn set_minimum_integer_digits(&self, value: u32) {
		self.set_minimum_integer_digits_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#minimumfractiondigits)
	#[must_use]
	pub fn minimum_fraction_digits(&self) -> Option<u32> {
		self.minimum_fraction_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#minimumfractiondigits)
	pub fn set_minimum_fraction_digits(&self, value: u32) {
		self.set_minimum_fraction_digits_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#maximumfractiondigits)
	#[must_use]
	pub fn maximum_fraction_digits(&self) -> Option<u32> {
		self.maximum_fraction_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#maximumfractiondigits)
	pub fn set_maximum_fraction_digits(&self, value: u32) {
		self.set_maximum_fraction_digits_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#minimumsignificantdigits)
	#[must_use]
	pub fn minimum_significant_digits(&self) -> Option<u32> {
		self.minimum_significant_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#minimumsignificantdigits)
	pub fn set_minimum_significant_digits(&self, value: u32) {
		self.set_minimum_significant_digits_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#maximumsignificantdigits)
	#[must_use]
	pub fn maximum_significant_digits(&self) -> Option<u32> {
		self.maximum_significant_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#maximumsignificantdigits)
	pub fn set_maximum_significant_digits(&self, value: u32) {
		self.set_maximum_significant_digits_raw(value);
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#roundingpriority)
	#[must_use]
	pub fn rounding_priority(&self) -> Option<NumberFormatRoundingPriority> {
		parse_string_option(
			self.rounding_priority_raw(),
			NumberFormatRoundingPriority::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#roundingpriority)
	pub fn set_rounding_priority(&self, value: NumberFormatRoundingPriority) {
		self.set_rounding_priority_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#roundingincrement)
	#[must_use]
	pub fn rounding_increment(&self) -> Option<NumberFormatRoundingIncrement> {
		self.rounding_increment_raw()
			.and_then(NumberFormatRoundingIncrement::from_u32)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#roundingincrement)
	pub fn set_rounding_increment(&self, value: NumberFormatRoundingIncrement) {
		self.set_rounding_increment_raw(value.as_u32());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#roundingmode)
	#[must_use]
	pub fn rounding_mode(&self) -> Option<NumberFormatRoundingMode> {
		parse_string_option(self.rounding_mode_raw(), NumberFormatRoundingMode::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#roundingmode)
	pub fn set_rounding_mode(&self, value: NumberFormatRoundingMode) {
		self.set_rounding_mode_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#trailingzerodisplay)
	#[must_use]
	pub fn trailing_zero_display(&self) -> Option<NumberFormatTrailingZeroDisplay> {
		parse_string_option(
			self.trailing_zero_display_raw(),
			NumberFormatTrailingZeroDisplay::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#trailingzerodisplay)
	pub fn set_trailing_zero_display(&self, value: NumberFormatTrailingZeroDisplay) {
		self.set_trailing_zero_display_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#notation)
	#[must_use]
	pub fn notation(&self) -> Option<NumberFormatNotation> {
		parse_string_option(self.notation_raw(), NumberFormatNotation::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#notation)
	pub fn set_notation(&self, value: NumberFormatNotation) {
		self.set_notation_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#compactdisplay)
	#[must_use]
	pub fn compact_display(&self) -> Option<NumberFormatCompactDisplay> {
		parse_string_option(
			self.compact_display_raw(),
			NumberFormatCompactDisplay::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#compactdisplay)
	pub fn set_compact_display(&self, value: NumberFormatCompactDisplay) {
		self.set_compact_display_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#usegrouping)
	#[must_use]
	pub fn use_grouping(&self) -> Option<NumberFormatUseGrouping> {
		parse_string_option(use_grouping_raw(self), NumberFormatUseGrouping::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#usegrouping)
	pub fn set_use_grouping(&self, value: NumberFormatUseGrouping) {
		match value {
			NumberFormatUseGrouping::Always => self.set_use_grouping_string_raw("always"),
			NumberFormatUseGrouping::Auto => self.set_use_grouping_string_raw("auto"),
			NumberFormatUseGrouping::Min2 => self.set_use_grouping_string_raw("min2"),
			NumberFormatUseGrouping::True => self.set_use_grouping_bool_raw(true),
			NumberFormatUseGrouping::False => self.set_use_grouping_bool_raw(false),
		}
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#signdisplay)
	#[must_use]
	pub fn sign_display(&self) -> Option<NumberFormatSignDisplay> {
		parse_string_option(self.sign_display_raw(), NumberFormatSignDisplay::from_str)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat#signdisplay)
	pub fn set_sign_display(&self, value: NumberFormatSignDisplay) {
		self.set_sign_display_raw(value.as_str());
	}
}

impl Default for NumberFormatOptions {
	fn default() -> Self {
		Self::new()
	}
}

impl Default for NumberFormat {
	fn default() -> Self {
		Self::new()
	}
}

impl NumberFormatResolvedOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn locale(&self) -> JsString {
		self.locale_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn numbering_system(&self) -> JsString {
		self.resolved_numbering_system_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn style(&self) -> Option<NumberFormatStyle> {
		NumberFormatStyle::from_str(&String::from(self.resolved_style_raw()))
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn currency(&self) -> Option<JsString> {
		self.resolved_currency_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn currency_display(&self) -> Option<NumberFormatCurrencyDisplay> {
		parse_string_option(
			self.resolved_currency_display_raw(),
			NumberFormatCurrencyDisplay::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn currency_sign(&self) -> Option<NumberFormatCurrencySign> {
		parse_string_option(
			self.resolved_currency_sign_raw(),
			NumberFormatCurrencySign::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn unit(&self) -> Option<JsString> {
		self.resolved_unit_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn unit_display(&self) -> Option<NumberFormatUnitDisplay> {
		parse_string_option(
			self.resolved_unit_display_raw(),
			NumberFormatUnitDisplay::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn minimum_integer_digits(&self) -> u32 {
		self.resolved_minimum_integer_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn minimum_fraction_digits(&self) -> Option<u32> {
		self.resolved_minimum_fraction_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn maximum_fraction_digits(&self) -> Option<u32> {
		self.resolved_maximum_fraction_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn minimum_significant_digits(&self) -> Option<u32> {
		self.resolved_minimum_significant_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn maximum_significant_digits(&self) -> Option<u32> {
		self.resolved_maximum_significant_digits_raw()
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn use_grouping(&self) -> Option<NumberFormatUseGrouping> {
		parse_string_option(
			resolved_use_grouping_raw(self),
			NumberFormatUseGrouping::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn notation(&self) -> Option<NumberFormatNotation> {
		NumberFormatNotation::from_str(&String::from(self.resolved_notation_raw()))
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn compact_display(&self) -> Option<NumberFormatCompactDisplay> {
		parse_string_option(
			self.resolved_compact_display_raw(),
			NumberFormatCompactDisplay::from_str,
		)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn sign_display(&self) -> Option<NumberFormatSignDisplay> {
		NumberFormatSignDisplay::from_str(&String::from(self.resolved_sign_display_raw()))
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn rounding_increment(&self) -> Option<NumberFormatRoundingIncrement> {
		NumberFormatRoundingIncrement::from_u32(self.resolved_rounding_increment_raw())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn rounding_mode(&self) -> Option<NumberFormatRoundingMode> {
		NumberFormatRoundingMode::from_str(&String::from(self.resolved_rounding_mode_raw()))
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn rounding_priority(&self) -> Option<NumberFormatRoundingPriority> {
		NumberFormatRoundingPriority::from_str(&String::from(self.resolved_rounding_priority_raw()))
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/resolvedOptions#return_value)
	#[must_use]
	pub fn trailing_zero_display(&self) -> Option<NumberFormatTrailingZeroDisplay> {
		NumberFormatTrailingZeroDisplay::from_str(&String::from(
			self.resolved_trailing_zero_display_raw(),
		))
	}
}

impl NumberFormatPart {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/formatToParts#return_value)
	#[must_use]
	pub fn type_(&self) -> Option<NumberFormatPartType> {
		NumberFormatPartType::from_str(&String::from(self.part_type_raw()))
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/formatToParts#return_value)
	#[must_use]
	pub fn value(&self) -> JsString {
		self.part_value_raw()
	}
}

impl NumberRangeFormatPart {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/formatRangeToParts#return_value)
	#[must_use]
	pub fn source(&self) -> Option<NumberFormatRangeSource> {
		NumberFormatRangeSource::from_str(&String::from(self.range_source_raw()))
	}
}
