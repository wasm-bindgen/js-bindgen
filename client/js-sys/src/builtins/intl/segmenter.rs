use alloc::string::String;

use super::{LocaleMatcher, LocaleMatcherOptions};
use crate::hazard::JsCast;
use crate::{Array, Iterable, JsIterator, JsString, JsValue, Object, js_sys};

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/Segmenter#granularity)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SegmenterGranularity {
	Grapheme,
	Word,
	Sentence,
}

impl SegmenterGranularity {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Grapheme => "grapheme",
			Self::Word => "word",
			Self::Sentence => "sentence",
		}
	}

	fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"grapheme" => Some(Self::Grapheme),
			"word" => Some(Self::Word),
			"sentence" => Some(Self::Sentence),
			_ => None,
		}
	}
}

#[js_sys(js_sys = crate, namespace = "Intl")]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter)
	#[js_sys(js_name = "Segmenter", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Segmenter;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/Segmenter#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type SegmenterOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/resolvedOptions#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type SegmenterResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/segment/Segments)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Segments;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/segment/Segments/containing#return_value)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type SegmentData;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/Segmenter)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> Segmenter;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/Segmenter)
	#[js_sys(constructor)]
	pub fn new_with_locales(locales: &JsValue) -> Result<Segmenter, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/Segmenter)
	#[js_sys(constructor)]
	pub fn new_with_locales_and_options(
		locales: &JsValue,
		options: &SegmenterOptions,
	) -> Result<Segmenter, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/supportedLocalesOf)
	#[js_sys(static_of = Segmenter, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of(locales: &JsValue) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/supportedLocalesOf)
	#[js_sys(static_of = Segmenter, js_name = "supportedLocalesOf")]
	pub fn supported_locales_of_with_options(
		locales: &JsValue,
		options: &LocaleMatcherOptions,
	) -> Result<Array<JsString>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/resolvedOptions)
	#[must_use]
	#[js_sys(js_name = "resolvedOptions")]
	pub fn resolved_options(self: &Segmenter) -> SegmenterResolvedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/segment)
	#[must_use]
	pub fn segment(self: &Segmenter, input: &str) -> Segments;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/segment/Segments/containing)
	#[must_use]
	pub fn containing(self: &Segments) -> Option<SegmentData>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/segment/Segments/containing)
	#[must_use]
	#[js_sys(js_name = "containing")]
	pub fn containing_at(self: &Segments, index: f64) -> Option<SegmentData>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn locale(self: &SegmenterResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/resolvedOptions#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn granularity(self: &SegmenterResolvedOptions) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/segment/Segments/containing#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn segment(self: &SegmentData) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/segment/Segments/containing#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn index(self: &SegmentData) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/segment/Segments/containing#return_value)
	#[must_use]
	#[js_sys(getter)]
	pub fn input(self: &SegmentData) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/segment/Segments/containing#return_value)
	#[must_use]
	#[js_sys(getter = "isWordLike")]
	pub fn is_word_like(self: &SegmentData) -> Option<bool>;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "intl.segments.iterator")]
	fn segments_symbol_iterator(segments: &Segments) -> JsIterator<SegmentData>;

	#[js_sys(getter = "localeMatcher")]
	fn locale_matcher_raw(self: &SegmenterOptions) -> Option<JsString>;

	#[js_sys(getter = "granularity")]
	fn granularity_raw(self: &SegmenterOptions) -> Option<JsString>;

	#[js_sys(setter = "localeMatcher")]
	fn set_locale_matcher_raw(self: &SegmenterOptions, value: &str);

	#[js_sys(setter = "granularity")]
	fn set_granularity_raw(self: &SegmenterOptions, value: &str);
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "intl.segments.iterator",
	"segments => segments[Symbol.iterator]()",
);

impl SegmenterOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/Segmenter#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/Segmenter#localematcher)
	#[must_use]
	pub fn locale_matcher(&self) -> Option<LocaleMatcher> {
		self.locale_matcher_raw()
			.as_ref()
			.and_then(LocaleMatcher::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/Segmenter#localematcher)
	pub fn set_locale_matcher(&self, value: LocaleMatcher) {
		self.set_locale_matcher_raw(value.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/Segmenter#granularity)
	#[must_use]
	pub fn granularity(&self) -> Option<SegmenterGranularity> {
		self.granularity_raw()
			.as_ref()
			.and_then(SegmenterGranularity::from_js_string)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/Segmenter#granularity)
	pub fn set_granularity(&self, value: SegmenterGranularity) {
		self.set_granularity_raw(value.as_str());
	}
}

impl Default for SegmenterOptions {
	fn default() -> Self {
		Self::new()
	}
}

impl Default for Segmenter {
	fn default() -> Self {
		Self::new()
	}
}

impl Segments {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Segmenter/segment/Segments/Symbol.iterator)
	#[must_use]
	pub fn symbol_iterator(&self) -> JsIterator<SegmentData> {
		segments_symbol_iterator(self)
	}
}

impl Iterable for Segments {
	type Item = SegmentData;
}
