use crate::ClosureHeader;
use crate::hazard::{FromJS, ReturnAbi, ReturnIntoJS, WasmRet};
use crate::wire::{
	WireExport, WireExportInput, WireExportInputType, WireExportOutput, WireExportOutputType,
	from_js_slots, into_js_slots,
};

trait MetadataFor<T>: 'static {
	const VALUE: &'static Self;
}

impl<T: FromJS> MetadataFor<T> for WireExportInputType {
	const VALUE: &'static Self = &Self::new(from_js_slots::<T::Abi>(), T::JS_CONV);
}

impl<T: ReturnIntoJS> MetadataFor<T> for WireExportOutputType {
	const VALUE: &'static Self = &{
		let mode = <T::Abi as ReturnAbi>::MODE;
		let result_layout = <T::Abi as ReturnAbi>::RESULT_LAYOUT;
		let slots = into_js_slots::<T::Abi>();
		let (frame_size, slot_offsets) = if mode.is_direct() {
			(0, [0; 4])
		} else {
			// LLVM keeps the Wasm stack pointer 16-byte aligned.
			let size = core::mem::size_of::<WasmRet<T::Abi>>();
			(
				(size + 15) & !15,
				[
					WasmRet::<T::Abi>::slot_offset::<0>(),
					WasmRet::<T::Abi>::slot_offset::<1>(),
					WasmRet::<T::Abi>::slot_offset::<2>(),
					WasmRet::<T::Abi>::slot_offset::<3>(),
				],
			)
		};

		Self::new(
			mode,
			T::JS_CONV,
			slots,
			frame_size,
			slot_offsets,
			result_layout,
		)
	};
}

/// Builds the wire descriptor for one exported argument.
#[doc(hidden)]
#[must_use]
pub const fn wire_export_input<T: FromJS>(name: &'static str) -> WireExportInput {
	WireExportInput::new(name, <WireExportInputType as MetadataFor<T>>::VALUE)
}

/// Builds the result reference for one JavaScript-facing Wasm export.
#[doc(hidden)]
#[must_use]
pub const fn wire_export_output<T: ReturnIntoJS>() -> WireExportOutput {
	WireExportOutput::new(<WireExportOutputType as MetadataFor<T>>::VALUE)
}

/// Builds a closure dispatcher export using the closure header's call shim.
#[doc(hidden)]
#[must_use]
pub const fn wire_closure_export<C>(
	module: &'static str,
	name: &'static str,
	inputs: &'static [WireExportInput],
	output: Option<WireExportOutput>,
) -> WireExport {
	WireExport::new_closure(
		module,
		name,
		ClosureHeader::call_shim_offset::<C>(),
		inputs,
		output,
	)
}
