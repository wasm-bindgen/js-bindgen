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

js_bindgen::embed_asm!(
	"js_sys.externref.table:",
	"    .tabletype js_sys.externref.table, externref",
	"",
	".import_module js_sys.externref.const, js_sys",
	".import_name js_sys.externref.const, const",
	".tabletype js_sys.externref.const, externref",
	"",
	".globl js_sys.externref.grow",
	"js_sys.externref.grow:",
	"    .functype js_sys.externref.grow (i32) -> (i32)",
	"    ref.null_extern",
	"    local.get 0",
	"    table.grow js_sys.externref.table",
	"    end_function",
	"",
	".globl js_sys.externref.get",
	"js_sys.externref.get:",
	"    .functype js_sys.externref.get (i32) -> (externref)",
	"    local.get 0",
	"    i32.const 0",
	"    i32.ge_s",
	"    if externref",
	"        local.get 0",
	"        table.get js_sys.externref.table",
	"    else",
	"        local.get 0",
	"        i32.const -1",
	"        i32.mul",
	"        i32.const 1",
	"        i32.sub",
	"        table.get js_sys.externref.const",
	"    end_if",
	"    end_function",
	"",
	".globl js_sys.externref.remove",
	"js_sys.externref.remove:",
	"    .functype js_sys.externref.remove (i32) -> ()",
	"    local.get 0",
	"    ref.null_extern",
	"    table.set js_sys.externref.table",
	"    end_function",
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
	pub const UNDEFINED: Self = Self::new(-1);

	const fn new(index: i32) -> Self {
		Self {
			index,
			_local: PhantomData,
		}
	}

	pub fn as_raw(&self) -> i32 {
		self.index
	}
}

impl Drop for JsValue {
	fn drop(&mut self) {
		if self.index >= 0 {
			EXTERNREF_TABLE.with(|table| table.borrow_mut().remove(self.index));
		}
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
