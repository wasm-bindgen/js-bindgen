use crate::ClosureHeader;
use crate::hazard::{FromJS, ReturnAbi, ReturnIntoJS, Slot, WasmRet};
use crate::wire::{
	FromJsSlot1, FromJsSlot2, FromJsSlot3, FromJsSlot4, ReturnSlot1, ReturnSlot2, ReturnSlot3,
	ReturnSlot4, WireExport, WireExportInput, WireExportInputType, WireExportOutput,
	WireExportOutputType, wat_slot,
};

trait MetadataFor<T>: 'static {
	const VALUE: &'static Self;
}

impl<T: FromJS> MetadataFor<T> for WireExportInputType {
	const VALUE: &'static Self = &Self::new(
		[
			wat_slot::<FromJsSlot1<T>>(<FromJsSlot1<T> as Slot>::FROM_JS_WAT_CONV),
			wat_slot::<FromJsSlot2<T>>(<FromJsSlot2<T> as Slot>::FROM_JS_WAT_CONV),
			wat_slot::<FromJsSlot3<T>>(<FromJsSlot3<T> as Slot>::FROM_JS_WAT_CONV),
			wat_slot::<FromJsSlot4<T>>(<FromJsSlot4<T> as Slot>::FROM_JS_WAT_CONV),
		],
		T::JS_CONV,
	);
}

impl<T: ReturnIntoJS> MetadataFor<T> for WireExportOutputType {
	const VALUE: &'static Self = &{
		let mode = <T::Abi as ReturnAbi>::MODE;
		let result_layout = <T::Abi as ReturnAbi>::RESULT_LAYOUT;
		let slots = [
			wat_slot::<ReturnSlot1<T>>(<ReturnSlot1<T> as Slot>::INTO_JS_WAT_CONV),
			wat_slot::<ReturnSlot2<T>>(<ReturnSlot2<T> as Slot>::INTO_JS_WAT_CONV),
			wat_slot::<ReturnSlot3<T>>(<ReturnSlot3<T> as Slot>::INTO_JS_WAT_CONV),
			wat_slot::<ReturnSlot4<T>>(<ReturnSlot4<T> as Slot>::INTO_JS_WAT_CONV),
		];
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
