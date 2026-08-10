use core::cell::RefCell;

#[cfg(target_feature = "atomics")]
mod atomic;
#[cfg(not(target_feature = "atomics"))]
mod single;

#[cfg(target_feature = "atomics")]
pub(super) use atomic::Task;
#[cfg(not(target_feature = "atomics"))]
pub(super) use single::Task;

pub(super) fn spawn(future: impl core::future::Future<Output = ()> + 'static) {
	Task::spawn(future);
}

struct ClearOnUnwind<'a, T> {
	value: &'a RefCell<Option<T>>,
	armed: bool,
}

impl<'a, T> ClearOnUnwind<'a, T> {
	fn new(value: &'a RefCell<Option<T>>) -> Self {
		Self { value, armed: true }
	}

	fn disarm(mut self) {
		self.armed = false;
	}
}

impl<T> Drop for ClearOnUnwind<'_, T> {
	fn drop(&mut self) {
		if self.armed {
			*self.value.borrow_mut() = None;
		}
	}
}
