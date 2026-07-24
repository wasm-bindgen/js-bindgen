#[doc(hidden)]
#[macro_export]
macro_rules! js_template {
	($template:expr, value = $value:expr $(,)?) => {
		$crate::r#macro::js_template!(@render $template, [$value, "", "", "", ""])
	};
	($template:expr, slots = $slots:expr $(,)?) => {{
		const JS_TEMPLATE_SLOTS: [&::core::primitive::str; 4] = $slots;
		$crate::r#macro::js_template!(@render $template, [
			"",
			JS_TEMPLATE_SLOTS[0],
			JS_TEMPLATE_SLOTS[1],
			JS_TEMPLATE_SLOTS[2],
			JS_TEMPLATE_SLOTS[3],
		])
	}};
	(@render $template:expr, [$value:expr, $slot1:expr, $slot2:expr, $slot3:expr, $slot4:expr $(,)?]) => {{
		const JS_TEMPLATE_REPLACEMENTS: [&::core::primitive::str; 5] =
			[$value, $slot1, $slot2, $slot3, $slot4];
		const JS_TEMPLATE_LEN: ::core::primitive::usize = $crate::r#macro::js_template_len(
			$template,
			&JS_TEMPLATE_REPLACEMENTS,
		);
		const JS_TEMPLATE_VALUE: [::core::primitive::u8; JS_TEMPLATE_LEN] =
			$crate::r#macro::render_js_template::<JS_TEMPLATE_LEN>(
				$template,
				&JS_TEMPLATE_REPLACEMENTS,
			);

		// SAFETY: Rendering only replaces complete ASCII placeholders with valid strings.
		unsafe { ::core::str::from_utf8_unchecked(&JS_TEMPLATE_VALUE) }
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! const_concat {
	($($value:expr),* $(,)?) => {{
		const VALUES: &[&::core::primitive::str] = &[$($value),*];
		const LEN: ::core::primitive::usize = $crate::r#macro::const_concat_len(VALUES);
		const VALUE: [::core::primitive::u8; LEN] = {
			let mut value = [0; LEN];
			let mut index = 0;
			let mut value_index = 0;

			while value_index < VALUES.len() {
				let mut local_index = 0;
				let bytes = ::core::primitive::str::as_bytes(VALUES[value_index]);

				while local_index < bytes.len() {
					value[index] = bytes[local_index];
					index += 1;
					local_index += 1;
				}

				value_index += 1;
			}

			value
		};

		// SAFETY: Joining valid strings keeps the result valid.
		unsafe { ::core::str::from_utf8_unchecked(&VALUE) }
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! const_concat_if {
	($($condition:expr => [$($value:expr),* $(,)?]),* $(,)?) => {{
		const GROUPS: &[(::core::primitive::bool, &[&::core::primitive::str])] = &[
			$(($condition, &[$($value),*]),)*
		];
		const LEN: ::core::primitive::usize =
			$crate::r#macro::const_concat_if_len(GROUPS);
		const VALUE: [::core::primitive::u8; LEN] = {
			let mut value = [0; LEN];
			let mut index = 0;
			let mut group_index = 0;

			while group_index < GROUPS.len() {
				if GROUPS[group_index].0 {
					let values = GROUPS[group_index].1;
					let mut value_index = 0;

					while value_index < values.len() {
						let bytes = ::core::primitive::str::as_bytes(values[value_index]);
						let mut byte_index = 0;

						while byte_index < bytes.len() {
							value[index] = bytes[byte_index];
							index += 1;
							byte_index += 1;
						}

						value_index += 1;
					}
				}

				group_index += 1;
			}

			value
		};

		// SAFETY: Joining valid strings keeps the result valid.
		unsafe { ::core::str::from_utf8_unchecked(&VALUE) }
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! const_integer_str {
	($value:expr $(,)?) => {{
		const INTEGER: $crate::js_bindgen::r#macro::ConstInteger<::core::primitive::usize> =
			$crate::js_bindgen::r#macro::ConstInteger($value);
		const LEN: ::core::primitive::usize = INTEGER.__jbg_len();
		const VALUE: [::core::primitive::u8; LEN] = INTEGER.__jbg_to_le_bytes::<LEN>();

		// SAFETY: The integer formatter only emits ASCII digits.
		unsafe { ::core::str::from_utf8_unchecked(&VALUE) }
	}};
}

#[must_use]
pub const fn separator(value: &str) -> &'static str {
	if value.is_empty() { "" } else { " " }
}

#[must_use]
pub const fn separator_between(left: &str, right: &str) -> &'static str {
	if left.is_empty() || right.is_empty() {
		""
	} else {
		", "
	}
}

#[must_use]
pub const fn const_concat_len(values: &[&str]) -> usize {
	let mut len = 0;
	let mut index = 0;

	while index < values.len() {
		len += values[index].len();
		index += 1;
	}

	len
}

#[must_use]
pub const fn const_concat_if_len(groups: &[(bool, &[&str])]) -> usize {
	let mut len = 0;
	let mut group_index = 0;

	while group_index < groups.len() {
		if groups[group_index].0 {
			let values = groups[group_index].1;
			let mut value_index = 0;

			while value_index < values.len() {
				len += values[value_index].len();
				value_index += 1;
			}
		}

		group_index += 1;
	}

	len
}

const JS_TEMPLATE_PLACEHOLDERS: [&str; 5] = ["$value", "$slot1", "$slot2", "$slot3", "$slot4"];

const fn js_template_placeholder(template: &[u8], index: usize) -> usize {
	if template[index] != b'$' {
		return JS_TEMPLATE_PLACEHOLDERS.len();
	}

	let mut placeholder = 0;

	while placeholder < JS_TEMPLATE_PLACEHOLDERS.len() {
		let candidate = JS_TEMPLATE_PLACEHOLDERS[placeholder].as_bytes();

		if index + candidate.len() <= template.len() {
			let mut byte = 0;
			let mut matches = true;

			while byte < candidate.len() {
				if template[index + byte] != candidate[byte] {
					matches = false;
					break;
				}

				byte += 1;
			}

			if matches {
				return placeholder;
			}
		}

		placeholder += 1;
	}

	JS_TEMPLATE_PLACEHOLDERS.len()
}

#[must_use]
pub const fn js_template_len(template: &str, replacements: &[&str; 5]) -> usize {
	let template = template.as_bytes();
	let mut input = 0;
	let mut output = 0;

	while input < template.len() {
		let placeholder = js_template_placeholder(template, input);

		if placeholder < JS_TEMPLATE_PLACEHOLDERS.len() {
			output += replacements[placeholder].len();
			input += JS_TEMPLATE_PLACEHOLDERS[placeholder].len();
		} else {
			output += 1;
			input += 1;
		}
	}

	output
}

#[must_use]
pub const fn render_js_template<const LEN: usize>(
	template: &str,
	replacements: &[&str; 5],
) -> [u8; LEN] {
	let template = template.as_bytes();
	let mut rendered = [0; LEN];
	let mut input = 0;
	let mut output = 0;

	while input < template.len() {
		let placeholder = js_template_placeholder(template, input);

		if placeholder < JS_TEMPLATE_PLACEHOLDERS.len() {
			let replacement = replacements[placeholder].as_bytes();
			let mut byte = 0;

			while byte < replacement.len() {
				rendered[output] = replacement[byte];
				output += 1;
				byte += 1;
			}

			input += JS_TEMPLATE_PLACEHOLDERS[placeholder].len();
		} else {
			rendered[output] = template[input];
			output += 1;
			input += 1;
		}
	}

	rendered
}
