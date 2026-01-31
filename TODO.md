# Critical Priority

- Replace `llvm-mc` with `wasm-tools`. See [bytecodealliance/wasm-tools#2405].
- Compatibility with `wasm-bindgen`.
- Do we agree with the new name `js-bindgen`? Maybe `web-bindgen` is the most accurate?
- `#[js_sys]` is probably also not the ideal name.
- The linker shim throws a warning on Windows because of the shebang. This is a problem because we
  want to e.g. deny linker warnings. Most likely solution is [Cosmopolitan].

[bytecodealliance/wasm-tools#2405]: https://github.com/bytecodealliance/wasm-tools/issues/2405
[Cosmopolitan]: https://justine.lol/cosmopolitan/index.html

# High Priority

- Test Runner:
  - Add various permutations to the CI.
    - Run all possible runners on each OS.
    - Run all worker types.
    - Wasm64.
    - Atomic.
    - Panic strategies.
  - Run tests in alphabetical order to mimic `libtest`.
  - (Browser) Test output should happen between tests and not after all are finished. This might not
    be possible to implement without something more sophisticated like WebSockets.
  - Convert JS files to TS, then compile and lint them. Compiling and linting could be built into
    the runner shim to ensure freshness. Don't forget to exclude the TS files from packaging.
  - Add support for `--test-threads` and multithread tests where possible for `panic = "abort"`.
  - Add support for `panic = "unwind"`.
  - Add support for `--force-run-in-process`.
  - Add support for `--quiet`, `--show-output`, `--color` and `--format pretty`.
  - Connect to existing Webdriver. Important to make Nextest actually be performant.
  - Headless test output should happen between tests and not after all are finished.
  - E2E testing including output comparison with regular `cargo test`.
  - Design a way to let users set the default driver, overwrite and whitelist drivers globally or
    for specific tests.
  - Add multithread support when running with `target_feature = "atomics"` where possible.
- E2E testing for the linker. Should also ensure deterministic output.
- Add a `disable_hygiene` attribute to `#[js_sys]` to reduce the compile-time of the output to an
  absolute minimum. This can avoid all `interpolate`s.
- Escape namespaces and function names if they are not valid JS identifiers.
- Test raw Rust identifiers in function and parameter names.
- `#[js_sys]` parameter and function name aren't allowed to collide. Internal
  `extern { fn <name> ... }` definition can shadow parameter values.
- Allocate slots on the `externref` table in batches.
- Determine what to do with `js_sys::UnwrapThrowExt`. Avoiding the panic machinery is nice for some
  very niche use-cases but it might be very annoying for most users. Maybe hide it behind a `cfg`
  flag?
- Validate and improve performance of `JsString` encoding/decoding. See [Emscripten's] or
  [`wasm-bindgen`'s] implementation for inspiration.
- Experiment if allocation is better for build times then iterator chaining in proc-macros.
- Find a way to prevent users from accidentally using the default linker.
  - Supply an invalid object file that would be removed by our custom linker.
  - Check `RUSTC_LINKER` in a `build.rs`. Which would require a separate library crate for
    `js-bindgen`.
- Version all names to make packages compatible with other versions of itself.
- Embed crate version to make linker capable of detecting unsupported versions.
- We still need to look into what happens with old files. We might need to delete them ourselves.
- Evaluate the output folder of our ASM objet files. Some ideas:
  - Store them next to the output file.
  - Pass an environment variable from a `build.rs` pointing to the target folder and go from there.
    This seems to have failed. No build script instruction can reach the linker on Wasm.
- Memory-mapped file reading should lock files to make it safe.

[Fantoccini]: https://crates.io/crates/fantoccini
[Thirtyfour]: https://crates.io/crates/thirtyfour

# Medium Priority

- Find a way to link to imports directly if no wrapper function is needed. This happens when no
  conversions on the ASM level are necessary.
- Provide an absolutely minimal allocator.
- The `js_sys` proc-macro should remove the `extern "C" { ... }` part of the input on error to avoid
  triggering the `unsafe` requirement downstream.

[Emscripten's]:
	https://github.com/emscripten-core/emscripten/blob/28bcb86466a273859b8adb43cb167b97e05e145d/src/lib/libstrings.js
[`wasm-bindgen`'s]:
	https://github.com/wasm-bindgen/wasm-bindgen/blob/086af5a849ba86a176ebbf60f4182e9b82607584/crates/cli-support/src/js/mod.rs#L1954-L1983

# Low Priority

- Linker functionality should live in its own crate so a newer linker versions can support multiple
  versions.
- Can we remove custom sections in pre-processing by modifying `.rlib`s?
- Re-evaluate caching via the linker.
- Nicer error/warning messages from linker and when appropriate instruct users to file bug reports.
- Polish LLD linker argument parsing. Maybe learn from [`wasm-component-ld`]. We need a way of
  handling the same argument being passed multiple times.
- Print more debugging information in the linker but gate it behind `-v`.

[`wasm-component-ld`]: https://github.com/bytecodealliance/wasm-component-ld

# Upstream

This is a list of upstream issues that could make our lives significantly easier:

- LLVM v22 delivers support for the GC proposal, with which we can implement the `externref` table
  much more efficiently.
- LLVM has incomplete GC support for our needs: [llvm/llvm-project#136594].
- Stable `asm!` support for Wasm: [rust-lang/rust#136382].
- `asm!` support with target features: [rust-lang/rust#113221]
- Verbatim `asm!` parameters: [rust-lang/rust#132083].
- Better stable proc-macro support:
  - `quote!`: [rust-lang/rust#54722].
  - Diagnostics: [rust-lang/rust#54140].
  - Execution in non-proc-macro crates: [rust-lang/rust#130856].
- Wasm RAB integration: [WebAssembly/spec#1292].
- Elevate `wasm64-unknown-unknown` to tier 2: [rust-lang/rust#146944].
- A way to flag proc-macros as `unsafe`: [rust-lang/rfcs#3715].
- Link internal functions without exporting them: [rust-lang/rust#29603] or [rust-lang/rfcs#3834].
- Our linker warnings should show up for users: [rust-lang/rust#136096].
- Safe slice to array conversion: [rust-lang/rust#133508].
- `TextDe/Encoder` could support `SharedArrayBuffer`s:
  - [Chrome Bug](https://issues.chromium.org/issues/40102463)
  - [Firefox Bug](https://bugzilla.mozilla.org/show_bug.cgi?id=1561594)
- `wasm-encoder` `io::Write` support: [bytecodealliance/wasm-tools#778]
- Improved test coverage data merging: [llvm/llvm-project#121194]

[llvm/llvm-project#136594]: https://github.com/llvm/llvm-project/issues/136594
[rust-lang/rust#136382]: https://github.com/rust-lang/rust/issues/136382
[rust-lang/rust#113221]: https://github.com/rust-lang/rust/issues/113221
[rust-lang/rust#132083]: https://github.com/rust-lang/rust/issues/132083
[rust-lang/rust#54722]: https://github.com/rust-lang/rust/issues/54722
[rust-lang/rust#54140]: https://github.com/rust-lang/rust/issues/54140
[rust-lang/rust#130856]: https://github.com/rust-lang/rust/issues/130856
[WebAssembly/spec#1292]: https://github.com/WebAssembly/spec/issues/1292
[rust-lang/rust#146944]: https://github.com/rust-lang/rust/issues/146944
[rust-lang/rfcs#3715]: https://github.com/rust-lang/rfcs/pull/3715
[rust-lang/rust#29603]: https://github.com/rust-lang/rust/issues/29603
[rust-lang/rfcs#3834]: https://github.com/rust-lang/rfcs/pull/3834
[rust-lang/rust#136096]: https://github.com/rust-lang/rust/issues/136096
[rust-lang/rust#133508]: https://github.com/rust-lang/rust/issues/133508
[bytecodealliance/wasm-tools#778]: https://github.com/bytecodealliance/wasm-tools/issues/778
[llvm/llvm-project#121194]: https://github.com/llvm/llvm-project/pull/121194
