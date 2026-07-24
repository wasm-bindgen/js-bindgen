use crate::hazard::{
	FromJS, IntoJS, ReturnAbi, ReturnFromJS, ReturnIntoJS, Slot, WasmAbi, WasmRet, WatConv,
};

// Rust ABI shims used by generated import and export functions.

pub type InputSlot1<T> = <<T as IntoJS>::Abi as WasmAbi>::Slot1;
pub type InputSlot2<T> = <<T as IntoJS>::Abi as WasmAbi>::Slot2;
pub type InputSlot3<T> = <<T as IntoJS>::Abi as WasmAbi>::Slot3;
pub type InputSlot4<T> = <<T as IntoJS>::Abi as WasmAbi>::Slot4;

pub type OutputSlot1<T> = <<T as ReturnFromJS>::Abi as WasmAbi>::Slot1;
pub type OutputSlot2<T> = <<T as ReturnFromJS>::Abi as WasmAbi>::Slot2;
pub type OutputSlot3<T> = <<T as ReturnFromJS>::Abi as WasmAbi>::Slot3;
pub type OutputSlot4<T> = <<T as ReturnFromJS>::Abi as WasmAbi>::Slot4;
pub type OutputRet<T> = core::mem::MaybeUninit<WasmRet<<T as ReturnFromJS>::Abi>>;

pub type ReturnSlot1<T> = <<T as ReturnIntoJS>::Abi as WasmAbi>::Slot1;
pub type ReturnSlot2<T> = <<T as ReturnIntoJS>::Abi as WasmAbi>::Slot2;
pub type ReturnSlot3<T> = <<T as ReturnIntoJS>::Abi as WasmAbi>::Slot3;
pub type ReturnSlot4<T> = <<T as ReturnIntoJS>::Abi as WasmAbi>::Slot4;

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
	WasmRet::from_abi(T::return_into_abi(value))
}

/// Lowers a value through a different [`IntoJS`] implementation with the same
/// ABI. This is reserved for generated `#[js_sys(type = ...)]` overrides, where
/// `T` must also describe the value's WAT and JavaScript conversions.
///
/// # Safety
///
/// The value's lowering must have the semantics expected by `T`; sharing an ABI
/// alone does not make two [`IntoJS`] implementations interchangeable.
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
	T::return_from_abi(value)
}

// Compile-time validation of conversion metadata.

#[must_use]
pub const fn into_js_is_multislot<T: IntoJS>() -> bool {
	!<InputSlot2<T> as Slot>::WAT_TYPE.is_empty()
		|| !<InputSlot3<T> as Slot>::WAT_TYPE.is_empty()
		|| !<InputSlot4<T> as Slot>::WAT_TYPE.is_empty()
}

pub const fn validate_into_js<T: IntoJS>() {
	assert!(
		!into_js_is_multislot::<T>() || T::JS_CONV.is_some(),
		"multi-slot IntoJS implementations must define IntoJS::JS_CONV",
	);
}

pub const fn validate_return_from_js<T: ReturnFromJS>() {
	let indirect = !return_from_js_is_direct::<T>();
	let conversion = T::JS_CONV.conversion();
	let (templates, sret) = match conversion {
		None => ([""; 4], None),
		Some(conv) => (conv.templates, conv.sret),
	};
	let slots = from_js_wat_slots::<T>();
	let mut slot = 0;

	while slot < slots.len() {
		assert!(
			conversion.is_none() || templates[slot].is_empty() == slots[slot].abi.is_empty(),
			"FromJS::JS_CONV templates must match its non-empty ABI slots",
		);
		slot += 1;
	}

	assert!(
		!indirect || conversion.is_some(),
		"indirect FromJS implementations must define FromJS::JS_CONV",
	);
	assert!(
		indirect == sret.is_some(),
		"FromJS::JS_CONV must define sret exactly for indirect returns",
	);
}

// WAT metadata shared by import and export adapters.

/// The `WAT` representation of one `ABI` slot at a JavaScript boundary.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct WatSlot {
	/// The carrier type in the Rust function ABI.
	pub abi: &'static str,
	/// The type visible at the JavaScript boundary.
	pub boundary: &'static str,
	/// An optional WAT import required by the conversion.
	pub import: &'static str,
	/// WAT instructions that convert between `abi` and `boundary`.
	pub conv: &'static str,
}

const fn wat_slot<S: Slot>(wat_conv: Option<WatConv>) -> WatSlot {
	let (boundary, import, conv) = match wat_conv {
		Some(WatConv {
			import,
			conv,
			r#type,
		}) => (
			r#type,
			match import {
				Some(import) => import,
				None => "",
			},
			conv,
		),
		None => (S::WAT_TYPE, "", ""),
	};

	WatSlot {
		abi: S::WAT_TYPE,
		boundary,
		import,
		conv,
	}
}

#[must_use]
pub const fn into_js_wat_slots<T: IntoJS>() -> [WatSlot; 4] {
	[
		wat_slot::<InputSlot1<T>>(<InputSlot1<T> as Slot>::INTO_JS_WAT_CONV),
		wat_slot::<InputSlot2<T>>(<InputSlot2<T> as Slot>::INTO_JS_WAT_CONV),
		wat_slot::<InputSlot3<T>>(<InputSlot3<T> as Slot>::INTO_JS_WAT_CONV),
		wat_slot::<InputSlot4<T>>(<InputSlot4<T> as Slot>::INTO_JS_WAT_CONV),
	]
}

#[must_use]
pub const fn from_js_wat_slots<T: ReturnFromJS>() -> [WatSlot; 4] {
	[
		wat_slot::<OutputSlot1<T>>(<OutputSlot1<T> as Slot>::FROM_JS_WAT_CONV),
		wat_slot::<OutputSlot2<T>>(<OutputSlot2<T> as Slot>::FROM_JS_WAT_CONV),
		wat_slot::<OutputSlot3<T>>(<OutputSlot3<T> as Slot>::FROM_JS_WAT_CONV),
		wat_slot::<OutputSlot4<T>>(<OutputSlot4<T> as Slot>::FROM_JS_WAT_CONV),
	]
}

#[must_use]
pub const fn return_into_js_wat_slots<T: ReturnIntoJS>() -> [WatSlot; 4] {
	[
		wat_slot::<ReturnSlot1<T>>(<ReturnSlot1<T> as Slot>::INTO_JS_WAT_CONV),
		wat_slot::<ReturnSlot2<T>>(<ReturnSlot2<T> as Slot>::INTO_JS_WAT_CONV),
		wat_slot::<ReturnSlot3<T>>(<ReturnSlot3<T> as Slot>::INTO_JS_WAT_CONV),
		wat_slot::<ReturnSlot4<T>>(<ReturnSlot4<T> as Slot>::INTO_JS_WAT_CONV),
	]
}

#[must_use]
pub const fn wat_direct<T: ReturnFromJS>() -> &'static str {
	if return_from_js_is_direct::<T>() {
		from_js_wat_slots::<T>()[0].abi
	} else {
		""
	}
}

#[must_use]
pub const fn wat_indirect_type<T: ReturnFromJS>() -> &'static str {
	if return_from_js_is_direct::<T>() {
		""
	} else {
		crate::util::WAT_PTR_TYPE
	}
}

#[must_use]
pub const fn wat_indirect_import_type<T: ReturnFromJS>() -> &'static str {
	if return_from_js_is_direct::<T>() {
		""
	} else {
		into_js_wat_slots::<crate::util::PtrMut<()>>()[0].boundary
	}
}

#[must_use]
pub const fn wat_indirect_conv<T: ReturnFromJS>() -> &'static str {
	if return_from_js_is_direct::<T>() {
		""
	} else {
		into_js_wat_slots::<crate::util::PtrMut<()>>()[0].conv
	}
}

#[must_use]
pub const fn wat_output_import<T: ReturnFromJS>() -> &'static str {
	if return_from_js_is_direct::<T>() {
		from_js_wat_slots::<T>()[0].import
	} else {
		""
	}
}

#[must_use]
pub const fn wat_output_import_type<T: ReturnFromJS>() -> &'static str {
	if return_from_js_is_direct::<T>() {
		from_js_wat_slots::<T>()[0].boundary
	} else {
		""
	}
}

#[must_use]
pub const fn wat_output_conv<T: ReturnFromJS>() -> &'static str {
	if return_from_js_is_direct::<T>() {
		from_js_wat_slots::<T>()[0].conv
	} else {
		""
	}
}

#[must_use]
pub const fn return_from_js_is_direct<T: ReturnFromJS>() -> bool {
	<T::Abi as ReturnAbi>::MODE.is_direct()
}

#[must_use]
pub const fn return_from_js_is_result<T: ReturnFromJS>() -> bool {
	T::JS_CONV.is_result()
}

pub const fn validate_return_into_js<T: ReturnIntoJS>() {
	let conv = T::JS_CONV.conversion();
	let multislot = if T::JS_CONV.is_result() {
		!<ReturnSlot4<T> as Slot>::WAT_TYPE.is_empty()
	} else {
		!<ReturnSlot2<T> as Slot>::WAT_TYPE.is_empty()
			|| !<ReturnSlot3<T> as Slot>::WAT_TYPE.is_empty()
			|| !<ReturnSlot4<T> as Slot>::WAT_TYPE.is_empty()
	};

	assert!(
		!multislot || conv.is_some(),
		"multi-slot ReturnIntoJS implementations must define a JavaScript conversion",
	);
}

#[must_use]
pub const fn return_into_js_is_direct<T: ReturnIntoJS>() -> bool {
	<T::Abi as ReturnAbi>::MODE.is_direct()
}

#[must_use]
pub const fn export_output_frame_size<T: ReturnIntoJS>() -> usize {
	// LLVM keeps the Wasm stack pointer 16-byte aligned. Rounding every adapter
	// frame to that alignment preserves the invariant when the frame is allocated.
	const STACK_ALIGNMENT: usize = 16;
	let size = core::mem::size_of::<WasmRet<T::Abi>>();

	(size + STACK_ALIGNMENT - 1) & !(STACK_ALIGNMENT - 1)
}

#[must_use]
pub const fn export_output_slot_offset<T: ReturnIntoJS, const SLOT: usize>() -> usize {
	WasmRet::<T::Abi>::slot_offset::<SLOT>()
}

#[must_use]
pub const fn wat_pointer_type() -> &'static str {
	crate::util::WAT_PTR_TYPE
}

// JavaScript conversion metadata.

#[must_use]
pub const fn js_input_embed<T: IntoJS>() -> (&'static str, &'static str) {
	js_embed(match T::JS_CONV {
		Some(conv) => conv.embed,
		None => None,
	})
}

#[must_use]
pub const fn js_return_embed<T: ReturnIntoJS>() -> (&'static str, &'static str) {
	js_embed(match T::JS_CONV.conversion() {
		Some(conv) => conv.embed,
		None => None,
	})
}

#[must_use]
pub const fn js_output_embed<T: ReturnFromJS>() -> (&'static str, &'static str) {
	js_embed(match T::JS_CONV.conversion() {
		Some(conv) => conv.embed,
		None => None,
	})
}

#[must_use]
pub const fn js_result_embed<T: ReturnFromJS>() -> (&'static str, &'static str) {
	if T::JS_CONV.is_result() {
		("js_sys", "externref.table")
	} else {
		("", "")
	}
}

const fn js_embed(embed: Option<(&'static str, &'static str)>) -> (&'static str, &'static str) {
	if let Some(embed) = embed {
		embed
	} else {
		("", "")
	}
}

#[must_use]
pub const fn js_input_template<T: IntoJS>() -> &'static str {
	if let Some(conv) = T::JS_CONV {
		conv.template
	} else {
		""
	}
}

#[must_use]
pub const fn js_export_output_template<T: ReturnIntoJS>() -> &'static str {
	let template = match T::JS_CONV.conversion() {
		Some(conv) => conv.template,
		None => "",
	};

	if template.is_empty() {
		"$slot1"
	} else {
		template
	}
}

#[must_use]
pub const fn return_into_js_is_result<T: ReturnIntoJS>() -> bool {
	T::JS_CONV.is_result()
}

#[must_use]
pub const fn js_output_templates<T: ReturnFromJS>() -> [&'static str; 4] {
	if let Some(conv) = T::JS_CONV.conversion() {
		conv.templates
	} else {
		["$value", "", "", ""]
	}
}

#[must_use]
pub const fn js_output_sret<T: ReturnFromJS>() -> &'static str {
	if let Some(conv) = T::JS_CONV.conversion()
		&& let Some(sret) = conv.sret
	{
		sret
	} else {
		""
	}
}
