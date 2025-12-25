use std::ffi::{OsStr, OsString};

use hashbrown::HashMap;

/// `wasm-ld`'s options and kinds.
///
/// How to generate:
///
/// ```sh
/// llvm-tblgen --dump-json lld/wasm/Options.td -o options.json -I llvm/include
/// ```
///
/// ```ignore
/// let opt_table: BTreeMap<String, Value> =
///     serde_json::from_slice(&std::fs::read("options.json").unwrap()).unwrap();
/// let mut vec = Vec::new();
/// for option in opt_table.values() {
///     if let Some(name) = option.get("Name").and_then(|n| n.as_str())
///         && let Some(kind) = option
///             .get("Kind")
///             .and_then(|def| def.get("def"))
///             .and_then(|s| s.as_str())
///     {
///         if kind == "KIND_INPUT" || kind == "KIND_UNKNOWN" {
///             continue;
///         }
///         vec.push((name, format!("OptKind::{kind}")));
///     }
/// }
///
/// println!("{vec:#?}");
/// ```
///
/// And copy to here.
static OPT_KIND: [(&str, OptKind); 149] = [
	("Bdynamic", OptKind::KIND_FLAG),
	("Bstatic", OptKind::KIND_FLAG),
	("Bsymbolic", OptKind::KIND_FLAG),
	("Map", OptKind::KIND_SEPARATE),
	("Map=", OptKind::KIND_JOINED),
	("O", OptKind::KIND_JOINED_OR_SEPARATE),
	("allow-multiple-definition", OptKind::KIND_FLAG),
	("allow-undefined", OptKind::KIND_FLAG),
	("allow-undefined-file=", OptKind::KIND_JOINED),
	("allow-undefined-file", OptKind::KIND_SEPARATE),
	("e", OptKind::KIND_JOINED_OR_SEPARATE),
	("entry=", OptKind::KIND_JOINED),
	("library=", OptKind::KIND_JOINED),
	("library-path", OptKind::KIND_SEPARATE),
	("library-path=", OptKind::KIND_JOINED),
	("M", OptKind::KIND_FLAG),
	("r", OptKind::KIND_FLAG),
	("s", OptKind::KIND_FLAG),
	("S", OptKind::KIND_FLAG),
	("t", OptKind::KIND_FLAG),
	("y", OptKind::KIND_JOINED_OR_SEPARATE),
	("u", OptKind::KIND_JOINED_OR_SEPARATE),
	("call_shared", OptKind::KIND_FLAG),
	("V", OptKind::KIND_FLAG),
	("dy", OptKind::KIND_FLAG),
	("dn", OptKind::KIND_FLAG),
	("non_shared", OptKind::KIND_FLAG),
	("static", OptKind::KIND_FLAG),
	("E", OptKind::KIND_FLAG),
	("i", OptKind::KIND_FLAG),
	("library", OptKind::KIND_SEPARATE),
	("build-id", OptKind::KIND_FLAG),
	("build-id=", OptKind::KIND_JOINED),
	("check-features", OptKind::KIND_FLAG),
	("color-diagnostics", OptKind::KIND_FLAG),
	("color-diagnostics=", OptKind::KIND_JOINED),
	("compress-relocations", OptKind::KIND_FLAG),
	("demangle", OptKind::KIND_FLAG),
	("disable-verify", OptKind::KIND_FLAG),
	("emit-relocs", OptKind::KIND_FLAG),
	("end-lib", OptKind::KIND_FLAG),
	("entry", OptKind::KIND_SEPARATE),
	("error-limit", OptKind::KIND_SEPARATE),
	("error-limit=", OptKind::KIND_JOINED),
	("error-unresolved-symbols", OptKind::KIND_FLAG),
	("experimental-pic", OptKind::KIND_FLAG),
	("export", OptKind::KIND_SEPARATE),
	("export-all", OptKind::KIND_FLAG),
	("export-dynamic", OptKind::KIND_FLAG),
	("export=", OptKind::KIND_JOINED),
	("export-if-defined", OptKind::KIND_SEPARATE),
	("export-if-defined=", OptKind::KIND_JOINED),
	("export-memory", OptKind::KIND_FLAG),
	("export-memory=", OptKind::KIND_JOINED),
	("export-table", OptKind::KIND_FLAG),
	("extra-features=", OptKind::KIND_COMMAJOINED),
	("fatal-warnings", OptKind::KIND_FLAG),
	("features=", OptKind::KIND_COMMAJOINED),
	("gc-sections", OptKind::KIND_FLAG),
	("global-base=", OptKind::KIND_JOINED),
	("growable-table", OptKind::KIND_FLAG),
	("help", OptKind::KIND_FLAG),
	("import-memory", OptKind::KIND_FLAG),
	("import-memory=", OptKind::KIND_JOINED),
	("import-table", OptKind::KIND_FLAG),
	("import-undefined", OptKind::KIND_FLAG),
	("initial-heap=", OptKind::KIND_JOINED),
	("initial-memory=", OptKind::KIND_JOINED),
	("keep-section", OptKind::KIND_SEPARATE),
	("keep-section=", OptKind::KIND_JOINED),
	("l", OptKind::KIND_JOINED_OR_SEPARATE),
	("L", OptKind::KIND_JOINED_OR_SEPARATE),
	("lto-CGO", OptKind::KIND_JOINED),
	("lto-O", OptKind::KIND_JOINED),
	("lto-debug-pass-manager", OptKind::KIND_FLAG),
	("lto-obj-path=", OptKind::KIND_JOINED),
	("lto-partitions=", OptKind::KIND_JOINED),
	("m", OptKind::KIND_JOINED_OR_SEPARATE),
	("max-memory=", OptKind::KIND_JOINED),
	("merge-data-segments", OptKind::KIND_FLAG),
	("mllvm", OptKind::KIND_SEPARATE),
	("mllvm=", OptKind::KIND_JOINED),
	("no-allow-multiple-definition", OptKind::KIND_FLAG),
	("no-check-features", OptKind::KIND_FLAG),
	("no-color-diagnostics", OptKind::KIND_FLAG),
	("no-demangle", OptKind::KIND_FLAG),
	("no-entry", OptKind::KIND_FLAG),
	("no-export-dynamic", OptKind::KIND_FLAG),
	("no-fatal-warnings", OptKind::KIND_FLAG),
	("no-gc-sections", OptKind::KIND_FLAG),
	("no-growable-memory", OptKind::KIND_FLAG),
	("no-merge-data-segments", OptKind::KIND_FLAG),
	("no-pie", OptKind::KIND_FLAG),
	("no-print-gc-sections", OptKind::KIND_FLAG),
	("no-shlib-sigcheck", OptKind::KIND_FLAG),
	("no-stack-first", OptKind::KIND_FLAG),
	("no-whole-archive", OptKind::KIND_FLAG),
	("noinhibit-exec", OptKind::KIND_FLAG),
	("o", OptKind::KIND_JOINED_OR_SEPARATE),
	("page-size=", OptKind::KIND_JOINED),
	("pie", OptKind::KIND_FLAG),
	("print-gc-sections", OptKind::KIND_FLAG),
	("print-map", OptKind::KIND_FLAG),
	("relocatable", OptKind::KIND_FLAG),
	("reproduce", OptKind::KIND_SEPARATE),
	("reproduce=", OptKind::KIND_JOINED),
	("rpath", OptKind::KIND_SEPARATE),
	("rpath=", OptKind::KIND_JOINED),
	("rsp-quoting", OptKind::KIND_SEPARATE),
	("rsp-quoting=", OptKind::KIND_JOINED),
	("save-temps", OptKind::KIND_FLAG),
	("shared", OptKind::KIND_FLAG),
	("shared-memory", OptKind::KIND_FLAG),
	("soname", OptKind::KIND_SEPARATE),
	("soname=", OptKind::KIND_JOINED),
	("stack-first", OptKind::KIND_FLAG),
	("start-lib", OptKind::KIND_FLAG),
	("strip-all", OptKind::KIND_FLAG),
	("strip-debug", OptKind::KIND_FLAG),
	("table-base=", OptKind::KIND_JOINED),
	("thinlto-cache-dir=", OptKind::KIND_JOINED),
	("thinlto-cache-policy", OptKind::KIND_SEPARATE),
	("thinlto-cache-policy=", OptKind::KIND_JOINED),
	("thinlto-emit-imports-files", OptKind::KIND_FLAG),
	("thinlto-emit-index-files", OptKind::KIND_FLAG),
	("thinlto-index-only", OptKind::KIND_FLAG),
	("thinlto-index-only=", OptKind::KIND_JOINED),
	("thinlto-jobs=", OptKind::KIND_JOINED),
	("thinlto-object-suffix-replace=", OptKind::KIND_JOINED),
	("thinlto-prefix-replace=", OptKind::KIND_JOINED),
	("threads", OptKind::KIND_SEPARATE),
	("threads=", OptKind::KIND_JOINED),
	("trace", OptKind::KIND_FLAG),
	("trace-symbol", OptKind::KIND_SEPARATE),
	("trace-symbol=", OptKind::KIND_JOINED),
	("undefined", OptKind::KIND_SEPARATE),
	("undefined=", OptKind::KIND_JOINED),
	("unresolved-symbols", OptKind::KIND_SEPARATE),
	("unresolved-symbols=", OptKind::KIND_JOINED),
	("v", OptKind::KIND_FLAG),
	("verbose", OptKind::KIND_FLAG),
	("version", OptKind::KIND_FLAG),
	("warn-unresolved-symbols", OptKind::KIND_FLAG),
	("whole-archive", OptKind::KIND_FLAG),
	("why-extract=", OptKind::KIND_JOINED),
	("wrap", OptKind::KIND_SEPARATE),
	("wrap=", OptKind::KIND_JOINED),
	("z", OptKind::KIND_JOINED_OR_SEPARATE),
	// rust-lld
	("flavor", OptKind::KIND_SEPARATE),
];

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
	pub(crate) table: HashMap<&'a str, Vec<&'a OsStr>>,
	pub(crate) inputs: Vec<&'a OsString>,
}

impl WasmLdArguments<'_> {
	// Referencing part of the LLVM parser implementation:
	//
	// <https://github.com/llvm/llvm-project/blob/991455e69e93c0ce88e927eddd28a9ab34d1f8b2/llvm/lib/Option/OptTable.cpp#L438>
	pub(crate) fn new(args: &[OsString]) -> WasmLdArguments<'_> {
		let mut args = args.iter();
		let mut table = HashMap::new();
		let mut inputs = Vec::new();

		let option_table: HashMap<&str, OptKind> = HashMap::from(OPT_KIND);

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
					let prefix = str::from_utf8(prefix).ok()?;
					let kind = option_table.get(prefix)?;
					let remain = unsafe {
						// SAFETY:
						// - Each `word` only contains content that originated from `OsString::as_encoded_bytes`
						// - `prefix is a valid UTF-8 string
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
				table.entry(prefix).or_insert(Vec::new()).push(value);
			} else {
				table.insert(prefix, Vec::new());
			}
		}

		WasmLdArguments { table, inputs }
	}
}
