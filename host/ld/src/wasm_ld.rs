use std::ffi::{OsStr, OsString};

use hashbrown::HashMap;

include!("opt.rs");

#[derive(Clone, Copy)]
enum OptKind {
	/// E.g. `--extra-features=a,b,c`.
	CommaJoined,
	/// E.g. `--gc-sections`.
	Flag,
	/// E.g. `--export=xx`.
	Joined,
	// E.g. `-ofoo.wasm` or `-o foo.wasm`.
	JoinedOrSeparate,
	// E.g. `--export xx`.
	Separate,
}

pub(crate) struct WasmLdArguments<'args> {
	table: HashMap<&'args str, Vec<&'args OsStr>>,
	inputs: Vec<&'args OsString>,
}

impl<'args> WasmLdArguments<'args> {
	// See the LLVM parser implementation:
	// https://github.com/llvm/llvm-project/blob/llvmorg-21.1.8/llvm/lib/Option/OptTable.cpp#L436-L498.
	pub(crate) fn new(args: &[OsString]) -> WasmLdArguments<'_> {
		let mut args = args.iter();
		let mut table = HashMap::new();
		let mut inputs = Vec::new();

		let option_table: HashMap<&str, OptKind> = HashMap::from(OPT_KIND);

		while let Some(arg) = args.next() {
			let bytes = arg.as_encoded_bytes();
			// If a value does not start with `-`, it is treated as `INPUT`.
			let Some(stripped) = bytes
				.strip_prefix(b"--")
				.or_else(|| bytes.strip_prefix(b"-"))
				// SAFETY:
				// - `bytes` originated from `OsStr::as_encoded_bytes`.
				// - We only split by valid UTF-8 strings.
				.map(|bytes| unsafe { OsStr::from_encoded_bytes_unchecked(bytes) })
			else {
				inputs.push(arg);
				continue;
			};

			// Find the `OptKind` and its longest corresponding prefix.
			let Some((kind, prefix, remain)) = (0..=stripped.len()).rev().find_map(|end| {
				let (prefix, remain) = stripped.as_encoded_bytes().split_at(end);
				let prefix = str::from_utf8(prefix).ok()?;
				let kind = option_table.get(prefix)?;
				// SAFETY:
				// - `remain` comes from `stripped`, which originated from
				//   `OsStr::as_encoded_bytes`.
				// - We only proceed when having split off a valid argument from `option_table`,
				//   which are UTF-8 and therefore `remain` is a valid `OsStr`.
				let remain = unsafe { OsStr::from_encoded_bytes_unchecked(remain) };
				Some((kind, prefix, remain))
			}) else {
				eprintln!("encountered unknown `wasm-ld` option: `{}`", arg.display());
				continue;
			};

			let mut next = || {
				args.next()
					.unwrap_or_else(|| panic!("`{}` argument should have a value", arg.display()))
					.as_os_str()
			};
			let value = match kind {
				OptKind::Flag => None,
				OptKind::Separate => Some(next()),
				OptKind::CommaJoined | OptKind::Joined => Some(remain),
				OptKind::JoinedOrSeparate => Some(if remain.is_empty() { next() } else { remain }),
			};

			if let Some(value) = value {
				table.entry(prefix).or_insert_with(Vec::new).push(value);
			} else {
				table.insert(prefix, Vec::new());
			}
		}

		WasmLdArguments { table, inputs }
	}

	pub(crate) fn arg_single(&self, arg: &str) -> Option<&'args OsStr> {
		match self.table.get(arg).map(Vec::as_slice) {
			Some([value]) => Some(value),
			Some([]) => panic!("found unexpected empty argument for `{arg}`"),
			Some(_) => panic!("found unexpected multiple arguments of `{arg}`"),
			None => None,
		}
	}

	pub(crate) fn arg_flag(&self, arg: &str) -> bool {
		match self.table.get(arg) {
			Some(values) if values.is_empty() => true,
			Some(_) => panic!("found unexpected values for argument `{arg}`"),
			None => false,
		}
	}

	pub(crate) fn inputs(&self) -> &[&OsString] {
		&self.inputs
	}
}
