#[must_use]
pub const fn wat_conv_prefix(value: &str) -> &'static str {
	if value.is_empty() { "" } else { "\n  " }
}

#[must_use]
const fn wat_line_end(value: &str, start: usize) -> usize {
	let bytes = value.as_bytes();
	let mut end = start;

	while end < bytes.len() && bytes[end] != b'\n' {
		end += 1;
	}

	end
}

const fn wat_lines_equal(
	left: &str,
	left_start: usize,
	left_end: usize,
	right: &str,
	right_start: usize,
	right_end: usize,
) -> bool {
	if left_end - left_start != right_end - right_start {
		return false;
	}

	let left = left.as_bytes();
	let right = right.as_bytes();
	let mut offset = 0;

	while left_start + offset < left_end {
		if left[left_start + offset] != right[right_start + offset] {
			return false;
		}

		offset += 1;
	}

	true
}

const fn wat_line_was_seen(
	values: &[&str],
	value_index: usize,
	line_start: usize,
	line_end: usize,
) -> bool {
	let value = values[value_index];
	let mut candidate_value_index = 0;

	while candidate_value_index <= value_index {
		let candidate = values[candidate_value_index];
		let limit = if candidate_value_index == value_index {
			line_start
		} else {
			candidate.len()
		};
		let mut candidate_start = 0;

		while candidate_start < limit {
			let candidate_end = wat_line_end(candidate, candidate_start);

			if candidate_end != candidate_start
				&& wat_lines_equal(
					value,
					line_start,
					line_end,
					candidate,
					candidate_start,
					candidate_end,
				) {
				return true;
			}

			candidate_start = candidate_end + 1;
		}

		candidate_value_index += 1;
	}

	false
}

#[must_use]
pub const fn wat_unique_lines_len(values: &[&str]) -> usize {
	let mut size = 0;
	let mut value_index = 0;

	while value_index < values.len() {
		let value = values[value_index];
		let mut line_start = 0;

		while line_start < value.len() {
			let line_end = wat_line_end(value, line_start);

			if line_end != line_start
				&& !wat_line_was_seen(values, value_index, line_start, line_end)
			{
				size += 1 + line_end - line_start;
			}

			line_start = line_end + 1;
		}

		value_index += 1;
	}

	size
}

#[must_use]
pub const fn render_wat_unique_lines<const SIZE: usize>(values: &[&str]) -> [u8; SIZE] {
	let mut output = [0; SIZE];
	let mut output_index = 0;
	let mut value_index = 0;

	while value_index < values.len() {
		let value = values[value_index];
		let bytes = value.as_bytes();
		let mut line_start = 0;

		while line_start < bytes.len() {
			let line_end = wat_line_end(value, line_start);

			if line_end != line_start
				&& !wat_line_was_seen(values, value_index, line_start, line_end)
			{
				output[output_index] = b'\n';
				output_index += 1;

				let mut byte_index = line_start;
				while byte_index < line_end {
					output[output_index] = bytes[byte_index];
					output_index += 1;
					byte_index += 1;
				}
			}

			line_start = line_end + 1;
		}

		value_index += 1;
	}

	output
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_unique_list {
	($($value:expr),* $(,)?) => {{
		const VALUES: &[&::core::primitive::str] = &[$($value),*];
		const SIZE: ::core::primitive::usize =
			$crate::r#macro::wat_unique_lines_len(VALUES);
		const OUTPUT: [::core::primitive::u8; SIZE] =
			$crate::r#macro::render_wat_unique_lines(VALUES);

		if let ::core::result::Result::Ok(value) = ::core::str::from_utf8(&OUTPUT) {
			value
		} else {
			::core::panic!()
		}
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_imports {
	(
		slots = [$($slots:expr),* $(,)?],
		extras = [$($extra:expr),* $(,)?],
	) => {
		$crate::r#macro::wat_unique_list!(
			$(
				($slots)[0].imports,
				($slots)[1].imports,
				($slots)[2].imports,
				($slots)[3].imports,
			)*
			$($extra,)*
		)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! wat_locals {
	(
		slots = [$($slots:expr),* $(,)?],
		extras = [$($extra:expr),* $(,)?],
	) => {
		$crate::r#macro::wat_unique_list!(
			$(
				($slots)[0].locals,
				($slots)[1].locals,
				($slots)[2].locals,
				($slots)[3].locals,
			)*
			$($extra,)*
		)
	};
}

/// Renders the repeated parts of a four-slot `WasmAbi`.
#[doc(hidden)]
#[macro_export]
macro_rules! wat_slots {
	(types, $slots:expr, $field:ident $(,)?) => {{
		const SLOTS: [$crate::r#macro::WatSlot; 4] = $slots;
		const SEP1: &::core::primitive::str =
			if SLOTS[0].$field.is_empty() { "" } else { " " };
		const SEP2: &::core::primitive::str =
			if SLOTS[0].$field.is_empty() && SLOTS[1].$field.is_empty() {
				""
			} else {
				" "
			};
		const SEP3: &::core::primitive::str =
			if SLOTS[0].$field.is_empty()
				&& SLOTS[1].$field.is_empty()
				&& SLOTS[2].$field.is_empty()
			{
				""
			} else {
				" "
			};

		$crate::r#macro::const_concat_if!(
			!SLOTS[0].$field.is_empty() => [SLOTS[0].$field],
			!SLOTS[1].$field.is_empty() => [SEP1, SLOTS[1].$field],
			!SLOTS[2].$field.is_empty() => [SEP2, SLOTS[2].$field],
			!SLOTS[3].$field.is_empty() => [SEP3, SLOTS[3].$field],
		)
	}};
	(grouped_param, $slots:expr, $field:ident $(,)?) => {{
		const TYPES: &::core::primitive::str =
			$crate::r#macro::wat_slots!(types, $slots, $field);

		$crate::r#macro::const_concat_if!(
			!TYPES.is_empty() => [" (param ", TYPES, ")"],
		)
	}};
	(params, $par:literal, $slots:expr, $field:ident $(,)?) => {{
		const SLOTS: [$crate::r#macro::WatSlot; 4] = $slots;

		$crate::r#macro::const_concat_if!(
			!SLOTS[0].abi.is_empty() => [" (param $", $par, "_0 ", SLOTS[0].$field, ")"],
			!SLOTS[1].abi.is_empty() => [" (param $", $par, "_1 ", SLOTS[1].$field, ")"],
			!SLOTS[2].abi.is_empty() => [" (param $", $par, "_2 ", SLOTS[2].$field, ")"],
			!SLOTS[3].abi.is_empty() => [" (param $", $par, "_3 ", SLOTS[3].$field, ")"],
		)
	}};
	(export_gets, $par:literal, $slots:expr $(,)?) => {{
		const SLOTS: [$crate::r#macro::WatSlot; 4] = $slots;

		$crate::r#macro::const_concat_if!(
			!SLOTS[0].abi.is_empty() => ["  local.get $", $par, "_0", $crate::r#macro::wat_conv_prefix(SLOTS[0].conv), SLOTS[0].conv, "\n"],
			!SLOTS[1].abi.is_empty() => ["  local.get $", $par, "_1", $crate::r#macro::wat_conv_prefix(SLOTS[1].conv), SLOTS[1].conv, "\n"],
			!SLOTS[2].abi.is_empty() => ["  local.get $", $par, "_2", $crate::r#macro::wat_conv_prefix(SLOTS[2].conv), SLOTS[2].conv, "\n"],
			!SLOTS[3].abi.is_empty() => ["  local.get $", $par, "_3", $crate::r#macro::wat_conv_prefix(SLOTS[3].conv), SLOTS[3].conv, "\n"],
		)
	}};
	(loads, $ty:ty, $slots:expr $(,)?) => {{
		const SLOTS: [$crate::r#macro::WatSlot; 4] = $slots;
		const OFFSET_0: &::core::primitive::str = $crate::r#macro::const_integer_str!(
			$crate::r#macro::export_output_slot_offset::<$ty, 0>()
		);
		const OFFSET_1: &::core::primitive::str = $crate::r#macro::const_integer_str!(
			$crate::r#macro::export_output_slot_offset::<$ty, 1>()
		);
		const OFFSET_2: &::core::primitive::str = $crate::r#macro::const_integer_str!(
			$crate::r#macro::export_output_slot_offset::<$ty, 2>()
		);
		const OFFSET_3: &::core::primitive::str = $crate::r#macro::const_integer_str!(
			$crate::r#macro::export_output_slot_offset::<$ty, 3>()
		);

		$crate::r#macro::const_concat_if!(
			!SLOTS[0].abi.is_empty() => ["  local.get $retptr\n  ", SLOTS[0].abi, ".load offset=", OFFSET_0, $crate::r#macro::wat_conv_prefix(SLOTS[0].conv), SLOTS[0].conv, "\n"],
			!SLOTS[1].abi.is_empty() => ["  local.get $retptr\n  ", SLOTS[1].abi, ".load offset=", OFFSET_1, $crate::r#macro::wat_conv_prefix(SLOTS[1].conv), SLOTS[1].conv, "\n"],
			!SLOTS[2].abi.is_empty() => ["  local.get $retptr\n  ", SLOTS[2].abi, ".load offset=", OFFSET_2, $crate::r#macro::wat_conv_prefix(SLOTS[2].conv), SLOTS[2].conv, "\n"],
			!SLOTS[3].abi.is_empty() => ["  local.get $retptr\n  ", SLOTS[3].abi, ".load offset=", OFFSET_3, $crate::r#macro::wat_conv_prefix(SLOTS[3].conv), SLOTS[3].conv, "\n"],
		)
	}};
}

/// Renders input fragments for Rust exports.
#[doc(hidden)]
#[macro_export]
macro_rules! wat_input {
	(export raw_param; $ty:ty $(,)?) => {
		$crate::r#macro::wat_slots!(
			grouped_param,
			$crate::r#macro::from_js_wat_slots::<$ty>(),
			abi,
		)
	};
	(export params; $par:literal, $ty:ty $(,)?) => {
		$crate::r#macro::wat_slots!(
			params,
			$par,
			$crate::r#macro::from_js_wat_slots::<$ty>(),
			boundary,
		)
	};
	(export gets; $par:literal, $ty:ty $(,)?) => {
		$crate::r#macro::wat_slots!(
			export_gets,
			$par,
			$crate::r#macro::from_js_wat_slots::<$ty>(),
		)
	};
}
