use alloc::sync::Arc;
use alloc::task::Wake;
use core::sync::atomic::{AtomicI32, Ordering};
use core::task::Waker;

use super::{AWAKE, POLLING, WAITING};
use crate::util::PtrConst;

js_bindgen::embed_js!(module = "js_sys", name = "future.jspi.waits", "new Map()");

js_bindgen::embed_js!(
	module = "js_sys",
	name = "future.jspi.suspend",
	required_embeds = [("js_sys", "future.jspi.waits")],
	"state => {{",
	"    const buffer = this.#memory.buffer",
	"    const signal = new Int32Array(buffer, state, 1)",
	"    if (typeof SharedArrayBuffer !== 'undefined'",
	"        && buffer instanceof SharedArrayBuffer) {{",
	"        if (typeof Atomics.waitAsync !== 'function') {{",
	"            throw new Error('shared-memory JSPI requires Atomics.waitAsync')",
	"        }}",
	"        const result = Atomics.waitAsync(signal, 0, 1)",
	"        return result.async ? result.value : undefined",
	"    }}",
	"    if (signal[0] !== 1) return",
	"    const waits = this.#jsEmbed.js_sys['future.jspi.waits']",
	"    return new Promise(resolve => waits.set(state, resolve))",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "future.jspi.notify",
	required_embeds = [("js_sys", "future.jspi.waits")],
	"state => {{",
	"    const buffer = this.#memory.buffer",
	"    if (typeof SharedArrayBuffer !== 'undefined'",
	"        && buffer instanceof SharedArrayBuffer) {{",
	"        Atomics.notify(new Int32Array(buffer, state, 1), 0, 1)",
	"        return",
	"    }}",
	"    const waits = this.#jsEmbed.js_sys['future.jspi.waits']",
	"    const resolve = waits.get(state)",
	"    if (resolve === undefined) return",
	"    waits.delete(state)",
	"    resolve()",
	"}}",
);

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "future.jspi.suspend", suspending)]
	fn jspi_suspend(state: PtrConst<AtomicI32>);

	#[js_sys(js_embed = "future.jspi.notify")]
	fn jspi_notify(state: PtrConst<AtomicI32>);
}

pub(super) struct Signal {
	state: AtomicI32,
}

impl Signal {
	fn notify(&self) {
		if self.state.swap(AWAKE, Ordering::SeqCst) == WAITING {
			jspi_notify(PtrConst::from_ref(&self.state));
		}
	}

	pub(super) fn new() -> Arc<Self> {
		Arc::new(Self {
			state: AtomicI32::new(AWAKE),
		})
	}

	pub(super) fn waker(self: &Arc<Self>) -> Waker {
		Waker::from(Arc::clone(self))
	}

	pub(super) fn begin_poll(&self) {
		self.state.store(POLLING, Ordering::SeqCst);
	}

	pub(super) fn begin_wait(&self) -> bool {
		self.state
			.compare_exchange(POLLING, WAITING, Ordering::SeqCst, Ordering::SeqCst)
			.is_ok()
	}

	pub(super) fn suspend(&self) {
		jspi_suspend(PtrConst::from_ref(&self.state));
	}
}

impl Wake for Signal {
	fn wake(self: Arc<Self>) {
		self.notify();
	}

	fn wake_by_ref(self: &Arc<Self>) {
		self.notify();
	}
}
