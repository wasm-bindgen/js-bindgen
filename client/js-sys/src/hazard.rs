use core::mem::{ManuallyDrop, MaybeUninit};
use core::ptr;

use crate::JsValue;

/// One carrier position in the Wasm function `ABI`.
///
/// # Safety
///
/// `WAT_TYPE` must describe the carrier's Rust Wasm `ABI`. Each conversion must
/// consume or produce that type as appropriate. `WAT_TYPE` must not be empty
/// except for [`EmptySlot`].
pub unsafe trait Slot {
	const WAT_TYPE: &'static str;
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

#[derive(Clone, Copy)]
pub enum ReturnMode {
	Direct,
	Indirect,
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
}

impl ReturnMode {
	#[must_use]
	pub const fn is_direct(self) -> bool {
		matches!(self, Self::Direct)
	}
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
	const WAT_TYPE: &'static str = "";
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

#[derive(Clone, Copy)]
pub struct WatConv {
	pub import: Option<&'static str>,
	pub conv: &'static str,
	pub r#type: &'static str,
}

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

/// Converts a Rust function result into its JavaScript return representation.
///
/// Ordinary values delegate to [`IntoJS`]. Types such as [`Result`] may also
/// describe JavaScript control flow, such as throwing an error.
pub trait ReturnIntoJS {
	const JS_CONV: ReturnConv<IntoJsConv>;

	type Abi: ReturnAbi;

	fn return_into_abi(self) -> Self::Abi;
}

impl<T> ReturnIntoJS for T
where
	T: IntoJS,
	T::Abi: ReturnAbi,
{
	const JS_CONV: ReturnConv<IntoJsConv> = ReturnConv::Value(T::JS_CONV);

	type Abi = T::Abi;

	fn return_into_abi(self) -> Self::Abi {
		self.into_abi()
	}
}

/// Extends [`IntoJS`] with the representation of `Option<Self>`.
///
/// # Safety
///
/// `OptionAbi`, `option_into_abi`, and `OPTION_JS_CONV` must describe one
/// consistent conversion from `Option<Self>` to a JavaScript value.
pub unsafe trait OptionIntoJS: IntoJS + Sized {
	const OPTION_JS_CONV: Option<IntoJsConv> = Self::JS_CONV;

	type OptionAbi: WasmAbi;

	fn option_into_abi(value: Option<Self>) -> Self::OptionAbi;
}

// SAFETY: Delegated to the `OptionIntoJS` implementation.
unsafe impl<T: OptionIntoJS> IntoJS for Option<T> {
	const JS_CONV: Option<IntoJsConv> = T::OPTION_JS_CONV;

	type Abi = T::OptionAbi;

	fn into_abi(self) -> Self::Abi {
		T::option_into_abi(self)
	}
}

/// Converts primitive `ABI` slots into one JavaScript value.
#[derive(Clone, Copy)]
pub struct IntoJsConv {
	pub(crate) embed: Option<(&'static str, &'static str)>,
	pub(crate) template: &'static str,
}

/// Describes how a function return is handled at the JavaScript boundary.
#[derive(Clone, Copy)]
pub enum ReturnConv<T> {
	/// The value is returned normally.
	Value(Option<T>),
	/// `Ok` is returned normally and `Err` follows the exception path.
	Result(Option<T>),
}

impl<T: Copy> ReturnConv<T> {
	#[must_use]
	pub const fn conversion(self) -> Option<T> {
		match self {
			Self::Value(value) | Self::Result(value) => value,
		}
	}

	#[must_use]
	pub const fn is_result(self) -> bool {
		matches!(self, Self::Result(_))
	}
}

/// Converts one JavaScript value into primitive `ABI` slots.
#[derive(Clone, Copy)]
pub struct FromJsConv {
	pub(crate) embed: Option<(&'static str, &'static str)>,
	pub(crate) templates: [&'static str; 4],
	pub(crate) sret: Option<&'static str>,
}

impl IntoJsConv {
	/// Produces one JavaScript value from `$slot1` through `$slot4`.
	#[must_use]
	pub const fn new(template: &'static str) -> Self {
		Self {
			embed: None,
			template,
		}
	}

	#[must_use]
	pub const fn with_embed(mut self, embed: (&'static str, &'static str)) -> Self {
		self.embed = Some(embed);
		self
	}
}

impl FromJsConv {
	/// Produces `ABI` slots from `$value`.
	#[must_use]
	pub const fn slot1(template: &'static str) -> Self {
		Self {
			embed: None,
			templates: [template, "", "", ""],
			sret: None,
		}
	}

	#[must_use]
	pub const fn slot2(mut self, template: &'static str) -> Self {
		self.templates[1] = template;
		self
	}

	#[must_use]
	pub const fn slot3(mut self, template: &'static str) -> Self {
		self.templates[2] = template;
		self
	}

	#[must_use]
	pub const fn slot4(mut self, template: &'static str) -> Self {
		self.templates[3] = template;
		self
	}

	/// Stores the slots in an indirect return area.
	///
	/// The function receives every non-empty slot in order, followed by the
	/// indirect return pointer.
	#[must_use]
	pub const fn sret(mut self, function: &'static str) -> Self {
		self.sret = Some(function);
		self
	}

	#[must_use]
	pub const fn with_embed(mut self, embed: (&'static str, &'static str)) -> Self {
		self.embed = Some(embed);
		self
	}
}

/// # Safety
///
/// `Abi`, `from_abi`, and `JS_CONV` must describe one consistent conversion
/// from a JavaScript value to a Rust value. `JS_CONV` produces the primitive
/// slots and `from_abi` reconstructs the Rust value. Multi-slot `ABI`
/// representations must define one slot template for every non-empty slot.
/// Indirect return `ABIs` must define an `sret` function.
pub unsafe trait FromJS {
	const JS_CONV: Option<FromJsConv> = None;

	type Abi: ReturnAbi;

	fn from_abi(raw: Self::Abi) -> Self;
}

/// Converts the return value of a JavaScript import into its Rust result.
///
/// `Abi` describes the successful return value. The raw carrier may be
/// uninitialized when JavaScript throws, so implementations that catch
/// exceptions must inspect the exception state before decoding it.
pub trait ReturnFromJS {
	const JS_CONV: ReturnConv<FromJsConv>;

	type Abi: ReturnAbi;

	fn return_from_abi(raw: MaybeUninit<WasmRet<Self::Abi>>) -> Self;
}

impl<T> ReturnFromJS for T
where
	T: FromJS,
{
	const JS_CONV: ReturnConv<FromJsConv> = ReturnConv::Value(T::JS_CONV);

	type Abi = T::Abi;

	fn return_from_abi(raw: MaybeUninit<WasmRet<Self::Abi>>) -> Self {
		// SAFETY: An ordinary JavaScript import always initializes its return
		// value before the shim returns.
		T::from_abi(unsafe { raw.assume_init() }.join())
	}
}

/// The return `ABI` for exporting [`Result`] to JavaScript.
///
/// The first two slots carry the error and its presence tag. The remaining two
/// slots carry the successful value.
#[doc(hidden)]
pub struct ResultIntoJsAbi<T: WasmAbi> {
	value: Result<T, <JsValue as IntoJS>::Abi>,
}

// SAFETY: The first slot transfers an error `externref`, the second is the
// error tag, and the remaining slots match the successful value's `ABI`.
unsafe impl<T> WasmAbi for ResultIntoJsAbi<T>
where
	T: WasmAbi<Slot3 = EmptySlot, Slot4 = EmptySlot>,
	T::Slot1: Default,
	T::Slot2: Default,
{
	type Slot1 = <JsValue as IntoJS>::Abi;
	type Slot2 = u32;
	type Slot3 = T::Slot1;
	type Slot4 = T::Slot2;

	fn split(self) -> (Self::Slot1, Self::Slot2, Self::Slot3, Self::Slot4) {
		match self.value {
			Ok(value) => {
				let (slot1, slot2, _, _) = value.split();
				(JsValue::UNDEFINED.into_abi(), 0, slot1, slot2)
			}
			Err(error) => (error, 1, Default::default(), Default::default()),
		}
	}

	fn join(
		error: Self::Slot1,
		is_error: Self::Slot2,
		slot1: Self::Slot3,
		slot2: Self::Slot4,
	) -> Self {
		let value = if is_error == 0 {
			Ok(T::join(slot1, slot2, EmptySlot::new(), EmptySlot::new()))
		} else {
			Err(error)
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
}

impl<T, E> ReturnIntoJS for Result<T, E>
where
	T: IntoJS,
	E: Into<JsValue>,
	T::Abi: ReturnAbi<Slot3 = EmptySlot, Slot4 = EmptySlot>,
	<T::Abi as WasmAbi>::Slot1: Default,
	<T::Abi as WasmAbi>::Slot2: Default,
{
	const JS_CONV: ReturnConv<IntoJsConv> = ReturnConv::Result(T::JS_CONV);

	type Abi = ResultIntoJsAbi<T::Abi>;

	fn return_into_abi(self) -> Self::Abi {
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
{
	const JS_CONV: ReturnConv<FromJsConv> = ReturnConv::Result(T::JS_CONV);

	type Abi = T::Abi;

	fn return_from_abi(raw: MaybeUninit<WasmRet<Self::Abi>>) -> Self {
		if let Some(error) = crate::exception::take() {
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
