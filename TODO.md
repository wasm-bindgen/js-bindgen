# Current

- Basic runner.

# Critical Priority

- Replace `llvm-mc` with `wasm-tools`. See
  https://github.com/bytecodealliance/wasm-tools/issues/2405.

# High Priority

- Allocate slots on the `externref` table in batches.
- Figure out what to do with the panic optimization.
- Not clear yet how to separate the `js-bindgen` proc-macros from `js-sys`. Probably `js-bindgen`
  needs to receive more powerful proc-macros.
- Find a way to prevent users from accidentally using the default linker. Could be done by supplying
  an invalid object file that would be removed by our custom linker.
- Also find a way to prevent users from accidentally using our linker with something else then Wasm.
- Version all names to make packages compatible with other versions of itself.
- Embed crate version to make linker capable of detecting unsupported versions.
- Add tracking for ASM object files in the linker, so we don't re-generate them each time.
- Evaluate the output folder of our ASM objet files. Some ideas:
  - Store them next to the output file.
  - Pass an environment variable from a `build.rs` pointing to the target folder and go from there.
    This seems to have failed. No build script instruction can reach the linker on Wasm.

# Medium Priority

- Optimize linker file interactions by using memory mapped files instead of reading and writing
  everything into memory.
- Run the assembly compiler on the proc-macro level so users see errors without having to engage the
  linker.
- Parse the JS on the proc-macro level so users see errors. E.g. `oxc-parser`.
- Use an AST for JS code generation so we don't play around with strings. E.g. `oxc-codegen`.

# Low Priority

- Linker functionality should live in its own crate so a newer linker versions can support multiple
  versions.
- We have a custom `LocalKey` replica for non-atomic or non-std builds. It differs because its
  methods don't take a `'static` lifetime. It would probably be easiest to just align actual Std's
  `LocalKey` unsafely to not require `'static`.
- Support 64-bit `externref` tables. The default should remain 32-bit. See
  https://github.com/llvm/llvm-project/issues/172868.
- When we detect nightly, we should switch to `global_asm!` internally. However, this is awaiting a
  couple of bug fixes:
  - On `wasm64` tables automatically use `i64` as the address type with no way to turn it off. See
    https://github.com/llvm/llvm-project/pull/173063#discussion_r2635871878.
  - `global_asm!` simply doesn't work with Wasm right now when using instructions that require
    target features. See https://github.com/rust-lang/rust/issues/113221.
  - This will largely be blocked by not being able to pass in `&str`s verbatim. See
    https://github.com/rust-lang/rust/issues/132083.
- Can we remove custom sections in pre-processing by modifying `.rlib`s?
- Re-evaluate caching via the linker.
- Polish LLD linker argument parsing.

# Upstream

This is a list of upstream issues that could make our lives significantly easier:

- LLVM 22 delivers support for the GC proposal, with which we can implement the `externref` table
  much more efficiently.
- Stable `asm!` support for Wasm: https://github.com/rust-lang/rust/issues/136382.
- Verbatim `asm!` parameters: https://github.com/rust-lang/rust/issues/132083.
- Better stable proc-macro support
  - `quote!`: https://github.com/rust-lang/rust/issues/54722.
  - Diagnostics: https://github.com/rust-lang/rust/issues/54140.
  - Execution in non-proc-macro crates: https://github.com/rust-lang/rust/issues/130856.
- Link internal functions without exporting them: https://github.com/rust-lang/rust/issues/29603 or
  https://github.com/rust-lang/rfcs/pull/3834.
- Elevate `wasm64-unknown-unknown` to tier 2: https://github.com/rust-lang/rust/issues/146944.
- Safe slice to array conversion: https://github.com/rust-lang/rust/issues/133508.
- A way to flag proc-macros as `unsafe`: https://github.com/rust-lang/rfcs/pull/3715.
