use core::cell::Cell;

use super::externref;
use crate::JsValue;
#[cfg(not(target_feature = "exception-handling"))]
use crate::hazard::{JsCatch, JsEmbed};
#[cfg(target_feature = "exception-handling")]
use crate::hazard::{WatCatch, WatImport, WatImportKind, WatType};
use js_bindgen_wire::WireImportCatch;

#[cfg(not(target_feature = "exception-handling"))]
const JS_CATCH_EMBEDS: &[JsEmbed] = &[JsEmbed::new("js_sys", "externref.table")];
#[cfg(not(target_feature = "exception-handling"))]
const JS_DIRECT_CATCH: &str = "
    } catch ($error) {
        const $index = this.#jsExports['js_sys.exception.store']()
        this.#jsEmbed.js_sys['externref.table'].set($index, $error)
        return false
    }
}";
#[cfg(not(target_feature = "exception-handling"))]
const JS_INDIRECT_CATCH: &str = "
    } catch ($error) {
        const $index = this.#jsExports['js_sys.exception.store']()
        this.#jsEmbed.js_sys['externref.table'].set($index, $error)
    }
}";

#[cfg(target_feature = "exception-handling")]
const WAT_TAG_IMPORT: WatImport = WatImport::new(
	"js_sys",
	"exception.tag",
	"js_sys.exception.tag",
	Some("js_sys.exception.tag"),
	WatImportKind::Tag {
		parameters: &[WatType::ExternRef],
	},
);
#[cfg(target_feature = "exception-handling")]
const WAT_STORE_IMPORT: WatImport = WatImport::new(
	"env",
	"js_sys.exception.store",
	"js_sys.exception.store",
	None,
	WatImportKind::Function {
		parameters: &[WatType::I32],
		results: &[],
	},
);
#[cfg(target_feature = "exception-handling")]
const WAT_CATCH_IMPORTS: &[WatImport] = &[
	WAT_TAG_IMPORT,
	externref::WAT_TABLE_IMPORT,
	externref::WAT_NEXT_IMPORT,
	WAT_STORE_IMPORT,
];
#[cfg(target_feature = "exception-handling")]
const WAT_TRY: &str = "
  (block $js_sys.exception.catch (result externref)
    (try_table (catch $js_sys.exception.tag $js_sys.exception.catch) (@reloc)";
#[cfg(target_feature = "exception-handling")]
const WAT_CATCH: &str = "
      return
    )
    unreachable
  )
  local.set $js_sys.externref.value
  call $js_sys.externref.next (@reloc)
  local.tee $js_sys.externref.index
  local.get $js_sys.externref.value
  table.set $js_sys.import.externref.table (@reloc)
  local.get $js_sys.externref.index
  call $js_sys.exception.store (@reloc)";

#[cfg(not(target_feature = "exception-handling"))]
pub(crate) const IMPORT_CATCH: WireImportCatch = WireImportCatch::JavaScript(JsCatch::new(
	JS_CATCH_EMBEDS,
	JS_DIRECT_CATCH,
	JS_INDIRECT_CATCH,
));
#[cfg(target_feature = "exception-handling")]
pub(crate) const IMPORT_CATCH: WireImportCatch = WireImportCatch::Wasm(WatCatch::new(
	WAT_CATCH_IMPORTS,
	externref::WAT_INSERT_LOCALS,
	WAT_TRY,
	WAT_CATCH,
));

thread_local! {
	static EXCEPTION: Cell<i32> = const { Cell::new(0) };
}

#[cfg(target_feature = "exception-handling")]
js_bindgen::import_js!(
	module = "js_sys",
	name = "exception.tag",
	"WebAssembly.JSTag",
);

fn set(index: i32) {
	EXCEPTION.with(|exception| {
		debug_assert_eq!(exception.get(), 0);
		exception.set(index);
	});
}

/// Stores the `externref` table index for an exception caught by Wasm.
#[cfg(target_feature = "exception-handling")]
#[unsafe(export_name = "js_sys.exception.store")]
extern "C" fn store(index: i32) {
	set(index);
}

/// Reserves an `externref` table entry for an exception caught by JavaScript.
///
/// JavaScript fills the returned table entry before returning to Wasm.
#[cfg(not(target_feature = "exception-handling"))]
#[unsafe(export_name = "js_sys.exception.store")]
extern "C" fn store() -> i32 {
	let index = externref::reserve();
	set(index);
	index
}

pub(crate) fn take() -> Option<JsValue> {
	let index = EXCEPTION.with(Cell::take);
	(index != 0).then(|| JsValue::new(index))
}
