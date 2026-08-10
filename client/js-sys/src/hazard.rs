use core::mem::{ManuallyDrop, MaybeUninit};
use core::ptr;

pub use js_bindgen_wire::abi::{
	FromJsConv, IntoJsConv, JsCatch, JsEmbed, RefType, ResultLayout, ReturnConv, ReturnMode, Sret,
	WatCatch, WatConv, WatImport, WatImportKind, WatIndexType, WatLocal, WatSlot, WatType,
};

use crate::JsValue;
use crate::runtime::externref::{WAT_INDEX_LOCAL, WAT_TAKE_IMPORTS};

// Wasm `ABI` carriers.

/// One carrier position in the Wasm function `ABI`.
///
/// # Safety
///
/// `WAT_TYPE` must describe the carrier's Rust Wasm `ABI`. Each conversion must
/// consume or produce that type as appropriate. Only [`EmptySlot`] may use
/// `None`.
pub unsafe trait Slot {
	const WAT_TYPE: Option<WatType>;
	const INTO_JS_WAT_CONV: Option<WatConv> = None;
	const FROM_JS_WAT_CONV: Option<WatConv> = None;
}

/// Converts a Rust-side `ABI` carrier to and from primitive Wasm slots.
///
/// Types that occupy one `ABI` slot use themselves as `Slot1`. Multi-slot
/// carriers represent each primitive independently.
///
/// # Safety
///
/// The slots and their order must match the generated `extern` function
/// signature and return layout. Unused trailing slots must be [`EmptySlot`],
/// which is zero-sized and omitted from the Wasm `ABI`.
pub unsafe trait WasmAbi: Sized {
	type Slot1: Slot;
	type Slot2: Slot;
	type Slot3: Slot;
	type Slot4: Slot;

	fn split(self) -> (Self::Slot1, Self::Slot2, Self::Slot3, Self::Slot4);
	fn join(slot1: Self::Slot1, slot2: Self::Slot2, slot3: Self::Slot3, slot4: Self::Slot4)
	-> Self;
}

/// A [`WasmAbi`] that can be returned through the Rust `extern "C"` `ABI`.
///
/// # Safety
///
/// `MODE` must match the `ABI` of [`WasmRet<Self>`]. A direct return must use
/// exactly one non-empty slot. An indirect return uses the target's native
/// pointer type for its hidden return parameter.
pub unsafe trait ReturnAbi: WasmAbi {
	const MODE: ReturnMode;
	const RESULT_LAYOUT: Option<ResultLayout> = None;
}

/// The FFI-safe return representation of a [`WasmAbi`] value.
#[doc(hidden)]
#[repr(C)]
pub struct WasmRet<T: ReturnAbi> {
	slot1: T::Slot1,
	slot2: T::Slot2,
	slot3: T::Slot3,
	slot4: T::Slot4,
}

impl<T: ReturnAbi> WasmRet<T> {
	#[must_use]
	#[inline]
	pub fn from_abi(value: T) -> Self {
		let (slot1, slot2, slot3, slot4) = value.split();

		Self {
			slot1,
			slot2,
			slot3,
			slot4,
		}
	}

	#[must_use]
	#[inline]
	pub fn join(self) -> T {
		T::join(self.slot1, self.slot2, self.slot3, self.slot4)
	}

	#[doc(hidden)]
	#[must_use]
	pub const fn slot_offset<const SLOT: usize>() -> usize {
		match SLOT {
			0 => core::mem::offset_of!(Self, slot1),
			1 => core::mem::offset_of!(Self, slot2),
			2 => core::mem::offset_of!(Self, slot3),
			3 => core::mem::offset_of!(Self, slot4),
			_ => panic!("invalid WasmRet slot"),
		}
	}
}

/// A zero-sized placeholder for an unused Wasm `ABI` slot.
#[doc(hidden)]
#[derive(Default)]
#[repr(C)]
pub struct EmptySlot([u8; 0]);

impl EmptySlot {
	#[must_use]
	pub const fn new() -> Self {
		Self([])
	}
}

// SAFETY: `EmptySlot` is an absent slot and therefore has no WAT type.
unsafe impl Slot for EmptySlot {
	const WAT_TYPE: Option<WatType> = None;
}

// SAFETY: Every non-empty `Slot` is a complete single-slot `ABI` carrier.
// `EmptySlot` maps to an entirely empty carrier.
unsafe impl<T: Slot + Sized> WasmAbi for T {
	type Slot1 = Self;
	type Slot2 = EmptySlot;
	type Slot3 = EmptySlot;
	type Slot4 = EmptySlot;

	fn split(self) -> (Self::Slot1, Self::Slot2, Self::Slot3, Self::Slot4) {
		(self, EmptySlot::new(), EmptySlot::new(), EmptySlot::new())
	}

	fn join(slot1: Self::Slot1, _: Self::Slot2, _: Self::Slot3, _: Self::Slot4) -> Self {
		slot1
	}
}

// SAFETY: The first slot is the presence tag, followed by up to three payload
// slots.
unsafe impl<T> WasmAbi for Option<T>
where
	T: WasmAbi<Slot4 = EmptySlot>,
	T::Slot1: Default,
	T::Slot2: Default,
	T::Slot3: Default,
{
	type Slot1 = u32;
	type Slot2 = T::Slot1;
	type Slot3 = T::Slot2;
	type Slot4 = T::Slot3;

	#[inline]
	fn split(self) -> (Self::Slot1, Self::Slot2, Self::Slot3, Self::Slot4) {
		match self {
			None => (
				0,
				Default::default(),
				Default::default(),
				Default::default(),
			),
			Some(value) => {
				let (slot1, slot2, slot3, _) = value.split();
				(1, slot1, slot2, slot3)
			}
		}
	}

	#[inline]
	fn join(
		is_some: Self::Slot1,
		slot1: Self::Slot2,
		slot2: Self::Slot3,
		slot3: Self::Slot4,
	) -> Self {
		if is_some == 0 {
			None
		} else {
			Some(T::join(slot1, slot2, slot3, EmptySlot::new()))
		}
	}
}

// Rust-to-JavaScript conversions.

/// # Safety
///
/// `Abi`, `into_abi`, and `JS_CONV` must describe one consistent conversion
/// from a Rust value to a JavaScript value. `into_abi` produces the primitive
/// slots and `JS_CONV` combines them. Multi-slot `ABI` representations must
/// define `JS_CONV`.
pub unsafe trait IntoJS {
	const JS_CONV: Option<IntoJsConv> = None;

	type Abi: WasmAbi;

	fn into_abi(self) -> Self::Abi;
}

/// Describes how a value using this `ABI` carrier is encoded as an [`Option`].
///
/// This is implemented on the carrier rather than the Rust value so types that
/// share a carrier can also share their optional representation.
///
/// # Safety
///
/// `Abi`, `into_option_abi`, and `JS_CONV` must describe one consistent
/// conversion from `Option<T>` to a JavaScript value.
#[doc(hidden)]
pub unsafe trait OptionIntoAbi<T: IntoJS>: WasmAbi {
	const JS_CONV: Option<IntoJsConv> = T::JS_CONV;

	type Abi: WasmAbi;

	fn into_option_abi(value: Option<T>) -> Self::Abi;
}

// SAFETY: Delegated to the optional representation of `T`'s `ABI` carrier.
unsafe impl<T: IntoJS> IntoJS for Option<T>
where
	T::Abi: OptionIntoAbi<T>,
{
	const JS_CONV: Option<IntoJsConv> = <T::Abi as OptionIntoAbi<T>>::JS_CONV;

	type Abi = <T::Abi as OptionIntoAbi<T>>::Abi;

	fn into_abi(self) -> Self::Abi {
		<T::Abi as OptionIntoAbi<T>>::into_option_abi(self)
	}
}

/// Converts a Rust function result into its JavaScript return representation.
///
/// Ordinary values delegate to [`IntoJS`]. Types such as [`Result`] may also
/// describe JavaScript control flow, such as throwing an error.
pub trait ReturnIntoJS {
	const JS_CONV: ReturnConv<IntoJsConv>;

	type Abi: ReturnAbi;

	fn into_return_abi(self) -> Self::Abi;
}

impl<T> ReturnIntoJS for T
where
	T: IntoJS,
	T::Abi: ReturnAbi,
{
	const JS_CONV: ReturnConv<IntoJsConv> = ReturnConv::Value(T::JS_CONV);

	type Abi = T::Abi;

	fn into_return_abi(self) -> Self::Abi {
		self.into_abi()
	}
}

// JavaScript-to-Rust conversions.

/// # Safety
///
/// `Abi`, `from_abi`, `JS_CONV`, and `JS_SRET` must describe one consistent
/// conversion from a JavaScript value to a Rust value. `JS_CONV` produces the
/// primitive slots and `from_abi` reconstructs the Rust value. Multi-slot
/// `ABI` representations must define one slot template for every non-empty
/// slot. Indirect import returns must also define `JS_SRET`.
pub unsafe trait FromJS {
	const JS_CONV: Option<FromJsConv> = None;
	const JS_SRET: Option<Sret> = None;

	type Abi: WasmAbi;

	fn from_abi(raw: Self::Abi) -> Self;
}

/// Describes how an [`Option`] is decoded for a value using this `ABI` carrier.
///
/// This is implemented on the carrier rather than the Rust value so types that
/// share a carrier can also share their optional representation.
///
/// # Safety
///
/// `Abi`, `from_option_abi`, `JS_CONV`, and `JS_SRET` must describe one
/// consistent conversion from a JavaScript value to `Option<T>`.
#[doc(hidden)]
pub unsafe trait OptionFromAbi<T: FromJS>: WasmAbi {
	const JS_CONV: Option<FromJsConv> = T::JS_CONV;
	const JS_SRET: Option<Sret> = T::JS_SRET;

	type Abi: WasmAbi;

	fn from_option_abi(raw: Self::Abi) -> Option<T>;
}

// SAFETY: Delegated to the optional representation of `T`'s `ABI` carrier.
unsafe impl<T: FromJS> FromJS for Option<T>
where
	T::Abi: OptionFromAbi<T>,
{
	const JS_CONV: Option<FromJsConv> = <T::Abi as OptionFromAbi<T>>::JS_CONV;
	const JS_SRET: Option<Sret> = <T::Abi as OptionFromAbi<T>>::JS_SRET;

	type Abi = <T::Abi as OptionFromAbi<T>>::Abi;

	fn from_abi(raw: Self::Abi) -> Self {
		<T::Abi as OptionFromAbi<T>>::from_option_abi(raw)
	}
}

/// Converts the return value of a JavaScript import into its Rust result.
///
/// `Abi` describes the successful return value and must support the Rust
/// return `ABI`. Indirect returns must define `JS_SRET`. The raw
/// carrier may be uninitialized when JavaScript throws, so implementations
/// that catch exceptions must inspect the exception state before decoding it.
pub trait ReturnFromJS {
	const JS_CONV: ReturnConv<FromJsConv>;
	const JS_SRET: Option<Sret>;

	type Abi: ReturnAbi;

	fn from_return_abi(raw: MaybeUninit<WasmRet<Self::Abi>>) -> Self;
}

impl<T> ReturnFromJS for T
where
	T: FromJS,
	T::Abi: ReturnAbi,
{
	const JS_CONV: ReturnConv<FromJsConv> = ReturnConv::Value(T::JS_CONV);
	const JS_SRET: Option<Sret> = T::JS_SRET;

	type Abi = T::Abi;

	fn from_return_abi(raw: MaybeUninit<WasmRet<Self::Abi>>) -> Self {
		// SAFETY: An ordinary JavaScript import always initializes its return
		// value before the shim returns.
		T::from_abi(unsafe { raw.assume_init() }.join())
	}
}

// Result returns.

/// The return `ABI` for exporting [`Result`] to JavaScript.
///
/// The first two slots carry the successful value. The remaining two carry the
/// error discriminant and table index.
#[doc(hidden)]
pub struct ResultIntoJsAbi<T: WasmAbi> {
	value: Result<T, <JsValue as IntoJS>::Abi>,
}

const RESULT_DISCRIMINANT_LOCAL: WatLocal =
	WatLocal::new("js_sys.result.discriminant", WatType::I32);
const RESULT_ERROR_WAT_CONV: &str = "\
  local.set $js_sys.externref.index
  local.get $js_sys.result.discriminant
  if (result externref)
    local.get $js_sys.externref.index
    table.get $js_sys.import.externref.table (@reloc)
    local.get $js_sys.externref.index
    i32.const 2
    i32.ge_u
    if
      local.get $js_sys.externref.index
      call $js_sys.externref.release (@reloc)
    end
  else
    ref.null extern
  end";

/// The discriminant of an exported [`Result`].
#[doc(hidden)]
#[repr(transparent)]
pub struct ResultDiscriminantAbi(u32);

// SAFETY: The transparent `i32` discriminant is also recorded in a local for
// the following error slot conversion.
unsafe impl Slot for ResultDiscriminantAbi {
	const WAT_TYPE: Option<WatType> = Some(WatType::I32);
	const INTO_JS_WAT_CONV: Option<WatConv> = Some(WatConv::new(
		&[],
		&[RESULT_DISCRIMINANT_LOCAL],
		"local.tee $js_sys.result.discriminant",
		WatType::I32,
	));
}

/// An owned `externref` table index transferred by a [`Result`] error.
///
/// The preceding [`ResultDiscriminantAbi`] controls whether the index is taken
/// from the table. Successful results produce a null placeholder without
/// accessing the table.
#[doc(hidden)]
#[repr(transparent)]
pub struct ResultErrorAbi(<JsValue as IntoJS>::Abi);

// SAFETY: `JsValue` uses a transparent `i32` table index as its Rust `ABI`. The
// preceding result discriminant is recorded before this conversion runs.
unsafe impl Slot for ResultErrorAbi {
	const WAT_TYPE: Option<WatType> = Some(WatType::I32);
	const INTO_JS_WAT_CONV: Option<WatConv> = Some(WatConv::new(
		WAT_TAKE_IMPORTS,
		&[WAT_INDEX_LOCAL],
		RESULT_ERROR_WAT_CONV,
		WatType::ExternRef,
	));
}

// SAFETY: The first two slots match the successful value's `ABI`. The third
// is the error discriminant and the fourth transfers an owned error table
// index.
unsafe impl<T> WasmAbi for ResultIntoJsAbi<T>
where
	T: WasmAbi<Slot3 = EmptySlot, Slot4 = EmptySlot>,
	T::Slot1: Default,
	T::Slot2: Default,
{
	type Slot1 = T::Slot1;
	type Slot2 = T::Slot2;
	type Slot3 = ResultDiscriminantAbi;
	type Slot4 = ResultErrorAbi;

	fn split(self) -> (Self::Slot1, Self::Slot2, Self::Slot3, Self::Slot4) {
		match self.value {
			Ok(value) => {
				let (slot1, slot2, _, _) = value.split();
				(
					slot1,
					slot2,
					ResultDiscriminantAbi(0),
					ResultErrorAbi(JsValue::UNDEFINED.into_abi()),
				)
			}
			Err(error) => (
				Default::default(),
				Default::default(),
				ResultDiscriminantAbi(1),
				ResultErrorAbi(error),
			),
		}
	}

	fn join(
		slot1: Self::Slot1,
		slot2: Self::Slot2,
		is_error: Self::Slot3,
		error: Self::Slot4,
	) -> Self {
		let value = if is_error.0 == 0 {
			Ok(T::join(slot1, slot2, EmptySlot::new(), EmptySlot::new()))
		} else {
			Err(error.0)
		};

		Self { value }
	}
}

// SAFETY: `ResultIntoJsAbi` is returned through a hidden pointer.
unsafe impl<T> ReturnAbi for ResultIntoJsAbi<T>
where
	T: WasmAbi<Slot3 = EmptySlot, Slot4 = EmptySlot>,
	T::Slot1: Default,
	T::Slot2: Default,
{
	const MODE: ReturnMode = ReturnMode::Indirect;
	const RESULT_LAYOUT: Option<ResultLayout> = {
		let discriminant = if T::Slot1::WAT_TYPE.is_none() {
			0
		} else if T::Slot2::WAT_TYPE.is_none() {
			1
		} else {
			2
		};

		Some(ResultLayout::new(discriminant, discriminant + 1))
	};
}

impl<T, E> ReturnIntoJS for Result<T, E>
where
	T: IntoJS,
	E: Into<JsValue>,
	T::Abi: WasmAbi<Slot3 = EmptySlot, Slot4 = EmptySlot>,
	<T::Abi as WasmAbi>::Slot1: Default,
	<T::Abi as WasmAbi>::Slot2: Default,
{
	const JS_CONV: ReturnConv<IntoJsConv> = ReturnConv::Result(T::JS_CONV);

	type Abi = ResultIntoJsAbi<T::Abi>;

	fn into_return_abi(self) -> Self::Abi {
		let value = match self {
			Ok(value) => Ok(value.into_abi()),
			Err(error) => Err(error.into().into_abi()),
		};

		ResultIntoJsAbi { value }
	}
}

impl<T> ReturnFromJS for Result<T, JsValue>
where
	T: FromJS,
	T::Abi: ReturnAbi,
{
	const JS_CONV: ReturnConv<FromJsConv> = ReturnConv::Result(T::JS_CONV);
	const JS_SRET: Option<Sret> = T::JS_SRET;

	type Abi = T::Abi;

	fn from_return_abi(raw: MaybeUninit<WasmRet<Self::Abi>>) -> Self {
		if let Some(error) = crate::runtime::exception::take() {
			#[cfg(not(target_feature = "exception-handling"))]
			if <T::Abi as ReturnAbi>::MODE.is_direct() {
				// SAFETY: A direct Wasm return is always initialized. On the
				// exception path it contains only the JavaScript fallback value.
				drop(T::from_abi(unsafe { raw.assume_init() }.join()));
			}

			Err(error)
		} else {
			// SAFETY: Without a stored exception, the JavaScript import
			// initialized its successful return value.
			Ok(T::from_abi(unsafe { raw.assume_init() }.join()))
		}
	}
}

// Borrowed and cast JavaScript values.

/// A type that can be borrowed from an owned JavaScript conversion.
///
/// The anchor owns the converted value for the duration of an exported
/// function call and provides the reference passed to that function.
pub trait RefFromJS {
	type Anchor: FromJS + core::borrow::Borrow<Self>;
}

impl<T: FromJS> RefFromJS for T {
	type Anchor = T;
}

/// # Safety
///
/// This must only be implemented for types that are transparent over
/// [`JsValue`].
pub unsafe trait JsCast: Sized {
	#[must_use]
	fn unchecked_as_ref(&self) -> &JsValue {
		let ptr: *const JsValue = ptr::from_ref(self).cast();
		// SAFETY: The trait assumes that `Self` is `#[transparent]` over a `JsValue`.
		unsafe { &*ptr }
	}

	#[must_use]
	fn unchecked_from(value: JsValue) -> Self {
		let value = ManuallyDrop::new(value);
		let ptr: *const Self = ptr::from_ref(&*value).cast();
		// SAFETY: The trait assumes that `Self` is `#[transparent]` over a `JsValue`.
		unsafe { ptr.read() }
	}

	#[must_use]
	fn unchecked_from_ref(value: &JsValue) -> &Self {
		let ptr: *const Self = ptr::from_ref(value).cast();
		// SAFETY: The trait assumes that `Self` is `#[transparent]` over a `JsValue`.
		unsafe { &*ptr }
	}
}
