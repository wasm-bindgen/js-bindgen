# Wasm Wishlist

## Description

This document serves as feedback to the Wasm specification from `js-bindgen` to as of yet unmet
needs.

It has come up multiple times that many new proposals that seem great for us at first glance turn
out to be unusable or more inefficient then the workarounds we already have in place. A major
sticking point is the lacking support of new proposals for languages that make use of linear memory
in comparison to the Wasm stack. Most notably `externref` and the `gc` proposal.

This will most likely align with other languages and toolchains that use linear memory, like
Emscripten.

Many if not all changes proposed here are probably covered by the component model proposal. However,
in comparison the proposed changes here have a very small scope and can be easily implemented and
pushed forward. While it is still up in the air how the component model proposal will play a part on
the Web.

## Overall Goal

We aim for a single well defined goal of what we are trying to achieve here: improve the performance
of interactions with the JS API by minimizing the times we have to cross the Wasm-JS boundary.

The way current Wasm toolchain implementations call a JS API is to pass information from Wasm to JS
and do any necessary adjustments in JS _before_ ultimately calling the actual JS API. A very simple
example is passing a `u32`:

```wat
(import "shim" "new_image_data" (func $new_image_data (param i32 i32)))
```

```js
const new_image_data = (width, height) => {
	width >>>= 0
	height >>>= 0
	ImageData(width, height)
}
```

Because Wasm doesn't not have a dedicated unsigned type, we need to convert the passed `i32` to its
unsigned form in JS. There are multiple issues here that we like to resolve:

- The implementation to generate all this code at compile-time for the whole API, e.g. via
  proc-macros, is prohibitively complex to implement and maintain.
- The conversion here is quite cheap, but there are more complex conversions, e.g. converting to a
  JS `string`.
- This adds a function call and a context switch between every Wasm to JS API call. Being able to
  call an imported JS function directly would be much better optimized in current JS engines.

One obvious solution would be to change the signature to accept JS values directly:

```wat
(import "shim" "new_image_data" (func $new_image_data (param externref externref)))
```

But to e.g. convert a `u32` to an `externref` we have to go through JS again, defeating the purpose
of this solution.

At this point we can show off how an already existing Wasm proposal can solve this problem for us:
the [JS Primitive Builtins Proposal]. Now we can simply expect `externref` and the user can
_cheaply_ do the conversion in Wasm themselves without having to cross the Wasm<->JS boundary.

In addition this has the advantage of not requiring the user to choose between language native types
and Wasm types. E.g. by expecting `u32` in Rust, we force the user to potentially convert existing
`externref`s to `u32`s just for them to be converted to `externref` internally or when crossing the
barrier anyway.

[JS Primitive Builtins Proposal]: https://github.com/WebAssembly/js-primitive-builtins

## Proposals

### JS String Encoding/Decoding Builtins

We require cheap methods to encode/decode JS strings directly from/to UTF-8 strings in linear
memory. The [Future Extensions section in the JS String Builtins Proposal] already outlines
everything that is required.

However [the API it proposes to interact with linear memory] does not align with any existing Wasm
API. So just in case, we propose an alternative route in case this doesn't pan out.

The main problem is that the API uses GC's `array` type. The [More Array Constructors Proposal]
proposes new instructions that are able to create `array`s from linear memory. This itself is not
enough, as it does incur an additional copy. What could fit here is the [Slice Proposal]. With
instructions able to create `sliceref` types from linear memory it would give us a path forward to a
zero-copy conversion. However, at the time of writing, this proposal is still in its early design
stage and its unclear if it ultimately will have the capabilities we just described.

[Future Extensions section in the JS String Builtins Proposal]:
	https://github.com/WebAssembly/js-string-builtins/blob/81bfc5fb7b8277c6b7d1b0a8f6e57cb31a7bf080/proposals/js-string-builtins/Overview.md#future-extensions
[the API it proposes to interact with linear memory]:
	https://github.com/WebAssembly/js-string-builtins/blob/81bfc5fb7b8277c6b7d1b0a8f6e57cb31a7bf080/proposals/js-string-builtins/Overview.md#binding-memory-to-builtins

### JS Array Builtins

Many Web APIs take or return JS arrays. Creating or moving those from/to linear memory usually comes
with terrible performance. Considering our goals, we ultimately want to be able to take a language
native array/slice and convert it to an `externref` representing a JS array and the reverse.

The important part is that we have to be able to use linear memory to create a JS array directly.
E.g. even if the GC `array` type could be adapted to return a JS array via `ToJSValue()` and we
could bulk construct `array`s via the [More Array Constructors Proposal], it would still involve an
extra copy.

Apart from needing a way to create JS arrays from numeric types, we also need to create JS arrays
from a list of `externref`s. With access to a JS array constructor and the `Array.push()` function,
we could move what we do in JS shims to Wasm. To convert a JS array to a list of `externref`s, we
would need access to `Array.at()`.

To summarize:

- Copy linear memory range to `externref` representing a JS array with any numeric type.
- Copy numeric values from a JS array represented as `externref` to linear memory range.
- Various JS array functions:
  - `new Array()`
  - `Array.isArray()`
  - `Array.length`
  - `Array.at()`
  - `Array.push()`

### JS `ArrayBuffer` Builtins

Many performance-sensitive Web APIs, deal with `ArrayBuffer`s instead of JS arrays.

For creating `ArrayBuffer`s in Wasm, there is already the [Slice Proposal]. For reading
`ArrayBuffer`s we need functions to bulk copy directly to linear memory. In practice access to
various `ArrayBuffer` methods would be highly desirable as well:

- `new ArrayBuffer()` and constructors for the various `TypedArray`s.
- `TypedArray.length`
- `TypedArray.at()`
- `TypedArray.set()`
- `TypedArray.fill()`
- `TypedArray.slice()`
- `TypedArray.subarray()`
- `ArrayBuffer.resizable`
- `ArrayBuffer.resize()`
- `ArrayBuffer.detached`
- `ArrayBuffer.transfer()`
- `ArrayBuffer.transferToFixedLength()`

### JS `Object` Builtins

Many Web APIs take or return JS objects. Often using a simple API can involve dozens of Wasm <-> JS
context switches just to create parameters or read return values. To that end having builtin
functions for the `Object` type can significantly reduce the size of the JS shim and the number of
context switches.

Instead of builtins, this could be done by mapping the GC `struct` type to JS's `Object` type.
However, it would require statically defining all these `struct`s to map to each required `Object`
instead of being able to dynamically construct and read `Object`s. On the other hand builtins would
require creating a JS string for every single field. Both approaches can serve our purpose.
Considering how we map the Web API in `web-sys`, going with the GC `struct` approach is probably
much better for performance.

Potentially, the builtin path could be expanded to allow access to getters, setters, methods and
constructors, giving almost full access to the entire Web API without generating imports for every
call. However, the scope of this proposal is limited to the goals outlined above:

- `new Object()`
- `Reflect.get()`
- `Reflect.set()`

### JS `Function` Builtins

Especially when interacting with the DOM API, closures are constructed often and require crossing
the Wasm <-> JS boundary multiple times to interact with a single API call. To construct closures
without the help of a JS shim we only need access to the `Function.bind()` API. That is because in
most implementations, including ours, closures are simply stored in the heap and a simple function
is exposed that JS can call the closure through its pointer. We use `Function.bind()` to "store" the
pointer as the first parameter. This is necessary as there is no way to dynamically create functions
in Wasm, ergo Wasm native closures.

But first, we would need to be able to create a JS `Function` in Wasm. Currently, it is already
possible to create a JS function with a `funcref` and `ToJSValue()`. So what we need is a builtin
for that so we don't have to cross the barrier. So this only leaves us with:

- `new Function()`
- `Function.bind()`

Lastly, we will need to destroy the closures, which gets us to our last proposal.

### GC Destructors

GC types could be very useful when users want to expose their own custom `struct`s to JS. However,
without being able determine when a GC object can be cleaned up this is currently limited to
`'static` content. Destructors have been briefly discussed in the GC proposal but has been moved to
post-MVP.

Because this would require quite a bit of rigorous design and consensus, a very simple alternative
proposal would be built-ins for [`FinalizationRegistry`].

Whats important for us is that we need the ability to register destructors for exported GC objects
and exported functions.

[`FinalizationRegistry`]:
	https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/FinalizationRegistry
[More Array Constructors Proposal]: https://github.com/WebAssembly/more-array-constructors
[Slice Proposal]: https://github.com/WebAssembly/design/issues/1555
