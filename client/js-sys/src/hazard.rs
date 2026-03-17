/// # Safety
///
/// This directly interacts with the assembly generator and therefor all
/// bets are off! (TODO)
pub unsafe trait Input {
	const ASM_TYPE: &str;
	const ASM_CONV: Option<InputAsmConv> = None;
	const JS_CONV: Option<InputJsConv> = None;

	type Type;

	fn into_raw(self) -> Self::Type;
}

pub struct InputAsmConv {
	pub import: Option<&'static str>,
	pub conv: &'static str,
	pub r#type: &'static str,
}

pub struct InputJsConv {
	pub embed: Option<(&'static str, &'static str)>,
	pub pre: &'static str,
	pub post: Option<&'static str>,
}

/// # Safety
///
/// This directly interacts with the assembly generator and therefor all
/// bets are off! (TODO)
pub unsafe trait Output {
	const ASM_TYPE: &str;
	const ASM_CONV: Option<OutputAsmConv> = None;
	const JS_CONV: Option<OutputJsConv> = None;

	type Type;

	fn from_raw(raw: Self::Type) -> Self;
}

pub struct OutputAsmConv {
	pub import: Option<&'static str>,
	pub direct: bool,
	pub conv: &'static str,
	pub r#type: &'static str,
}

pub struct OutputJsConv {
	pub embed: Option<(&'static str, &'static str)>,
	pub pre: &'static str,
	pub post: &'static str,
}
