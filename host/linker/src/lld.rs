use std::ffi::{OsStr, OsString};

use hashbrown::HashMap;

include!("lld-opt.rs");

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
enum OptKind {
	// --extra-features=a,b,c
	KIND_COMMAJOINED,
	// --gc-sections
	KIND_FLAG,
	// --export=xx
	KIND_JOINED,
	// -ofoo.wasm OR -o foo.wasm
	KIND_JOINED_OR_SEPARATE,
	// --export xx
	KIND_SEPARATE,
}

pub(crate) struct WasmLdArguments<'a> {
	pub(crate) table: HashMap<&'a [u8], Vec<&'a OsStr>>,
	pub(crate) inputs: Vec<&'a OsString>,
}

impl WasmLdArguments<'_> {
	// See the LLVM parser implementation:
	// https://github.com/llvm/llvm-project/blob/991455e69e93c0ce88e927eddd28a9ab34d1f8b2/llvm/lib/Option/OptTable.cpp#L438
	pub(crate) fn new(args: &[OsString]) -> WasmLdArguments<'_> {
		let mut args = args.iter();
		let mut table = HashMap::new();
		let mut inputs = Vec::new();

		let option_table: HashMap<&[u8], OptKind> = HashMap::from(OPT_KIND);

		while let Some(arg) = args.next() {
			let bytes = arg.as_encoded_bytes();
			// If a value does not start with `-`, it is treated as `INPUT``.
			let Some(stripped) = bytes
				.strip_prefix(b"--")
				.or_else(|| bytes.strip_prefix(b"-"))
			else {
				inputs.push(arg);
				continue;
			};

			// Find the `OptKind` and its longest corresponding prefix.
			let Some((kind, prefix, remain)) = (0..=stripped.len())
				.rev()
				.filter_map(|end| {
					let (prefix, remain) = stripped.split_at(end);
					let kind = option_table.get(prefix)?;
					let remain = unsafe {
						// SAFETY:
						// - Each `word` only contains content that originated from
						//   `OsString::as_encoded_bytes`.
						// - `prefix` is a valid UTF-8 string.
						OsStr::from_encoded_bytes_unchecked(remain)
					};
					Some((kind, prefix, remain))
				})
				.next()
			else {
				eprintln!("encountered unknown LLD option: {arg:?}");
				continue;
			};

			let mut next = || {
				args.next()
					.unwrap_or_else(|| panic!("`{arg:?}` argument should have a value"))
					.as_os_str()
			};
			let value = match kind {
				OptKind::KIND_FLAG => None,
				OptKind::KIND_SEPARATE => Some(next()),
				OptKind::KIND_COMMAJOINED | OptKind::KIND_JOINED => Some(remain),
				OptKind::KIND_JOINED_OR_SEPARATE => {
					Some(if remain.is_empty() { next() } else { remain })
				}
			};

			if let Some(value) = value {
				table.entry(prefix).or_insert_with(Vec::new).push(value);
			} else {
				table.insert(prefix, Vec::new());
			}
		}

		WasmLdArguments { table, inputs }
	}
}
