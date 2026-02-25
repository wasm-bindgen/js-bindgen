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
