use std::hash::{Hash, Hasher};
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
use xxhash_rust::xxh3;

#[doc(hidden)]
pub static TEST_UPDATES: LazyLock<TestUpdates> = LazyLock::new(TestUpdates::new);

#[doc(hidden)]
pub struct TestUpdates(Mutex<HashMap<TestFile, Vec<TestUpdate>>>);

struct TestFile {
	path: &'static str,
	size: usize,
	hash: u64,
}

struct TestUpdate {
	r#type: Type,
	output: String,
	start_line: usize,
	start_col: usize,
	end_line: usize,
	end_col: usize,
}

#[derive(Clone, Copy)]
enum Type {
	Tokens,
	String,
}

impl TestUpdates {
	fn new() -> Self {
		Self(Mutex::new(HashMap::new()))
	}

	pub fn add_tokens(
		&self,
		path: &'static str,
		size: usize,
		hash: u64,
		output: String,
		(start_line, start_col): (usize, usize),
		(end_line, end_col): (usize, usize),
	) {
		let file = TestFile { path, size, hash };
		let update = TestUpdate {
			r#type: Type::Tokens,
			output,
			start_line,
			start_col,
			end_line,
			end_col,
		};

		self.0.lock().unwrap().entry(file).or_default().push(update);
	}

	pub fn add_string(
		&self,
		path: &'static str,
		size: usize,
		hash: u64,
		output: String,
		(start_line, start_col): (usize, usize),
		(end_line, end_col): (usize, usize),
	) {
		let file = TestFile { path, size, hash };
		let update = TestUpdate {
			r#type: Type::String,
			output,
			start_line,
			start_col,
			end_line,
			end_col,
		};

		self.0.lock().unwrap().entry(file).or_default().push(update);
	}
}

#[dtor(unsafe)]
fn run_test_updates() {
	let updates = mem::take(TEST_UPDATES.0.lock().unwrap().deref_mut());

	for (file, mut updates) in updates {
		updates.sort_by(|a, b| {
			b.start_line
				.cmp(&a.start_line)
				.then(b.start_col.cmp(&a.start_col))
		});

		if file.path.is_empty() {
			unreachable!("`rust-analyzer` should not execute with `BLESS=1`")
		}

		let mut src = fs::read_to_string(file.path).unwrap();

		assert!(
			src.len() == file.size && xxh3::xxh3_64(src.as_bytes()) == file.hash,
			"File modified before being able to update. This can happen when running tests from \
			 the same file in multiple processes, e.g. Nextest."
		);

		for TestUpdate {
			r#type,
			output,
			start_line,
			start_col,
			end_line,
			end_col,
		} in updates
		{
			let start = line_col_to_byte_offset(&src, start_line, start_col);
			let end = line_col_to_byte_offset(&src, end_line, end_col);

			let output = normalize_output(r#type, &output, start_col);
			src.replace_range(start..end, &output);
		}

		fs::write(file.path, src).unwrap();
	}
}

impl Eq for TestFile {}

impl Hash for TestFile {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.path.hash(state);
	}
}

impl PartialEq for TestFile {
	fn eq(&self, other: &Self) -> bool {
		self.path == other.path
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

fn normalize_output(r#type: Type, input: &str, level: usize) -> String {
	let tabs: String = "\t".repeat(level);
	let mut extra = tabs.len() * input.lines().count().saturating_sub(1);

	if let Type::String = r#type {
		extra += 2;
	}

	let mut out = String::with_capacity(input.len() + extra);

	if let Type::String = r#type {
		out.push('"');
	}

	for (position, mut line) in input.split_inclusive('\n').with_position() {
		if let Position::Middle | Position::Last = position
			&& !line.trim_end().is_empty()
		{
			out.push_str(&tabs);
		}

		if let Position::Last = position {
			line = line.trim_end();
		}

		match r#type {
			Type::Tokens => out.push_str(line),
			Type::String => {
				for c in line.chars() {
					match c {
						'"' => out.push_str("\\\""),
						'\\' => out.push_str("\\\\"),
						c => out.push(c),
					}
				}
			}
		}
	}

	if let Type::String = r#type {
		out.push('"');
	}

	out
}

#[doc(hidden)]
#[must_use]
pub fn normalize_wat_input(wat: &str) -> String {
	let mut out = String::with_capacity(wat.len());

	for line in wat.split_inclusive('\n') {
		out.push_str(line.trim_start_matches('\t'));
	}

	out
}
