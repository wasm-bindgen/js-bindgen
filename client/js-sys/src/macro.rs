#[doc(hidden)]
#[macro_export]
macro_rules! asm_imports {
	(($($input:ty),*) $(, $output:ty)? $(,)?) => {{
		const VALUES: &[&str] = &[
			$($crate::r#macro::asm_input_import::<$input>(),)*
			$($crate::r#macro::asm_output_import::<$output>(),)?
		];
		const SIZE: usize = {
			let mut size = 0;
			let mut index = 0;

			while index < VALUES.len() {
				if let Some(value) = $crate::r#macro::asm_import_iter(VALUES, index) {
					size += 1 + value.len();
				}

				index += 1;
			}

			size
		};

		const IMPORTS: [u8; SIZE] = {
			let mut imports = [0; SIZE];
			let mut byte_index = 0;
			let mut value_index = 0;

			while value_index < VALUES.len() {
				if let Some(value) = $crate::r#macro::asm_import_iter(VALUES, value_index) {
					imports[byte_index] = b'\n';
					byte_index += 1;

					let value = value.as_bytes();
					let mut index = 0;

					while index < value.len() {
						imports[byte_index] = value[index];
						byte_index += 1;
						index += 1;
					}
				}

				value_index += 1;
			}

			imports
		};

		if let ::core::result::Result::Ok(value) = ::core::str::from_utf8(&IMPORTS) {
			value
		} else {
			::core::panic!()
		}
	}};
}

pub use asm_imports;

#[doc(hidden)]
#[macro_export]
macro_rules! asm_indirect {
	($ty:ty) => {
		if $crate::r#macro::direct::<$ty>() {
			""
		} else {
			$crate::r#macro::const_concat!(<$ty as $crate::hazard::Output>::ASM_TYPE, " ")
		}
	};
}

pub use asm_indirect;

#[doc(hidden)]
#[macro_export]
macro_rules! asm_input {
	($ty:ty) => {
		if ::core::option::Option::is_some(&<$ty as $crate::hazard::Input>::ASM_CONV) {
			const CONV: &::core::primitive::str = $crate::r#macro::asm_input_conv::<$ty>();

			$crate::r#macro::const_concat!("\n  ", CONV)
		} else {
			""
		}
	};
}

pub use asm_input;

#[doc(hidden)]
#[macro_export]
macro_rules! asm_output {
	($ty:ty) => {
		if ::core::option::Option::is_some(&<$ty as $crate::hazard::Output>::ASM_CONV) {
			const CONV: &::core::primitive::str = $crate::r#macro::asm_output_conv::<$ty>();

			if $crate::r#macro::direct::<$ty>() {
				$crate::r#macro::const_concat!("\n  ", CONV)
			} else {
				$crate::r#macro::const_concat!("\n  local.get 0\n  ", CONV)
			}
		} else {
			""
		}
	};
}

pub use asm_output;

#[doc(hidden)]
#[macro_export]
macro_rules! js_select {
	($a:expr, $b:expr, ($($input:ty),*) $(, $output:ty)? $(,)?) => {'outer: {
		$(
			if ::core::option::Option::is_some(&<$input as $crate::hazard::Input>::JS_CONV) {
				break 'outer $b;
			}
		)*

		$(
			if ::core::option::Option::is_some(&<$output as $crate::hazard::Output>::JS_CONV) {
				break 'outer $b;
			}
		)?

		$a
	}};
}

pub use js_select;

#[doc(hidden)]
#[macro_export]
macro_rules! js_parameter {
	($par:literal, $ty:ty $(,)?) => {
		if let ::core::option::Option::Some($crate::hazard::InputJsConv { post, .. }) =
			<$ty as $crate::hazard::Input>::JS_CONV
		{
			const CONV: &::core::primitive::str = $crate::r#macro::js_input_conv_pre::<$ty>();

			if ::core::option::Option::is_some(&post) {
				const POST_CONV: &::core::primitive::str =
					$crate::r#macro::js_input_conv_post::<$ty>();

				$crate::r#macro::const_concat!("\t", $par, CONV, $par, POST_CONV, "\n")
			} else {
				$crate::r#macro::const_concat!("\t", $par, CONV, "\n")
			}
		} else {
			""
		}
	};
}

pub use js_parameter;

#[doc(hidden)]
#[macro_export]
macro_rules! js_output {
	($start:literal, $direct_call:literal, $indirect_call:literal, $output:ty, $($input:ty),* $(,)?) => {{
		let indirect_condition = ::core::option::Option::is_some(&<$output as $crate::hazard::Output>::JS_CONV)
			$(|| ::core::option::Option::is_some(&<$input as $crate::hazard::Input>::JS_CONV))*;

		if ::core::option::Option::is_some(&<$output as $crate::hazard::Output>::JS_CONV) {
			const CONV: [&::core::primitive::str; 2] = $crate::r#macro::js_output_conv::<$output>();

			if indirect_condition {
				$crate::r#macro::const_concat!($start, CONV[0], $indirect_call, CONV[1], "\n}")
			} else {
				$crate::r#macro::const_concat!(CONV[0], $direct_call, CONV[1])
			}
		} else {
			if indirect_condition {
				$crate::r#macro::const_concat!($start, $indirect_call, "\n}")
			} else {
				$crate::r#macro::const_concat!($direct_call)
			}
		}
	}};
}

pub use js_output;

#[doc(hidden)]
#[macro_export]
macro_rules! const_concat {
	($($value:expr),*) => {{
		const LEN: ::core::primitive::usize = $(::core::primitive::str::len($value) +)* 0;
		const VALUE: [::core::primitive::u8; LEN] = {
			let mut value = [0; LEN];

			let mut index = 0;

			$(
				let mut local_index = 0;
				let limit = index + ::core::primitive::str::len($value);
				let bytes = ::core::primitive::str::as_bytes($value);
				while index < limit {
					value[index] = bytes[local_index];
					index += 1;
					local_index += 1;
				}
			)*

			value
		};

		if let ::core::result::Result::Ok(value) = ::core::str::from_utf8(&VALUE) {
			value
		} else {
			::core::panic!()
		}
	}};
}

pub use const_concat;

use crate::hazard::{Input, InputAsmConv, InputJsConv, Output, OutputAsmConv, OutputJsConv};

#[must_use]
pub const fn asm_direct<T: Output>() -> &'static str {
	if direct::<T>() { T::ASM_TYPE } else { "" }
}

#[must_use]
pub const fn asm_import_iter<'a>(values: &[&'a str], index: usize) -> Option<&'a str> {
	let value = values[index];

	if value.is_empty() {
		return None;
	}

	let mut c_index = 0;

	while c_index < index {
		let c_value = values[c_index];

		if value.len() == c_value.len() {
			let mut l_index = 0;
			let mut equal = true;

			while l_index < value.len() {
				if value.as_bytes()[l_index] != c_value.as_bytes()[l_index] {
					equal = false;
					break;
				}

				l_index += 1;
			}

			if equal {
				return None;
			}
		}

		c_index += 1;
	}

	Some(value)
}

#[must_use]
pub const fn asm_input_import<T: Input>() -> &'static str {
	if let Some(InputAsmConv {
		import: Some(import),
		..
	}) = T::ASM_CONV
	{
		import
	} else {
		""
	}
}

#[must_use]
pub const fn asm_input_import_type<T: Input>() -> &'static str {
	if let Some(InputAsmConv { r#type, .. }) = T::ASM_CONV {
		r#type
	} else {
		T::ASM_TYPE
	}
}

#[must_use]
pub const fn asm_input_conv<T: Input>() -> &'static str {
	if let Some(InputAsmConv { conv, .. }) = T::ASM_CONV {
		conv
	} else {
		""
	}
}

#[must_use]
pub const fn asm_output_import<T: Output>() -> &'static str {
	if let Some(OutputAsmConv {
		import: Some(import),
		..
	}) = T::ASM_CONV
	{
		import
	} else {
		""
	}
}

#[must_use]
pub const fn asm_output_import_type<T: Output>() -> &'static str {
	if let Some(OutputAsmConv { r#type, .. }) = T::ASM_CONV {
		r#type
	} else {
		T::ASM_TYPE
	}
}

#[must_use]
pub const fn asm_output_conv<T: Output>() -> &'static str {
	if let Some(OutputAsmConv { conv, .. }) = T::ASM_CONV {
		conv
	} else {
		""
	}
}

#[must_use]
pub const fn direct<T: Output>() -> bool {
	if let Some(OutputAsmConv { direct, .. }) = T::ASM_CONV {
		direct
	} else {
		true
	}
}

#[must_use]
pub const fn js_input_embed<T: Input>() -> (&'static str, &'static str) {
	if let Some(InputJsConv {
		embed: Some(embed), ..
	}) = T::JS_CONV
	{
		embed
	} else {
		("", "")
	}
}

#[must_use]
pub const fn js_output_embed<T: Output>() -> (&'static str, &'static str) {
	if let Some(OutputJsConv {
		embed: Some(embed), ..
	}) = T::JS_CONV
	{
		embed
	} else {
		("", "")
	}
}

#[must_use]
pub const fn js_input_conv_pre<T: Input>() -> &'static str {
	if let Some(InputJsConv { pre, .. }) = T::JS_CONV {
		pre
	} else {
		""
	}
}

#[must_use]
pub const fn js_input_conv_post<T: Input>() -> &'static str {
	if let Some(InputJsConv {
		post: Some(post), ..
	}) = T::JS_CONV
	{
		post
	} else {
		""
	}
}

#[must_use]
pub const fn js_output_conv<T: Output>() -> [&'static str; 2] {
	if let Some(OutputJsConv { pre, post, .. }) = T::JS_CONV {
		[pre, post]
	} else {
		[""; 2]
	}
}
