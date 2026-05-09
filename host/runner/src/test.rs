use anyhow::{Context, Result, bail};
use js_bindgen_shared::IS_TEST_SECTION;
use serde::{Serialize, Serializer};
use wasmparser::{Parser, Payload};

pub struct TestData {
	pub is_test: bool,
	pub filtered_count: usize,
	pub tests: Vec<TestEntry>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestEntry {
	pub name: String,
	import_name: String,
	ignore: TestAttr,
	should_panic: TestAttr,
}

enum TestAttr {
	None,
	Present,
	WithText(String),
}

impl TestEntry {
	pub fn read(
		wasm_bytes: &[u8],
		filter: &[String],
		ignored_only: bool,
		exact: bool,
	) -> Result<TestData> {
		let mut is_test = false;
		let mut tests = Vec::new();
		let mut total = 0;

		for payload in Parser::new(0).parse_all(wasm_bytes) {
			let Payload::CustomSection(section) = payload? else {
				continue;
			};

			if section.name() == IS_TEST_SECTION {
				is_test = true;
				continue;
			}

			if section.name() != "js_bindgen.test" {
				continue;
			}

			is_test = true;
			let mut data = section.data();

			while !data.is_empty() {
				let len = u32::from_le_bytes(
					data.split_off(..4)
						.context("invalid test encoding")?
						.try_into()?,
				) as usize;
				let mut data = data.split_off(..len).context("invalid test encoding")?;

				let ignore = TestAttr::parse(&mut data)?;
				let should_panic = TestAttr::parse(&mut data)?;
				let import_name = str::from_utf8(data)?;
				let name = import_name
					.split_once("::")
					.unwrap_or_else(|| panic!("unexpected test name: {import_name}"))
					.1;

				total += 1;

				let matches_ignore = !ignored_only || ignore.is_some();
				let matches_filter = filter.is_empty()
					|| filter.iter().any(|filter| {
						if exact {
							filter == name
						} else {
							name.contains(filter)
						}
					});

				if matches_ignore && matches_filter {
					tests.push(Self {
						name: name.to_string(),
						import_name: import_name.to_string(),
						ignore,
						should_panic,
					});
				}
			}
		}

		tests.sort_unstable_by(|a, b| a.name.cmp(&b.name));
		let filtered_count = total - tests.len();

		Ok(TestData {
			is_test,
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
