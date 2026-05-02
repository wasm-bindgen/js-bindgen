use core::mem::ManuallyDrop;
use core::ptr;

use crate::JsValue;

/// # Safety
///
/// This directly manipulates Wasm output and therefor all bets are off! (TODO)
pub unsafe trait Input {
	const WAT_TYPE: &str;
	const WAT_CONV: Option<InputWatConv> = None;
	const JS_CONV: Option<InputJsConv> = None;

	type Type;

	fn into_raw(self) -> Self::Type;
}

pub struct InputWatConv {
	pub import: Option<&'static str>,
	pub conv: &'static str,
	pub r#type: &'static str,
}

pub struct InputJsConv {
	pub embed: Option<(&'static str, &'static str)>,
	pub pre: &'static str,
	pub post: Option<&'static str>,
}

/// # Safety
///
/// This directly manipulates Wasm output and therefor all bets are off! (TODO)
pub unsafe trait Output {
	const WAT_TYPE: &str;
	const WAT_CONV: Option<OutputWatConv> = None;
	const JS_CONV: Option<OutputJsConv> = None;

	type Type;

	fn from_raw(raw: Self::Type) -> Self;
}

pub struct OutputWatConv {
	pub import: Option<&'static str>,
	pub direct: bool,
	pub conv: &'static str,
	pub r#type: &'static str,
}

pub struct OutputJsConv {
	pub embed: Option<(&'static str, &'static str)>,
	pub pre: &'static str,
	pub post: &'static str,
}

/// # Safety
///
/// This MUST only be implemented on types that are `#[transparent]` over a
/// [`JsValue`]. (TODO)
pub unsafe trait JsCast: Sized {
	#[must_use]
	fn unchecked_from(value: JsValue) -> Self {
		// This seems to be the only way to transmute between two owned types without
		// copying when the size is unknown. In this case the size is unknown because
		// `Self` is a generic.

		union Transmute<A, B> {
			from: ManuallyDrop<A>,
			to: ManuallyDrop<B>,
		}

		let transmute = Transmute {
			from: ManuallyDrop::new(value),
		};
		// SAFETY: The trait assumes that `Self` is `#[transparent]` over a `JsValue`.
		let result = unsafe { transmute.to };
		ManuallyDrop::into_inner(result)
	}

	#[must_use]
	fn unchecked_from_ref(value: &JsValue) -> &Self {
		let ptr: *const Self = ptr::from_ref(value).cast();
		// SAFETY: The trait assumes that `Self` is `#[transparent]` over a `JsValue`.
		unsafe { &*ptr }
	}
}
