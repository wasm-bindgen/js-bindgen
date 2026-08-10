use super::{Array, Object};
use crate::hazard::JsCast;
use crate::{JsString, JsValue, js_sys};

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/Error#cause)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug)]
	pub type ErrorOptions;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause)
	#[must_use]
	#[js_sys(getter)]
	pub fn get_cause(self: &ErrorOptions) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause)
	#[js_sys(setter)]
	pub fn set_cause(self: &ErrorOptions, cause: &JsValue);
}

impl ErrorOptions {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause)
	#[must_use]
	pub fn new(cause: &JsValue) -> Self {
		let ret: Self = JsCast::unchecked_from(Object::new().into());
		ret.set_cause(cause);
		ret
	}
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error)
	#[js_sys(extends = Object)]
	#[derive(Clone, Debug)]
	pub type Error;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/Error)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new(message: &str) -> Error;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/Error)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_options(message: &str, options: &ErrorOptions) -> Error;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/isError)
	#[must_use]
	#[js_sys(static_of = Error, js_name = "isError")]
	pub fn is_error(value: &JsValue) -> bool;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause)
	#[must_use]
	#[js_sys(getter)]
	pub fn cause(self: &Error) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/cause)
	#[js_sys(setter)]
	pub fn set_cause(self: &Error, cause: &JsValue);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/message)
	#[must_use]
	#[js_sys(getter)]
	pub fn message(self: &Error) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/message)
	#[js_sys(setter)]
	pub fn set_message(self: &Error, message: &str);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/name)
	#[must_use]
	#[js_sys(getter)]
	pub fn name(self: &Error) -> JsString;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/name)
	#[js_sys(setter)]
	pub fn set_name(self: &Error, name: &str);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error/toString)
	#[must_use]
	#[js_sys(js_name = "toString")]
	pub fn to_string(self: &Error) -> JsString;
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AggregateError)
	#[js_sys(extends = Error, extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type AggregateError;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AggregateError/AggregateError)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new(errors: &[JsValue]) -> AggregateError;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AggregateError/AggregateError)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_message(errors: &[JsValue], message: &str) -> AggregateError;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AggregateError/AggregateError)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_options(
		errors: &[JsValue],
		message: &str,
		options: &ErrorOptions,
	) -> AggregateError;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AggregateError/errors)
	#[must_use]
	#[js_sys(getter = "errors")]
	pub fn errors(self: &AggregateError) -> Array<JsValue>;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/AggregateError/errors)
	#[js_sys(setter)]
	pub fn set_errors(self: &AggregateError, errors: &Array<JsValue>);
}

#[js_sys(js_sys = crate)]
extern "js-sys" {
	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SuppressedError)
	#[js_sys(extends = Error, extends = Object)]
	#[derive(Clone, Debug, PartialEq)]
	pub type SuppressedError;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SuppressedError/SuppressedError)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new(error: &JsValue, suppressed: &JsValue) -> SuppressedError;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SuppressedError/SuppressedError)
	#[must_use]
	#[js_sys(constructor)]
	pub fn new_with_message(
		error: &JsValue,
		suppressed: &JsValue,
		message: &str,
	) -> SuppressedError;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SuppressedError/error)
	#[must_use]
	#[js_sys(getter)]
	pub fn error(self: &SuppressedError) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SuppressedError/error)
	#[js_sys(setter)]
	pub fn set_error(self: &SuppressedError, error: &JsValue);

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SuppressedError/suppressed)
	#[must_use]
	#[js_sys(getter)]
	pub fn suppressed(self: &SuppressedError) -> JsValue;

	/// [`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SuppressedError/suppressed)
	#[js_sys(setter)]
	pub fn set_suppressed(self: &SuppressedError, suppressed: &JsValue);
}

macro_rules! standard_error_types {
	($(
		$type:ident = $js_name:literal {
			type_doc = $type_doc:literal,
			constructor_doc = $constructor_doc:literal,
		}
	)*) => {$(
		#[js_sys(js_sys = crate)]
		extern "js-sys" {
			#[doc = $type_doc]
			#[js_sys(js_name = $js_name, extends = Error, extends = Object)]
			#[derive(Clone, Debug, PartialEq)]
			pub type $type;

			#[doc = $constructor_doc]
			#[must_use]
			#[js_sys(constructor)]
			pub fn new(message: &str) -> $type;

			#[doc = $constructor_doc]
			#[must_use]
			#[js_sys(constructor)]
			pub fn new_with_options(message: &str, options: &ErrorOptions) -> $type;
		}
	)*};
}

standard_error_types! {
	EvalError = "EvalError" {
		type_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/EvalError)",
		constructor_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/EvalError/EvalError)",
	}
	RangeError = "RangeError" {
		type_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RangeError)",
		constructor_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RangeError/RangeError)",
	}
	ReferenceError = "ReferenceError" {
		type_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ReferenceError)",
		constructor_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ReferenceError/ReferenceError)",
	}
	SyntaxError = "SyntaxError" {
		type_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SyntaxError)",
		constructor_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SyntaxError/SyntaxError)",
	}
	TypeError = "TypeError" {
		type_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypeError)",
		constructor_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypeError/TypeError)",
	}
	UriError = "URIError" {
		type_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/URIError)",
		constructor_doc = "[`MDN` documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/URIError/URIError)",
	}
}
