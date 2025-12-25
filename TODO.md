# Current

- Generate bindings via proc-macro.
- Basic runner.

# Critical Priority

- Replace `llvm-mc` with `wasm-tools`. See
  https://github.com/bytecodealliance/wasm-tools/issues/2405.

# High Priority

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
- Support 64-bit `externref` tables. The default should remain 32-bit.
  https://github.com/llvm/llvm-project/issues/172868
- When we detect nightly, we should switch to `global_asm!` internally. However, this is awaiting a
  couple of bug fixes:
  - On `wasm64` tables automatically use `i64` as the address type with no way to turn it off.
    https://github.com/llvm/llvm-project/pull/173063#discussion_r2635871878
  - `global_asm!` simply doesn't work with Wasm right now when using instructions that require
    target features. https://github.com/rust-lang/rust/issues/113221
- Can we remove custom sections in pre-processing by modifying `.rlib`s?
- Re-evaluate caching via the linker.
