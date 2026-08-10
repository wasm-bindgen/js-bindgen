use core::future::Future;
use core::task::{Context, Poll, Waker};

use js_bindgen_test::test;
use js_sys::hazard::JsCast;
use js_sys::{
	AsyncIterator, JsFuture, JsIterator, JsString, JsValue, Number, js_sys, try_async_iter,
	try_iter,
};

js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.strings",
	"() => ['one', 'two'][Symbol.iterator]()",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.values",
	"() => [null, undefined]",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.not_iterable",
	"() => ({{}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.symbol_throws",
	"() => Object.defineProperty({{}}, Symbol.iterator, {{",
	"    get() {{ throw new Error('symbol') }}",
	"}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.symbol_not_callable",
	"() => ({{ [Symbol.iterator]: 1 }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.invalid_iterator",
	"() => ({{ [Symbol.iterator]: () => 1 }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.missing_next",
	"() => ({{ [Symbol.iterator]: () => ({{}}) }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.next_throws",
	"() => ({{ next() {{ throw new Error('next') }} }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.next_primitive",
	"() => ({{ next() {{ return 1 }} }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.done_truthy",
	"() => ({{ next() {{ return {{",
	"    done: 'yes',",
	"    get value() {{ throw new Error('value must not be read') }},",
	"}} }} }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.done_throws",
	"() => ({{ next() {{ return {{",
	"    get done() {{ throw new Error('done') }},",
	"}} }} }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.value_throws",
	"() => ({{ next() {{ return {{",
	"    done: false,",
	"    get value() {{ throw new Error('value') }},",
	"}} }} }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "async.strings",
	"() => (async function* () {{ yield 'one'; yield 'two' }})()",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "async.cached_next",
	"() => ({{",
	"    reads: 0,",
	"    calls: 0,",
	"    [Symbol.asyncIterator]() {{ return this }},",
	"    get next() {{",
	"        this.reads++",
	"        const value = this.reads",
	"        return function() {{",
	"            this.calls++",
	"            return Promise.resolve({{ done: false, value }})",
	"        }}",
	"    }},",
	"}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "async.next_getter_throws",
	"() => ({{",
	"    [Symbol.asyncIterator]() {{ return this }},",
	"    get next() {{ throw new Error('next getter') }},",
	"}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "async.plain_result",
	"() => ({{",
	"    done: false,",
	"    [Symbol.asyncIterator]() {{ return this }},",
	"    next() {{",
	"        if (this.done) return {{ done: true }}",
	"        this.done = true",
	"        return {{ done: false, value: 'plain' }}",
	"    }},",
	"}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.promise_values",
	"() => [Promise.resolve('one'), 'two']",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.promise_result",
	"() => ({{",
	"    [Symbol.iterator]() {{ return this }},",
	"    next() {{ return Promise.resolve({{ done: true, value: 'not awaited' }}) }},",
	"}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.argument_counts",
	"() => ({{",
	"    [Symbol.iterator]() {{ return this }},",
	"    next() {{ return {{ done: false, value: arguments.length }} }},",
	"    return() {{ return {{ done: true, value: arguments.length }} }},",
	"}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.close_without_throw",
	"() => ({{",
	"    closed: false,",
	"    [Symbol.iterator]() {{ return this }},",
	"    next() {{ return {{ done: false }} }},",
	"    return() {{ this.closed = true; return {{ done: true }} }},",
	"}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.cached_next",
	"() => ({{",
	"    reads: 0,",
	"    calls: 0,",
	"    [Symbol.iterator]() {{ return this }},",
	"    get next() {{",
	"        this.reads++",
	"        const value = this.reads",
	"        return function() {{",
	"            this.calls++",
	"            return {{ done: false, value }}",
	"        }}",
	"    }},",
	"}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.next_getter_throws",
	"() => ({{",
	"    [Symbol.iterator]() {{ return this }},",
	"    get next() {{ throw new Error('next getter') }},",
	"}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "next.replace",
	"(iterator) => Object.defineProperty(iterator, 'next', {{",
	"    value() {{ return {{ done: false, value: 99 }} }},",
	"}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.rejected_value",
	"() => ({{",
	"    closed: false,",
	"    [Symbol.iterator]() {{ return this }},",
	"    next() {{ return {{ done: false, value: Promise.reject(null) }} }},",
	"    throw() {{ return {{ done: false, value: Promise.reject(null) }} }},",
	"    return() {{ this.closed = true; throw new Error('close') }},",
	"}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "async.next_throws",
	"() => ({{ next() {{ throw new Error('next') }} }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "async.rejects",
	"() => ({{ next() {{ return Promise.reject(new Error('reject')) }} }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "async.next_primitive",
	"() => ({{ next() {{ return Promise.resolve(1) }} }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "async.done_throws",
	"() => ({{ next() {{ return Promise.resolve({{",
	"    get done() {{ throw new Error('done') }},",
	"}}) }} }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "async.value_throws",
	"() => ({{ next() {{ return Promise.resolve({{",
	"    done: false,",
	"    get value() {{ throw new Error('value') }},",
	"}}) }} }})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "async.cancel",
	"() => ({{",
	"    calls: 0,",
	"    next() {{",
	"        this.calls++",
	"        return Promise.resolve(this.calls === 1",
	"            ? {{ done: false, value: 'kept' }}",
	"            : {{ done: true }})",
	"    }},",
	"}})",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "async.calls",
	"(iterator) => iterator.calls",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "closed",
	"(iterator) => iterator.closed",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.reads",
	"(iterator) => iterator.reads",
);
js_bindgen::embed_js!(
	module = "iterator",
	name = "sync.calls",
	"(iterator) => iterator.calls",
);

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "sync.strings")]
	fn sync_strings() -> JsIterator<JsString>;

	#[js_sys(js_embed = "sync.values")]
	fn sync_values() -> JsValue;

	#[js_sys(js_embed = "sync.not_iterable")]
	fn sync_not_iterable() -> JsValue;

	#[js_sys(js_embed = "sync.symbol_throws")]
	fn sync_symbol_throws() -> JsValue;

	#[js_sys(js_embed = "sync.symbol_not_callable")]
	fn sync_symbol_not_callable() -> JsValue;

	#[js_sys(js_embed = "sync.invalid_iterator")]
	fn sync_invalid_iterator() -> JsValue;

	#[js_sys(js_embed = "sync.missing_next")]
	fn sync_missing_next() -> JsValue;

	#[js_sys(js_embed = "sync.next_throws")]
	fn sync_next_throws() -> JsIterator;

	#[js_sys(js_embed = "sync.next_primitive")]
	fn sync_next_primitive() -> JsIterator;

	#[js_sys(js_embed = "sync.done_truthy")]
	fn sync_done_truthy() -> JsIterator;

	#[js_sys(js_embed = "sync.done_throws")]
	fn sync_done_throws() -> JsIterator;

	#[js_sys(js_embed = "sync.value_throws")]
	fn sync_value_throws() -> JsIterator;

	#[js_sys(js_embed = "sync.promise_values")]
	fn sync_promise_values() -> JsValue;

	#[js_sys(js_embed = "sync.promise_result")]
	fn sync_promise_result() -> JsValue;

	#[js_sys(js_embed = "sync.argument_counts")]
	fn sync_argument_counts() -> JsValue;

	#[js_sys(js_embed = "sync.close_without_throw")]
	fn sync_close_without_throw() -> JsValue;

	#[js_sys(js_embed = "sync.cached_next")]
	fn sync_cached_next() -> JsValue;

	#[js_sys(js_embed = "sync.next_getter_throws")]
	fn sync_next_getter_throws() -> JsValue;

	#[js_sys(js_embed = "next.replace")]
	fn replace_next(iterator: &JsValue);

	#[js_sys(js_embed = "sync.rejected_value")]
	fn sync_rejected_value() -> JsValue;

	#[js_sys(js_embed = "async.strings")]
	fn async_strings() -> AsyncIterator<JsString>;

	#[js_sys(js_embed = "async.strings")]
	fn async_values() -> JsValue;

	#[js_sys(js_embed = "async.cached_next")]
	fn async_cached_next() -> JsValue;

	#[js_sys(js_embed = "async.next_getter_throws")]
	fn async_next_getter_throws() -> JsValue;

	#[js_sys(js_embed = "async.plain_result")]
	fn async_plain_result() -> AsyncIterator<JsString>;

	#[js_sys(js_embed = "async.next_throws")]
	fn async_next_throws() -> AsyncIterator;

	#[js_sys(js_embed = "async.rejects")]
	fn async_rejects() -> AsyncIterator;

	#[js_sys(js_embed = "async.next_primitive")]
	fn async_next_primitive() -> AsyncIterator;

	#[js_sys(js_embed = "async.done_throws")]
	fn async_done_throws() -> AsyncIterator;

	#[js_sys(js_embed = "async.value_throws")]
	fn async_value_throws() -> AsyncIterator;

	#[js_sys(js_embed = "async.cancel")]
	fn async_cancel() -> AsyncIterator<JsString>;

	#[js_sys(js_embed = "async.calls")]
	fn async_calls(iterator: &AsyncIterator<JsString>) -> u32;

	#[js_sys(js_embed = "closed")]
	fn closed(value: &JsValue) -> bool;

	#[js_sys(js_embed = "sync.reads")]
	fn sync_reads(value: &JsValue) -> u32;

	#[js_sys(js_embed = "sync.calls")]
	fn sync_calls(value: &JsValue) -> u32;
}

fn number(value: JsValue) -> f64 {
	Number::<f64>::unchecked_from(value).value_of()
}

fn assert_number(value: JsValue, expected: f64) {
	assert!((number(value) - expected).abs() < f64::EPSILON);
}

#[test]
fn sync_iterator() {
	let iterator = sync_strings();
	let values: Vec<_> = iterator.iter().map(Result::unwrap).collect();
	assert_eq!(values, [JsString::from("one"), JsString::from("two")]);

	let values: Vec<_> = sync_strings().into_iter().map(Result::unwrap).collect();
	assert_eq!(values, [JsString::from("one"), JsString::from("two")]);
}

#[test]
fn dynamic_iterator() {
	let values: Vec<_> = try_iter(&sync_values())
		.unwrap()
		.unwrap()
		.map(Result::unwrap)
		.collect();
	assert_eq!(values, [JsValue::NULL, JsValue::UNDEFINED]);

	assert!(try_iter(&sync_not_iterable()).unwrap().is_none());
	assert!(try_iter(&JsValue::NULL).unwrap().is_none());
	assert!(try_iter(&sync_symbol_throws()).is_err());
	assert!(try_iter(&sync_symbol_not_callable()).is_err());
	assert!(try_iter(&sync_invalid_iterator()).is_err());
	assert!(try_iter(&sync_missing_next()).is_err());
	assert!(try_iter(&sync_next_getter_throws()).is_err());
}

#[test]
fn sync_next_is_cached() {
	let source = sync_cached_next();
	let mut iterator = try_iter(&source).unwrap().unwrap();
	assert_eq!(sync_reads(&source), 1);

	replace_next(&source);
	assert_number(iterator.next().unwrap().unwrap(), 1.0);
	assert_eq!(sync_reads(&source), 1);
	assert_eq!(sync_calls(&source), 1);

	let source = sync_cached_next();
	let iterator: JsIterator = JsIterator::unchecked_from(source.clone());
	let result = iterator.next_result().unwrap();
	assert_number(result.value().unwrap(), 1.0);
	replace_next(&source);
	let result = iterator.next_result().unwrap();
	assert_number(result.value().unwrap(), 99.0);
}

#[test]
fn sync_errors_are_fused() {
	for iterator in [
		sync_next_throws(),
		sync_next_primitive(),
		sync_done_throws(),
		sync_value_throws(),
	] {
		let mut iterator = iterator.into_iter();
		assert!(iterator.next().unwrap().is_err());
		assert!(iterator.next().is_none());
	}

	let mut iterator = sync_done_truthy().into_iter();
	assert!(iterator.next().is_none());
	assert!(iterator.next().is_none());
}

#[test]
async fn async_iterator() {
	let mut iterator = async_strings().into_async_iter();
	assert_eq!(
		iterator.next().await.unwrap().unwrap(),
		JsString::from("one")
	);
	assert_eq!(
		iterator.next().await.unwrap().unwrap(),
		JsString::from("two")
	);
	assert!(iterator.next().await.is_none());

	let mut iterator = async_plain_result().into_async_iter();
	assert_eq!(
		iterator.next().await.unwrap().unwrap(),
		JsString::from("plain")
	);
	assert!(iterator.next().await.is_none());
}

#[test]
async fn dynamic_async_iterator() {
	let mut iterator = try_async_iter(&async_values()).unwrap().unwrap();
	let first = iterator.next().await.unwrap().unwrap();
	assert_eq!(JsString::unchecked_from(first), "one");

	assert!(try_async_iter(&sync_not_iterable()).unwrap().is_none());
	assert!(try_async_iter(&JsValue::UNDEFINED).unwrap().is_none());
	assert!(try_async_iter(&async_next_getter_throws()).is_err());

	let mut iterator = try_async_iter(&sync_promise_values()).unwrap().unwrap();
	let first = iterator.next().await.unwrap().unwrap();
	assert_eq!(JsString::unchecked_from(first), "one");
	let second = iterator.next().await.unwrap().unwrap();
	assert_eq!(JsString::unchecked_from(second), "two");
	assert!(iterator.next().await.is_none());
}

#[test]
async fn async_next_is_cached() {
	let source = async_cached_next();
	let mut iterator = try_async_iter(&source).unwrap().unwrap();
	assert_eq!(sync_reads(&source), 1);

	replace_next(&source);
	assert_number(iterator.next().await.unwrap().unwrap(), 1.0);
	assert_eq!(sync_reads(&source), 1);
	assert_eq!(sync_calls(&source), 1);
}

#[test]
async fn async_from_sync() {
	assert!(AsyncIterator::<JsValue>::try_from_value(&sync_symbol_not_callable()).is_err());

	let iterator = AsyncIterator::<JsValue>::try_from_value(&sync_promise_result())
		.unwrap()
		.unwrap();
	let result = JsFuture::from(iterator.next_result().unwrap())
		.await
		.unwrap();
	assert!(!result.done().unwrap());
	assert_eq!(result.value().unwrap(), JsValue::UNDEFINED);

	let iterator = AsyncIterator::<JsValue>::try_from_value(&sync_argument_counts())
		.unwrap()
		.unwrap();
	let result = JsFuture::from(iterator.next_result().unwrap())
		.await
		.unwrap();
	assert_number(result.value().unwrap(), 0.0);
	let result = JsFuture::from(iterator.next_result_with_value(&JsValue::NULL).unwrap())
		.await
		.unwrap();
	assert_number(result.value().unwrap(), 1.0);
	let result = JsFuture::from(iterator.return_result().unwrap().unwrap())
		.await
		.unwrap();
	assert_number(result.value().unwrap(), 0.0);
	let result = JsFuture::from(
		iterator
			.return_result_with_value(&JsValue::NULL)
			.unwrap()
			.unwrap(),
	)
	.await
	.unwrap();
	assert_number(result.value().unwrap(), 1.0);

	let source = sync_close_without_throw();
	let iterator = AsyncIterator::<JsValue>::try_from_value(&source)
		.unwrap()
		.unwrap();
	assert!(
		JsFuture::from(iterator.throw_result(&JsValue::NULL).unwrap().unwrap())
			.await
			.is_err()
	);
	assert!(closed(&source));

	let source = sync_cached_next();
	let iterator = AsyncIterator::<JsValue>::try_from_value(&source)
		.unwrap()
		.unwrap();
	let result = JsFuture::from(iterator.next_result().unwrap())
		.await
		.unwrap();
	assert_number(result.value().unwrap(), 1.0);
	assert_eq!(sync_reads(&source), 1);
	assert_eq!(sync_calls(&source), 1);

	for use_throw in [false, true] {
		let source = sync_rejected_value();
		let iterator = AsyncIterator::<JsValue>::try_from_value(&source)
			.unwrap()
			.unwrap();
		let promise = if use_throw {
			iterator.throw_result(&JsValue::NULL).unwrap().unwrap()
		} else {
			iterator.next_result().unwrap()
		};
		assert_eq!(JsFuture::from(promise).await.unwrap_err(), JsValue::NULL);
		assert!(closed(&source));
	}
}

#[test]
async fn async_errors_are_fused() {
	for iterator in [
		async_next_throws(),
		async_rejects(),
		async_next_primitive(),
		async_done_throws(),
		async_value_throws(),
	] {
		let mut iterator = iterator.into_async_iter();
		assert!(iterator.next().await.unwrap().is_err());
		assert!(iterator.next().await.is_none());
	}
}

#[test]
async fn next_is_cancellation_safe() {
	let iterator = async_cancel();
	let observed = iterator.clone();
	let mut iterator = iterator.into_async_iter();

	{
		let mut next = core::pin::pin!(iterator.next());
		let mut context = Context::from_waker(Waker::noop());
		assert!(matches!(next.as_mut().poll(&mut context), Poll::Pending));
	}

	assert_eq!(
		iterator.next().await.unwrap().unwrap(),
		JsString::from("kept")
	);
	assert_eq!(async_calls(&observed), 1);
}
