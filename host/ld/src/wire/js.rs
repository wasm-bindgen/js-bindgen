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
