//! Shared JavaScript template rendering support.

/// One placeholder recognized in a JavaScript conversion template.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Placeholder {
	Value,
	Prepared,
	Slot(usize),
}

const PLACEHOLDERS: [(&str, Placeholder); 6] = [
	("$value", Placeholder::Value),
	("$prepared", Placeholder::Prepared),
	("$slot1", Placeholder::Slot(0)),
	("$slot2", Placeholder::Slot(1)),
	("$slot3", Placeholder::Slot(2)),
	("$slot4", Placeholder::Slot(3)),
];

/// Quotes one JavaScript string literal.
///
/// JavaScript export names are valid strings rather than identifiers, so they
/// must not be interpolated directly into generated source.
pub(super) fn quote_string(value: &str) -> String {
	let mut output = String::with_capacity(value.len() + 2);
	output.push('"');
	for character in value.chars() {
		match character {
			'"' => output.push_str("\\\""),
			'\\' => output.push_str("\\\\"),
			'\u{0008}' => output.push_str("\\b"),
			'\u{000c}' => output.push_str("\\f"),
			'\n' => output.push_str("\\n"),
			'\r' => output.push_str("\\r"),
			'\t' => output.push_str("\\t"),
			'\u{2028}' => output.push_str("\\u2028"),
			'\u{2029}' => output.push_str("\\u2029"),
			character if character <= '\u{001f}' => {
				use std::fmt::Write;
				write!(output, "\\u{:04x}", u32::from(character))
					.expect("writing to a String cannot fail");
			}
			character => output.push(character),
		}
	}
	output.push('"');
	output
}

/// Renders a conversion template using the supplied placeholder resolver.
///
/// Unrecognized `$` sequences are copied without interpretation.
pub(super) fn render_template(
	template: &str,
	mut resolve: impl FnMut(&mut String, Placeholder),
) -> String {
	let mut output = String::new();
	let bytes = template.as_bytes();
	let mut input = 0;

	while input < bytes.len() {
		if bytes[input] == b'$'
			&& let Some((name, placeholder)) = PLACEHOLDERS
				.iter()
				.find(|(name, _)| bytes[input..].starts_with(name.as_bytes()))
		{
			resolve(&mut output, *placeholder);
			input += name.len();
			continue;
		}

		let start = input;
		input += 1;
		while input < bytes.len() && bytes[input] != b'$' {
			input += 1;
		}
		output.push_str(&template[start..input]);
	}

	output
}

#[cfg(test)]
mod tests {
	use std::fmt::Write;

	use super::{Placeholder, quote_string, render_template};

	#[test]
	fn quotes_javascript_strings() {
		assert_eq!(
			quote_string("single' double\" slash\\ line\n雪\u{2028}"),
			"\"single' double\\\" slash\\\\ line\\n雪\\u2028\"",
		);
	}

	#[test]
	fn renders_placeholders() {
		let cases = [
			("$value", "<value>"),
			("$prepared", "<prepared>"),
			(
				"$slot1, $slot2, $slot3, $slot4",
				"<slot0>, <slot1>, <slot2>, <slot3>",
			),
			("雪 $unknown $$value", "雪 $unknown $<value>"),
		];

		for (template, expected) in cases {
			let rendered = render_template(template, |output, placeholder| match placeholder {
				Placeholder::Value => output.push_str("<value>"),
				Placeholder::Prepared => output.push_str("<prepared>"),
				Placeholder::Slot(index) => {
					write!(output, "<slot{index}>").expect("writing to a String cannot fail");
				}
			});
			assert_eq!(rendered, expected, "template: {template}");
		}
	}
}
