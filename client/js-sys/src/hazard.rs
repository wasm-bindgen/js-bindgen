/// # Safety
///
/// This directly interacts with the assembly generator and therefor all
/// bets are off! (TODO)
pub unsafe trait Input {
	const ASM_IMPORT_FUNC: Option<&str> = None;
	const ASM_IMPORT_TYPE: &str;
	const ASM_TYPE: &str;
	const ASM_CONV: Option<&str> = None;
	const JS_EMBED: Option<(&str, &str)> = None;
	const JS_CONV: Option<(&str, Option<&str>)> = None;

	type Type;

	fn into_raw(self) -> Self::Type;
}

/// # Safety
///
/// This directly interacts with the assembly generator and therefor all
/// bets are off! (TODO)
pub unsafe trait Output {
	const ASM_IMPORT_FUNC: Option<&str> = None;
	const ASM_IMPORT_TYPE: &str;
	const ASM_TYPE: &str;
	const ASM_CONV: Option<&str> = None;
	const JS_EMBED: Option<(&str, &str)> = None;
	const JS_CONV: Option<(&str, &str)> = None;

	type Type;

	fn from_raw(raw: Self::Type) -> Self;
}
