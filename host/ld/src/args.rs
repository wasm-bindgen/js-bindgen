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
const LINKER_GUARD_LIBRARY: &str = "js-bindgen-needs-js-bindgen-ld";

pub(crate) struct Arguments<'args> {
	custom_args: HashMap<&'args str, Vec<&'args OsStr>>,
	wasm_ld_args: HashMap<&'args str, Vec<&'args OsStr>>,
	pass_args: Vec<&'args OsString>,
	inputs: Vec<&'args OsString>,
}

impl<'args> Arguments<'args> {
	// See the LLVM parser implementation:
	// https://github.com/llvm/llvm-project/blob/llvmorg-21.1.8/llvm/lib/Option/OptTable.cpp#L436-L498.
	pub(crate) fn new(args: &[OsString]) -> Arguments<'_> {
		let mut args = args.iter();
		let mut custom_args = HashMap::new();
		let mut wasm_ld_args = HashMap::new();
		let mut pass_args = Vec::with_capacity(args.len());
		let mut inputs = Vec::new();

		let table: HashMap<&str, OptKind> = HashMap::from_iter(CUSTOM_ARGS);
		let wasm_ld_table: HashMap<&str, OptKind> = HashMap::from_iter(OPT_KIND);

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
				pass_args.push(arg);
				continue;
			};

			// Find the `OptKind` and its longest corresponding prefix.
			let Some((kind, prefix, remain, is_wasm_ld)) =
				(0..=stripped.len()).rev().find_map(|end| {
					let (prefix, remain) = stripped.as_encoded_bytes().split_at(end);
					let prefix = str::from_utf8(prefix).ok()?;
					let (wasm_ld, kind) = match (wasm_ld_table.get(prefix), table.get(prefix)) {
						(None, None) => return None,
						(Some(kind), None) => (true, kind),
						(None, Some(kind)) => (false, kind),
						(Some(_), Some(_)) => {
							panic!("encountered argument collision: `{}`", arg.display());
						}
					};
					// SAFETY:
					// - `remain` comes from `stripped`, which originated from
					//   `OsStr::as_encoded_bytes`.
					// - We only proceed when having split off a valid argument from `option_table`,
					//   which are UTF-8 and therefore `remain` is a valid `OsStr`.
					let remain = unsafe { OsStr::from_encoded_bytes_unchecked(remain) };
					Some((kind, prefix, remain, wasm_ld))
				})
			else {
				eprintln!("encountered unknown `wasm-ld` option: `{}`", arg.display());
				continue;
			};

			let mut arg_value = None;
			let mut next = || {
				let s = args
					.next()
					.unwrap_or_else(|| panic!("`{}` argument should have a value", arg.display()));
				arg_value = Some(s);
				s.as_os_str()
			};

			let value = match kind {
				OptKind::Flag => None,
				OptKind::Separate => Some(next()),
				OptKind::CommaJoined | OptKind::Joined => Some(remain),
				OptKind::JoinedOrSeparate => Some(if remain.is_empty() { next() } else { remain }),
			};

			let is_linker_guard = is_wasm_ld
				&& matches!(prefix, "l" | "library" | "library=")
				&& value.is_some_and(|value| value == LINKER_GUARD_LIBRARY);

			if is_linker_guard {
				continue;
			}

			let ld_args = if is_wasm_ld {
				pass_args.push(arg);
				if let Some(value) = arg_value {
					pass_args.push(value);
				}
				&mut wasm_ld_args
			} else {
				&mut custom_args
			};

			if let Some(value) = value {
				ld_args.entry(prefix).or_insert_with(Vec::new).push(value);
			} else {
				ld_args.insert(prefix, Vec::new());
			}
		}

		Arguments {
			custom_args,
			wasm_ld_args,
			pass_args,
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

	pub(crate) fn pass_args(&self) -> &[&OsString] {
		&self.pass_args
	}
}

#[cfg(test)]
mod tests {
	use std::ffi::OsString;

	use crate::args::Arguments;

	#[test]
	fn test_custom() {
		let args = &["--web".into(), "--no-entry".into()];
		let args = Arguments::new(args);
		assert!(args.web());

		let mut iter = args.pass_args().iter();
		assert_eq!(iter.next().copied(), Some(&OsString::from("--no-entry")));
		assert!(iter.next().is_none());
	}
}
