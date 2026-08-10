use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::{Cell, RefCell};
use core::future::Future;
use core::mem::ManuallyDrop;
use core::pin::Pin;
use core::task::{Context, RawWaker, RawWakerVTable, Waker};

use super::ClearOnUnwind;
use crate::runtime::future::queue;

struct TaskState {
	future: Pin<Box<dyn Future<Output = ()>>>,
	waker: Waker,
}

pub(in crate::runtime::future) struct Task {
	state: RefCell<Option<TaskState>>,
	queued: Cell<bool>,
}

impl Task {
	pub(super) fn spawn(future: impl Future<Output = ()> + 'static) {
		let task = Rc::new(Self {
			state: RefCell::new(None),
			queued: Cell::new(true),
		});
		// SAFETY: This target has no Wasm `atomics`, so its `Waker` cannot cross
		// threads. The raw `Waker` owns this cloned `Rc`.
		let waker = unsafe { Waker::from_raw(Self::raw_waker(Rc::clone(&task))) };
		*task.state.borrow_mut() = Some(TaskState {
			future: Box::pin(future),
			waker,
		});
		queue::push(task);
	}

	fn wake(task: Rc<Self>) {
		if !task.queued.replace(true) {
			queue::push(task);
		}
	}

	fn wake_by_ref(task: &Rc<Self>) {
		if !task.queued.replace(true) {
			queue::push(Rc::clone(task));
		}
	}

	unsafe fn raw_waker(task: Rc<Self>) -> RawWaker {
		unsafe fn clone(pointer: *const ()) -> RawWaker {
			// SAFETY: Every pointer in this table comes from `Rc::into_raw`.
			let task = ManuallyDrop::new(unsafe { Rc::from_raw(pointer.cast::<Task>()) });
			// SAFETY: The clone becomes the ownership represented by the new
			// `RawWaker`.
			unsafe { Task::raw_waker(Rc::clone(&task)) }
		}

		unsafe fn wake(pointer: *const ()) {
			// SAFETY: `wake` consumes the ownership represented by this `Waker`.
			Task::wake(unsafe { Rc::from_raw(pointer.cast::<Task>()) });
		}

		unsafe fn wake_by_ref(pointer: *const ()) {
			// SAFETY: `wake_by_ref` borrows the ownership represented by this
			// `Waker`.
			let task = ManuallyDrop::new(unsafe { Rc::from_raw(pointer.cast::<Task>()) });
			Task::wake_by_ref(&task);
		}

		unsafe fn drop(pointer: *const ()) {
			// SAFETY: `drop` consumes the ownership represented by this `Waker`.
			core::mem::drop(unsafe { Rc::from_raw(pointer.cast::<Task>()) });
		}

		const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

		RawWaker::new(Rc::into_raw(task).cast(), &VTABLE)
	}

	pub(in crate::runtime::future) fn run(&self) {
		let guard = ClearOnUnwind::new(&self.state);
		let mut slot = self.state.borrow_mut();
		let Some(task_state) = slot.as_mut() else {
			guard.disarm();
			return;
		};

		self.queued.set(false);
		let mut context = Context::from_waker(&task_state.waker);
		if task_state.future.as_mut().poll(&mut context).is_ready() {
			*slot = None;
		}

		guard.disarm();
	}
}
