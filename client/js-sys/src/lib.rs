#![no_std]
#![cfg_attr(
	all(target_feature = "atomics", not(feature = "std")),
	feature(thread_local)
)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::vec::Vec;
use core::cell::RefCell;
use core::marker::PhantomData;

use crate::hazard::Input;

#[cfg(not(target_feature = "reference-types"))]
compile_error!("`js-sys` requires the `reference-types` target feature");

macro_rules! thread_local {
    ($(static $name:ident: $ty:ty = $value:expr;)*) => {
        #[cfg(not(target_feature = "atomics"))]
        $(static $name: LocalKey<$ty> = LocalKey($value);)*
        #[cfg(all(target_feature = "atomics", not(feature = "std")))]
        $(
            #[thread_local]
            static $name: LocalKey<$ty> = LocalKey($value);
        )*
        #[cfg(all(target_feature = "atomics", feature = "std"))]
        ::std::thread_local! {
            $(static $name: $ty = $value;)*
        }
    };
}

#[cfg(not(all(target_feature = "atomics", feature = "std")))]
struct LocalKey<T: 'static>(T);

#[cfg(not(target_feature = "atomics"))]
unsafe impl<T: 'static> Send for LocalKey<T> {}

#[cfg(not(target_feature = "atomics"))]
unsafe impl<T: 'static> Sync for LocalKey<T> {}

#[cfg(not(all(target_feature = "atomics", feature = "std")))]
impl<T: 'static> LocalKey<T> {
	fn with<F, R>(&self, f: F) -> R
	where
		F: FnOnce(&T) -> R,
	{
		f(&self.0)
	}
}

pub mod hazard {
	/// # Safety
	///
	/// This directly interacts with the assembly generator and therefor all
	/// bets are off! (TODO)
	pub unsafe trait Input {
		const IMPORT_FUNC: &str;
		const IMPORT_TYPE: &str;
		const TYPE: &str;
		const CONV: &str;

		type Type;

		fn as_raw(&self) -> Self::Type;
	}
}

js_bindgen::unsafe_embed_asm!(
	".import_module js_sys.externref.table, js_sys",
	".import_name js_sys.externref.table, externref.table",
	".tabletype js_sys.externref.table, externref, 1",
	"",
	".globl js_sys.externref.grow",
	"js_sys.externref.grow:",
	"	.functype js_sys.externref.grow (i32) -> (i32)",
	"	ref.null_extern",
	"	local.get 0",
	"	table.grow js_sys.externref.table",
	"	end_function",
	"",
	".globl js_sys.externref.get",
	"js_sys.externref.get:",
	"	.functype js_sys.externref.get (i32) -> (externref)",
	"	local.get 0",
	"	table.get js_sys.externref.table",
	"	end_function",
	"",
	".globl js_sys.externref.remove",
	"js_sys.externref.remove:",
	"	.functype js_sys.externref.remove (i32) -> ()",
	"	local.get 0",
	"	ref.null_extern",
	"	table.set js_sys.externref.table",
	"	end_function",
);

js_bindgen::js_import!(
	name = "externref.table",
	"new WebAssembly.Table({{ initial: 1, element: \"externref\" }})",
);

extern "C" {
	#[link_name = "js_sys.externref.grow"]
	fn grow(size: i32) -> i32;
	#[link_name = "js_sys.externref.remove"]
	fn remove(index: i32);
}

pub struct JsValue {
	index: i32,
	_local: PhantomData<*const ()>,
}

impl JsValue {
	pub const UNDEFINED: Self = Self::new(0);

	const fn new(index: i32) -> Self {
		Self {
			index,
			_local: PhantomData,
		}
	}
}

impl Drop for JsValue {
	fn drop(&mut self) {
		if self.index > 0 {
			EXTERNREF_TABLE.with(|table| table.borrow_mut().remove(self.index));
		}
	}
}

unsafe impl Input for JsValue {
	const IMPORT_FUNC: &str = ".functype js_sys.externref.get (i32) -> (externref)";
	const IMPORT_TYPE: &str = "externref";
	const TYPE: &str = "i32";
	const CONV: &str = "call js_sys.externref.get";

	type Type = i32;

	fn as_raw(&self) -> Self::Type {
		self.index
	}
}

thread_local! {
	static EXTERNREF_TABLE: RefCell<Slab> = RefCell::new(Slab::new());
}

struct Slab {
	head: i32,
	empty: Vec<i32>,
}

impl Slab {
	const fn new() -> Self {
		Slab {
			head: 0,
			empty: Vec::new(),
		}
	}

	fn remove(&mut self, index: i32) {
		self.empty.push(index);
		unsafe { remove(index) }
	}
}
