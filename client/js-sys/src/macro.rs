#[doc(hidden)]
#[macro_export]
macro_rules! asm_import {
	($import:ty as $trait:ident) => {
		if let ::core::option::Option::Some(import) =
			<$import as $crate::hazard::$trait>::ASM_IMPORT_FUNC
		{
			import
		} else {
			""
		}
	};
}

pub use asm_import;

#[doc(hidden)]
#[macro_export]
macro_rules! asm_conv {
	($conv:ty as $trait:ident) => {
		if let ::core::option::Option::Some(conv) = <$conv as $crate::hazard::$trait>::ASM_CONV {
			conv
		} else {
			""
		}
	};
}

pub use asm_conv;

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
	($a:expr, $b:expr, [$($input:ty),*] $(, $output:ty)?) => {'outer: {
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
	($par:literal, $ty:ty) => {{
		const PAR_LEN: ::core::primitive::usize = ::core::primitive::str::len($par);
		const CONV_LEN: ::core::primitive::usize = if let ::core::option::Option::Some((conv, _)) =
			<$ty as $crate::hazard::Input>::JS_CONV
		{
			::core::primitive::str::len(conv)
		} else {
			0
		};
		const POST_CONV_LEN: ::core::primitive::usize =
			if let ::core::option::Option::Some((_, ::core::option::Option::Some(post_conv))) =
				<$ty as $crate::hazard::Input>::JS_CONV
			{
				::core::primitive::str::len(post_conv)
			} else {
				0
			};
		const LEN: ::core::primitive::usize = if let ::core::option::Option::Some((_, post_conv)) =
			&<$ty as $crate::hazard::Input>::JS_CONV
		{
			if ::core::option::Option::is_some(post_conv) {
				2 + PAR_LEN + CONV_LEN + PAR_LEN + POST_CONV_LEN
			} else {
				2 + PAR_LEN + CONV_LEN
			}
		} else {
			0
		};

		const VALUE: [::core::primitive::u8; LEN] = {
			let mut value = [0; LEN];

			if let ::core::option::Option::Some((conv, post_conv)) =
				<$ty as $crate::hazard::Input>::JS_CONV
			{
				let mut index = 0;
				value[index] = b'\t';
				index += 1;

				let par = ::core::primitive::str::as_bytes($par);
				let conv = ::core::primitive::str::as_bytes(conv);

				let mut par_index = 0;
				while index < 1 + PAR_LEN {
					value[index] = par[par_index];
					index += 1;
					par_index += 1;
				}
				let mut conv_index = 0;
				while index < 1 + PAR_LEN + CONV_LEN {
					value[index] = conv[conv_index];
					index += 1;
					conv_index += 1;
				}

				if let ::core::option::Option::Some(post_conv) = post_conv {
					let post_conv = ::core::primitive::str::as_bytes(post_conv);

					let mut par_index = 0;
					while index < LEN - 1 - POST_CONV_LEN {
						value[index] = par[par_index];
						index += 1;
						par_index += 1;
					}
					let mut post_conv_index = 0;
					while index < LEN - 1 {
						value[index] = post_conv[post_conv_index];
						index += 1;
						post_conv_index += 1;
					}
				}

				value[index] = b'\n';
			}

			value
		};

		if let ::core::result::Result::Ok(value) = ::core::str::from_utf8(&VALUE) {
			value
		} else {
			panic!()
		}
	}};
}

pub use js_parameter;

#[doc(hidden)]
#[macro_export]
macro_rules! js_output {
	($start:literal, $direct_call:literal, $indirect_call:literal, $output:ty, $($input:ty),*) => {{
		const INDIRECT_CONDITION: ::core::primitive::bool = {
			::core::option::Option::is_some(&<$output as $crate::hazard::Output>::JS_CONV)
			$(|| ::core::option::Option::is_some(&<$input as $crate::hazard::Input>::JS_CONV))*
		};
		const CONV_CONDITION: ::core::primitive::bool = ::core::option::Option::is_some(&<$output as $crate::hazard::Output>::JS_CONV);

		const START_LEN: ::core::primitive::usize = ::core::primitive::str::len($start);
		const DIRECT_CALL_LEN: ::core::primitive::usize = ::core::primitive::str::len($direct_call);
		const INDIRECT_CALL_LEN: ::core::primitive::usize = ::core::primitive::str::len($indirect_call);
		const CONV_LEN: ::core::primitive::usize =
			if let ::core::option::Option::Some((conv, _)) = <$output as $crate::hazard::Output>::JS_CONV {
				::core::primitive::str::len(conv)
			} else {
				0
			};
		const POST_CONV_LEN: ::core::primitive::usize =
			if let ::core::option::Option::Some((_, post_conv)) =
				<$output as $crate::hazard::Output>::JS_CONV
			{
				::core::primitive::str::len(post_conv)
			} else {
				0
			};
		const LEN: ::core::primitive::usize = {
			let mut len = 0;

			if INDIRECT_CONDITION {
				len += START_LEN + INDIRECT_CALL_LEN + 2;
			} else {
				len += DIRECT_CALL_LEN;
			}

			if CONV_CONDITION {
				len += CONV_LEN + POST_CONV_LEN;
			}

			len
		};

		const VALUE: [::core::primitive::u8; LEN] = {
			let mut value = [0; LEN];
			let mut index = 0;

			if INDIRECT_CONDITION {
				let start = ::core::primitive::str::as_bytes($start);

				while index < START_LEN {
					value[index] = start[index];
					index += 1;
				}
			}

			if let ::core::option::Option::Some((conv, _)) = <$output as $crate::hazard::Output>::JS_CONV {
				let conv = ::core::primitive::str::as_bytes(conv);

				let mut conv_index = 0;
				while index < START_LEN + CONV_LEN {
					value[index] = conv[conv_index];
					index += 1;
					conv_index += 1
				}
			}

			if INDIRECT_CONDITION {
				let indirect_call = ::core::primitive::str::as_bytes($indirect_call);

				let mut indirect_call_index = 0;
				while index < START_LEN + CONV_LEN + INDIRECT_CALL_LEN {
					value[index] = indirect_call[indirect_call_index];
					index += 1;
					indirect_call_index += 1;
				}
			} else {
				let direct_call = ::core::primitive::str::as_bytes($direct_call);

				let mut direct_call_index = 0;
				while index < DIRECT_CALL_LEN {
					value[index] = direct_call[direct_call_index];
					index += 1;
					direct_call_index += 1;
				}
			}

			if let ::core::option::Option::Some((_, post_conv)) = <$output as $crate::hazard::Output>::JS_CONV {
				let post_conv = ::core::primitive::str::as_bytes(post_conv);

				let mut post_conv_index = 0;
				while index < START_LEN + CONV_LEN + INDIRECT_CALL_LEN + POST_CONV_LEN {
					value[index] = post_conv[post_conv_index];
					index += 1;
					post_conv_index += 1
				}
			}

			if INDIRECT_CONDITION {
				value[index] = b'\n';
				value[index + 1] = b'}';
			}

			value
		};

		if let ::core::result::Result::Ok(value) = ::core::str::from_utf8(&VALUE) {
			value
		} else {
			panic!()
		}
	}};
}

pub use js_output;
