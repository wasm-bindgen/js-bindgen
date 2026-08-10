use alloc::boxed::Box;
use core::marker::PhantomData;
use core::mem::{self, ManuallyDrop};
use core::ops::Deref;
use core::ptr;

use crate::JsValue;
use crate::builtins::Function;
use crate::hazard::{IntoJS, IntoJsConv, JsCast};

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "closure.unref")]
	fn closure_unref(callback: &JsValue);
}

/// Type-erased information stored at the start of every closure allocation.
#[doc(hidden)]
#[repr(C)]
pub struct ClosureHeader {
	drop: unsafe fn(*mut Self),
}

// Both `repr(C)` prefixes start with `ClosureHeader`. This lets JavaScript
// retain one thin pointer while Rust recovers the signature and callback types.
#[repr(C)]
struct ClosurePrefix<C> {
	header: ClosureHeader,
	call_shim: C,
}

#[repr(C)]
struct ClosureState<F, C> {
	prefix: ClosurePrefix<C>,
	callback: F,
}

impl ClosureHeader {
	#[must_use]
	const fn new(drop: unsafe fn(*mut Self)) -> Self {
		Self { drop }
	}

	/// Returns the byte offset of the call shim in a closure allocation.
	#[doc(hidden)]
	#[must_use]
	pub const fn call_shim_offset<C>() -> usize {
		// This is the byte offset used to load `call_shim` from the allocation;
		// the loaded field value, not this offset, is the function table index.
		core::mem::offset_of!(ClosurePrefix<C>, call_shim)
	}

	unsafe fn release(pointer: *mut Self) {
		// SAFETY: The caller guarantees that `pointer` identifies a live header.
		// Read the function pointer before it releases the containing allocation.
		let drop = unsafe { (*pointer).drop };
		// SAFETY: The same caller guarantee satisfies the stored drop function.
		unsafe { drop(pointer) };
	}

	/// Returns the captured callback stored after this header.
	///
	/// # Safety
	///
	/// `pointer` must come from [`ClosureAllocation::new`], and `F` and `C`
	/// must be the types used to create that allocation. The caller must
	/// uphold the aliasing rules appropriate for `F`.
	#[inline]
	pub unsafe fn callback<F, C>(pointer: *mut Self) -> *mut F {
		let pointer = pointer.cast::<ClosureState<F, C>>();
		// SAFETY: The caller guarantees the allocation, `F`, and `C` match.
		unsafe { &raw mut (*pointer).callback }
	}
}

/// A closure allocation guarded until ownership reaches JavaScript.
#[doc(hidden)]
pub struct ClosureAllocation(*mut ClosureHeader);

impl ClosureAllocation {
	#[must_use]
	pub fn new<F: 'static, C: 'static>(callback: F, call_shim: C) -> Self {
		unsafe fn drop<F, C>(pointer: *mut ClosureHeader) {
			// SAFETY: This function is stored only in the matching allocation.
			unsafe {
				mem::drop(Box::from_raw(pointer.cast::<ClosureState<F, C>>()));
			}
		}

		let state = Box::new(ClosureState {
			prefix: ClosurePrefix {
				header: ClosureHeader::new(drop::<F, C>),
				call_shim,
			},
			callback,
		});
		Self(Box::into_raw(state).cast())
	}

	#[must_use]
	pub fn data(&self) -> usize {
		self.0.expose_provenance()
	}

	/// Leaves this allocation under JavaScript ownership.
	pub fn forget(self) {
		mem::forget(self);
	}
}

impl Drop for ClosureAllocation {
	fn drop(&mut self) {
		// SAFETY: This guard uniquely owns the live allocation until transferred.
		unsafe { ClosureHeader::release(self.0) };
	}
}

#[crate::js_sys(js_sys = crate)]
fn closure_drop(data: usize) {
	if data == 0 {
		return;
	}

	// SAFETY: JavaScript owns one live reference to the allocation until it
	// invokes this function, and clears `state.data` before doing so.
	unsafe {
		ClosureHeader::release(ptr::with_exposed_provenance_mut::<ClosureHeader>(data));
	}
}

js_bindgen::embed_js!(
	module = "js_sys",
	name = "closure.unref",
	"(callback) => callback.unref()",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "closure.finalization",
	"typeof FinalizationRegistry === 'undefined'",
	"    ? {{ register: () => {{}}, unregister: () => {{}} }}",
	"    : new FinalizationRegistry(state => {{",
	// The `unref` function can outlive its callback. Clear the pointer before
	// releasing it so a later call cannot release the allocation twice.
	"        const data = state.data",
	"        state.data = 0",
	"        if (data) this.#jsExports.closure_drop(data)",
	"    }})",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "closure.own",
	required_embeds = [("js_sys", "closure.finalization")],
	"(callback, state) => {{",
	"    let owned = true",
	"    const release = () => {{",
	"        state.references -= 1",
	"        if (state.references === 0) {{",
	"            const data = state.data",
	"            state.data = 0",
	"            this.#jsEmbed.js_sys['closure.finalization'].unregister(state)",
	"            if (data) this.#jsExports.closure_drop(data)",
	"        }}",
	"    }}",
	"    callback.unref = () => {{",
	"        if (!owned) return",
	"        owned = false",
	"        release()",
	"    }}",
	"    this.#jsEmbed.js_sys['closure.finalization'].register(",
	"        callback, state, state",
	"    )",
	"    return release",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "closure.make",
	required_embeds = [("js_sys", "closure.own")],
	"(data, call) => {{",
	"    const state = {{ data, references: 1 }}",
	"    const callback = (...args) => {{",
	"        if (!state.data) {{",
	"            throw new Error('closure invoked after being dropped')",
	"        }}",
	"        state.references += 1",
	"        try {{",
	"            return call(state.data, ...args)",
	"        }} finally {{",
	"            release()",
	"        }}",
	"    }}",
	"    const release = this.#jsEmbed.js_sys['closure.own'](callback, state)",
	"    return callback",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "closure.make_mut",
	required_embeds = [("js_sys", "closure.own")],
	"(data, call) => {{",
	"    const state = {{ data, references: 1 }}",
	"    const callback = (...args) => {{",
	"        if (!state.data) {{",
	"            throw new Error('closure invoked recursively or after being dropped')",
	"        }}",
	"        state.references += 1",
	"        const data = state.data",
	"        state.data = 0",
	"        try {{",
	"            return call(data, ...args)",
	"        }} finally {{",
	"            state.data = data",
	"            release()",
	"        }}",
	"    }}",
	"    const release = this.#jsEmbed.js_sys['closure.own'](callback, state)",
	"    return callback",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "closure.make_once",
	required_embeds = [("js_sys", "closure.own")],
	"(data, call) => {{",
	"    const state = {{ data, references: 1, called: false }}",
	"    const callback = (...args) => {{",
	"        if (!state.data) {{",
	"            throw new Error('closure invoked recursively or after being dropped')",
	"        }}",
	"        if (state.called) {{",
	"            throw new Error('FnOnce called more than once')",
	"        }}",
	"        state.called = true",
	"        state.references += 1",
	"        const data = state.data",
	"        state.data = 0",
	"        try {{",
	"            return call(data, ...args)",
	"        }} finally {{",
	"            state.data = data",
	"            release()",
	"        }}",
	"    }}",
	"    const release = this.#jsEmbed.js_sys['closure.own'](callback, state)",
	"    return callback",
	"}}",
);

/// An owned Rust closure exposed as a JavaScript function.
#[repr(transparent)]
pub struct Closure<T: ?Sized> {
	value: JsValue,
	_type: PhantomData<Box<T>>,
}

impl<T: ?Sized> Deref for Closure<T> {
	type Target = Function;

	#[inline]
	fn deref(&self) -> &Self::Target {
		Function::unchecked_from_ref(self.as_js_value())
	}
}

impl<T: ?Sized> From<Closure<T>> for Function {
	#[inline]
	fn from(value: Closure<T>) -> Self {
		Self::unchecked_from(value.into_js_value())
	}
}

impl<T: ?Sized> Closure<T> {
	#[must_use]
	pub fn as_js_value(&self) -> &JsValue {
		&self.value
	}

	/// # Safety
	///
	/// `value` must be a callback produced by the matching `js-sys` closure
	/// factory. In particular, it must carry the `unref` method used by
	/// [`Closure::drop`].
	#[doc(hidden)]
	#[must_use]
	pub unsafe fn from_js_value(value: JsValue) -> Self {
		Self {
			value,
			_type: PhantomData,
		}
	}

	/// Transfers this closure to JavaScript ownership.
	///
	/// When supported by the JavaScript `runtime`, the captured Rust values are
	/// released after the JavaScript function becomes unreachable. Otherwise,
	/// the Rust allocation remains alive.
	#[must_use]
	pub fn into_js_value(self) -> JsValue {
		let this = ManuallyDrop::new(self);
		// SAFETY: `this` will not run `Closure::drop`, and `value` is moved out
		// exactly once into the returned owner.
		unsafe { ptr::read(&raw const this.value) }
	}

	/// Leaves this closure under JavaScript ownership permanently.
	///
	/// Prefer [`Closure::into_js_value`] when the JavaScript function can be
	/// retained as a [`JsValue`].
	pub fn forget(self) {
		mem::forget(self);
	}
}

impl<T: ?Sized> AsRef<JsValue> for Closure<T> {
	fn as_ref(&self) -> &JsValue {
		self.as_js_value()
	}
}

// SAFETY: This delegates to the borrowed conversion of the underlying
// `JsValue`; ownership of the callback remains with `Closure`.
unsafe impl<'a, T: ?Sized> IntoJS for &'a Closure<T> {
	const JS_CONV: Option<IntoJsConv> = <&'a JsValue as IntoJS>::JS_CONV;

	type Abi = <&'a JsValue as IntoJS>::Abi;

	fn into_abi(self) -> Self::Abi {
		IntoJS::into_abi(&self.value)
	}
}

// SAFETY: The owned `JsValue` is transferred to JavaScript. The callback's
// finalization registry owns the corresponding Rust closure allocation.
unsafe impl<T: ?Sized> IntoJS for Closure<T> {
	const JS_CONV: Option<IntoJsConv> = <JsValue as IntoJS>::JS_CONV;

	type Abi = <JsValue as IntoJS>::Abi;

	fn into_abi(self) -> Self::Abi {
		IntoJS::into_abi(self.into_js_value())
	}
}

impl<T: ?Sized> Drop for Closure<T> {
	fn drop(&mut self) {
		closure_unref(&self.value);
	}
}
