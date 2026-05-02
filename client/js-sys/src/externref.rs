use alloc::vec::Vec;
use core::cell::RefCell;

use crate::panic::panic;
use crate::util::PtrConst;

js_bindgen::unsafe_global_wat!(
	"(import \"js_sys\" \"externref.table\" (table $js_sys.import.externref.table (@sym (name \
	 \"js_sys.import.externref.table\")) 1 externref))",
	"(import \"env\" \"js_sys.externref.next\" (func $js_sys.externref.next (@sym) (result i32)))",
	"(func $js_sys.externref.grow (@sym) (param i32) (result i32)",
	"  ref.null extern",
	"  local.get 0",
	"  table.grow $js_sys.import.externref.table (@reloc)",
	")",
	"(func $js_sys.externref.insert (@sym) (param externref) (result i32)",
	"  (local i32)",
	"  call $js_sys.externref.next (@reloc)",
	"  local.tee 1",
	"  local.get 0",
	"  table.set $js_sys.import.externref.table (@reloc)",
	"  local.get 1",
	")",
	"(func $js_sys.externref.get (@sym) (param i32) (result externref)",
	"  local.get 0",
	"  table.get $js_sys.import.externref.table (@reloc)",
	")",
	"(func $js_sys.externref.remove (@sym) (param i32)",
	"  local.get 0",
	"  ref.null extern",
	"  table.set $js_sys.import.externref.table (@reloc)",
	")",
);

js_bindgen::embed_js!(
	module = "js_sys",
	name = "externref.table",
	"(() => {{",
	"	const table = new WebAssembly.Table({{ initial: 2, element: 'externref' }})",
	"	table.set(1, null)",
	"	return table",
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

thread_local! {
	pub(crate) static EXTERNREF_TABLE: RefCell<ExternrefTable> = RefCell::new(ExternrefTable::new());
}

pub(crate) struct ExternrefTable(Vec<i32>);

pub(crate) struct ExternrefTablePtr {
	pub(crate) ptr: PtrConst<i32>,
	pub(crate) len: i32,
}

impl ExternrefTable {
	const fn new() -> Self {
		Self(Vec::new())
	}

	fn next(&mut self) -> i32 {
		if let Some(slot) = self.0.pop() {
			slot
		} else {
			match grow(1) {
				-1 => panic("`externref` table allocation failure"),
				slot => slot,
			}
		}
	}

	pub(crate) fn remove(&mut self, index: i32) {
		self.0.try_reserve(1).expect("failure to grow memory");

		self.0.push(index);
		remove(index);
	}

	/// Export a pointer and length to the current list.
	///
	/// # Safety
	///
	/// Reading from that pointer and length is only valid as long as the list
	/// is not modified.
	pub(crate) fn current_ptr() -> ExternrefTablePtr {
		EXTERNREF_TABLE.with(|table| {
			let table = &table.try_borrow().unwrap().0;

			ExternrefTablePtr {
				ptr: PtrConst::new(table),
				len: table.len().try_into().unwrap(),
			}
		})
	}

	/// When using empty slots through [`ExternrefTablePtr`], we report back how
	/// many we used.
	pub(crate) fn report_used_slots(slots: usize) {
		EXTERNREF_TABLE.with(|table| {
			let mut table = table.try_borrow_mut().unwrap();
			let new_len = table.0.len().saturating_sub(slots);
			table.0.truncate(new_len);
		});
	}
}

#[unsafe(export_name = "js_sys.externref.next")]
extern "C" fn next() -> i32 {
	EXTERNREF_TABLE.with(|table| table.try_borrow_mut().unwrap().next())
}
