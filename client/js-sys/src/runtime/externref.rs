use alloc::vec::Vec;
use core::cell::RefCell;
use core::mem;

use super::allocator;
use super::panic::panic;
use crate::JsValue;
use crate::hazard::{JsCast, RefType, WatImport, WatImportKind, WatIndexType, WatLocal, WatType};
use crate::util::{PtrConst, PtrLength};

pub(crate) const WAT_TABLE_IMPORT: WatImport = WatImport::new(
	"js_sys",
	"externref.table",
	"js_sys.import.externref.table",
	Some("js_sys.externref.table"),
	WatImportKind::Table {
		index_type: WatIndexType::I32,
		minimum: 2,
		maximum: None,
		element: RefType::ExternRef,
	},
);
pub(crate) const WAT_NEXT_IMPORT: WatImport = WatImport::new(
	"env",
	"js_sys.externref.next",
	"js_sys.externref.next",
	None,
	WatImportKind::Function {
		parameters: &[],
		results: &[WatType::I32],
	},
);
const WAT_RELEASE_IMPORT: WatImport = WatImport::new(
	"env",
	"js_sys.externref.release",
	"js_sys.externref.release",
	None,
	WatImportKind::Function {
		parameters: &[WatType::I32],
		results: &[],
	},
);
const WAT_VALUE_LOCAL: WatLocal = WatLocal::new("js_sys.externref.value", WatType::ExternRef);
pub(crate) const WAT_INDEX_LOCAL: WatLocal = WatLocal::new("js_sys.externref.index", WatType::I32);
pub(crate) const WAT_TABLE_IMPORTS: &[WatImport] = &[WAT_TABLE_IMPORT];
pub(crate) const WAT_INSERT_IMPORTS: &[WatImport] = &[WAT_TABLE_IMPORT, WAT_NEXT_IMPORT];
pub(crate) const WAT_TAKE_IMPORTS: &[WatImport] = &[WAT_TABLE_IMPORT, WAT_RELEASE_IMPORT];
pub(crate) const WAT_INSERT_LOCALS: &[WatLocal] = &[WAT_VALUE_LOCAL, WAT_INDEX_LOCAL];
pub(crate) const WAT_INSERT_CONV: &str = "\
  local.set $js_sys.externref.value
  call $js_sys.externref.next (@reloc)
  local.tee $js_sys.externref.index
  local.get $js_sys.externref.value
  table.set $js_sys.import.externref.table (@reloc)
  local.get $js_sys.externref.index";
pub(crate) const WAT_OPTIONAL_INSERT_CONV: &str = "\
  local.set $js_sys.externref.value
  local.get $js_sys.externref.value
  ref.is_null
  if (result i32)
    i32.const 0
  else
    call $js_sys.externref.next (@reloc)
    local.tee $js_sys.externref.index
    local.get $js_sys.externref.value
    table.set $js_sys.import.externref.table (@reloc)
    local.get $js_sys.externref.index
  end";
pub(crate) const WAT_GET_CONV: &str = "table.get $js_sys.import.externref.table (@reloc)";
pub(crate) const WAT_TAKE_CONV: &str = "\
  local.tee $js_sys.externref.index
  table.get $js_sys.import.externref.table (@reloc)
  local.get $js_sys.externref.index
  i32.const 2
  i32.ge_u
  if
    local.get $js_sys.externref.index
    call $js_sys.externref.release (@reloc)
  end";

js_bindgen::unsafe_global_wat!(
	// Imports need an explicit name.
	// See https://github.com/llvm/llvm-project/issues/198509.
	"(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
	 \"js_sys.externref.table\")) 2 externref))",
	"(func $js_sys.externref.grow (@sym) (param $size i32) (result i32)",
	"  ref.null extern",
	"  local.get $size",
	"  table.grow $js_sys.import.externref.table (@reloc)",
	")",
	"(func $js_sys.externref.remove (@sym) (param $index i32)",
	"  local.get $index",
	"  ref.null extern",
	"  table.set $js_sys.import.externref.table (@reloc)",
	")",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "externref.table",
	"(() => {{",
	"    const table = new WebAssembly.Table({{ initial: 2, element: 'externref' }})",
	"    table.set(1, null)",
	"    return table",
	"}})()"
);

js_bindgen::import_js!(
	module = "js_sys",
	name = "externref.table",
	required_embeds = [("js_sys", "externref.table")],
	"this.#jsEmbed.js_sys['externref.table']",
);

unsafe extern "C" {
	#[link_name = "js_sys.externref.grow"]
	safe fn grow(size: i32) -> i32;
	#[link_name = "js_sys.externref.remove"]
	safe fn remove(index: i32);
}

struct Slab {
	data: Vec<usize>,
	head: usize,
	base: usize,
	table_len: usize,
}

impl Slab {
	const fn new() -> Self {
		Self {
			data: Vec::new(),
			head: 0,
			base: 0,
			table_len: 0,
		}
	}

	// `js-sys` is linked as a separate crate. Without forced `inlining`, each
	// `externref` conversion retains an extra Wasm function call.
	#[expect(
		clippy::inline_always,
		reason = "avoids a call in every externref conversion"
	)]
	#[inline(always)]
	fn alloc(&mut self) -> usize {
		let slot = self.head;
		if slot == self.data.len() {
			let len = self.data.len();
			if len == self.table_len {
				let additional = len.max(128);
				let first = grow(index_to_abi(additional));
				if first == -1 {
					panic("`externref` table allocation failure");
				}

				let first = index_from_abi(first);
				if self.base == 0 {
					self.base = first;
				} else if self.base + self.table_len != first {
					panic("non-contiguous `externref` table growth");
				}

				if self.data.try_reserve_exact(additional).is_err() {
					panic("`externref` slab allocation failure");
				}
				self.table_len += additional;
			}

			if self.data.len() >= self.table_len {
				panic("`externref` slab capacity mismatch");
			}
			self.data.push(slot + 1);
		}

		match self.data.get_mut(slot) {
			Some(next) => self.head = *next,
			None => panic("`externref` slot out of bounds"),
		}

		slot + self.base
	}

	#[expect(
		clippy::inline_always,
		reason = "avoids a call in every externref conversion"
	)]
	#[inline(always)]
	fn dealloc(&mut self, index: usize) {
		if index < self.base {
			panic("attempted to free a reserved `externref` slot");
		}
		let slot = index - self.base;

		match self.data.get_mut(slot) {
			Some(next) => {
				*next = self.head;
				self.head = slot;
			}
			None => panic("`externref` slot out of bounds"),
		}
	}
}

// Replacing `RefCell` with `UnsafeCell` makes the `JsValue` benchmarks about
// 4-8% faster, but `RefCell` detects accidental nested access on one thread.
thread_local! {
	static EXTERNREF_SLAB: RefCell<Slab> = const { RefCell::new(Slab::new()) };
}

pub(crate) struct ReservedSlots {
	slots: Vec<i32>,
	committed: bool,
}

impl ReservedSlots {
	pub(crate) fn ptr(&self) -> PtrConst<i32> {
		PtrConst::new(&self.slots)
	}

	pub(crate) fn len(&self) -> PtrLength<i32> {
		PtrLength::new(&self.slots)
	}

	pub(crate) fn commit(mut self) {
		self.committed = true;
	}

	/// Moves the values stored in the reserved slots into an initialized slice,
	/// dropping every replaced value through its Rust type.
	pub(crate) fn replace<T: JsCast>(mut self, destination: &mut [T]) {
		assert_eq!(self.slots.len(), destination.len());

		// Popping in reverse preserves the original slot order without shifting
		// the vector on every replacement.
		for value in destination.iter_mut().rev() {
			let index = self.slots.pop().unwrap();
			let old = mem::replace(value, T::unchecked_from(JsValue::new(index)));
			drop(old);
		}

		self.committed = true;
	}
}

impl Drop for ReservedSlots {
	fn drop(&mut self) {
		if !self.committed {
			let mut slab = EXTERNREF_SLAB.0.borrow_mut();
			for &index in &self.slots {
				remove(index);
				slab.dealloc(index_from_abi(index));
			}
		}
	}
}

pub(crate) fn reserve_slots(count: usize) -> ReservedSlots {
	let mut slots = Vec::new();
	slots
		.try_reserve_exact(count)
		.expect("failure to grow memory");
	let mut reserved = ReservedSlots {
		slots,
		committed: false,
	};

	let mut slab = EXTERNREF_SLAB.0.borrow_mut();
	while reserved.slots.len() < count {
		reserved.slots.push(index_to_abi(slab.alloc()));
	}

	reserved
}

#[unsafe(export_name = "js_sys.externref.reserve_slice")]
extern "C" fn reserve_slice(len: usize) -> *mut i32 {
	let ptr: *mut i32 = allocator::allocate_slice(len);
	// SAFETY: The allocator provides writable storage for exactly `len` indices.
	let slots =
		unsafe { core::slice::from_raw_parts_mut(ptr.cast::<mem::MaybeUninit<i32>>(), len) };
	let mut slab = EXTERNREF_SLAB.0.borrow_mut();
	for slot in slots {
		slot.write(index_to_abi(slab.alloc()));
	}
	ptr
}

#[unsafe(export_name = "js_sys.externref.recycle_slice")]
unsafe extern "C" fn recycle_slice(ptr: *const i32, len: usize) {
	// SAFETY: The caller provides exactly `len` initialized table indices and
	// transfers ownership of each one.
	let slots = unsafe { core::slice::from_raw_parts(ptr, len) };
	let mut slab = EXTERNREF_SLAB.0.borrow_mut();
	for &index in slots {
		if u32::from_ne_bytes(index.to_ne_bytes()) >= 2 {
			remove(index);
			slab.dealloc(index_from_abi(index));
		}
	}
	drop(slab);
	// SAFETY: The caller transfers the exact allocation returned by
	// `reserve_slice` or an owned `JsValue` slice with the same representation.
	unsafe { allocator::release_slice(ptr.cast_mut(), len) };
}

#[cfg(not(target_feature = "exception-handling"))]
#[inline]
pub(crate) fn reserve() -> i32 {
	next()
}

#[unsafe(export_name = "js_sys.externref.release")]
pub(crate) extern "C" fn release(index: i32) {
	remove(index);
	EXTERNREF_SLAB.0.borrow_mut().dealloc(index_from_abi(index));
}

#[unsafe(export_name = "js_sys.externref.next")]
extern "C" fn next() -> i32 {
	index_to_abi(EXTERNREF_SLAB.0.borrow_mut().alloc())
}

#[inline]
fn index_to_abi(index: usize) -> i32 {
	let index =
		u32::try_from(index).unwrap_or_else(|_| panic("`externref` table capacity overflow"));
	i32::from_ne_bytes(index.to_ne_bytes())
}

#[inline]
fn index_from_abi(index: i32) -> usize {
	usize::try_from(u32::from_ne_bytes(index.to_ne_bytes())).unwrap()
}
