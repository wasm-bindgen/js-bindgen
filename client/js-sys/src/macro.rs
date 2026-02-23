#[must_use]
pub const fn select(a: &'static str, b: &'static str, conversions: &[&str]) -> &'static str {
	let mut any_conversions = false;
	let mut index = 0;

	while index < conversions.len() {
		if !conversions[index].is_empty() {
			any_conversions = true;
			break;
		}

		index += 1;
	}

	if any_conversions { b } else { a }
}
