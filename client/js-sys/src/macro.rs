#[doc(hidden)]
#[macro_export]
macro_rules! asm_import {
	($import:ty as $trait:ident) => {
		$crate::r#macro::unwrap_or_default(<$import as $crate::hazard::$trait>::ASM_IMPORT_FUNC)
	};
}

pub use asm_import;

#[doc(hidden)]
#[macro_export]
macro_rules! asm_indirect {
	($ty:ty) => {
		if <$ty as $crate::hazard::Output>::ASM_DIRECT {
			""
		} else {
			$crate::r#macro::const_concat!(<$ty as $crate::hazard::Output>::ASM_TYPE, ", ")
		}
	};
}

pub use asm_indirect;

#[doc(hidden)]
#[macro_export]
macro_rules! asm_direct {
	($ty:ty) => {
		if <$ty as $crate::hazard::Output>::ASM_DIRECT {
			<$ty as $crate::hazard::Output>::ASM_TYPE
		} else {
			""
		}
	};
}

pub use asm_direct;

#[doc(hidden)]
#[macro_export]
macro_rules! asm_input {
	($local:literal, $ty:ty) => {
		if ::core::option::Option::is_some(&<$ty as $crate::hazard::Input>::ASM_CONV) {
			const CONV: &str =
				$crate::r#macro::unwrap_or_default(<$ty as $crate::hazard::Input>::ASM_CONV);

			$crate::r#macro::const_concat!($local, "\n\t", CONV)
		} else {
			$local
		}
	};
	($direct_local:literal, $indirect_local:literal, $ty:ty, $output:ty) => {
		if ::core::option::Option::is_some(&<$ty as $crate::hazard::Input>::ASM_CONV) {
			const CONV: &str =
				$crate::r#macro::unwrap_or_default(<$ty as $crate::hazard::Input>::ASM_CONV);

			if <$output as $crate::hazard::Output>::ASM_DIRECT {
				$crate::r#macro::const_concat!($direct_local, "\n\t", CONV)
			} else {
				$crate::r#macro::const_concat!($indirect_local, "\n\t", CONV)
			}
		} else if <$output as $crate::hazard::Output>::ASM_DIRECT {
			$direct_local
		} else {
			$indirect_local
		}
	};
}

pub use asm_input;

#[doc(hidden)]
#[macro_export]
macro_rules! asm_output {
	($ty:ty) => {
		if ::core::option::Option::is_some(&<$ty as $crate::hazard::Output>::ASM_CONV) {
			const CONV: &str =
				$crate::r#macro::unwrap_or_default(<$ty as $crate::hazard::Output>::ASM_CONV);

			if <$ty as $crate::hazard::Output>::ASM_DIRECT {
				$crate::r#macro::const_concat!("\n\t", CONV)
			} else {
				$crate::r#macro::const_concat!("\n\tlocal.get 0\n\t", CONV)
			}
		} else {
			""
		}
	};
}

pub use asm_output;

#[doc(hidden)]
#[macro_export]
macro_rules! js_import {
	($import:ty as $trait:ident) => {
		if let ::core::option::Option::Some(import) = <$import as $crate::hazard::$trait>::JS_EMBED
		{
			import
		} else {
			("", "")
		}
	};
}

pub use js_import;

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
		if let ::core::option::Option::Some((_, post_conv)) =
			<$ty as $crate::hazard::Input>::JS_CONV
		{
			const CONV: &::core::primitive::str = if let ::core::option::Option::Some((conv, _)) =
				<$ty as $crate::hazard::Input>::JS_CONV
			{
				conv
			} else {
				""
			};

			if ::core::option::Option::is_some(&post_conv) {
				const POST_CONV: &::core::primitive::str = if let ::core::option::Option::Some((
					_,
					::core::option::Option::Some(post_conv),
				)) =
					<$ty as $crate::hazard::Input>::JS_CONV
				{
					post_conv
				} else {
					""
				};

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
			const CONV: [&::core::primitive::str; 2] =
				if let ::core::option::Option::Some((conv, post_conv)) =
					<$output as $crate::hazard::Output>::JS_CONV
				{
					[conv, post_conv]
				} else {
					["", ""]
				};

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

#[must_use]
pub const fn unwrap_or_default(option: Option<&str>) -> &str {
	if let Some(value) = option { value } else { "" }
}
