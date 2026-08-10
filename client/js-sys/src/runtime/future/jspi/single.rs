use alloc::rc::Rc;
use core::cell::Cell;
use core::mem::ManuallyDrop;
use core::task::{RawWaker, RawWakerVTable, Waker};

use super::{AWAKE, POLLING, WAITING};
use crate::util::PtrConst;

js_bindgen::embed_js!(module = "js_sys", name = "future.jspi.waits", "new Map()");

js_bindgen::embed_js!(
	module = "js_sys",
	name = "future.jspi.suspend",
	required_embeds = [("js_sys", "future.jspi.waits")],
	"state => {{",
	"	const waits = this.#jsEmbed.js_sys['future.jspi.waits']",
	"	return new Promise(resolve => waits.set(state, resolve))",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "future.jspi.notify",
	required_embeds = [("js_sys", "future.jspi.waits")],
	"state => {{",
	"	const waits = this.#jsEmbed.js_sys['future.jspi.waits']",
	"	const resolve = waits.get(state)",
	"	if (resolve === undefined) return",
	"	waits.delete(state)",
	"	resolve()",
	"}}",
);

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "future.jspi.suspend", suspending)]
	fn jspi_suspend(state: PtrConst<Cell<i32>>);

	#[js_sys(js_embed = "future.jspi.notify")]
	fn jspi_notify(state: PtrConst<Cell<i32>>);
}

pub(super) struct Signal {
	state: Cell<i32>,
}

impl Signal {
	fn notify(&self) {
		if self.state.replace(AWAKE) == WAITING {
			jspi_notify(PtrConst::from_ref(&self.state));
		}
	}

	unsafe fn raw_waker(this: Rc<Self>) -> RawWaker {
		unsafe fn clone(pointer: *const ()) -> RawWaker {
			// SAFETY: Every pointer in this table comes from `Rc::into_raw`.
			let signal = ManuallyDrop::new(unsafe { Rc::from_raw(pointer.cast::<Signal>()) });
			// SAFETY: The clone becomes the ownership represented by the new
			// `RawWaker`.
			unsafe { Signal::raw_waker(Rc::clone(&signal)) }
		}

		unsafe fn wake(pointer: *const ()) {
			// SAFETY: `wake` consumes the ownership represented by this `Waker`.
			let signal = unsafe { Rc::from_raw(pointer.cast::<Signal>()) };
			signal.notify();
		}

		unsafe fn wake_by_ref(pointer: *const ()) {
			// SAFETY: `wake_by_ref` borrows the ownership represented by this
			// `Waker`.
			let signal = ManuallyDrop::new(unsafe { Rc::from_raw(pointer.cast::<Signal>()) });
			signal.notify();
		}

		unsafe fn drop(pointer: *const ()) {
			// SAFETY: `drop` consumes the ownership represented by this `Waker`.
			core::mem::drop(unsafe { Rc::from_raw(pointer.cast::<Signal>()) });
		}

		const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

		RawWaker::new(Rc::into_raw(this).cast(), &VTABLE)
	}

	pub(super) fn new() -> Rc<Self> {
		Rc::new(Self {
			state: Cell::new(AWAKE),
		})
	}

	pub(super) fn waker(self: &Rc<Self>) -> Waker {
		// SAFETY: The raw `Waker` owns this cloned `Rc`. This implementation is
		// only compiled for targets without Wasm `atomics`, so it cannot cross
		// threads.
		unsafe { Waker::from_raw(Self::raw_waker(Rc::clone(self))) }
	}

	pub(super) fn begin_poll(&self) {
		self.state.set(POLLING);
	}

	pub(super) fn begin_wait(&self) -> bool {
		if self.state.get() != POLLING {
			return false;
		}
		self.state.set(WAITING);
		true
	}

	pub(super) fn suspend(&self) {
		jspi_suspend(PtrConst::from_ref(&self.state));
	}
}
