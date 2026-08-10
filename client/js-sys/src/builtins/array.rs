use core::fmt::{self, Formatter};

use super::{Function, Iterable, JsIterator, Number, Object, Promise};
use crate::JsValue;
use crate::hazard::JsCast;

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array)
	///
	/// `T` is an unchecked marker for the intended element type; JavaScript
	/// arrays remain dynamic and may contain holes or values of another type.
	#[js_sys(js_name = "Array", extends = Object)]
	pub type Array<T = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Array)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new() -> Array;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Array)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_length(length: u32) -> Array;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Array)
	#[must_use]
	#[js_sys(constructor, return_abi = Array)]
	pub fn new_typed<T>() -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Array)
	#[must_use]
	#[js_sys(constructor, return_abi = Array)]
	pub fn new_typed_with_length<T>(length: u32) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/from)
	#[js_sys(static_of = Array, js_name = "from")]
	pub fn from_value(value: &JsValue) -> Result<Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/from)
	#[js_sys(static_of = Array, js_name = "from")]
	pub fn from_value_with_map(value: &JsValue, map: &Function) -> Result<Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/from)
	#[js_sys(static_of = Array, js_name = "from")]
	pub fn from_value_with_map_and_this(
		value: &JsValue,
		map: &Function,
		this: &JsValue,
	) -> Result<Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/fromAsync)
	#[js_sys(static_of = Array, js_name = "fromAsync")]
	pub fn from_async(value: &JsValue) -> Result<Promise<Array>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/fromAsync)
	#[js_sys(static_of = Array, js_name = "fromAsync")]
	pub fn from_async_with_map(value: &JsValue, map: &Function) -> Result<Promise<Array>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/fromAsync)
	#[js_sys(static_of = Array, js_name = "fromAsync")]
	pub fn from_async_with_map_and_this(
		value: &JsValue,
		map: &Function,
		this: &JsValue,
	) -> Result<Promise<Array>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/isArray)
	#[must_use]
	#[js_sys(static_of = Array, js_name = "isArray")]
	pub fn is_array(value: &JsValue) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/of)
	#[must_use]
	#[js_sys(static_of = Array<T>, variadic, return_abi = Array)]
	pub fn of<T: JsCast>(#[js_sys(type = &[JsValue])] values: &[T]) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/constructor)
	#[must_use]
	#[js_sys(getter)]
	pub fn constructor<T>(self: &Array<T>) -> Function;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/length)
	#[must_use]
	#[js_sys(getter)]
	pub fn length<T>(self: &Array<T>) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/length)
	#[js_sys(setter)]
	pub fn set_length<T>(self: &Array<T>, length: u32);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/at)
	#[must_use]
	pub fn at<T>(self: &Array<T>, index: f64) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/concat)
	#[must_use]
	#[js_sys(return_abi = Array)]
	pub fn concat<T>(self: &Array<T>, #[js_sys(type = &JsValue)] value: &Array<T>) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/concat)
	#[must_use]
	#[js_sys(js_name = "concat", variadic, return_abi = Array)]
	pub fn concat_many<T>(
		self: &Array<T>,
		#[js_sys(type = &[JsValue])] values: &[Array<T>],
	) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/copyWithin)
	#[must_use]
	#[js_sys(js_name = "copyWithin", return_abi = Array)]
	pub fn copy_within<T>(self: &Array<T>, target: f64, start: f64) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/copyWithin)
	#[must_use]
	#[js_sys(js_name = "copyWithin", return_abi = Array)]
	pub fn copy_within_range<T>(self: &Array<T>, target: f64, start: f64, end: f64) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/entries)
	#[must_use]
	pub fn entries<T>(self: &Array<T>) -> JsIterator<Array>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every)
	pub fn every<T>(self: &Array<T>, callback: &Function) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/every)
	#[js_sys(js_name = "every")]
	pub fn every_with_this<T>(
		self: &Array<T>,
		callback: &Function,
		this: &JsValue,
	) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/fill)
	#[must_use]
	#[js_sys(return_abi = Array)]
	pub fn fill<T: JsCast>(self: &Array<T>, #[js_sys(type = &JsValue)] value: &T) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/fill)
	#[must_use]
	#[js_sys(js_name = "fill", return_abi = Array)]
	pub fn fill_from<T: JsCast>(
		self: &Array<T>,
		#[js_sys(type = &JsValue)] value: &T,
		start: f64,
	) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/fill)
	#[must_use]
	#[js_sys(js_name = "fill", return_abi = Array)]
	pub fn fill_range<T: JsCast>(
		self: &Array<T>,
		#[js_sys(type = &JsValue)] value: &T,
		start: f64,
		end: f64,
	) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter)
	#[js_sys(return_abi = Result<Array, JsValue>)]
	pub fn filter<T>(self: &Array<T>, callback: &Function) -> Result<Array<T>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/filter)
	#[js_sys(js_name = "filter", return_abi = Result<Array, JsValue>)]
	pub fn filter_with_this<T>(
		self: &Array<T>,
		callback: &Function,
		this: &JsValue,
	) -> Result<Array<T>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find)
	pub fn find<T>(self: &Array<T>, callback: &Function) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/find)
	#[js_sys(js_name = "find")]
	pub fn find_with_this<T>(
		self: &Array<T>,
		callback: &Function,
		this: &JsValue,
	) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findIndex)
	#[js_sys(js_name = "findIndex")]
	pub fn find_index<T>(self: &Array<T>, callback: &Function) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findIndex)
	#[js_sys(js_name = "findIndex")]
	pub fn find_index_with_this<T>(
		self: &Array<T>,
		callback: &Function,
		this: &JsValue,
	) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findLast)
	#[js_sys(js_name = "findLast")]
	pub fn find_last<T>(self: &Array<T>, callback: &Function) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findLast)
	#[js_sys(js_name = "findLast")]
	pub fn find_last_with_this<T>(
		self: &Array<T>,
		callback: &Function,
		this: &JsValue,
	) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findLastIndex)
	#[js_sys(js_name = "findLastIndex")]
	pub fn find_last_index<T>(self: &Array<T>, callback: &Function) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/findLastIndex)
	#[js_sys(js_name = "findLastIndex")]
	pub fn find_last_index_with_this<T>(
		self: &Array<T>,
		callback: &Function,
		this: &JsValue,
	) -> Result<f64, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flat)
	#[must_use]
	pub fn flat<T>(self: &Array<T>) -> Array;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flat)
	#[must_use]
	#[js_sys(js_name = "flat")]
	pub fn flat_with_depth<T>(self: &Array<T>, depth: f64) -> Array;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flatMap)
	#[js_sys(js_name = "flatMap")]
	pub fn flat_map<T>(self: &Array<T>, callback: &Function) -> Result<Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flatMap)
	#[js_sys(js_name = "flatMap")]
	pub fn flat_map_with_this<T>(
		self: &Array<T>,
		callback: &Function,
		this: &JsValue,
	) -> Result<Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/forEach)
	#[js_sys(js_name = "forEach")]
	pub fn for_each<T>(self: &Array<T>, callback: &Function) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/forEach)
	#[js_sys(js_name = "forEach")]
	pub fn for_each_with_this<T>(
		self: &Array<T>,
		callback: &Function,
		this: &JsValue,
	) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/includes)
	#[must_use]
	pub fn includes<T: JsCast>(self: &Array<T>, #[js_sys(type = &JsValue)] value: &T) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/includes)
	#[must_use]
	#[js_sys(js_name = "includes")]
	pub fn includes_from<T: JsCast>(
		self: &Array<T>,
		#[js_sys(type = &JsValue)] value: &T,
		from_index: f64,
	) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/indexOf)
	#[must_use]
	#[js_sys(js_name = "indexOf")]
	pub fn index_of<T: JsCast>(self: &Array<T>, #[js_sys(type = &JsValue)] value: &T) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/indexOf)
	#[must_use]
	#[js_sys(js_name = "indexOf")]
	pub fn index_of_from<T: JsCast>(
		self: &Array<T>,
		#[js_sys(type = &JsValue)] value: &T,
		from_index: f64,
	) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/join)
	#[must_use]
	pub fn join<T>(self: &Array<T>) -> crate::JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/join)
	#[must_use]
	#[js_sys(js_name = "join")]
	pub fn join_with<T>(self: &Array<T>, separator: &str) -> crate::JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/keys)
	#[must_use]
	pub fn keys<T>(self: &Array<T>) -> JsIterator<Number<u32>>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/lastIndexOf)
	#[must_use]
	#[js_sys(js_name = "lastIndexOf")]
	pub fn last_index_of<T: JsCast>(self: &Array<T>, #[js_sys(type = &JsValue)] value: &T) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/lastIndexOf)
	#[must_use]
	#[js_sys(js_name = "lastIndexOf")]
	pub fn last_index_of_from<T: JsCast>(
		self: &Array<T>,
		#[js_sys(type = &JsValue)] value: &T,
		from_index: f64,
	) -> f64;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map)
	pub fn map<T>(self: &Array<T>, callback: &Function) -> Result<Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/map)
	#[js_sys(js_name = "map")]
	pub fn map_with_this<T>(
		self: &Array<T>,
		callback: &Function,
		this: &JsValue,
	) -> Result<Array, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/pop)
	#[must_use]
	pub fn pop<T>(self: &Array<T>) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/push)
	#[must_use]
	pub fn push<T: JsCast>(self: &Array<T>, #[js_sys(type = &JsValue)] value: &T) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/push)
	#[must_use]
	#[js_sys(js_name = "push", variadic)]
	pub fn push_many<T: JsCast>(self: &Array<T>, #[js_sys(type = &[JsValue])] values: &[T]) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce)
	pub fn reduce<T>(self: &Array<T>, callback: &Function) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduce)
	#[js_sys(js_name = "reduce")]
	pub fn reduce_with_initial<T>(
		self: &Array<T>,
		callback: &Function,
		initial: &JsValue,
	) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduceRight)
	#[js_sys(js_name = "reduceRight")]
	pub fn reduce_right<T>(self: &Array<T>, callback: &Function) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reduceRight)
	#[js_sys(js_name = "reduceRight")]
	pub fn reduce_right_with_initial<T>(
		self: &Array<T>,
		callback: &Function,
		initial: &JsValue,
	) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/reverse)
	#[must_use]
	#[js_sys(return_abi = Array)]
	pub fn reverse<T>(self: &Array<T>) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/shift)
	#[must_use]
	pub fn shift<T>(self: &Array<T>) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/slice)
	#[must_use]
	#[js_sys(return_abi = Array)]
	pub fn slice<T>(self: &Array<T>) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/slice)
	#[must_use]
	#[js_sys(js_name = "slice", return_abi = Array)]
	pub fn slice_from<T>(self: &Array<T>, start: f64) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/slice)
	#[must_use]
	#[js_sys(js_name = "slice", return_abi = Array)]
	pub fn slice_range<T>(self: &Array<T>, start: f64, end: f64) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some)
	pub fn some<T>(self: &Array<T>, callback: &Function) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/some)
	#[js_sys(js_name = "some")]
	pub fn some_with_this<T>(
		self: &Array<T>,
		callback: &Function,
		this: &JsValue,
	) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/sort)
	#[must_use]
	#[js_sys(return_abi = Array)]
	pub fn sort<T>(self: &Array<T>) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/sort)
	#[js_sys(js_name = "sort", return_abi = Result<Array, JsValue>)]
	pub fn sort_by<T>(self: &Array<T>, callback: &Function) -> Result<Array<T>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/splice)
	#[must_use]
	#[js_sys(return_abi = Array)]
	pub fn splice<T>(self: &Array<T>, start: f64) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/splice)
	#[must_use]
	#[js_sys(js_name = "splice", return_abi = Array)]
	pub fn splice_delete<T>(self: &Array<T>, start: f64, delete_count: f64) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/splice)
	#[must_use]
	#[js_sys(js_name = "splice", variadic, return_abi = Array)]
	pub fn splice_many<T: JsCast>(
		self: &Array<T>,
		start: f64,
		delete_count: f64,
		#[js_sys(type = &[JsValue])] values: &[T],
	) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toLocaleString)
	#[must_use]
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string<T>(self: &Array<T>) -> crate::JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toLocaleString)
	#[must_use]
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_locales<T>(self: &Array<T>, locales: &JsValue) -> crate::JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toLocaleString)
	#[must_use]
	#[js_sys(js_name = "toLocaleString")]
	pub fn to_locale_string_with_options<T>(
		self: &Array<T>,
		locales: &JsValue,
		options: &JsValue,
	) -> crate::JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toReversed)
	#[must_use]
	#[js_sys(js_name = "toReversed", return_abi = Array)]
	pub fn to_reversed<T>(self: &Array<T>) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSorted)
	#[must_use]
	#[js_sys(js_name = "toSorted", return_abi = Array)]
	pub fn to_sorted<T>(self: &Array<T>) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSorted)
	#[js_sys(js_name = "toSorted", return_abi = Result<Array, JsValue>)]
	pub fn to_sorted_by<T>(self: &Array<T>, callback: &Function) -> Result<Array<T>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSpliced)
	#[must_use]
	#[js_sys(js_name = "toSpliced", return_abi = Array)]
	pub fn to_spliced<T>(self: &Array<T>, start: f64) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSpliced)
	#[must_use]
	#[js_sys(js_name = "toSpliced", return_abi = Array)]
	pub fn to_spliced_delete<T>(self: &Array<T>, start: f64, delete_count: f64) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toSpliced)
	#[must_use]
	#[js_sys(js_name = "toSpliced", variadic, return_abi = Array)]
	pub fn to_spliced_many<T: JsCast>(
		self: &Array<T>,
		start: f64,
		delete_count: f64,
		#[js_sys(type = &[JsValue])] values: &[T],
	) -> Array<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string<T>(self: &Array<T>) -> crate::JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/unshift)
	#[must_use]
	pub fn unshift<T: JsCast>(self: &Array<T>, #[js_sys(type = &JsValue)] value: &T) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/unshift)
	#[must_use]
	#[js_sys(js_name = "unshift", variadic)]
	pub fn unshift_many<T: JsCast>(
		self: &Array<T>,
		#[js_sys(type = &[JsValue])] values: &[T],
	) -> u32;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/values)
	#[must_use]
	#[js_sys(return_abi = JsIterator)]
	pub fn values<T: JsCast>(self: &Array<T>) -> JsIterator<T>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/with)
	#[js_sys(js_name = "with", return_abi = Result<Array, JsValue>)]
	pub fn with<T: JsCast>(
		self: &Array<T>,
		index: f64,
		#[js_sys(type = &JsValue)] value: &T,
	) -> Result<Array<T>, JsValue>;
}

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[must_use]
	#[js_sys(indexing_getter)]
	pub fn get<T>(self: &Array<T>, index: u32) -> JsValue;

	#[must_use]
	#[js_sys(indexing_getter, return_abi = JsValue)]
	pub fn get_unchecked<T: JsCast>(self: &Array<T>, index: u32) -> T;

	#[js_sys(indexing_setter)]
	pub fn set<T: JsCast>(self: &Array<T>, index: u32, #[js_sys(type = &JsValue)] value: &T);

	#[js_sys(indexing_setter)]
	pub fn try_set<T: JsCast>(
		self: &Array<T>,
		index: u32,
		#[js_sys(type = &JsValue)] value: &T,
	) -> Result<(), JsValue>;

	#[must_use]
	#[js_sys(indexing_deleter)]
	pub fn delete<T>(self: &Array<T>, index: u32) -> bool;

	#[js_sys(indexing_deleter)]
	pub fn try_delete<T>(self: &Array<T>, index: u32) -> Result<bool, JsValue>;
}

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "array.species")]
	fn array_species() -> Function;

	#[js_sys(js_embed = "array.symbol_iterator")]
	fn array_symbol_iterator(array: &Array) -> JsIterator;

	#[js_sys(js_embed = "array.symbol_unscopables")]
	fn array_symbol_unscopables(array: &Array) -> JsValue;
}

impl<T> Clone for Array<T> {
	fn clone(&self) -> Self {
		Self::unchecked_from(<Self as AsRef<JsValue>>::as_ref(self).clone())
	}
}

impl<T> fmt::Debug for Array<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(<Self as AsRef<JsValue>>::as_ref(self), f)
	}
}

impl<T> Default for Array<T> {
	fn default() -> Self {
		Self::new_typed()
	}
}

impl<T: JsCast> Array<T> {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Symbol.iterator)
	#[must_use]
	pub fn symbol_iterator(&self) -> JsIterator<T> {
		JsIterator::unchecked_from(array_symbol_iterator(self.as_untyped()).into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Symbol.unscopables)
	#[must_use]
	pub fn symbol_unscopables(&self) -> JsValue {
		array_symbol_unscopables(self.as_untyped())
	}
}

impl<T> Array<T> {
	#[must_use]
	pub fn as_untyped(&self) -> &Array {
		Array::unchecked_from_ref(self.as_ref())
	}

	#[must_use]
	pub fn into_untyped(self) -> Array {
		Array::unchecked_from(self.into())
	}
}

impl Array {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/from)
	pub fn from_iterable<I: Iterable>(value: &I) -> Result<Array<I::Item>, JsValue> {
		let array = Self::from_value(value.as_ref())?;
		Ok(Array::unchecked_from(array.into()))
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/Symbol.species)
	#[must_use]
	pub fn species() -> Function {
		array_species()
	}
}

impl<T: JsCast> Iterable for Array<T> {
	type Item = T;
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "array.symbol_iterator",
	"(array) => array[Symbol.iterator]()",
);
js_bindgen::embed_js!(
	module = "js_sys",
	name = "array.species",
	"() => Array[Symbol.species]",
);
js_bindgen::embed_js!(
	module = "js_sys",
	name = "array.symbol_unscopables",
	"(array) => array[Symbol.unscopables]",
);
