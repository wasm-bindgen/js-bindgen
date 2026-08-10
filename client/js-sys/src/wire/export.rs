use crate::hazard::{FromJS, ReturnAbi, ReturnConv, ReturnIntoJS, ReturnMode, WasmRet};
use crate::wire::{
	WireExportInput, WireExportInputType, WireExportOutput, WireExportOutputType, WireReturnFrame,
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
		let slots = into_js_slots::<T::Abi>();
		match mode {
			ReturnMode::Direct => {
				let ReturnConv::Value(conversion) = T::JS_CONV else {
					panic!("a direct export cannot return Result");
				};
				let conversion = if matches!(slots, [None, None, None, None]) {
					// A zero-slot Wasm return is already JavaScript `undefined`.
					None
				} else {
					conversion
				};
				Self::direct(slots, conversion)
			}
			ReturnMode::Indirect => {
				// LLVM keeps the Wasm stack pointer 16-byte aligned.
				let size = core::mem::size_of::<WasmRet<T::Abi>>();
				let frame = WireReturnFrame::new(
					(size + 15) & !15,
					[
						WasmRet::<T::Abi>::slot_offset::<0>(),
						WasmRet::<T::Abi>::slot_offset::<1>(),
						WasmRet::<T::Abi>::slot_offset::<2>(),
						WasmRet::<T::Abi>::slot_offset::<3>(),
					],
				);
				Self::indirect(
					slots,
					T::JS_CONV,
					frame,
					<T::Abi as ReturnAbi>::RESULT_LAYOUT,
				)
			}
		}
	};
}

/// Builds the wire descriptor for one exported argument.
#[doc(hidden)]
#[must_use]
pub const fn wire_export_input<T: FromJS>() -> WireExportInput {
	WireExportInput::new(<WireExportInputType as MetadataFor<T>>::VALUE)
}

/// Builds the result reference for one JavaScript-facing Wasm export.
#[doc(hidden)]
#[must_use]
pub const fn wire_export_output<T: ReturnIntoJS>() -> WireExportOutput {
	WireExportOutput::new(<WireExportOutputType as MetadataFor<T>>::VALUE)
}
