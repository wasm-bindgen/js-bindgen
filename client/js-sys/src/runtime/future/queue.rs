use alloc::collections::VecDeque;
use alloc::rc::Rc;
use core::cell::{Cell, RefCell};

use super::task::Task;

js_bindgen::embed_js!(
	module = "js_sys",
	name = "future.schedule",
	"() => globalThis.queueMicrotask(this.#jsExports.future_poll)",
);

#[crate::js_sys(js_sys = crate)]
extern "js-sys" {
	#[js_sys(js_embed = "future.schedule")]
	fn schedule();
}

struct Queue {
	tasks: RefCell<VecDeque<Rc<Task>>>,
	scheduled: Cell<bool>,
}

impl Queue {
	const fn new() -> Self {
		Self {
			tasks: RefCell::new(VecDeque::new()),
			scheduled: Cell::new(false),
		}
	}

	fn push(&self, task: Rc<Task>) -> bool {
		self.tasks.borrow_mut().push_back(task);
		!self.scheduled.replace(true)
	}

	fn pop(&self) -> Option<Rc<Task>> {
		self.tasks.borrow_mut().pop_front()
	}

	fn begin_tick(&self) -> usize {
		self.scheduled.set(false);
		self.tasks.borrow().len()
	}

	fn reschedule(&self) -> bool {
		!self.tasks.borrow().is_empty() && !self.scheduled.replace(true)
	}
}

thread_local! {
	static QUEUE: Queue = const { Queue::new() };
}

pub(super) fn push(task: Rc<Task>) {
	if QUEUE.with(|queue| queue.push(task)) {
		schedule();
	}
}

struct RescheduleOnUnwind(bool);

impl RescheduleOnUnwind {
	fn new() -> Self {
		Self(true)
	}

	fn disarm(mut self) {
		self.0 = false;
	}
}

impl Drop for RescheduleOnUnwind {
	fn drop(&mut self) {
		if self.0 && QUEUE.with(Queue::reschedule) {
			schedule();
		}
	}
}

#[crate::js_sys(js_sys = crate)]
fn future_poll() {
	let guard = RescheduleOnUnwind::new();

	for _ in 0..QUEUE.with(Queue::begin_tick) {
		let Some(task) = QUEUE.with(Queue::pop) else {
			break;
		};
		task.run();
	}

	guard.disarm();
}
