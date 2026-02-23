use alloc::vec::Vec;
use core::cell::RefCell;

use crate::panic::panic;
use crate::util::PtrLength;

macro_rules! thread_local {
    ($($vis:vis static $name:ident: $ty:ty = $value:expr;)*) => {
        #[cfg_attr(target_feature = "atomics", thread_local)]
        $($vis static $name: LocalKey<$ty> = LocalKey($value);)*
    };
}

pub(crate) struct LocalKey<T>(T);

// SAFETY: Multi-threading is not possible without `atomics`.
#[cfg(not(target_feature = "atomics"))]
unsafe impl<T> Send for LocalKey<T> {}

// SAFETY: Multi-threading is not possible without `atomics`.
#[cfg(not(target_feature = "atomics"))]
unsafe impl<T> Sync for LocalKey<T> {}

impl<T> LocalKey<T> {
	pub(crate) fn with<F, R>(&self, f: F) -> R
	where
		F: FnOnce(&T) -> R,
	{
		f(&self.0)
	}
}

js_bindgen::unsafe_embed_asm!(
	".import_module js_sys.externref.table, js_sys",
	".import_name js_sys.externref.table, externref.table",
	".tabletype js_sys.externref.table, externref, 1",
	"",
	".functype js_sys.externref.next () -> (i32)",
	"",
	".globl js_sys.externref.grow",
	"js_sys.externref.grow:",
	"	.functype js_sys.externref.grow (i32) -> (i32)",
	"	ref.null_extern",
	"	local.get 0",
	"	table.grow js_sys.externref.table",
	"	end_function",
	"",
	".globl js_sys.externref.insert",
	"js_sys.externref.insert:",
	"	.functype js_sys.externref.insert (externref) -> (i32)",
	"	.local i32",
	"	call js_sys.externref.next",
	"   local.tee 1",
	"	local.get 0",
	"	table.set js_sys.externref.table",
	"	local.get 1",
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

js_bindgen::embed_js!(
	name = "externref.table",
	"(() => {{",
	"	const table = new WebAssembly.Table({{ initial: 2, element: 'externref' }})",
	"	table.set(1, null)",
	"	return table",
	"}})()"
);

js_bindgen::import_js!(
	name = "externref.table",
	required_embed = "externref.table",
	"this.#jsEmbed.js_sys['externref.table']",
);

unsafe extern "C" {
	#[link_name = "js_sys.externref.grow"]
	fn grow(size: i32) -> i32;
	#[link_name = "js_sys.externref.remove"]
	fn remove(index: i32);
}

thread_local! {
	pub(crate) static EXTERNREF_TABLE: RefCell<Slab> = RefCell::new(Slab::new());
}

pub(crate) struct Slab(Vec<i32>);

impl Slab {
	const fn new() -> Self {
		Self(Vec::new())
	}

	fn next(&mut self) -> i32 {
		if let Some(slot) = self.0.pop() {
			slot
		} else {
			// SAFETY: Implementation is safe.
			match unsafe { grow(1) } {
				-1 => panic("`externref` table allocation failure"),
				slot => slot,
			}
		}
	}

	pub(crate) fn remove(&mut self, index: i32) {
		self.0.try_reserve(1).expect("failure to grow memory");

		self.0.push(index);
		// SAFETY: Implementation is safe.
		unsafe { remove(index) }
	}
}

#[unsafe(export_name = "js_sys.externref.next")]
extern "C" fn next() -> i32 {
	EXTERNREF_TABLE.with(|slab| slab.try_borrow_mut().unwrap().next())
}

pub(crate) struct ExternrefTable;

impl ExternrefTable {
	pub(crate) fn current_into() -> ExternrefTableInfo {
		let slab = &EXTERNREF_TABLE.0.try_borrow().unwrap().0;

		ExternrefTableInfo {
			ptr: slab.as_ptr(),
			len: PtrLength::new(slab),
		}
	}

	pub(crate) fn report_growth(size: usize) {
		EXTERNREF_TABLE.0.try_borrow_mut().unwrap().0.truncate(size);
	}
}

pub(crate) struct ExternrefTableInfo {
	pub(crate) ptr: *const i32,
	pub(crate) len: PtrLength,
}
