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

const CUSTOM_ARGS: [(&str, OptKind); 1usize] = [("web", OptKind::Flag)];

pub(crate) struct LdArguments<'args> {
	raw: &'args [OsString],
	custom_args: HashMap<&'args str, Vec<&'args OsStr>>,
	wasm_ld_args: HashMap<&'args str, Vec<&'args OsStr>>,
	custom_indices: Vec<usize>,
	inputs: Vec<&'args OsString>,
}

impl<'args> LdArguments<'args> {
	// See the LLVM parser implementation:
	// https://github.com/llvm/llvm-project/blob/llvmorg-21.1.8/llvm/lib/Option/OptTable.cpp#L436-L498.
	pub(crate) fn new(args: &[OsString]) -> LdArguments<'_> {
		let raw = args;
		let mut args = raw.iter().enumerate();
		let mut custom_args = HashMap::new();
		let mut wasm_ld_args = HashMap::new();
		let mut custom_indices = Vec::new();
		let mut inputs = Vec::new();

		let table: HashMap<&str, OptKind> = HashMap::from_iter(CUSTOM_ARGS);
		let wasm_ld_table: HashMap<&str, OptKind> = HashMap::from_iter(OPT_KIND);

		while let Some((idx, arg)) = args.next() {
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
			let Some((kind, prefix, remain, is_custom)) =
				(0..=stripped.len()).rev().find_map(|end| {
					let (prefix, remain) = stripped.as_encoded_bytes().split_at(end);
					let prefix = str::from_utf8(prefix).ok()?;
					let (custom, kind) = match (table.get(prefix), wasm_ld_table.get(prefix)) {
						(None, None) => return None,
						(None, Some(kind)) => (false, kind),
						(Some(kind), None) => (true, kind),
						(Some(_), Some(kind)) => {
							eprintln!("encountered duplicated defined option: `{}`", arg.display());
							(false, kind)
						}
					};
					// SAFETY:
					// - `remain` comes from `stripped`, which originated from
					//   `OsStr::as_encoded_bytes`.
					// - We only proceed when having split off a valid argument from `option_table`,
					//   which are UTF-8 and therefore `remain` is a valid `OsStr`.
					let remain = unsafe { OsStr::from_encoded_bytes_unchecked(remain) };
					Some((kind, prefix, remain, custom))
				})
			else {
				eprintln!("encountered unknown `wasm-ld` option: `{}`", arg.display());
				continue;
			};

			if is_custom {
				custom_indices.push(idx);
			}

			let mut next = || {
				let (idx, s) = args
					.next()
					.unwrap_or_else(|| panic!("`{}` argument should have a value", arg.display()));
				if is_custom {
					custom_indices.push(idx);
				}
				s.as_os_str()
			};

			let value = match kind {
				OptKind::Flag => None,
				OptKind::Separate => Some(next()),
				OptKind::CommaJoined | OptKind::Joined => Some(remain),
				OptKind::JoinedOrSeparate => Some(if remain.is_empty() { next() } else { remain }),
			};

			let ld_args = if is_custom {
				&mut custom_args
			} else {
				&mut wasm_ld_args
			};

			if let Some(value) = value {
				ld_args.entry(prefix).or_insert_with(Vec::new).push(value);
			} else {
				ld_args.insert(prefix, Vec::new());
			}
		}

		LdArguments {
			raw,
			custom_args,
			wasm_ld_args,
			custom_indices,
			inputs,
		}
	}

	pub(crate) fn arg_single(&self, arg: &str) -> Option<&'args OsStr> {
		match self
			.wasm_ld_args
			.get(arg)
			.or_else(|| self.custom_args.get(arg))
			.map(Vec::as_slice)
		{
			Some([value]) => Some(value),
			Some([]) => panic!("found unexpected empty argument for `{arg}`"),
			Some(_) => panic!("found unexpected multiple arguments of `{arg}`"),
			None => None,
		}
	}

	pub(crate) fn arg_flag(&self, arg: &str) -> bool {
		match self
			.wasm_ld_args
			.get(arg)
			.or_else(|| self.custom_args.get(arg))
		{
			Some(values) if values.is_empty() => true,
			Some(_) => panic!("found unexpected values for argument `{arg}`"),
			None => false,
		}
	}

	pub(crate) fn inputs(&self) -> &[&OsString] {
		&self.inputs
	}

	pub(crate) fn web(&self) -> bool {
		self.arg_flag("web")
	}

	pub(crate) fn raw_wasm_ld_args(&self) -> impl Iterator<Item = &OsString> {
		let mut custom_indices = self.custom_indices.iter().copied().peekable();

		self.raw.iter().enumerate().filter_map(move |(idx, arg)| {
			if custom_indices.next_if_eq(&idx).is_some() {
				None
			} else {
				Some(arg)
			}
		})
	}
}

#[cfg(test)]
mod tests {
	use std::ffi::OsString;

	use crate::ld_args::LdArguments;

	#[test]
	fn test_custom() {
		let args = &["--web".into(), "--no-entry".into()];
		let args = LdArguments::new(args);
		let mut iter = args.raw_wasm_ld_args();
		assert!(args.web());
		assert_eq!(iter.next(), Some(&OsString::from("--no-entry")));
		assert!(iter.next().is_none());
	}
}
