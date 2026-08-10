use super::{Array, Function, Iterable, JsIterator, JsString, Number, Object};
use crate::hazard::JsCast;
use crate::{JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(
		js_embed = "typed_array.slice",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_slice<T: JsCast>(array: &JsValue) -> Result<T, JsValue>;

	#[js_sys(
		js_embed = "typed_array.slice_from",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_slice_from<T: JsCast>(array: &JsValue, begin: f64) -> Result<T, JsValue>;

	#[js_sys(
		js_embed = "typed_array.slice_range",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_slice_range<T: JsCast>(
		array: &JsValue,
		begin: f64,
		end: f64,
	) -> Result<T, JsValue>;

	#[js_sys(
		js_embed = "typed_array.subarray",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_subarray<T: JsCast>(array: &JsValue) -> Result<T, JsValue>;

	#[js_sys(
		js_embed = "typed_array.subarray_from",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_subarray_from<T: JsCast>(array: &JsValue, begin: f64) -> Result<T, JsValue>;

	#[js_sys(
		js_embed = "typed_array.subarray_range",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_subarray_range<T: JsCast>(
		array: &JsValue,
		begin: f64,
		end: f64,
	) -> Result<T, JsValue>;

	#[js_sys(js_embed = "typed_array.property.buffer")]
	fn typed_array_buffer(array: &JsValue) -> JsValue;

	#[js_sys(js_embed = "typed_array.property.byte_length")]
	fn typed_array_byte_length(array: &JsValue) -> f64;

	#[js_sys(js_embed = "typed_array.property.byte_offset")]
	fn typed_array_byte_offset(array: &JsValue) -> f64;

	#[js_sys(js_embed = "typed_array.property.length")]
	fn typed_array_length(array: &JsValue) -> f64;

	#[js_sys(
		js_embed = "typed_array.copy_within",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_copy_within<T: JsCast>(
		array: &JsValue,
		target: f64,
		start: f64,
	) -> Result<T, JsValue>;

	#[js_sys(
		js_embed = "typed_array.copy_within_range",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_copy_within_range<T: JsCast>(
		array: &JsValue,
		target: f64,
		start: f64,
		end: f64,
	) -> Result<T, JsValue>;

	#[js_sys(js_embed = "typed_array.set")]
	fn typed_array_set(array: &JsValue, source: &JsValue) -> Result<(), JsValue>;

	#[js_sys(js_embed = "typed_array.set_with_offset")]
	fn typed_array_set_with_offset(
		array: &JsValue,
		source: &JsValue,
		offset: f64,
	) -> Result<(), JsValue>;

	#[js_sys(js_embed = "typed_array.property.constructor")]
	fn typed_array_constructor(array: &JsValue) -> Function;

	#[js_sys(js_embed = "typed_array.property.bytes_per_element")]
	fn typed_array_bytes_per_element(array: &JsValue) -> u32;

	#[js_sys(js_embed = "typed_array.species")]
	fn typed_array_species(constructor: &str) -> Result<Function, JsValue>;

	#[js_sys(js_embed = "typed_array.to_string_tag")]
	fn typed_array_to_string_tag(array: &JsValue) -> JsString;

	#[js_sys(js_embed = "typed_array.symbol_iterator")]
	fn typed_array_symbol_iterator(array: &JsValue) -> Result<JsIterator, JsValue>;

	#[js_sys(js_embed = "typed_array.entries")]
	fn typed_array_entries(array: &JsValue) -> Result<JsIterator<Array>, JsValue>;

	#[js_sys(js_embed = "typed_array.every")]
	fn typed_array_every(array: &JsValue, callback: &Function) -> Result<bool, JsValue>;

	#[js_sys(js_embed = "typed_array.every_this")]
	fn typed_array_every_with_this(
		array: &JsValue,
		callback: &Function,
		this: &JsValue,
	) -> Result<bool, JsValue>;

	#[js_sys(
		js_embed = "typed_array.filter",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_filter<T: JsCast>(array: &JsValue, callback: &Function) -> Result<T, JsValue>;

	#[js_sys(
		js_embed = "typed_array.filter_this",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_filter_with_this<T: JsCast>(
		array: &JsValue,
		callback: &Function,
		this: &JsValue,
	) -> Result<T, JsValue>;

	#[js_sys(js_embed = "typed_array.find_index")]
	fn typed_array_find_index(array: &JsValue, callback: &Function) -> Result<f64, JsValue>;

	#[js_sys(js_embed = "typed_array.find_index_this")]
	fn typed_array_find_index_with_this(
		array: &JsValue,
		callback: &Function,
		this: &JsValue,
	) -> Result<f64, JsValue>;

	#[js_sys(js_embed = "typed_array.find_last_index")]
	fn typed_array_find_last_index(array: &JsValue, callback: &Function) -> Result<f64, JsValue>;

	#[js_sys(js_embed = "typed_array.find_last_index_this")]
	fn typed_array_find_last_index_with_this(
		array: &JsValue,
		callback: &Function,
		this: &JsValue,
	) -> Result<f64, JsValue>;

	#[js_sys(js_embed = "typed_array.for_each")]
	fn typed_array_for_each(array: &JsValue, callback: &Function) -> Result<(), JsValue>;

	#[js_sys(js_embed = "typed_array.for_each_this")]
	fn typed_array_for_each_with_this(
		array: &JsValue,
		callback: &Function,
		this: &JsValue,
	) -> Result<(), JsValue>;

	#[js_sys(js_embed = "typed_array.join")]
	fn typed_array_join(array: &JsValue) -> Result<JsString, JsValue>;

	#[js_sys(js_embed = "typed_array.join_separator")]
	fn typed_array_join_with_separator(
		array: &JsValue,
		separator: &str,
	) -> Result<JsString, JsValue>;

	#[js_sys(js_embed = "typed_array.keys")]
	fn typed_array_keys(array: &JsValue) -> Result<JsIterator<Number<f64>>, JsValue>;

	#[js_sys(
		js_embed = "typed_array.map",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_map<T: JsCast>(array: &JsValue, callback: &Function) -> Result<T, JsValue>;

	#[js_sys(
		js_embed = "typed_array.map_this",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_map_with_this<T: JsCast>(
		array: &JsValue,
		callback: &Function,
		this: &JsValue,
	) -> Result<T, JsValue>;

	#[js_sys(js_embed = "typed_array.reduce")]
	fn typed_array_reduce(array: &JsValue, callback: &Function) -> Result<JsValue, JsValue>;

	#[js_sys(js_embed = "typed_array.reduce_initial")]
	fn typed_array_reduce_with_initial(
		array: &JsValue,
		callback: &Function,
		initial: &JsValue,
	) -> Result<JsValue, JsValue>;

	#[js_sys(js_embed = "typed_array.reduce_right")]
	fn typed_array_reduce_right(array: &JsValue, callback: &Function) -> Result<JsValue, JsValue>;

	#[js_sys(js_embed = "typed_array.reduce_right_initial")]
	fn typed_array_reduce_right_with_initial(
		array: &JsValue,
		callback: &Function,
		initial: &JsValue,
	) -> Result<JsValue, JsValue>;

	#[js_sys(
		js_embed = "typed_array.reverse",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_reverse<T: JsCast>(array: &JsValue) -> Result<T, JsValue>;

	#[js_sys(js_embed = "typed_array.some")]
	fn typed_array_some(array: &JsValue, callback: &Function) -> Result<bool, JsValue>;

	#[js_sys(js_embed = "typed_array.some_this")]
	fn typed_array_some_with_this(
		array: &JsValue,
		callback: &Function,
		this: &JsValue,
	) -> Result<bool, JsValue>;

	#[js_sys(
		js_embed = "typed_array.sort",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_sort<T: JsCast>(array: &JsValue) -> Result<T, JsValue>;

	#[js_sys(
		js_embed = "typed_array.sort_by",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_sort_by<T: JsCast>(array: &JsValue, callback: &Function) -> Result<T, JsValue>;

	#[js_sys(js_embed = "typed_array.to_locale_string")]
	fn typed_array_to_locale_string(array: &JsValue) -> Result<JsString, JsValue>;

	#[js_sys(js_embed = "typed_array.to_locale_string_locales")]
	fn typed_array_to_locale_string_with_locales(
		array: &JsValue,
		locales: &JsValue,
	) -> Result<JsString, JsValue>;

	#[js_sys(js_embed = "typed_array.to_locale_string_options")]
	fn typed_array_to_locale_string_with_options(
		array: &JsValue,
		locales: &JsValue,
		options: &JsValue,
	) -> Result<JsString, JsValue>;

	#[js_sys(
		js_embed = "typed_array.to_reversed",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_to_reversed<T: JsCast>(array: &JsValue) -> Result<T, JsValue>;

	#[js_sys(
		js_embed = "typed_array.to_sorted",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_to_sorted<T: JsCast>(array: &JsValue) -> Result<T, JsValue>;

	#[js_sys(
		js_embed = "typed_array.to_sorted_by",
		return_abi = Result<JsValue, JsValue>
	)]
	fn typed_array_to_sorted_by<T: JsCast>(
		array: &JsValue,
		callback: &Function,
	) -> Result<T, JsValue>;

	#[js_sys(js_embed = "typed_array.to_string")]
	fn typed_array_to_string(array: &JsValue) -> Result<JsString, JsValue>;

	#[js_sys(js_embed = "typed_array.values")]
	fn typed_array_values(array: &JsValue) -> Result<JsIterator, JsValue>;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.brand",
	"(() => {{",
	"    const prototype = Object.getPrototypeOf(Uint8Array.prototype)",
	"    const brand = Object.getOwnPropertyDescriptor(prototype, Symbol.toStringTag).get",
	"    return (source, result) => {{",
	"        const sourceBrand = brand.call(source)",
	"        if (sourceBrand === undefined || sourceBrand !== brand.call(result))",
	"            throw new TypeError('typed array species changed the element type')",
	"        return result",
	"    }}",
	"}})()",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.slice",
	required_embeds = [("js_sys", "typed_array.brand")],
	"(array) => this.#jsEmbed.js_sys['typed_array.brand'](array, array.slice())",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.slice_from",
	required_embeds = [("js_sys", "typed_array.brand")],
	"(array, begin) => this.#jsEmbed.js_sys['typed_array.brand'](",
	"    array, array.slice(begin)",
	")",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.slice_range",
	required_embeds = [("js_sys", "typed_array.brand")],
	"(array, begin, end) => this.#jsEmbed.js_sys['typed_array.brand'](",
	"    array, array.slice(begin, end)",
	")",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.subarray",
	required_embeds = [("js_sys", "typed_array.brand")],
	"(array) => this.#jsEmbed.js_sys['typed_array.brand'](array, array.subarray())",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.subarray_from",
	required_embeds = [("js_sys", "typed_array.brand")],
	"(array, begin) => this.#jsEmbed.js_sys['typed_array.brand'](",
	"    array, array.subarray(begin)",
	")",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.subarray_range",
	required_embeds = [("js_sys", "typed_array.brand")],
	"(array, begin, end) => this.#jsEmbed.js_sys['typed_array.brand'](",
	"    array, array.subarray(begin, end)",
	")",
);

macro_rules! typed_array_embed {
	($name:tt, $source:tt) => {
		js_bindgen::embed_js!(module = "js_sys", name = $name, $source);
	};
}

macro_rules! typed_array_brand_embed {
	($name:tt, $source:tt) => {
		js_bindgen::embed_js!(
			module = "js_sys",
			name = $name,
			required_embeds = [("js_sys", "typed_array.brand")],
			$source,
		);
	};
}

typed_array_embed!("typed_array.property.buffer", "(array) => array.buffer");
typed_array_embed!(
	"typed_array.property.byte_length",
	"(array) => array.byteLength"
);
typed_array_embed!(
	"typed_array.property.byte_offset",
	"(array) => array.byteOffset"
);
typed_array_embed!("typed_array.property.length", "(array) => array.length");
typed_array_embed!(
	"typed_array.copy_within",
	"(array, target, start) => array.copyWithin(target, start)"
);
typed_array_embed!(
	"typed_array.copy_within_range",
	"(array, target, start, end) => array.copyWithin(target, start, end)"
);
typed_array_embed!("typed_array.set", "(array, source) => array.set(source)");
typed_array_embed!(
	"typed_array.set_with_offset",
	"(array, source, offset) => array.set(source, offset)"
);
typed_array_embed!(
	"typed_array.property.constructor",
	"(array) => array.constructor"
);
typed_array_embed!(
	"typed_array.property.bytes_per_element",
	"(array) => array.BYTES_PER_ELEMENT"
);

typed_array_embed!(
	"typed_array.to_string_tag",
	"(array) => array[Symbol.toStringTag]"
);
js_bindgen::embed_js!(
	module = "js_sys",
	name = "typed_array.species",
	"(constructor) => {{",
	"    const value = globalThis[constructor]",
	"    if (typeof value !== 'function')",
	"        throw new TypeError(`${{constructor}} is not available`)",
	"    const species = value[Symbol.species]",
	"    if (typeof species !== 'function')",
	"        throw new TypeError(`${{constructor}} does not provide Symbol.species`)",
	"    return species",
	"}}",
);
typed_array_embed!(
	"typed_array.symbol_iterator",
	"(array) => array[Symbol.iterator]()"
);
typed_array_embed!("typed_array.entries", "(array) => array.entries()");
typed_array_embed!(
	"typed_array.every",
	"(array, callback) => array.every(callback)"
);
typed_array_embed!(
	"typed_array.every_this",
	"(array, callback, thisArg) => array.every(callback, thisArg)"
);
typed_array_brand_embed!(
	"typed_array.filter",
	"(array, callback) => this.#jsEmbed.js_sys['typed_array.brand'](array, array.filter(callback))"
);
typed_array_brand_embed!(
	"typed_array.filter_this",
	"(array, callback, thisArg) => this.#jsEmbed.js_sys['typed_array.brand'](array, \
	 array.filter(callback, thisArg))"
);
typed_array_embed!(
	"typed_array.find_index",
	"(array, callback) => array.findIndex(callback)"
);
typed_array_embed!(
	"typed_array.find_index_this",
	"(array, callback, thisArg) => array.findIndex(callback, thisArg)"
);
typed_array_embed!(
	"typed_array.find_last_index",
	"(array, callback) => array.findLastIndex(callback)"
);
typed_array_embed!(
	"typed_array.find_last_index_this",
	"(array, callback, thisArg) => array.findLastIndex(callback, thisArg)"
);
typed_array_embed!(
	"typed_array.for_each",
	"(array, callback) => array.forEach(callback)"
);
typed_array_embed!(
	"typed_array.for_each_this",
	"(array, callback, thisArg) => array.forEach(callback, thisArg)"
);
typed_array_embed!("typed_array.join", "(array) => array.join()");
typed_array_embed!(
	"typed_array.join_separator",
	"(array, separator) => array.join(separator)"
);
typed_array_embed!("typed_array.keys", "(array) => array.keys()");
typed_array_brand_embed!(
	"typed_array.map",
	"(array, callback) => this.#jsEmbed.js_sys['typed_array.brand'](array, array.map(callback))"
);
typed_array_brand_embed!(
	"typed_array.map_this",
	"(array, callback, thisArg) => this.#jsEmbed.js_sys['typed_array.brand'](array, \
	 array.map(callback, thisArg))"
);
typed_array_embed!(
	"typed_array.reduce",
	"(array, callback) => array.reduce(callback)"
);
typed_array_embed!(
	"typed_array.reduce_initial",
	"(array, callback, initial) => array.reduce(callback, initial)"
);
typed_array_embed!(
	"typed_array.reduce_right",
	"(array, callback) => array.reduceRight(callback)"
);
typed_array_embed!(
	"typed_array.reduce_right_initial",
	"(array, callback, initial) => array.reduceRight(callback, initial)"
);
typed_array_embed!("typed_array.reverse", "(array) => array.reverse()");
typed_array_embed!(
	"typed_array.some",
	"(array, callback) => array.some(callback)"
);
typed_array_embed!(
	"typed_array.some_this",
	"(array, callback, thisArg) => array.some(callback, thisArg)"
);
typed_array_embed!("typed_array.sort", "(array) => array.sort()");
typed_array_embed!(
	"typed_array.sort_by",
	"(array, callback) => array.sort(callback)"
);
typed_array_embed!(
	"typed_array.to_locale_string",
	"(array) => array.toLocaleString()"
);
typed_array_embed!(
	"typed_array.to_locale_string_locales",
	"(array, locales) => array.toLocaleString(locales)"
);
typed_array_embed!(
	"typed_array.to_locale_string_options",
	"(array, locales, options) => array.toLocaleString(locales, options)"
);
typed_array_brand_embed!(
	"typed_array.to_reversed",
	"(array) => this.#jsEmbed.js_sys['typed_array.brand'](array, array.toReversed())"
);
typed_array_brand_embed!(
	"typed_array.to_sorted",
	"(array) => this.#jsEmbed.js_sys['typed_array.brand'](array, array.toSorted())"
);
typed_array_brand_embed!(
	"typed_array.to_sorted_by",
	"(array, callback) => this.#jsEmbed.js_sys['typed_array.brand'](array, \
	 array.toSorted(callback))"
);
typed_array_embed!("typed_array.to_string", "(array) => array.toString()");
typed_array_embed!("typed_array.values", "(array) => array.values()");

macro_rules! typed_array_stable_api {
	(@normal $name:ident : $value:ty, constructor = $constructor:literal) => {
		typed_array_stable_api! {
			@impl $name: $value,
			constructor = $constructor,
			find = find,
			find_with_this = find_with_this,
			find_last = find_last,
			find_last_with_this = find_last_with_this,
			includes = includes,
			includes_from = includes_from,
			index_of = index_of,
			index_of_from = index_of_from,
			last_index_of = last_index_of,
			last_index_of_from = last_index_of_from,
			with = with,
		}
	};
	(@float16 $name:ident : $value:ty, constructor = $constructor:literal) => {
		typed_array_stable_api! {
			@impl $name: $value,
			constructor = $constructor,
			find = find_as_f32,
			find_with_this = find_as_f32_with_this,
			find_last = find_last_as_f32,
			find_last_with_this = find_last_as_f32_with_this,
			includes = includes_f32,
			includes_from = includes_f32_from,
			index_of = index_of_f32,
			index_of_from = index_of_f32_from,
			last_index_of = last_index_of_f32,
			last_index_of_from = last_index_of_f32_from,
			with = with_f32,
		}
	};
	(
		@impl $name:ident : $value:ty,
		constructor = $constructor:literal,
		find = $find:ident,
		find_with_this = $find_with_this:ident,
		find_last = $find_last:ident,
		find_last_with_this = $find_last_with_this:ident,
		includes = $includes:ident,
		includes_from = $includes_from:ident,
		index_of = $index_of:ident,
		index_of_from = $index_of_from:ident,
		last_index_of = $last_index_of:ident,
		last_index_of_from = $last_index_of_from:ident,
		with = $with:ident,
	) => {
		#[js_sys(js_sys = crate)]
		extern "js-sys" {
			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/from)
			#[js_sys(static_of = $name, js_name = "from")]
			pub fn from_value(value: &JsValue) -> Result<$name, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/from)
			#[js_sys(static_of = $name, js_name = "from")]
			pub fn from_value_with_map(value: &JsValue, map: &Function) -> Result<$name, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/from)
			#[js_sys(static_of = $name, js_name = "from")]
			pub fn from_value_with_map_and_this(
				value: &JsValue,
				map: &Function,
				this: &JsValue,
			) -> Result<$name, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/of)
			#[js_sys(static_of = $name, variadic)]
			pub fn of(values: &[JsValue]) -> Result<$name, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/find)
			#[js_sys(js_name = "find")]
			pub fn $find(self: &$name, callback: &Function) -> Result<Option<$value>, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/find)
			#[js_sys(js_name = "find")]
			pub fn $find_with_this(
				self: &$name,
				callback: &Function,
				this: &JsValue,
			) -> Result<Option<$value>, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/findLast)
			#[js_sys(js_name = "findLast")]
			pub fn $find_last(self: &$name, callback: &Function)
			-> Result<Option<$value>, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/findLast)
			#[js_sys(js_name = "findLast")]
			pub fn $find_last_with_this(
				self: &$name,
				callback: &Function,
				this: &JsValue,
			) -> Result<Option<$value>, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/includes)
			#[js_sys(js_name = "includes")]
			pub fn $includes(self: &$name, value: $value) -> Result<bool, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/includes)
			#[js_sys(js_name = "includes")]
			pub fn $includes_from(
				self: &$name,
				value: $value,
				from_index: f64,
			) -> Result<bool, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/indexOf)
			#[js_sys(js_name = "indexOf")]
			pub fn $index_of(self: &$name, value: $value) -> Result<f64, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/indexOf)
			#[js_sys(js_name = "indexOf")]
			pub fn $index_of_from(
				self: &$name,
				value: $value,
				from_index: f64,
			) -> Result<f64, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/lastIndexOf)
			#[js_sys(js_name = "lastIndexOf")]
			pub fn $last_index_of(self: &$name, value: $value) -> Result<f64, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/lastIndexOf)
			#[js_sys(js_name = "lastIndexOf")]
			pub fn $last_index_of_from(
				self: &$name,
				value: $value,
				from_index: f64,
			) -> Result<f64, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/with)
			#[js_sys(js_name = "with")]
			pub fn $with(self: &$name, index: f64, value: $value) -> Result<$name, JsValue>;
		}

		impl $name {
			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/Symbol.species)
			pub fn species() -> Result<Function, JsValue> {
				typed_array_species($constructor)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/constructor)
			#[must_use]
			#[inline]
			pub fn constructor(&self) -> Function {
				typed_array_constructor(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/BYTES_PER_ELEMENT)
			#[must_use]
			#[inline]
			pub fn bytes_per_element(&self) -> u32 {
				typed_array_bytes_per_element(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/buffer)
			#[must_use]
			#[inline]
			pub fn buffer(&self) -> JsValue {
				typed_array_buffer(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/byteLength)
			#[must_use]
			#[inline]
			pub fn byte_length(&self) -> f64 {
				typed_array_byte_length(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/byteOffset)
			#[must_use]
			#[inline]
			pub fn byte_offset(&self) -> f64 {
				typed_array_byte_offset(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/length)
			#[must_use]
			#[inline]
			pub fn length(&self) -> f64 {
				typed_array_length(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/copyWithin)
			#[inline]
			pub fn copy_within(&self, target: f64, start: f64) -> Result<Self, JsValue> {
				typed_array_copy_within(self.unchecked_as_ref(), target, start)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/copyWithin)
			#[inline]
			pub fn copy_within_range(
				&self,
				target: f64,
				start: f64,
				end: f64,
			) -> Result<Self, JsValue> {
				typed_array_copy_within_range(self.unchecked_as_ref(), target, start, end)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/set)
			#[inline]
			pub fn set(&self, source: &JsValue) -> Result<(), JsValue> {
				typed_array_set(self.unchecked_as_ref(), source)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/set)
			#[inline]
			pub fn set_with_offset(&self, source: &JsValue, offset: f64) -> Result<(), JsValue> {
				typed_array_set_with_offset(self.unchecked_as_ref(), source, offset)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/Symbol.toStringTag)
			#[must_use]
			pub fn symbol_to_string_tag(&self) -> JsString {
				typed_array_to_string_tag(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/Symbol.iterator)
			pub fn symbol_iterator(&self) -> Result<JsIterator, JsValue> {
				typed_array_symbol_iterator(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/entries)
			pub fn entries(&self) -> Result<JsIterator<Array>, JsValue> {
				typed_array_entries(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/every)
			pub fn every(&self, callback: &Function) -> Result<bool, JsValue> {
				typed_array_every(self.unchecked_as_ref(), callback)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/every)
			pub fn every_with_this(
				&self,
				callback: &Function,
				this: &JsValue,
			) -> Result<bool, JsValue> {
				typed_array_every_with_this(self.unchecked_as_ref(), callback, this)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/filter)
			pub fn filter(&self, callback: &Function) -> Result<Self, JsValue> {
				typed_array_filter(self.unchecked_as_ref(), callback)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/filter)
			pub fn filter_with_this(
				&self,
				callback: &Function,
				this: &JsValue,
			) -> Result<Self, JsValue> {
				typed_array_filter_with_this(self.unchecked_as_ref(), callback, this)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/findIndex)
			pub fn find_index(&self, callback: &Function) -> Result<f64, JsValue> {
				typed_array_find_index(self.unchecked_as_ref(), callback)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/findIndex)
			pub fn find_index_with_this(
				&self,
				callback: &Function,
				this: &JsValue,
			) -> Result<f64, JsValue> {
				typed_array_find_index_with_this(self.unchecked_as_ref(), callback, this)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/findLastIndex)
			pub fn find_last_index(&self, callback: &Function) -> Result<f64, JsValue> {
				typed_array_find_last_index(self.unchecked_as_ref(), callback)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/findLastIndex)
			pub fn find_last_index_with_this(
				&self,
				callback: &Function,
				this: &JsValue,
			) -> Result<f64, JsValue> {
				typed_array_find_last_index_with_this(self.unchecked_as_ref(), callback, this)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/forEach)
			pub fn for_each(&self, callback: &Function) -> Result<(), JsValue> {
				typed_array_for_each(self.unchecked_as_ref(), callback)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/forEach)
			pub fn for_each_with_this(
				&self,
				callback: &Function,
				this: &JsValue,
			) -> Result<(), JsValue> {
				typed_array_for_each_with_this(self.unchecked_as_ref(), callback, this)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/join)
			pub fn join(&self) -> Result<JsString, JsValue> {
				typed_array_join(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/join)
			pub fn join_with_separator(&self, separator: &str) -> Result<JsString, JsValue> {
				typed_array_join_with_separator(self.unchecked_as_ref(), separator)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/keys)
			pub fn keys(&self) -> Result<JsIterator<Number<f64>>, JsValue> {
				typed_array_keys(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/map)
			pub fn map(&self, callback: &Function) -> Result<Self, JsValue> {
				typed_array_map(self.unchecked_as_ref(), callback)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/map)
			pub fn map_with_this(
				&self,
				callback: &Function,
				this: &JsValue,
			) -> Result<Self, JsValue> {
				typed_array_map_with_this(self.unchecked_as_ref(), callback, this)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/reduce)
			pub fn reduce(&self, callback: &Function) -> Result<JsValue, JsValue> {
				typed_array_reduce(self.unchecked_as_ref(), callback)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/reduce)
			pub fn reduce_with_initial(
				&self,
				callback: &Function,
				initial: &JsValue,
			) -> Result<JsValue, JsValue> {
				typed_array_reduce_with_initial(self.unchecked_as_ref(), callback, initial)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/reduceRight)
			pub fn reduce_right(&self, callback: &Function) -> Result<JsValue, JsValue> {
				typed_array_reduce_right(self.unchecked_as_ref(), callback)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/reduceRight)
			pub fn reduce_right_with_initial(
				&self,
				callback: &Function,
				initial: &JsValue,
			) -> Result<JsValue, JsValue> {
				typed_array_reduce_right_with_initial(self.unchecked_as_ref(), callback, initial)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/reverse)
			pub fn reverse(&self) -> Result<Self, JsValue> {
				typed_array_reverse(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/some)
			pub fn some(&self, callback: &Function) -> Result<bool, JsValue> {
				typed_array_some(self.unchecked_as_ref(), callback)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/some)
			pub fn some_with_this(
				&self,
				callback: &Function,
				this: &JsValue,
			) -> Result<bool, JsValue> {
				typed_array_some_with_this(self.unchecked_as_ref(), callback, this)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/sort)
			pub fn sort(&self) -> Result<Self, JsValue> {
				typed_array_sort(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/sort)
			pub fn sort_by(&self, callback: &Function) -> Result<Self, JsValue> {
				typed_array_sort_by(self.unchecked_as_ref(), callback)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/toLocaleString)
			pub fn to_locale_string(&self) -> Result<JsString, JsValue> {
				typed_array_to_locale_string(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/toLocaleString)
			pub fn to_locale_string_with_locales(
				&self,
				locales: &JsValue,
			) -> Result<JsString, JsValue> {
				typed_array_to_locale_string_with_locales(self.unchecked_as_ref(), locales)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/toLocaleString)
			pub fn to_locale_string_with_options(
				&self,
				locales: &JsValue,
				options: &JsValue,
			) -> Result<JsString, JsValue> {
				typed_array_to_locale_string_with_options(self.unchecked_as_ref(), locales, options)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/toReversed)
			pub fn to_reversed(&self) -> Result<Self, JsValue> {
				typed_array_to_reversed(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/toSorted)
			pub fn to_sorted(&self) -> Result<Self, JsValue> {
				typed_array_to_sorted(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/toSorted)
			pub fn to_sorted_by(&self, callback: &Function) -> Result<Self, JsValue> {
				typed_array_to_sorted_by(self.unchecked_as_ref(), callback)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/toString)
			pub fn to_string(&self) -> Result<JsString, JsValue> {
				typed_array_to_string(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/values)
			pub fn values(&self) -> Result<JsIterator, JsValue> {
				typed_array_values(self.unchecked_as_ref())
			}
		}

		impl Iterable for $name {
			type Item = JsValue;
		}
	};
}

macro_rules! typed_array {
	(
		$name:ident : $element:ty,
		constructor = $constructor:literal,
		mdn = $mdn:literal,
	) => {
		#[js_sys(js_sys = crate)]
		extern "js-sys" {
			#[doc = "[`MDN` documentation]("]
			#[doc = $mdn]
			#[doc = ")"]
			#[js_sys(js_name = $constructor, extends = Object)]
			#[derive(Clone, Debug, PartialEq)]
			pub type $name;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/TypedArray)
			#[js_sys(constructor)]
			pub fn new(value: &JsValue) -> Result<$name, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/TypedArray)
			#[js_sys(constructor)]
			pub fn new_with_length(
				length: f64,
			) -> Result<$name, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/TypedArray)
			#[js_sys(constructor)]
			pub fn new_with_byte_offset(
				buffer: &JsValue,
				byte_offset: f64,
			) -> Result<$name, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/TypedArray)
			#[js_sys(constructor)]
			pub fn new_with_byte_offset_and_length(
				buffer: &JsValue,
				byte_offset: f64,
				length: f64,
			) -> Result<$name, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/at)
			pub fn at(
				self: &$name,
				index: f64,
			) -> Result<Option<$element>, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill)
			pub fn fill(self: &$name, value: $element) -> Result<$name, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill)
			#[js_sys(js_name = "fill")]
			pub fn fill_from(
				self: &$name,
				value: $element,
				start: f64,
			) -> Result<$name, JsValue>;

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill)
			#[js_sys(js_name = "fill")]
			pub fn fill_range(
				self: &$name,
				value: $element,
				start: f64,
				end: f64,
			) -> Result<$name, JsValue>;
		}

		typed_array_stable_api!(@normal $name: $element, constructor = $constructor);

		#[js_sys(js_sys = crate)]
		extern "js-sys" {
			#[must_use]
			#[js_sys(indexing_getter)]
			pub fn get(
				self: &$name,
				index: f64,
			) -> Option<$element>;

			#[js_sys(indexing_setter)]
			pub fn set_index(
				self: &$name,
				index: f64,
				value: $element,
			);
		}

		impl $name {
			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/slice)
			#[inline]
			pub fn slice(&self) -> Result<Self, JsValue> {
				typed_array_slice(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/slice)
			#[inline]
			pub fn slice_from(&self, begin: f64) -> Result<Self, JsValue> {
				typed_array_slice_from(self.unchecked_as_ref(), begin)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/slice)
			#[inline]
			pub fn slice_range(&self, begin: f64, end: f64) -> Result<Self, JsValue> {
				typed_array_slice_range(self.unchecked_as_ref(), begin, end)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/subarray)
			#[inline]
			pub fn subarray(&self) -> Result<Self, JsValue> {
				typed_array_subarray(self.unchecked_as_ref())
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/subarray)
			#[inline]
			pub fn subarray_from(&self, begin: f64) -> Result<Self, JsValue> {
				typed_array_subarray_from(self.unchecked_as_ref(), begin)
			}

			/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/subarray)
			#[inline]
			pub fn subarray_range(&self, begin: f64, end: f64) -> Result<Self, JsValue> {
				typed_array_subarray_range(self.unchecked_as_ref(), begin, end)
			}
		}
	};
}

typed_array! {
	Int8Array: i8,
	constructor = "Int8Array",
	mdn = "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Int8Array",
}

typed_array! {
	Uint8Array: u8,
	constructor = "Uint8Array",
	mdn = "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array",
}

typed_array! {
	Uint8ClampedArray: u8,
	constructor = "Uint8ClampedArray",
	mdn = "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8ClampedArray",
}

typed_array! {
	Int16Array: i16,
	constructor = "Int16Array",
	mdn = "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Int16Array",
}

typed_array! {
	Uint16Array: u16,
	constructor = "Uint16Array",
	mdn = "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint16Array",
}

// Stable Rust does not have `f16`. Scalar APIs therefore use `f32`, while
// bulk APIs preserve the raw `IEEE 754 binary16` representation in `u16`
// values.
#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Float16Array)
	#[js_sys(js_name = "Float16Array", extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type Float16Array;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/TypedArray)
	#[js_sys(constructor)]
	pub fn new(value: &JsValue) -> Result<Float16Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/TypedArray)
	#[js_sys(constructor)]
	pub fn new_with_length(length: f64) -> Result<Float16Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/TypedArray)
	#[js_sys(constructor)]
	pub fn new_with_byte_offset(
		buffer: &JsValue,
		byte_offset: f64,
	) -> Result<Float16Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/TypedArray)
	#[js_sys(constructor)]
	pub fn new_with_byte_offset_and_length(
		buffer: &JsValue,
		byte_offset: f64,
		length: f64,
	) -> Result<Float16Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/at)
	#[js_sys(js_name = "at")]
	pub fn at_as_f32(self: &Float16Array, index: f64) -> Result<Option<f32>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill)
	#[js_sys(js_name = "fill")]
	pub fn fill_with_f32(self: &Float16Array, value: f32) -> Result<Float16Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill)
	#[js_sys(js_name = "fill")]
	pub fn fill_with_f32_from(
		self: &Float16Array,
		value: f32,
		start: f64,
	) -> Result<Float16Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/fill)
	#[js_sys(js_name = "fill")]
	pub fn fill_with_f32_range(
		self: &Float16Array,
		value: f32,
		start: f64,
		end: f64,
	) -> Result<Float16Array, JsValue>;
}

typed_array_stable_api!(@float16 Float16Array: f32, constructor = "Float16Array");

#[js_sys(js_sys = crate)]
extern "js-sys" {
	#[must_use]
	#[js_sys(indexing_getter)]
	pub fn get_as_f32(self: &Float16Array, index: f64) -> Option<f32>;

	#[js_sys(indexing_setter)]
	pub fn set_index_from_f32(self: &Float16Array, index: f64, value: f32);
}

impl Float16Array {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/slice)
	#[inline]
	pub fn slice(&self) -> Result<Self, JsValue> {
		typed_array_slice(self.unchecked_as_ref())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/slice)
	#[inline]
	pub fn slice_from(&self, begin: f64) -> Result<Self, JsValue> {
		typed_array_slice_from(self.unchecked_as_ref(), begin)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/slice)
	#[inline]
	pub fn slice_range(&self, begin: f64, end: f64) -> Result<Self, JsValue> {
		typed_array_slice_range(self.unchecked_as_ref(), begin, end)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/subarray)
	#[inline]
	pub fn subarray(&self) -> Result<Self, JsValue> {
		typed_array_subarray(self.unchecked_as_ref())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/subarray)
	#[inline]
	pub fn subarray_from(&self, begin: f64) -> Result<Self, JsValue> {
		typed_array_subarray_from(self.unchecked_as_ref(), begin)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/subarray)
	#[inline]
	pub fn subarray_range(&self, begin: f64, end: f64) -> Result<Self, JsValue> {
		typed_array_subarray_range(self.unchecked_as_ref(), begin, end)
	}
}

typed_array! {
	Int32Array: i32,
	constructor = "Int32Array",
	mdn = "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Int32Array",
}

typed_array! {
	Uint32Array: u32,
	constructor = "Uint32Array",
	mdn = "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint32Array",
}

typed_array! {
	Float32Array: f32,
	constructor = "Float32Array",
	mdn = "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Float32Array",
}

typed_array! {
	Float64Array: f64,
	constructor = "Float64Array",
	mdn = "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Float64Array",
}

typed_array! {
	BigInt64Array: i64,
	constructor = "BigInt64Array",
	mdn = "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt64Array",
}

typed_array! {
	BigUint64Array: u64,
	constructor = "BigUint64Array",
	mdn = "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigUint64Array",
}
