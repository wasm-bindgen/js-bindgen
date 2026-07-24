#[must_use]
pub const fn wat_conv_prefix(value: &str) -> &'static str {
	if value.is_empty() { "" } else { "\n  " }
}

#[must_use]
pub const fn wat_import_iter<'a>(values: &[&'a str], index: usize) -> Option<&'a str> {
	let value = values[index];

	if value.is_empty() {
		return None;
	}

	let mut candidate_index = 0;

	while candidate_index < index {
		let candidate = values[candidate_index];

		if value.len() == candidate.len() {
			let mut byte_index = 0;
			let mut equal = true;

			while byte_index < value.len() {
				if value.as_bytes()[byte_index] != candidate.as_bytes()[byte_index] {
					equal = false;
					break;
				}

				byte_index += 1;
			}

			if equal {
				return None;
			}
		}

		candidate_index += 1;
	}

	Some(value)
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_import_list {
	($($value:expr),* $(,)?) => {{
		const VALUES: &[&::core::primitive::str] = &[$($value),*];
		const SIZE: ::core::primitive::usize = {
			let mut size = 0;
			let mut index = 0;

			while index < VALUES.len() {
				if let ::core::option::Option::Some(value) =
					$crate::r#macro::wat_import_iter(VALUES, index)
				{
					size += 1 + value.len();
				}

				index += 1;
			}

			size
		};

		const IMPORTS: [::core::primitive::u8; SIZE] = {
			let mut imports = [0; SIZE];
			let mut byte_index = 0;
			let mut value_index = 0;

			while value_index < VALUES.len() {
				if let ::core::option::Option::Some(value) =
					$crate::r#macro::wat_import_iter(VALUES, value_index)
				{
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

#[doc(hidden)]
#[macro_export]
macro_rules! wat_slot_types {
	($slots:expr, $field:ident $(,)?) => {{
		const SLOTS: [$crate::r#macro::WatSlot; 4] = $slots;

		$crate::r#macro::const_concat!(
			SLOTS[0].$field,
			$crate::r#macro::separator(SLOTS[1].$field),
			SLOTS[1].$field,
			$crate::r#macro::separator(SLOTS[2].$field),
			SLOTS[2].$field,
			$crate::r#macro::separator(SLOTS[3].$field),
			SLOTS[3].$field
		)
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_slot_params {
	($par:literal, $slots:expr, $field:ident $(,)?) => {{
		const SLOTS: [$crate::r#macro::WatSlot; 4] = $slots;

		$crate::r#macro::const_concat_if!(
			!SLOTS[0].abi.is_empty() => [" (param $", $par, "_0 ", SLOTS[0].$field, ")"],
			!SLOTS[1].abi.is_empty() => [" (param $", $par, "_1 ", SLOTS[1].$field, ")"],
			!SLOTS[2].abi.is_empty() => [" (param $", $par, "_2 ", SLOTS[2].$field, ")"],
			!SLOTS[3].abi.is_empty() => [" (param $", $par, "_3 ", SLOTS[3].$field, ")"],
		)
	}};
}
