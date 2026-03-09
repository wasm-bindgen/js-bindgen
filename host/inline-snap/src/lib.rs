use std::ops::DerefMut;
use std::sync::{LazyLock, Mutex};
use std::{fs, mem};

use dtor::dtor;
use hashbrown::HashMap;
pub use inline_snap_macro::inline_snap;
use itertools::{Itertools, Position};
#[doc(hidden)]
pub use prettyplease;
#[doc(hidden)]
pub use similar_asserts;
#[doc(hidden)]
pub use syn;

#[doc(hidden)]
pub static TEST_UPDATES: LazyLock<TestUpdates> = LazyLock::new(TestUpdates::new);

#[doc(hidden)]
pub struct TestUpdates(Mutex<HashMap<&'static str, Vec<TestUpdate>>>);

struct TestUpdate {
	output: String,
	start_line: usize,
	start_col: usize,
	end_line: usize,
	end_col: usize,
}

impl TestUpdates {
	fn new() -> Self {
		Self(Mutex::new(HashMap::new()))
	}

	pub fn add(
		&self,
		path: &'static str,
		output: String,
		start_line: usize,
		start_col: usize,
		end_line: usize,
		end_col: usize,
	) {
		let update = TestUpdate {
			output,
			start_line,
			start_col,
			end_line,
			end_col,
		};

		self.0.lock().unwrap().entry(path).or_default().push(update);
	}
}

#[dtor]
fn run_test_updates() {
	let updates = mem::take(TEST_UPDATES.0.lock().unwrap().deref_mut());

	for (path, mut updates) in updates {
		updates.sort_by(|a, b| {
			b.start_line
				.cmp(&a.start_line)
				.then(b.start_col.cmp(&a.start_col))
		});

		if path.is_empty() {
			unreachable!("`rust-analyzer` should not execute with `BLESS=1`")
		}

		let mut src = fs::read_to_string(path).unwrap();

		for TestUpdate {
			output,
			start_line,
			start_col,
			end_line,
			end_col,
		} in updates
		{
			let start = line_col_to_byte_offset(&src, start_line, start_col);
			let end = line_col_to_byte_offset(&src, end_line, end_col);

			let output = normalize_output(&output, start_col);
			src.replace_range(start..end, &output);
		}

		fs::write(path, src).unwrap();
	}
}

fn line_col_to_byte_offset(file: &str, line: usize, col: usize) -> usize {
	let mut bytes = 0;

	for (current_line, full_line) in (1..).zip(file.split_inclusive('\n')) {
		let line_str = full_line
			.strip_suffix('\n')
			.ok_or_else(|| full_line.strip_suffix('\r'))
			.unwrap_or(full_line);

		if current_line == line {
			for (current_col, (byte, _)) in line_str.char_indices().enumerate() {
				if current_col == col {
					return bytes + byte;
				}
			}

			return bytes + line_str.len();
		}

		bytes += full_line.len();
	}

	file.len()
}

fn normalize_output(input: &str, level: usize) -> String {
	let tabs: String = "\t".repeat(level);
	let extra = tabs.len() * input.lines().count().saturating_sub(1);
	let mut out = String::with_capacity(input.len() + extra);

	for (position, mut line) in input.split_inclusive('\n').with_position() {
		if let Position::Middle | Position::Last = position
			&& !line.trim_end().is_empty()
		{
			out.push_str(&tabs);
		}

		if let Position::Last = position {
			line = line.trim_end();
		}

		out.push_str(line);
	}

	out
}
