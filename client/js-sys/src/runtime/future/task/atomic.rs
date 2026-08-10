use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use core::cell::RefCell;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicI32, Ordering};
use core::task::{Context, Waker};

use super::ClearOnUnwind;
use crate::Closure;
use crate::runtime::future::queue;
use crate::util::PtrConst;

const SLEEPING: i32 = 0;
const AWAKE: i32 = 1;

js_bindgen::embed_js!(
	module = "js_sys",
	name = "future.atomic.state",
	"({{ buffer: undefined, view: undefined, waits: new Map() }})",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "future.atomic.wait",
	required_embeds = [("js_sys", "future.atomic.state")],
	"(state, awake, resume) => {{",
	"	const atomic = this.#jsEmbed.js_sys['future.atomic.state']",
	"	if (awake) {{",
	"		globalThis.queueMicrotask(resume)",
	"		return",
	"	}}",
	"	const buffer = this.#memory.buffer",
	"	if (typeof SharedArrayBuffer === 'undefined'",
	"		|| !(buffer instanceof SharedArrayBuffer)) {{",
	"		atomic.waits.set(state, resume)",
	"		return",
	"	}}",
	"	if (typeof Atomics.waitAsync !== 'function') {{",
	"		throw new Error('Wasm atomics futures require Atomics.waitAsync')",
	"	}}",
	"	if (buffer !== atomic.buffer) {{",
	"		atomic.buffer = buffer",
	"		atomic.view = new Int32Array(buffer)",
	"	}}",
	"	const result = Atomics.waitAsync(",
	"			atomic.view,",
	"			state / Int32Array.BYTES_PER_ELEMENT,",
	"			0,",
	"		)",
	"	if (result.async) result.value.then(resume)",
	"	else globalThis.queueMicrotask(resume)",
	"}}",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "future.atomic.notify",
	required_embeds = [("js_sys", "future.atomic.state")],
	"state => {{",
	"	const atomic = this.#jsEmbed.js_sys['future.atomic.state']",
	"	const buffer = this.#memory.buffer",
	"	if (typeof SharedArrayBuffer !== 'undefined'",
	"		&& buffer instanceof SharedArrayBuffer) {{",
	"		if (buffer !== atomic.buffer) {{",
	"			atomic.buffer = buffer",
	"			atomic.view = new Int32Array(buffer)",
	"		}}",
	"		Atomics.notify(",
	"			atomic.view,",
	"			state / Int32Array.BYTES_PER_ELEMENT,",
	"			1,",
	"		)",
	"		return",
	"	}}",
	"	const waits = atomic.waits",
	"	const resume = waits.get(state)",
	"	if (resume === undefined) return",
	"	waits.delete(state)",
	"	globalThis.queueMicrotask(resume)",
	"}}",
);

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "future.atomic.wait")]
	fn wait(state: PtrConst<AtomicI32>, awake: bool, resume: &Closure<dyn FnMut()>);

	#[js_sys(js_embed = "future.atomic.notify")]
	fn notify(state: PtrConst<AtomicI32>);
}

struct Wake {
	state: AtomicI32,
}

impl Wake {
	fn new() -> Arc<Self> {
		Arc::new(Self {
			state: AtomicI32::new(AWAKE),
		})
	}

	fn signal(&self) {
		if self.state.swap(AWAKE, Ordering::SeqCst) == AWAKE {
			return;
		}

		notify(PtrConst::from_ref(&self.state));
	}
}

impl alloc::task::Wake for Wake {
	fn wake(self: Arc<Self>) {
		self.signal();
	}

	fn wake_by_ref(self: &Arc<Self>) {
		self.signal();
	}
}

struct TaskState {
	future: Pin<Box<dyn Future<Output = ()>>>,
	waker: Waker,
	resume: Closure<dyn FnMut()>,
}

pub(in crate::runtime::future) struct Task {
	state: RefCell<Option<TaskState>>,
	wake: Arc<Wake>,
}

impl Task {
	pub(super) fn spawn(future: impl Future<Output = ()> + 'static) {
		let wake = Wake::new();
		let waker = Waker::from(Arc::clone(&wake));
		let task = Rc::new(Self {
			state: RefCell::new(None),
			wake,
		});
		let resumed_task = Rc::clone(&task);
		let resume = crate::closure!(js_sys = crate, dyn FnMut(), move || {
			// A delayed notification from the preceding wait may arrive after a
			// new wait starts. Normalize the state before polling in either case.
			resumed_task.wake.signal();
			resumed_task.run();
		});
		*task.state.borrow_mut() = Some(TaskState {
			future: Box::pin(future),
			waker,
			resume,
		});
		queue::push(task);
	}

	fn wait(&self, resume: &Closure<dyn FnMut()>) {
		wait(
			PtrConst::from_ref(&self.wake.state),
			self.wake.state.load(Ordering::SeqCst) == AWAKE,
			resume,
		);
	}

	pub(in crate::runtime::future) fn run(self: &Rc<Self>) {
		let guard = ClearOnUnwind::new(&self.state);
		let mut slot = self.state.borrow_mut();
		let Some(task_state) = slot.as_mut() else {
			guard.disarm();
			return;
		};

		let previous = self.wake.state.swap(SLEEPING, Ordering::SeqCst);
		debug_assert_eq!(previous, AWAKE);
		let mut context = Context::from_waker(&task_state.waker);

		if task_state.future.as_mut().poll(&mut context).is_ready() {
			*slot = None;
		} else {
			self.wait(&task_state.resume);
		}

		guard.disarm();
	}
}
