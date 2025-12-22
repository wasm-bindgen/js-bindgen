# Cached Archives

This describes how ``cached_embed_asm!` works. Its function is to cache assembly to archive conversion right into the crate files to reduce compile times.

## Bootstrap Mode

While developing a crate, archive files have to be continuously updated to adapt to code changes. To that effect we introduce a `JS_BINDGEN_BOOTSTRAP_{package} = "1"` environment variable and a `js-bingen/bootstrap` crate feature that enables a bootstrap mode with this behavior.

When using `cached_embed_asm!` the first time without enabling bootstrap mode, the user will encounter build errors.

If bootstrap mode is neglected archive files might become out-of-date. To prevent this we check in regular mode if all archives are up-to-date by comparing file modification timestamps.

## Implementation

### `build.rs`

Registers archive directory with the linker depending on the target and target features.
Provides the archive directory path to the proc-macro via an environment variable.

#### Bootstrap Mode

Deletes all generated archives to prepare for the proc-macro to generate new ones.

### Proc-Macro

Links the archive file.

#### Regular Mode

Checks that all archives are up-to-date by comparing their file modification timestamps against the source file.

During development Rust Analyzer causes race conditions that can cause these checks to fail. Until we can resolve this you can opt-out of those checks via the environment variable `JS_BINDGEN_CACHE_DISABLE_CHECK = 1`.

#### Bootstrap Mode

Generate archive files from the input and stores them in the archive directory.

## **Untested** Split Archives Into Crates

Archives can be split into separate crates that can be individually depended on to reduce crate size.
`build.rs` implementations must live in those crates to be able to generate the correct archive directory path.

However, for bootstrap mode to work the crate name and version have to be kept in sync with the crate the proc-macro is being called from.

## FAQ

### Why use a crate feature and environment variable instead of a single environment variable?

Dependencies can't be gated behind environment variables. It should remain possible for end-users to compile everything without `syn` in the dependency tree, which is often a source of major compile-time overhead.

### Why use a crate feature and environment variable instead of a single `cfg`?

Currently passing `cfg`s to proc-macros during cross-compilation is not possible.
See https://github.com/rust-lang/cargo/issues/4423.

## TODOs

- Measure how much time archive caching is really saving us.
- Measure how expensive it is to check if archive caches are up-to-date.
  Potentially find alternative solutions or disable entirely.
- Test if `build.rs` in dedicated archive dependencies is properly re-run after a change in the crate actually calling the proc-macros.
- Consider fallback to non-cached behavior when archives are not present and emit a warning.
