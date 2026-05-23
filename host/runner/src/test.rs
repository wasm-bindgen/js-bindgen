use std::ffi::OsString;

use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use js_bindgen_shared::IS_TEST_SECTION;
use serde::{Serialize, Serializer};
use wasmparser::Payload;

use crate::run_data::RunData;

#[derive(Parser)]
#[command(name = "js-bindgen-runner", version, about, long_about = None)]
pub struct TestCli {
	/// Run ignored and not ignored tests.
	#[arg(long, conflicts_with = "ignored")]
	include_ignored: bool,
	/// Run only ignored tests.
	#[arg(long, conflicts_with = "include_ignored")]
	ignored: bool,
	/// Exactly match filters rather than by substring.
	#[arg(long)]
	exact: bool,
	/// List all tests and benchmarks.
	#[arg(long)]
	list: bool,
	/// don't capture `console.*()` of each task, allow printing directly.
	#[arg(long, alias = "nocapture")]
	no_capture: bool,
	/// Configure formatting of output.
	#[arg(long, value_enum)]
	format: Option<FormatSetting>,
	/// The FILTER string is tested against the name of all tests, and only
	/// those tests whose names contain the filter are run. Multiple filter
	/// strings may be passed, which will run all tests matching any of the
	/// filters.
	filter: Vec<String>,
}

/// Possible values for the `--format` option.
#[derive(Clone, Copy, ValueEnum)]
enum FormatSetting {
	/// Display one character per test
	Terse,
}

struct TestEntries<'a> {
	filtered_count: usize,
	tests: Vec<TestEntry<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestEntry<'a> {
	name: &'a str,
	import_name: &'a str,
	ignore: TestAttr,
	should_panic: TestAttr,
}

enum TestAttr {
	None,
	Present,
	WithText(String),
}

pub enum TestParser<'a> {
	Parsing { is_test: bool },
	Found(Vec<TestEntry<'a>>),
}

impl<'a> TestParser<'a> {
	pub fn new() -> Self {
		Self::Parsing { is_test: false }
	}

	pub fn found(&self) -> bool {
		matches!(self, Self::Found(_))
	}

	pub fn parse(&mut self, payload: &Payload<'a>) -> Result<()> {
		let Self::Parsing { is_test } = self else {
			return Ok(());
		};

		let Payload::CustomSection(section) = payload else {
			return Ok(());
		};

		if section.name() == IS_TEST_SECTION {
			*is_test = true;
			return Ok(());
		}

		if section.name() != "js_bindgen.test" {
			return Ok(());
		}

		let mut tests = Vec::new();
		let mut section = section.data();

		while !section.is_empty() {
			let len = u32::from_le_bytes(
				section
					.split_off(..4)
					.context("invalid test encoding")?
					.try_into()?,
			) as usize;
			let mut section = section.split_off(..len).context("invalid test encoding")?;

			let ignore = TestAttr::parse(&mut section)?;
			let should_panic = TestAttr::parse(&mut section)?;
			let import_name = str::from_utf8(section)?;
			let name = import_name
				.split_once("::")
				.unwrap_or_else(|| panic!("unexpected test name: {import_name}"))
				.1;

			tests.push(TestEntry {
				name,
				import_name,
				ignore,
				should_panic,
			});
		}

		*self = Self::Found(tests);

		Ok(())
	}

	pub fn into_tests(self) -> Option<Vec<TestEntry<'a>>> {
		match self {
			TestParser::Parsing { is_test } => is_test.then_some(Vec::new()),
			TestParser::Found(tests) => Some(tests),
		}
	}
}

impl<'a> TestEntries<'a> {
	fn new(
		mut tests: Vec<TestEntry<'a>>,
		filter: &[String],
		ignored_only: bool,
		exact: bool,
	) -> Self {
		let total = tests.len();

		tests.retain(|entry| {
			let matches_ignore = !ignored_only || entry.ignore.is_some();
			let matches_filter = filter.is_empty()
				|| filter.iter().any(|filter| {
					if exact {
						filter == entry.name
					} else {
						entry.name.contains(filter)
					}
				});

			matches_ignore && matches_filter
		});

		tests.sort_unstable_by(|a, b| a.name.cmp(b.name));
		let filtered_count = total - tests.len();

		Self {
			filtered_count,
			tests,
		}
	}
}

impl TestCli {
	pub fn run(args: impl Iterator<Item = OsString>, tests: Vec<TestEntry>) -> Option<RunData> {
		let cli = Self::parse_from(args);

		let TestEntries {
			filtered_count,
			tests,
		} = TestEntries::new(tests, cli.filter.as_ref(), cli.ignored, cli.exact);

		if cli.list {
			match cli.format {
				Some(FormatSetting::Terse) => {
					for test in &tests {
						println!("{}: test", test.name);
					}
				}
				None => {
					for test in &tests {
						println!("{}: test", test.name);
					}
					println!();
					println!("{} tests, 0 benchmarks", tests.len());
				}
			}
			return None;
		}

		if tests.is_empty() {
			const GREEN: &str = "\u{001b}[32m";
			const RESET: &str = "\u{001b}[0m";

			println!();
			println!("running 0 tests");
			println!();
			println!(
				"test result: {GREEN}ok{RESET}. 0 passed; 0 failed; 0 ignored; 0 measured; \
				 {filtered_count} filtered out; finished in 0.00s"
			);
			println!();
			return None;
		}

		Some(RunData::Test {
			no_capture: cli.no_capture,
			filtered_count,
			tests,
		})
	}
}

impl Serialize for TestAttr {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match self {
			Self::None => serializer.serialize_unit(),
			Self::Present => serializer.serialize_bool(true),
			Self::WithText(text) => serializer.serialize_str(text),
		}
	}
}

impl TestAttr {
	/// - `None`:        `[0]`
	/// - `Present`:     `[1]`
	/// - `WithText(s)`: `[2][len(s)][s]`
	fn parse(data: &mut &[u8]) -> Result<Self> {
		let value = match data
			.split_off_first()
			.context("invalid test flag encoding")?
		{
			0 => Self::None,
			1 => Self::Present,
			2 => {
				let len = u16::from_le_bytes(
					data.split_off(..2)
						.context("invalid test flag length encoding")?
						.try_into()?,
				)
				.into();
				let s = str::from_utf8(
					data.split_off(..len)
						.context("invalid test flag reason encoding")?,
				)?
				.to_string();
				Self::WithText(s)
			}
			_ => bail!("mismatch flag value"),
		};
		Ok(value)
	}

	fn is_some(&self) -> bool {
		match self {
			Self::None => false,
			Self::Present | Self::WithText(_) => true,
		}
	}
}
