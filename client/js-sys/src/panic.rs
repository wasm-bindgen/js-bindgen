#[cfg(not(debug_assertions))]
use alloc::format;
use core::fmt::Debug;

pub trait UnwrapThrowExt<T> {
	#[track_caller]
	fn expect_throw(self, message: &str) -> T;

	#[track_caller]
	fn unwrap_throw(self) -> T;
}

#[cfg(debug_assertions)]
impl<T> UnwrapThrowExt<T> for Option<T> {
	fn expect_throw(self, message: &str) -> T {
		self.expect(message)
	}

	fn unwrap_throw(self) -> T {
		self.unwrap()
	}
}

#[cfg(not(debug_assertions))]
impl<T> UnwrapThrowExt<T> for Option<T> {
	fn expect_throw(self, message: &str) -> T {
		match self {
			Some(value) => value,
			None => panic(message),
		}
	}

	fn unwrap_throw(self) -> T {
		match self {
			Some(value) => value,
			None => panic("called `Option::unwrap_throw()` on a `None` value"),
		}
	}
}

#[cfg(debug_assertions)]
impl<T, E: Debug> UnwrapThrowExt<T> for Result<T, E> {
	fn expect_throw(self, message: &str) -> T {
		self.expect(message)
	}

	fn unwrap_throw(self) -> T {
		self.unwrap()
	}
}

#[cfg(not(debug_assertions))]
impl<T, E: Debug> UnwrapThrowExt<T> for Result<T, E> {
	fn expect_throw(self, message: &str) -> T {
		match self {
			Ok(value) => value,
			Err(error) => panic(&format!("{message}: {error:?}")),
		}
	}

	fn unwrap_throw(self) -> T {
		match self {
			Ok(value) => value,
			Err(error) => panic(&format!(
				"called `Result::unwrap()` on an `Err` value: {error:?}"
			)),
		}
	}
}

#[track_caller]
#[cfg(debug_assertions)]
pub fn panic(message: &str) -> ! {
	// TODO: actually throw.
	panic!("{message}");
}

#[track_caller]
#[cfg(not(debug_assertions))]
pub fn panic(_: &str) -> ! {
	// TODO: print message.
	#[cfg(target_arch = "wasm32")]
	core::arch::wasm32::unreachable();
	#[cfg(target_arch = "wasm64")]
	core::arch::wasm64::unreachable();
}
