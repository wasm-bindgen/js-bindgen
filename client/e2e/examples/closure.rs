#[rustfmt::skip]
fn main() {
	// ;; exports["closure_i32"](20) === 43
	// ;; exports["closure_same_signature"](20) === 4280
	// ;; exports["closure_macro_repetition"](20) === 84
	// ;; exports["closure_macro_mixed_signatures"]()
	// ;; exports["closure_u128"](1n << 96n) === (1n << 96n) + 1n
	// ;; (() => { const value = {}; return exports["closure_js_value"](value) === value })()
	// ;; exports["closure_js_string_ref"]("closure") === "closure"
	// ;; exports["closure_result"](41, false) === 42
	// ;; (() => { try { exports["closure_result"](41, true); return false } catch (error) { return error === "closure error" } })()
	// ;; exports["closure_lifecycle"]()
	// ;; exports["closure_fn_reentrant"](20) === 22
	// ;; exports["closure_unref_during_call"]()
	// ;; exports["closure_owned"](20) === 21
	// ;; exports["closure_owned_lifecycle"]()
	// ;; (() => { if (exports["closure_once"](20) !== 21) return false; try { exports["closure_once_again"](20); return false } catch (error) { return error.message === "FnOnce called more than once" } })()
	// ;; exports["closure_option_ref"](20)
	// ;; exports["closure_option_owned"](20)
	// ;; (() => { const callback = exports["closure_return"](2); return callback(40) === 42 })()
	// ;; exports["closure_error_lifecycle"]()
	// ;; exports["closure_once_lifecycle"]()
	// ;; typeof globalThis.gc !== "function" || typeof FinalizationRegistry === "undefined" || await (async () => { let callback = exports["closure_finalization"](); const unref = callback.unref; callback = null; for (let i = 0; i < 100 && exports["closure_finalization_drops"]() === 0; i++) { globalThis.gc(); await new Promise(resolve => globalThis.setTimeout(resolve, 0)) } if (exports["closure_finalization_drops"]() !== 1) return false; unref(); return exports["closure_finalization_drops"]() === 1 })()
}

use std::cell::Cell;
use std::sync::atomic::{AtomicU32, Ordering};

use js_sys::{Closure, JsString, JsValue, closure, js_sys};

static DROPS: AtomicU32 = AtomicU32::new(0);

struct DropCounter;

macro_rules! repeated_closures {
	($($offset:expr),+ $(,)?) => {
		($(closure!(dyn FnMut(i32) -> i32, move |value| value + $offset)),+)
	};
}

macro_rules! repeated_typed_closures {
	($($ty:ty),+ $(,)?) => {
		($(closure!(dyn FnMut($ty) -> $ty, |value| value)),+)
	};
}

impl Drop for DropCounter {
	fn drop(&mut self) {
		DROPS.fetch_add(1, Ordering::Relaxed);
	}
}

js_sys::js_bindgen::embed_js!(
	module = "closure",
	name = "invoke.twice",
	"(callback, value) => callback(value) + callback(value)",
);

js_sys::js_bindgen::embed_js!(
	module = "closure",
	name = "invoke",
	"(callback, value) => callback(value)",
);

js_sys::js_bindgen::embed_js!(
	module = "closure",
	name = "storage",
	"({{ callback: undefined }})",
);

js_sys::js_bindgen::embed_js!(
	module = "closure",
	name = "save",
	required_embeds = [("closure", "storage")],
	"(callback) => {{",
	"    this.#jsEmbed.closure.storage.callback = callback",
	"}}",
);

js_sys::js_bindgen::embed_js!(
	module = "closure",
	name = "is_invalid",
	required_embeds = [("closure", "storage")],
	"() => {{",
	"    try {{",
	"        this.#jsEmbed.closure.storage.callback()",
	"        return false",
	"    }} catch (error) {{",
	"        return error instanceof Error",
	"            && error.message === 'closure invoked recursively or after being dropped'",
	"    }}",
	"}}",
);

js_sys::js_bindgen::embed_js!(
	module = "closure",
	name = "invoke.saved",
	required_embeds = [("closure", "storage")],
	"(value) => this.#jsEmbed.closure.storage.callback(value)",
);

js_sys::js_bindgen::embed_js!(
	module = "closure",
	name = "invoke.optional",
	"(callback, value) => callback?.(value)",
);

js_sys::js_bindgen::embed_js!(
	module = "closure",
	name = "release",
	required_embeds = [("closure", "storage")],
	"() => {{",
	"    this.#jsEmbed.closure.storage.callback.unref()",
	"    return true",
	"}}",
);

js_sys::js_bindgen::embed_js!(
	module = "closure",
	name = "release.twice",
	required_embeds = [("closure", "storage")],
	"() => {{",
	"    const callback = this.#jsEmbed.closure.storage.callback",
	"    callback.unref()",
	"    callback.unref()",
	"    return true",
	"}}",
);

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "invoke.twice")]
	fn invoke_i32_twice(callback: &Closure<dyn FnMut(i32) -> i32>, value: i32) -> i32;

	#[js_sys(js_embed = "invoke")]
	fn invoke_u128(callback: &Closure<dyn FnMut(u128) -> u128>, value: u128) -> u128;

	#[js_sys(js_embed = "invoke")]
	fn invoke_js_value(
		callback: &Closure<dyn FnMut(JsValue) -> JsValue>,
		value: JsValue,
	) -> JsValue;

	#[js_sys(js_embed = "invoke")]
	fn invoke_js_string_ref(
		callback: &Closure<dyn FnMut(&JsString) -> JsString>,
		value: &JsString,
	) -> JsString;

	#[js_sys(js_embed = "invoke")]
	fn invoke_result(
		callback: &Closure<dyn FnMut(i32) -> Result<i32, JsValue>>,
		value: i32,
	) -> Result<i32, JsValue>;

	#[js_sys(js_embed = "invoke")]
	fn invoke_unit(callback: &Closure<dyn FnMut()>);

	#[js_sys(js_embed = "invoke.optional")]
	fn invoke_optional_ref(
		callback: Option<&Closure<dyn FnMut(i32) -> i32>>,
		value: i32,
	) -> Option<i32>;

	#[js_sys(js_embed = "invoke.optional")]
	fn invoke_optional_owned(
		callback: Option<Closure<dyn FnMut(i32) -> i32>>,
		value: i32,
	) -> Option<i32>;

	#[js_sys(js_embed = "save")]
	fn save(callback: &Closure<dyn FnMut()>);

	#[js_sys(js_embed = "save")]
	fn save_fn(callback: &Closure<dyn Fn(i32) -> i32>);

	#[js_sys(js_embed = "save")]
	fn save_owned(callback: Closure<dyn FnMut(i32) -> i32>);

	#[js_sys(js_embed = "save")]
	fn save_owned_unit(callback: Closure<dyn FnMut()>);

	#[js_sys(js_embed = "is_invalid")]
	fn is_invalid() -> bool;

	#[js_sys(js_embed = "invoke.saved")]
	fn invoke_saved(value: i32) -> i32;

	#[js_sys(js_embed = "release")]
	fn release() -> bool;

	#[js_sys(js_embed = "release.twice")]
	fn release_twice() -> bool;
}

#[js_sys]
fn closure_i32(value: i32) -> i32 {
	let mut offset = 0;
	let callback = closure!(dyn FnMut(i32) -> i32, move |value| {
		offset += 1;
		value + offset
	});

	invoke_i32_twice(&callback, value)
}

#[js_sys]
fn closure_same_signature(value: i32) -> i32 {
	let first = closure!(dyn FnMut(i32) -> i32, |value| value + 1);
	let second = closure!(dyn FnMut(i32) -> i32, |value| value * 2);

	invoke_i32_twice(&first, value) * 100 + invoke_i32_twice(&second, value)
}

#[js_sys]
fn closure_macro_repetition(value: i32) -> i32 {
	let (first, second) = repeated_closures!(1, 1);

	invoke_i32_twice(&first, value) + invoke_i32_twice(&second, value)
}

#[js_sys]
fn closure_macro_mixed_signatures() -> bool {
	let _ = repeated_typed_closures!(i32, u128);
	true
}

#[js_sys]
fn closure_u128(value: u128) -> u128 {
	let callback = closure!(dyn FnMut(u128) -> u128, move |value| value + 1);

	invoke_u128(&callback, value)
}

#[js_sys]
fn closure_js_value(value: JsValue) -> JsValue {
	let callback = closure!(dyn FnMut(JsValue) -> JsValue, move |value| value);

	invoke_js_value(&callback, value)
}

#[js_sys]
fn closure_js_string_ref(value: &JsString) -> JsString {
	let callback = closure!(dyn FnMut(&JsString) -> JsString, JsString::clone);

	invoke_js_string_ref(&callback, value)
}

#[js_sys]
fn closure_result(value: i32, error: bool) -> Result<i32, JsValue> {
	let callback = closure!(dyn FnMut(i32) -> Result<i32, JsValue>, move |value| {
		if error {
			Err(JsString::from("closure error").into())
		} else {
			Ok(value + 1)
		}
	});

	invoke_result(&callback, value)
}

#[js_sys]
fn closure_lifecycle() -> bool {
	DROPS.store(0, Ordering::Relaxed);
	let counter = DropCounter;
	let callback = closure!(dyn FnMut(), move || {
		let _ = &counter;
	});
	save(&callback);
	drop(callback);
	DROPS.load(Ordering::Relaxed) == 1 && is_invalid()
}

#[js_sys]
fn closure_fn_reentrant(value: i32) -> i32 {
	let entered = Cell::new(false);
	let callback = closure!(dyn Fn(i32) -> i32, move |value| {
		if entered.replace(true) {
			value + 1
		} else {
			invoke_saved(value) + 1
		}
	});
	save_fn(&callback);

	invoke_saved(value)
}

#[js_sys]
fn closure_unref_during_call() -> bool {
	DROPS.store(0, Ordering::Relaxed);
	let counter = DropCounter;
	let callback = closure!(dyn Fn(i32) -> i32, move |value| {
		let _ = &counter;
		let released = release_twice();
		let alive = DROPS.load(Ordering::Relaxed) == 0;

		if released && alive { value + 1 } else { 0 }
	});
	save_fn(&callback);

	invoke_saved(41) == 42 && DROPS.load(Ordering::Relaxed) == 1
}

#[js_sys]
fn closure_owned(value: i32) -> i32 {
	let callback = closure!(dyn FnMut(i32) -> i32, move |value| value + 1);
	save_owned(callback);

	invoke_saved(value)
}

#[js_sys]
fn closure_owned_lifecycle() -> bool {
	DROPS.store(0, Ordering::Relaxed);
	let counter = DropCounter;
	let callback = closure!(dyn FnMut(), move || {
		let _ = &counter;
	});
	save_owned_unit(callback);

	release() && DROPS.load(Ordering::Relaxed) == 1 && is_invalid()
}

#[js_sys]
fn closure_once(value: i32) -> i32 {
	let callback = closure!(dyn FnOnce(i32) -> i32, move |value| value + 1);
	save_owned(callback);

	invoke_saved(value)
}

#[js_sys]
fn closure_once_again(value: i32) -> i32 {
	invoke_saved(value)
}

#[js_sys]
fn closure_option_ref(value: i32) -> bool {
	let callback = closure!(dyn FnMut(i32) -> i32, move |value| value + 1);

	invoke_optional_ref(Some(&callback), value) == Some(value + 1)
		&& invoke_optional_ref(None, value).is_none()
}

#[js_sys]
fn closure_option_owned(value: i32) -> bool {
	let callback = closure!(dyn FnMut(i32) -> i32, move |value| value + 1);

	invoke_optional_owned(Some(callback), value) == Some(value + 1)
		&& invoke_optional_owned(None, value).is_none()
}

#[js_sys]
fn closure_return(offset: i32) -> Closure<dyn FnMut(i32) -> i32> {
	closure!(dyn FnMut(i32) -> i32, move |value| value + offset)
}

#[js_sys]
fn closure_error_lifecycle() -> bool {
	DROPS.store(0, Ordering::Relaxed);
	let counter = DropCounter;
	let callback = closure!(dyn FnMut(i32) -> Result<i32, JsValue>, move |_| {
		let _ = &counter;
		Err(JsString::from("closure error").into())
	});
	let result = invoke_result(&callback, 0);
	drop(callback);

	result.is_err() && DROPS.load(Ordering::Relaxed) == 1
}

#[js_sys]
fn closure_once_lifecycle() -> bool {
	DROPS.store(0, Ordering::Relaxed);
	let counter = DropCounter;
	let callback = closure!(dyn FnOnce(), move || drop(counter));
	invoke_unit(&callback);
	let dropped_after_call = DROPS.load(Ordering::Relaxed) == 1;
	drop(callback);

	dropped_after_call && DROPS.load(Ordering::Relaxed) == 1
}

#[js_sys]
fn closure_finalization() -> Closure<dyn Fn()> {
	DROPS.store(0, Ordering::Relaxed);
	let counter = DropCounter;

	closure!(dyn Fn(), move || {
		let _ = &counter;
	})
}

#[js_sys]
fn closure_finalization_drops() -> u32 {
	DROPS.load(Ordering::Relaxed)
}
