mod export;
mod import;
mod r#macro;

use core::mem::MaybeUninit;

pub use export::*;
pub use import::*;
pub use js_bindgen_wire::abi::{JsCatch, JsEmbed, WatCatch};
pub use js_bindgen_wire::{
	Wire, WireBlob, WireExport, WireExportInput, WireExportInputType, WireExportOutput,
	WireExportOutputType, WireImport, WireImportBinding, WireImportCatch, WireImportInput,
	WireImportInputType, WireImportOutput, WireImportOutputType, WireImportTypeTable,
	wire_blob_len,
};
pub use r#macro::*;

// Text rendering.
pub use crate::const_concat;
use crate::hazard::{
	FromJS, IntoJS, ReturnFromJS, ReturnIntoJS, Slot, WasmAbi, WasmRet, WatConv, WatSlot,
};

pub(crate) const fn wat_slot<S: Slot>(conversion: Option<WatConv>) -> Option<WatSlot> {
	match S::WAT_TYPE {
		Some(rust) => Some(WatSlot::new(rust, conversion)),
		None => None,
	}
}

pub(crate) const fn into_js_slots<A: WasmAbi>() -> [Option<WatSlot>; 4] {
	[
		wat_slot::<A::Slot1>(<A::Slot1 as Slot>::INTO_JS_WAT_CONV),
		wat_slot::<A::Slot2>(<A::Slot2 as Slot>::INTO_JS_WAT_CONV),
		wat_slot::<A::Slot3>(<A::Slot3 as Slot>::INTO_JS_WAT_CONV),
		wat_slot::<A::Slot4>(<A::Slot4 as Slot>::INTO_JS_WAT_CONV),
	]
}

pub(crate) const fn from_js_slots<A: WasmAbi>() -> [Option<WatSlot>; 4] {
	[
		wat_slot::<A::Slot1>(<A::Slot1 as Slot>::FROM_JS_WAT_CONV),
		wat_slot::<A::Slot2>(<A::Slot2 as Slot>::FROM_JS_WAT_CONV),
		wat_slot::<A::Slot3>(<A::Slot3 as Slot>::FROM_JS_WAT_CONV),
		wat_slot::<A::Slot4>(<A::Slot4 as Slot>::FROM_JS_WAT_CONV),
	]
}
// Rust `ABI` shims used by generated import and export functions.

pub type InputSlot1<T> = <<T as IntoJS>::Abi as WasmAbi>::Slot1;
pub type InputSlot2<T> = <<T as IntoJS>::Abi as WasmAbi>::Slot2;
pub type InputSlot3<T> = <<T as IntoJS>::Abi as WasmAbi>::Slot3;
pub type InputSlot4<T> = <<T as IntoJS>::Abi as WasmAbi>::Slot4;

pub type FromJsSlot1<T> = <<T as FromJS>::Abi as WasmAbi>::Slot1;
pub type FromJsSlot2<T> = <<T as FromJS>::Abi as WasmAbi>::Slot2;
pub type FromJsSlot3<T> = <<T as FromJS>::Abi as WasmAbi>::Slot3;
pub type FromJsSlot4<T> = <<T as FromJS>::Abi as WasmAbi>::Slot4;

pub type OutputRet<T> = MaybeUninit<WasmRet<<T as ReturnFromJS>::Abi>>;

#[must_use]
#[inline]
pub fn split_input<T: IntoJS>(
	value: T,
) -> (InputSlot1<T>, InputSlot2<T>, InputSlot3<T>, InputSlot4<T>) {
	WasmAbi::split(T::into_abi(value))
}

#[must_use]
#[inline]
pub fn join_from_js<T: FromJS>(
	slot1: <T::Abi as WasmAbi>::Slot1,
	slot2: <T::Abi as WasmAbi>::Slot2,
	slot3: <T::Abi as WasmAbi>::Slot3,
	slot4: <T::Abi as WasmAbi>::Slot4,
) -> T {
	T::from_abi(T::Abi::join(slot1, slot2, slot3, slot4))
}

#[must_use]
#[inline]
pub fn return_to_js<T: ReturnIntoJS>(value: T) -> WasmRet<T::Abi> {
	WasmRet::from_abi(T::into_return_abi(value))
}

/// Lowers a value through a different [`IntoJS`] implementation with the same
/// `ABI`. This is reserved for generated `#[js_sys(type = ...)]` overrides,
/// where `T` must also describe the value's WAT and JavaScript conversions.
///
/// # Safety
///
/// The value's lowering must have the semantics expected by `T`; sharing an
/// `ABI` alone does not make two [`IntoJS`] implementations interchangeable.
#[must_use]
#[inline]
pub unsafe fn split_input_as<T: IntoJS>(
	value: impl IntoJS<Abi = T::Abi>,
) -> (InputSlot1<T>, InputSlot2<T>, InputSlot3<T>, InputSlot4<T>) {
	WasmAbi::split(IntoJS::into_abi(value))
}

#[must_use]
#[inline]
pub fn join_output<T: ReturnFromJS>(value: OutputRet<T>) -> T {
	T::from_return_abi(value)
}

/// Lifts a return value whose JavaScript conversion is described by another
/// type with the same `ABI`.
///
/// # Safety
///
/// The JavaScript value produced for `A` must have the semantics expected by
/// `T`; sharing an `ABI` alone does not make the conversions interchangeable.
#[must_use]
#[inline]
pub unsafe fn join_output_as<T, A>(value: OutputRet<A>) -> T
where
	T: ReturnFromJS<Abi = A::Abi>,
	A: ReturnFromJS,
{
	const {
		assert!(
			T::JS_CONV.is_result() == A::JS_CONV.is_result(),
			"return conversion overrides must preserve Result semantics",
		);
	}

	T::from_return_abi(value)
}
