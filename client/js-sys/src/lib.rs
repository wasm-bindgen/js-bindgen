#![feature(asm_experimental_arch)]
#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use core::cell::RefCell;
use core::marker::PhantomData;

use alloc::vec::Vec;
use js_bindgen::global_asm;

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
    fn with<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        f(&self.0)
    }
}

global_asm!(
    "js_sys.externref.table:",
    ".tabletype js_sys.externref.table, externref",
);

global_asm!(
    "js_sys.externref.const:",
    ".import_module js_sys.externref.const, js_sys",
    ".import_name js_sys.externref.const, const",
    ".tabletype js_sys.externref.const, externref, 1, 1",
);

global_asm!(
    "js_sys.externref.grow:",
    ".globl js_sys.externref.grow",
    #[cfg(target_pointer_width = "32")]
    ".functype js_sys.externref.grow (i32) -> (i32)",
    #[cfg(target_pointer_width = "64")]
    ".functype js_sys.externref.grow (i64) -> (i64)",
    "ref.null_extern",
    "local.get 0",
    "table.grow js_sys.externref.table",
    "end_function",
);

global_asm!(
    "js_sys.externref.get:",
    ".globl js_sys.externref.get",
    #[cfg(target_pointer_width = "32")]
    ".functype js_sys.externref.get (i32) -> (externref)",
    #[cfg(target_pointer_width = "64")]
    ".functype js_sys.externref.get (i64) -> (externref)",
    "local.get 0",
    #[cfg(target_pointer_width = "32")]
    "i32.const 0",
    #[cfg(target_pointer_width = "32")]
    "i32.ge_s",
    #[cfg(target_pointer_width = "64")]
    "i64.const 0",
    #[cfg(target_pointer_width = "64")]
    "i64.ge_s",
    "if externref",
    "local.get 0",
    "table.get js_sys.externref.table",
    "else",
    "local.get 0",
    #[cfg(target_pointer_width = "32")]
    "i32.const -1",
    #[cfg(target_pointer_width = "32")]
    "i32.mul",
    #[cfg(target_pointer_width = "32")]
    "i32.const 1",
    #[cfg(target_pointer_width = "32")]
    "i32.sub",
    #[cfg(target_pointer_width = "64")]
    "i64.const -1",
    #[cfg(target_pointer_width = "64")]
    "i64.mul",
    #[cfg(target_pointer_width = "64")]
    "i64.const 1",
    #[cfg(target_pointer_width = "64")]
    "i64.sub",
    "table.get js_sys.externref.const",
    "end_if",
    "end_function",
);

global_asm!(
    "js_sys.externref.remove:",
    ".globl js_sys.externref.remove",
    #[cfg(target_pointer_width = "32")]
    ".functype js_sys.externref.remove (i32) -> ()",
    #[cfg(target_pointer_width = "64")]
    ".functype js_sys.externref.remove (i64) -> ()",
    "local.get 0",
    "ref.null_extern",
    "table.set js_sys.externref.table",
    "end_function",
);

extern "C" {
    #[link_name = "js_sys.externref.grow"]
    fn grow(size: isize) -> isize;
    #[link_name = "js_sys.externref.remove"]
    fn remove(index: isize);
}

pub struct JsValue {
    index: isize,
    _local: PhantomData<*const ()>,
}

impl JsValue {
    pub const UNDEFINED: Self = Self::new(-1);

    const fn new(index: isize) -> Self {
        Self {
            index,
            _local: PhantomData,
        }
    }

    pub fn as_raw(&self) -> isize {
        self.index
    }
}

impl Drop for JsValue {
    fn drop(&mut self) {
        if self.index >= 0 {
            EXTERNREF_TABLE.with(|table| table.borrow_mut().remove(self.index))
        }
    }
}

thread_local! {
    static EXTERNREF_TABLE: RefCell<Slab> = RefCell::new(Slab::new());
}

struct Slab {
    head: isize,
    empty: Vec<isize>,
}

impl Slab {
    const fn new() -> Self {
        Slab {
            head: 0,
            empty: Vec::new(),
        }
    }

    fn remove(&mut self, index: isize) {
        self.empty.push(index);
        unsafe { remove(index) }
    }
}
