#![no_std]
#![cfg_attr(target_feature = "atomics", feature(thread_local))]
#![cfg_attr(
	all(not(debug_assertions), target_arch = "wasm64"),
	feature(simd_wasm64)
)]

extern crate alloc;

#[macro_use]
mod util;

// JavaScript standard built-in objects.
mod builtins;
// `Runtime` support for Rust and JavaScript `interop`.
mod runtime;

pub mod hazard;
// JavaScript `ABI` implementations for Rust types and Rust-facing APIs for
// JavaScript values.
mod interop;
#[doc(hidden)]
pub mod r#macro;

pub use builtins::{
	AggregateError, Array, ArrayBuffer, ArrayBufferOptions, AsyncDisposableStack, AsyncFunction,
	AsyncGenerator, AsyncGeneratorFunction, AsyncIterable, AsyncIterator, Atomics, Base64Alphabet,
	Base64DecodeOptions, Base64EncodeOptions, Base64LastChunkHandling, BigInt, BigInt64Array,
	BigUint64Array, Boolean, DataView, Date, DisposableStack, Error, ErrorOptions, EvalError,
	FinalizationRegistry, Float16Array, Float32Array, Float64Array, Function, Generator,
	GeneratorFunction, Int8Array, Int16Array, Int32Array, Intl, Iterable, IteratorResult,
	IteratorZipKeyedOptions, IteratorZipMode, IteratorZipOptions, JSON, JsIterator, JsString, Map,
	Math, Number, Object, Promise, PromiseWithResolvers, PropertyDescriptor, Proxy, ProxyRevocable,
	RangeError, ReferenceError, Reflect, RegExp, RegExpIndicesArray, RegExpMatchArray, Set,
	SharedArrayBuffer, SuppressedError, Symbol, SyntaxError, Temporal, TypeError, Uint8Array,
	Uint8ArraySetResult, Uint8ClampedArray, Uint16Array, Uint32Array, UriError, WeakMap, WeakRef,
	WeakSet, WebAssembly, decode_uri, decode_uri_component, encode_uri, encode_uri_component, eval,
	global_this, is_finite, is_nan, parse_float, parse_int, parse_int_with_radix,
};
pub use interop::{
	ArrayIntoIter, ArrayIter, AsyncIter, JsIntoIter, JsIter, TryFromArrayError, TypedArray,
	TypedArrayCopyError, TypedArrayIntoIter, TypedArrayIter, try_async_iter, try_iter,
};
pub use js_bindgen;
pub use js_sys_macro::{closure, js_sys};
pub use runtime::{
	Closure, ClosureAllocation, ClosureHeader, JsFuture, JsValue, UnwrapThrowExt, block_on,
	future_to_promise, panic, spawn_local,
};

#[cfg(not(target_feature = "reference-types"))]
compile_error!("`js-sys` requires the `reference-types` target feature");

#[cfg(not(target_feature = "multivalue"))]
compile_error!("`js-sys` requires the `multivalue` target feature");
