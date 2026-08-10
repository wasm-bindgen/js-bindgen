use alloc::string::String;

use crate::JsString;

/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/JavaScript_interface/Memory/Memory#address)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AddressType {
	I32,
	I64,
}

impl AddressType {
	pub(super) const fn as_str(self) -> &'static str {
		match self {
			Self::I32 => "i32",
			Self::I64 => "i64",
		}
	}

	pub(super) fn from_js_string(value: &JsString) -> Option<Self> {
		match String::from(value).as_str() {
			"i32" => Some(Self::I32),
			"i64" => Some(Self::I64),
			_ => None,
		}
	}
}
