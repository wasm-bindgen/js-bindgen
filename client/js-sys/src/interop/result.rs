use core::mem::MaybeUninit;

use crate::JsValue;
use crate::hazard::{
	EmptySlot, FromJS, FromJsConv, IntoJS, IntoJsConv, ResultLayout, ReturnAbi, ReturnConv,
	ReturnFromJS, ReturnIntoJS, ReturnMode, Slot, Sret, WasmAbi, WasmRet, WatConv, WatLocal,
	WatType,
};
use crate::runtime::externref::{WAT_INDEX_LOCAL, WAT_TAKE_IMPORTS};

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
