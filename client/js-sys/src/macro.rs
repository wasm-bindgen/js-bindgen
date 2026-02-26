#[must_use]
pub const fn select_any(a: &'static str, b: &'static str, import: &[Option<&str>]) -> &'static str {
	let mut any_import = false;
	let mut index = 0;

	while index < import.len() {
		if import[index].is_some() {
			any_import = true;
			break;
		}

		index += 1;
	}

	if any_import { b } else { a }
}

#[must_use]
pub const fn select(a: &'static str, b: &'static str, import: Option<&str>) -> &'static str {
	if import.is_some() { b } else { a }
}

#[must_use]
pub const fn or(a: &'static str, import: Option<&'static str>) -> &'static str {
	if let Some(conversion) = import {
		conversion
	} else {
		a
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! parameter {
	($par:literal, $ty:ty) => {{
		const PAR_LEN: ::core::primitive::usize = ::core::primitive::str::len($par);
		const CONV_LEN: ::core::primitive::usize =
			if let ::core::option::Option::Some(conv) = <$ty as $crate::hazard::Input>::JS_CONV {
				::core::primitive::str::len(conv)
			} else {
				0
			};
		const CONV_POST_LEN: ::core::primitive::usize =
			if let ::core::option::Option::Some(conv_post) =
				<$ty as $crate::hazard::Input>::JS_CONV_POST
			{
				::core::primitive::str::len(conv_post)
			} else {
				0
			};
		const LEN: ::core::primitive::usize =
			if ::core::option::Option::is_some(&<$ty as $crate::hazard::Input>::JS_CONV) {
				if ::core::option::Option::is_some(&<$ty as $crate::hazard::Input>::JS_CONV_POST) {
					2 + PAR_LEN + CONV_LEN + PAR_LEN + CONV_POST_LEN
				} else {
					2 + PAR_LEN + CONV_LEN
				}
			} else {
				0
			};

		const VALUE: [::core::primitive::u8; LEN] = {
			let mut value = [0; LEN];

			if let ::core::option::Option::Some(conv) = <$ty as $crate::hazard::Input>::JS_CONV {
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

				if let ::core::option::Option::Some(conv_post) =
					<$ty as $crate::hazard::Input>::JS_CONV_POST
				{
					let conv_post = ::core::primitive::str::as_bytes(conv_post);

					let mut par_index = 0;
					while index < LEN - 1 - CONV_POST_LEN {
						value[index] = par[par_index];
						index += 1;
						par_index += 1;
					}
					let mut conv_post_index = 0;
					while index < LEN - 1 {
						value[index] = conv_post[conv_post_index];
						index += 1;
						conv_post_index += 1;
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

pub use parameter;

#[doc(hidden)]
#[macro_export]
macro_rules! output {
	($start:literal, $direct_call:literal, $indirect_call:literal, $output:ty, $($input:ty),*) => {{
		const INDIRECT_CONDITION: ::core::primitive::bool = 'outer: {
			let conditions = [<$output as $crate::hazard::Output>::JS_CONV, $(<$input as $crate::hazard::Input>::JS_CONV),*];
			let mut index = 0;

			while index < <[_]>::len(&conditions) {
				if ::core::option::Option::is_some(&conditions[index]) {
					break 'outer true;
				}

				index += 1;
			}

			false
		};
		const CONV_CONDITION: ::core::primitive::bool = ::core::option::Option::is_some(&<$output as $crate::hazard::Output>::JS_CONV);
		const CONV_POST_CONDITION: ::core::primitive::bool = ::core::option::Option::is_some(&<$output as $crate::hazard::Output>::JS_CONV_POST);

		const START_LEN: ::core::primitive::usize = ::core::primitive::str::len($start);
		const DIRECT_CALL_LEN: ::core::primitive::usize = ::core::primitive::str::len($direct_call);
		const INDIRECT_CALL_LEN: ::core::primitive::usize = ::core::primitive::str::len($indirect_call);
		const CONV_LEN: ::core::primitive::usize =
			if let ::core::option::Option::Some(conv) = <$output as $crate::hazard::Output>::JS_CONV {
				::core::primitive::str::len(conv)
			} else {
				0
			};
		const CONV_POST_LEN: ::core::primitive::usize =
			if let ::core::option::Option::Some(conv_post) =
				<$output as $crate::hazard::Output>::JS_CONV_POST
			{
				::core::primitive::str::len(conv_post)
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
				len += CONV_LEN;
			}

			if CONV_POST_CONDITION {
				len += CONV_POST_LEN;
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

			if let ::core::option::Option::Some(conv) = <$output as $crate::hazard::Output>::JS_CONV {
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

			if let ::core::option::Option::Some(conv_post) = <$output as $crate::hazard::Output>::JS_CONV_POST {
				let conv_post = ::core::primitive::str::as_bytes(conv_post);

				let mut conv_post_index = 0;
				while index < START_LEN + CONV_LEN + INDIRECT_CALL_LEN + CONV_POST_LEN {
					value[index] = conv_post[conv_post_index];
					index += 1;
					conv_post_index += 1
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

pub use output;
