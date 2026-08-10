use crate::hazard::{IntoJS, ReturnAbi, ReturnFromJS};
use crate::util::PtrMut;
use crate::wire::{
	WireImportCatch, WireImportInputType, WireImportOutputType, from_js_slots, into_js_slots,
};

trait MetadataFor<T>: 'static {
	const VALUE: &'static Self;
}

impl<T: IntoJS> MetadataFor<T> for WireImportInputType {
	const VALUE: &'static Self = &Self::new(into_js_slots::<T::Abi>(), T::JS_CONV);
}

impl<T: ReturnFromJS> MetadataFor<T> for WireImportOutputType {
	const VALUE: &'static Self = &{
		Self::new(
			<T::Abi as ReturnAbi>::MODE,
			T::JS_CONV,
			T::JS_SRET,
			from_js_slots::<T::Abi>(),
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
