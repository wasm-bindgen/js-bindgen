#[rustfmt::skip]
fn main() {
	// ;; await exports["jspi_block_on"]() === "resolved"
	// ;; await exports["jspi_u32"](0xffff_ffff) === 0xffff_ffff
	// ;; await exports["jspi_u128"](1n << 96n) === 1n << 96n
	// ;; await (async () => { const enabled = exports["jspi_has_exception_handling"](); const result = exports["jspi_result"]; if ((typeof result === "function") !== enabled) return false; if (!enabled) return true; if (await result(42) !== 42) return false; const [first, second] = await Promise.allSettled([result(-1), result(-2)]); return first.status === "rejected" && first.reason === -1 && second.status === "rejected" && second.reason === -2 })()
	// ;; await (async () => { const value = { answer: 42 }; return await exports["jspi_js_value"](value) === value })()
}

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use js_sys::{JsString, JsValue, Promise, block_on, js_sys};

js_sys::js_bindgen::embed_js!(
	module = "jspi",
	name = "jspi.resolve",
	"value => Promise.resolve(value)",
);

js_sys::js_bindgen::embed_js!(
	module = "jspi",
	name = "jspi.result",
	"value => value >= 0 ? Promise.resolve(value) : Promise.reject(value)",
);

#[js_sys]
extern "js-sys" {
	#[js_sys(js_embed = "jspi.resolve", suspending)]
	fn suspend_u32(value: u32) -> u32;

	#[js_sys(js_embed = "jspi.resolve", suspending)]
	fn suspend_u128(value: u128) -> u128;

	#[cfg(target_feature = "exception-handling")]
	#[js_sys(js_embed = "jspi.result", suspending)]
	fn suspend_result(value: i32) -> Result<i32, JsValue>;
}

struct YieldOnce(bool);

impl Future for YieldOnce {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
		if self.0 {
			Poll::Ready(())
		} else {
			self.0 = true;
			context.waker().wake_by_ref();
			Poll::Pending
		}
	}
}

#[js_sys(promising)]
fn jspi_block_on() -> JsString {
	block_on(async {
		YieldOnce(false).await;
		Promise::resolve(&JsValue::UNDEFINED).await.unwrap();
		Promise::resolve(&JsValue::UNDEFINED).await.unwrap();
		JsString::from("resolved")
	})
}

#[js_sys(promising)]
fn jspi_u32(value: u32) -> u32 {
	suspend_u32(value)
}

#[js_sys(promising)]
fn jspi_u128(value: u128) -> u128 {
	suspend_u128(value)
}

#[js_sys]
fn jspi_has_exception_handling() -> bool {
	cfg!(target_feature = "exception-handling")
}

#[cfg(target_feature = "exception-handling")]
#[js_sys(promising)]
fn jspi_result(value: i32) -> Result<i32, JsValue> {
	suspend_result(value)
}

#[js_sys(promising)]
fn jspi_js_value(value: JsValue) -> JsValue {
	value
}
