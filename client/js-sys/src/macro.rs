pub const fn select<const L: usize>(
	a: &'static str,
	b: &'static str,
	conversions: [&'static str; L],
) -> &'static str {
	let mut any_conversions = false;
	let mut index = 0;

	while index < L {
		if !conversions[index].is_empty() {
			any_conversions = true;
			break;
		}

		index += 1;
	}

	if any_conversions { b } else { a }
}
