use core::fmt::{self, Formatter};

use super::{Array, Function, JsString, Object, Promise};
use crate::JsValue;
use crate::hazard::JsCast;

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zip#mode)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IteratorZipMode {
	Shortest,
	Longest,
	Strict,
}

impl IteratorZipMode {
	const fn as_str(self) -> &'static str {
		match self {
			Self::Shortest => "shortest",
			Self::Longest => "longest",
			Self::Strict => "strict",
		}
	}
}

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator)
	#[js_sys(js_name = "Iterator", extends = Object)]
	pub type JsIterator<T = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncIterator)
	#[js_sys(extends = Object)]
	pub type AsyncIterator<T = JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug)]
	pub type IteratorResult;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zip#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type IteratorZipOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zipKeyed#options)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type IteratorZipKeyedOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/from)
	#[js_sys(static_of = JsIterator, js_name = "from")]
	pub fn from_value(value: &JsValue) -> Result<JsIterator, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/concat)
	#[js_sys(static_of = JsIterator, variadic)]
	pub fn concat(iterables: &[JsValue]) -> Result<JsIterator, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zip)
	#[js_sys(static_of = JsIterator)]
	pub fn zip<I: Iterable>(
		#[js_sys(type = &JsValue)] iterables: &I,
	) -> Result<JsIterator<Array>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zip)
	#[js_sys(
		static_of = JsIterator,
		js_name = "zip"
	)]
	pub fn zip_with_options<I: Iterable>(
		#[js_sys(type = &JsValue)] iterables: &I,
		options: &IteratorZipOptions,
	) -> Result<JsIterator<Array>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zipKeyed)
	#[js_sys(
		static_of = JsIterator,
		js_name = "zipKeyed"
	)]
	pub fn zip_keyed(iterables: &Object) -> Result<JsIterator<Object>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zipKeyed)
	#[js_sys(
		static_of = JsIterator,
		js_name = "zipKeyed"
	)]
	pub fn zip_keyed_with_options(
		iterables: &Object,
		options: &IteratorZipKeyedOptions,
	) -> Result<JsIterator<Object>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/drop)
	#[js_sys(js_name = "drop", return_abi = Result<JsIterator, JsValue>)]
	pub fn drop<T>(self: &JsIterator<T>, count: f64) -> Result<JsIterator<T>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/every)
	pub fn every<T>(self: &JsIterator<T>, callback: &Function) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/filter)
	#[js_sys(return_abi = Result<JsIterator, JsValue>)]
	pub fn filter<T>(self: &JsIterator<T>, callback: &Function) -> Result<JsIterator<T>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/find)
	pub fn find<T>(self: &JsIterator<T>, callback: &Function) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/flatMap)
	#[js_sys(js_name = "flatMap")]
	pub fn flat_map<T>(self: &JsIterator<T>, callback: &Function) -> Result<JsIterator, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/forEach)
	#[js_sys(js_name = "forEach")]
	pub fn for_each<T>(self: &JsIterator<T>, callback: &Function) -> Result<(), JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/includes)
	pub fn includes<T: JsCast>(
		self: &JsIterator<T>,
		#[js_sys(type = &JsValue)] search_element: &T,
	) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/includes)
	#[js_sys(js_name = "includes")]
	pub fn includes_from<T: JsCast>(
		self: &JsIterator<T>,
		#[js_sys(type = &JsValue)] search_element: &T,
		from_index: f64,
	) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/join)
	pub fn join<T>(self: &JsIterator<T>) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/join)
	#[js_sys(js_name = "join")]
	pub fn join_with_separator<T>(
		self: &JsIterator<T>,
		separator: &str,
	) -> Result<JsString, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/map)
	pub fn map<T>(self: &JsIterator<T>, callback: &Function) -> Result<JsIterator, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/reduce)
	pub fn reduce<T>(self: &JsIterator<T>, callback: &Function) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/reduce)
	#[js_sys(js_name = "reduce")]
	pub fn reduce_with_initial_value<T>(
		self: &JsIterator<T>,
		callback: &Function,
		initial_value: &JsValue,
	) -> Result<JsValue, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/some)
	pub fn some<T>(self: &JsIterator<T>, callback: &Function) -> Result<bool, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/take)
	#[js_sys(return_abi = Result<JsIterator, JsValue>)]
	pub fn take<T>(self: &JsIterator<T>, limit: f64) -> Result<JsIterator<T>, JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/toArray)
	#[js_sys(js_name = "toArray", return_abi = Result<Array, JsValue>)]
	pub fn to_array<T>(self: &JsIterator<T>) -> Result<Array<T>, JsValue>;
}

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(setter = "mode")]
	fn set_zip_mode_raw(self: &IteratorZipOptions, mode: &str);

	#[js_sys(setter = "padding")]
	fn set_zip_padding_raw(self: &IteratorZipOptions, padding: &JsValue);

	#[js_sys(setter = "mode")]
	fn set_zip_keyed_mode_raw(self: &IteratorZipKeyedOptions, mode: &str);

	#[js_sys(setter = "padding")]
	fn set_zip_keyed_padding_raw(self: &IteratorZipKeyedOptions, padding: &Object);
}

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "iterator.next")]
	fn iterator_next<T>(
		#[js_sys(type = &JsValue)] iterator: &JsIterator<T>,
	) -> Result<IteratorResult, JsValue>;

	#[js_sys(js_embed = "iterator.next.value")]
	fn iterator_next_with_value<T>(
		#[js_sys(type = &JsValue)] iterator: &JsIterator<T>,
		value: &JsValue,
	) -> Result<IteratorResult, JsValue>;

	#[js_sys(js_embed = "iterator.return")]
	fn iterator_return<T>(
		#[js_sys(type = &JsValue)] iterator: &JsIterator<T>,
	) -> Result<Option<IteratorResult>, JsValue>;

	#[js_sys(js_embed = "iterator.return.value")]
	fn iterator_return_with_value<T>(
		#[js_sys(type = &JsValue)] iterator: &JsIterator<T>,
		value: &JsValue,
	) -> Result<Option<IteratorResult>, JsValue>;

	#[js_sys(js_embed = "iterator.throw")]
	fn iterator_throw<T>(
		#[js_sys(type = &JsValue)] iterator: &JsIterator<T>,
		value: &JsValue,
	) -> Result<Option<IteratorResult>, JsValue>;

	#[js_sys(js_embed = "iterator.dispose")]
	fn iterator_dispose<T>(
		#[js_sys(type = &JsValue)] iterator: &JsIterator<T>,
	) -> Result<(), JsValue>;

	#[js_sys(js_embed = "async_iterator.next")]
	pub(crate) fn async_iterator_next<T>(
		#[js_sys(type = &JsValue)] iterator: &AsyncIterator<T>,
	) -> Result<Promise<IteratorResult>, JsValue>;

	#[js_sys(js_embed = "async_iterator.next.value")]
	fn async_iterator_next_with_value<T>(
		#[js_sys(type = &JsValue)] iterator: &AsyncIterator<T>,
		value: &JsValue,
	) -> Result<Promise<IteratorResult>, JsValue>;

	#[js_sys(js_embed = "async_iterator.return")]
	fn async_iterator_return<T>(
		#[js_sys(type = &JsValue)] iterator: &AsyncIterator<T>,
	) -> Result<Option<Promise<IteratorResult>>, JsValue>;

	#[js_sys(js_embed = "async_iterator.return.value")]
	fn async_iterator_return_with_value<T>(
		#[js_sys(type = &JsValue)] iterator: &AsyncIterator<T>,
		value: &JsValue,
	) -> Result<Option<Promise<IteratorResult>>, JsValue>;

	#[js_sys(js_embed = "async_iterator.throw")]
	fn async_iterator_throw<T>(
		#[js_sys(type = &JsValue)] iterator: &AsyncIterator<T>,
		value: &JsValue,
	) -> Result<Option<Promise<IteratorResult>>, JsValue>;

	#[js_sys(js_embed = "async_iterator.dispose")]
	fn async_iterator_dispose<T>(
		#[js_sys(type = &JsValue)] iterator: &AsyncIterator<T>,
	) -> Result<Promise, JsValue>;

	#[js_sys(js_embed = "iterator_result.done")]
	fn iterator_result_done(result: &IteratorResult) -> Result<bool, JsValue>;

	#[js_sys(js_embed = "iterator_result.value")]
	fn iterator_result_value(result: &IteratorResult) -> Result<JsValue, JsValue>;
}

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "iterator.from")]
	pub fn iterator_from(value: &JsValue) -> Result<Option<JsIterator>, JsValue>;

	#[js_sys(js_embed = "async_iterator.from")]
	pub fn async_iterator_from(value: &JsValue) -> Result<Option<AsyncIterator>, JsValue>;
}

macro_rules! impl_wrapper {
	($type:ident) => {
		impl<T> Clone for $type<T> {
			fn clone(&self) -> Self {
				Self::unchecked_from(<Self as AsRef<JsValue>>::as_ref(self).clone())
			}
		}

		impl<T> fmt::Debug for $type<T> {
			fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
				fmt::Debug::fmt(<Self as AsRef<JsValue>>::as_ref(self), formatter)
			}
		}
	};
}

impl_wrapper!(JsIterator);
impl_wrapper!(AsyncIterator);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "iterator.next",
	"(iterator) => {{",
	"    const result = iterator.next()",
	"    if (result == null || (typeof result !== 'object' && typeof result !== 'function'))",
	"        throw new TypeError('iterator next method returned a non-object value')",
	"    return result",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "iterator.next.value",
	"(iterator, value) => {{",
	"    const result = iterator.next(value)",
	"    if (result == null || (typeof result !== 'object' && typeof result !== 'function'))",
	"        throw new TypeError('iterator next method returned a non-object value')",
	"    return result",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "iterator.return",
	"(iterator) => {{",
	"    const method = iterator.return",
	"    if (method == null) return null",
	"    if (typeof method !== 'function')",
	"        throw new TypeError('iterator return property is not callable')",
	"    const result = method.call(iterator)",
	"    if (result == null || (typeof result !== 'object' && typeof result !== 'function'))",
	"        throw new TypeError('iterator return method returned a non-object value')",
	"    return result",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "iterator.return.value",
	"(iterator, value) => {{",
	"    const method = iterator.return",
	"    if (method == null) return null",
	"    if (typeof method !== 'function')",
	"        throw new TypeError('iterator return property is not callable')",
	"    const result = method.call(iterator, value)",
	"    if (result == null || (typeof result !== 'object' && typeof result !== 'function'))",
	"        throw new TypeError('iterator return method returned a non-object value')",
	"    return result",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "iterator.throw",
	"(iterator, value) => {{",
	"    const method = iterator.throw",
	"    if (method == null) return null",
	"    if (typeof method !== 'function')",
	"        throw new TypeError('iterator throw property is not callable')",
	"    const result = method.call(iterator, value)",
	"    if (result == null || (typeof result !== 'object' && typeof result !== 'function'))",
	"        throw new TypeError('iterator throw method returned a non-object value')",
	"    return result",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "iterator.dispose",
	"(iterator) => {{",
	"    const method = iterator[Symbol.dispose]",
	"    if (typeof method !== 'function')",
	"        throw new TypeError('iterator does not provide Symbol.dispose')",
	"    method.call(iterator)",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "iterator_result.done",
	"(result) => Boolean(result.done)",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "iterator_result.value",
	"(result) => result.value",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "iterator.from",
	"(value) => {{",
	"    if (value == null) return null",
	"    const method = value[Symbol.iterator]",
	"    if (method == null) return null",
	"    if (typeof method !== 'function')",
	"        throw new TypeError('Symbol.iterator property is not callable')",
	"    const iterator = method.call(value)",
	"    if (iterator == null || (typeof iterator !== 'object' && typeof iterator !== 'function'))",
	"        throw new TypeError('iterator method returned a non-object value')",
	"    return iterator",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "async_iterator.from",
	"(value) => {{",
	"    if (value == null) return null",
	"    const asyncMethod = value[Symbol.asyncIterator]",
	"    if (asyncMethod != null) {{",
	"        if (typeof asyncMethod !== 'function')",
	"            throw new TypeError('Symbol.asyncIterator property is not callable')",
	"        const iterator = asyncMethod.call(value)",
	"        if (iterator == null || (typeof iterator !== 'object' && typeof iterator !== \
	 'function'))",
	"            throw new TypeError('async iterator method returned a non-object value')",
	"        return iterator",
	"    }}",
	"",
	"    const syncMethod = value[Symbol.iterator]",
	"    if (syncMethod == null) return null",
	"    if (typeof syncMethod !== 'function')",
	"        throw new TypeError('Symbol.iterator property is not callable')",
	"    const iterator = syncMethod.call(value)",
	"    if (iterator == null || (typeof iterator !== 'object' && typeof iterator !== 'function'))",
	"        throw new TypeError('iterator method returned a non-object value')",
	"    const next = iterator.next",
	"    if (typeof next !== 'function')",
	"        throw new TypeError('iterator does not provide a next method')",
	"",
	"    const reject = error => Promise.reject(error)",
	"    const close = reason => {{",
	"        try {{",
	"            const method = iterator.return",
	"            if (method != null) {{",
	"                if (typeof method !== 'function')",
	"                    throw new TypeError('iterator return property is not callable')",
	"                const result = method.call(iterator)",
	"                if (result == null || (typeof result !== 'object' && typeof result !== \
	 'function'))",
	"                    throw new TypeError('iterator return method returned a non-object value')",
	"            }}",
	"        }} catch {{}}",
	"        throw reason",
	"    }}",
	"    const continueWith = (result, closeOnRejection) => {{",
	"        try {{",
	"            if (result == null || (typeof result !== 'object' && typeof result !== \
	 'function'))",
	"                throw new TypeError('iterator method returned a non-object value')",
	"            const done = Boolean(result.done)",
	"            const value = result.value",
	"            const unwrap = value => ({{ done, value }})",
	"            return !done && closeOnRejection",
	"                ? Promise.resolve(value).then(unwrap, close)",
	"                : Promise.resolve(value).then(unwrap)",
	"        }} catch (error) {{",
	"            return reject(error)",
	"        }}",
	"    }}",
	"    return {{",
	"        next(value) {{",
	"            try {{",
	"                return continueWith(",
	"                    arguments.length === 0 ? next.call(iterator) : next.call(iterator, \
	 value),",
	"                    true,",
	"                )",
	"            }} catch (error) {{",
	"                return reject(error)",
	"            }}",
	"        }},",
	"        return(value) {{",
	"            try {{",
	"                const method = iterator.return",
	"                if (method == null)",
	"                    return Promise.resolve({{",
	"                        done: true,",
	"                        value: arguments.length === 0 ? undefined : value,",
	"                    }})",
	"                if (typeof method !== 'function')",
	"                    throw new TypeError('iterator return property is not callable')",
	"                return continueWith(",
	"                    arguments.length === 0 ? method.call(iterator) : method.call(iterator, \
	 value),",
	"                    false,",
	"                )",
	"            }} catch (error) {{",
	"                return reject(error)",
	"            }}",
	"        }},",
	"        throw(value) {{",
	"            try {{",
	"                const method = iterator.throw",
	"                if (method == null) {{",
	"                    const close = iterator.return",
	"                    if (close != null) {{",
	"                        if (typeof close !== 'function')",
	"                            throw new TypeError('iterator return property is not callable')",
	"                        const result = close.call(iterator)",
	"                        if (result == null || (typeof result !== 'object' && typeof result \
	 !== 'function'))",
	"                            throw new TypeError('iterator return method returned a \
	 non-object value')",
	"                    }}",
	"                    throw new TypeError('sync iterator does not provide a throw method')",
	"                }}",
	"                if (typeof method !== 'function')",
	"                    throw new TypeError('iterator throw property is not callable')",
	"                return continueWith(",
	"                    arguments.length === 0 ? method.call(iterator) : method.call(iterator, \
	 value),",
	"                    true,",
	"                )",
	"            }} catch (error) {{",
	"                return reject(error)",
	"            }}",
	"        }},",
	"        [Symbol.asyncIterator]() {{ return this }},",
	"    }}",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "async_iterator.next",
	"(iterator) => Promise.resolve(iterator.next()).then(result => {{",
	"    if (result == null || (typeof result !== 'object' && typeof result !== 'function'))",
	"        throw new TypeError('async iterator next method returned a non-object value')",
	"    return result",
	"}})",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "async_iterator.next.value",
	"(iterator, value) => Promise.resolve(iterator.next(value)).then(result => {{",
	"    if (result == null || (typeof result !== 'object' && typeof result !== 'function'))",
	"        throw new TypeError('async iterator next method returned a non-object value')",
	"    return result",
	"}})",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "async_iterator.return",
	"(iterator) => {{",
	"    const method = iterator.return",
	"    if (method == null) return null",
	"    if (typeof method !== 'function')",
	"        throw new TypeError('async iterator return property is not callable')",
	"    return Promise.resolve(method.call(iterator)).then(result => {{",
	"        if (result == null || (typeof result !== 'object' && typeof result !== 'function'))",
	"            throw new TypeError('async iterator return method returned a non-object value')",
	"        return result",
	"    }})",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "async_iterator.return.value",
	"(iterator, value) => {{",
	"    const method = iterator.return",
	"    if (method == null) return null",
	"    if (typeof method !== 'function')",
	"        throw new TypeError('async iterator return property is not callable')",
	"    return Promise.resolve(method.call(iterator, value)).then(result => {{",
	"        if (result == null || (typeof result !== 'object' && typeof result !== 'function'))",
	"            throw new TypeError('async iterator return method returned a non-object value')",
	"        return result",
	"    }})",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "async_iterator.throw",
	"(iterator, value) => {{",
	"    const method = iterator.throw",
	"    if (method == null) return null",
	"    if (typeof method !== 'function')",
	"        throw new TypeError('async iterator throw property is not callable')",
	"    return Promise.resolve(method.call(iterator, value)).then(result => {{",
	"        if (result == null || (typeof result !== 'object' && typeof result !== 'function'))",
	"            throw new TypeError('async iterator throw method returned a non-object value')",
	"        return result",
	"    }})",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "async_iterator.dispose",
	"(iterator) => {{",
	"    const method = iterator[Symbol.asyncDispose]",
	"    if (typeof method !== 'function')",
	"        throw new TypeError('async iterator does not provide Symbol.asyncDispose')",
	"    return Promise.resolve(method.call(iterator))",
	"}}",
);

/// A JavaScript type known to implement `Symbol.iterator`.
pub trait Iterable: JsCast + AsRef<JsValue> {
	type Item: JsCast;
}

/// A JavaScript type known to implement `Symbol.asyncIterator`.
pub trait AsyncIterable: JsCast + AsRef<JsValue> {
	type Item: JsCast;
}

impl IteratorZipOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zip#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zip#mode)
	pub fn set_mode(&self, mode: IteratorZipMode) {
		self.set_zip_mode_raw(mode.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zip#padding)
	pub fn set_padding<I: Iterable>(&self, padding: &I) {
		self.set_zip_padding_raw(padding.as_ref());
	}
}

impl Default for IteratorZipOptions {
	fn default() -> Self {
		Self::new()
	}
}

impl IteratorZipKeyedOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zipKeyed#options)
	#[must_use]
	pub fn new() -> Self {
		Self::unchecked_from(Object::new().into())
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zipKeyed#mode)
	pub fn set_mode(&self, mode: IteratorZipMode) {
		self.set_zip_keyed_mode_raw(mode.as_str());
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/zipKeyed#padding)
	pub fn set_padding(&self, padding: &Object) {
		self.set_zip_keyed_padding_raw(padding);
	}
}

impl Default for IteratorZipKeyedOptions {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: JsCast> Iterable for JsIterator<T> {
	type Item = T;
}

impl<T: JsCast> AsyncIterable for AsyncIterator<T> {
	type Item = T;
}

impl<T: JsCast> JsIterator<T> {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/from)
	pub fn from_iterable<I: Iterable<Item = T>>(value: &I) -> Result<Self, JsValue> {
		let iterator = JsIterator::from_value(value.as_ref())?;
		Ok(Self::unchecked_from(iterator.into()))
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_next_method)
	pub fn next_result(&self) -> Result<IteratorResult, JsValue> {
		iterator_next(self)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_next_method)
	pub fn next_result_with_value(&self, value: &JsValue) -> Result<IteratorResult, JsValue> {
		iterator_next_with_value(self, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_return_method)
	pub fn return_result(&self) -> Result<Option<IteratorResult>, JsValue> {
		iterator_return(self)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_return_method)
	pub fn return_result_with_value(
		&self,
		value: &JsValue,
	) -> Result<Option<IteratorResult>, JsValue> {
		iterator_return_with_value(self, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_throw_method)
	pub fn throw_result(&self, value: &JsValue) -> Result<Option<IteratorResult>, JsValue> {
		iterator_throw(self, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Iterator/Symbol.dispose)
	pub fn dispose(&self) -> Result<(), JsValue> {
		iterator_dispose(self)
	}
}

impl<T: JsCast> AsyncIterator<T> {
	/// Creates an `async` iterator from an asynchronous or synchronous
	/// `iterable`.
	///
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for-await...of#description)
	pub fn try_from_value(value: &JsValue) -> Result<Option<Self>, JsValue> {
		Ok(async_iterator_from(value)?.map(|iterator| Self::unchecked_from(iterator.into())))
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_next_method)
	pub fn next_result(&self) -> Result<Promise<IteratorResult>, JsValue> {
		async_iterator_next(self)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_next_method)
	pub fn next_result_with_value(
		&self,
		value: &JsValue,
	) -> Result<Promise<IteratorResult>, JsValue> {
		async_iterator_next_with_value(self, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_return_method)
	pub fn return_result(&self) -> Result<Option<Promise<IteratorResult>>, JsValue> {
		async_iterator_return(self)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_return_method)
	pub fn return_result_with_value(
		&self,
		value: &JsValue,
	) -> Result<Option<Promise<IteratorResult>>, JsValue> {
		async_iterator_return_with_value(self, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_throw_method)
	pub fn throw_result(
		&self,
		value: &JsValue,
	) -> Result<Option<Promise<IteratorResult>>, JsValue> {
		async_iterator_throw(self, value)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AsyncIterator/Symbol.asyncDispose)
	pub fn dispose(&self) -> Result<Promise, JsValue> {
		async_iterator_dispose(self)
	}
}

impl IteratorResult {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_iterator_protocol)
	pub fn done(&self) -> Result<bool, JsValue> {
		iterator_result_done(self)
	}

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols#the_iterator_protocol)
	pub fn value(&self) -> Result<JsValue, JsValue> {
		iterator_result_value(self)
	}
}
