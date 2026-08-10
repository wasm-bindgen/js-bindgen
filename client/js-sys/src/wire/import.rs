use crate::hazard::{IntoJS, ReturnAbi, ReturnFromJS, Slot};
use crate::util::PtrMut;
use crate::wire::{
	InputSlot1, InputSlot2, InputSlot3, InputSlot4, OutputSlot1, OutputSlot2, OutputSlot3,
	OutputSlot4, WireImportCatch, WireImportInputType, WireImportOutputType, wat_slot,
};

trait MetadataFor<T>: 'static {
	const VALUE: &'static Self;
}

impl<T: IntoJS> MetadataFor<T> for WireImportInputType {
	const VALUE: &'static Self = &Self::new(
		[
			wat_slot::<InputSlot1<T>>(<InputSlot1<T> as Slot>::INTO_JS_WAT_CONV),
			wat_slot::<InputSlot2<T>>(<InputSlot2<T> as Slot>::INTO_JS_WAT_CONV),
			wat_slot::<InputSlot3<T>>(<InputSlot3<T> as Slot>::INTO_JS_WAT_CONV),
			wat_slot::<InputSlot4<T>>(<InputSlot4<T> as Slot>::INTO_JS_WAT_CONV),
		],
		T::JS_CONV,
	);
}

impl<T: ReturnFromJS> MetadataFor<T> for WireImportOutputType {
	const VALUE: &'static Self = &{
		Self::new(
			<T::Abi as ReturnAbi>::MODE,
			T::JS_CONV,
			T::JS_SRET,
			[
				wat_slot::<OutputSlot1<T>>(<OutputSlot1<T> as Slot>::FROM_JS_WAT_CONV),
				wat_slot::<OutputSlot2<T>>(<OutputSlot2<T> as Slot>::FROM_JS_WAT_CONV),
				wat_slot::<OutputSlot3<T>>(<OutputSlot3<T> as Slot>::FROM_JS_WAT_CONV),
				wat_slot::<OutputSlot4<T>>(<OutputSlot4<T> as Slot>::FROM_JS_WAT_CONV),
			],
		)
	};
}

/// Returns the shared wire descriptor for one imported argument type.
#[doc(hidden)]
#[must_use]
pub const fn wire_import_input_type<T: IntoJS>() -> &'static WireImportInputType {
	<WireImportInputType as MetadataFor<T>>::VALUE
}

/// Returns the `ABI` data for an indirect import's return pointer.
#[doc(hidden)]
#[must_use]
pub const fn wire_import_retptr_type() -> &'static WireImportInputType {
	<WireImportInputType as MetadataFor<PtrMut<()>>>::VALUE
}

/// Returns the shared wire descriptor for one imported result type.
#[doc(hidden)]
#[must_use]
pub const fn wire_import_output_type<T: ReturnFromJS>() -> &'static WireImportOutputType {
	<WireImportOutputType as MetadataFor<T>>::VALUE
}

/// Returns the exception lowering shared by imported `Result` types.
#[doc(hidden)]
#[must_use]
pub const fn wire_import_catch() -> WireImportCatch {
	crate::runtime::exception::IMPORT_CATCH
}
