/// # Safety
///
/// This directly interacts with the assembly generator and therefor all
/// bets are off! (TODO)
pub unsafe trait Input {
	const IMPORT_FUNC: &str = "";
	const IMPORT_TYPE: &str;
	const TYPE: &str;
	const CONV: &str = "";
	const JS_CONV_EMBED: (&str, &str) = ("", "");
	const JS_CONV: &str = "";
	const JS_CONV_POST: &str = "";

	type Type;

	fn into_raw(self) -> Self::Type;
}

/// # Safety
///
/// This directly interacts with the assembly generator and therefor all
/// bets are off! (TODO)
pub unsafe trait Output {
	const IMPORT_FUNC: &str = "";
	const IMPORT_TYPE: &str;
	const TYPE: &str;
	const CONV: &str = "";

	type Type;

	fn from_raw(raw: Self::Type) -> Self;
}
