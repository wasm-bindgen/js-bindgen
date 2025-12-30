# Current

- Work some more on `embed_asm!`:
  - Implement external types.
  - Namespace ASM.
- Basic runner.

# Critical Priority

- Replace `llvm-mc` with `wasm-tools`. See [bytecodealliance/wasm-tools#2405].

[bytecodealliance/wasm-tools#2405]: https://github.com/bytecodealliance/wasm-tools/issues/2405

# High Priority

- Allocate slots on the `externref` table in batches.
- Figure out what to do with the panic optimization.
- Validate and improve performance of `JsString` encoding/decoding. See [Emscripten's] or
  [`wasm-bindgen`'s] implementation for inspiration.
- Experiment if allocation is better for build times then iterator chaining in proc-macros.
- Find a way to prevent users from accidentally using the default linker. Could be done by supplying
  an invalid object file that would be removed by our custom linker.
- Version all names to make packages compatible with other versions of itself.
- Embed crate version to make linker capable of detecting unsupported versions.
- Add tracking for ASM object files in the linker, so we don't re-generate them each time.
- Evaluate the output folder of our ASM objet files. Some ideas:
  - Store them next to the output file.
  - Pass an environment variable from a `build.rs` pointing to the target folder and go from there.
    This seems to have failed. No build script instruction can reach the linker on Wasm.

# Medium Priority

- Provide an absolutely minimal allocator.
- The `js_sys` proc-macro should remove the `extern "C" { ... }` part of the input on error to avoid
  triggering the `unsafe` requirement downstream.
- Optimize linker file interactions by using memory mapped files instead of reading and writing
  everything into memory.
- Run the assembly compiler on the proc-macro level so users see errors without having to engage the
  linker.
- Parse the JS on the proc-macro level so users see errors. E.g. `oxc-parser`.
- Use an AST for JS code generation so we don't play around with strings. E.g. `oxc-codegen`.

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
