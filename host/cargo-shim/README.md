# Cargo Shim

`js-bindgen` uses a custom linker and runner. However, during development we can't use the
pre-installed or pre-compiled binaries, we need the ones being worked on now!

There are two issues we have to circumvent:

- Specifying a custom linker via
  [`target.<triple>.linker`](https://doc.rust-lang.org/1.92.0/cargo/reference/config.html#targettriplelinker)
  requires a path to a binary and can't contain arbitrary commands like `cargo run`.
- We want to change the working directory to avoid inheriting `client/.cargo/.config.toml`.

To solve this we introduce simple bash files, `linker` and `runner` that call
`cargo run -p js-bindgen-ld/runner` for us. For Windows we introduce minimal `*.exe` files that do
the same as the bash files. On Linux and macOS the bash files are recognized as executables, but on
Windows it will automatically add the executable extension and search for a `*.exe` file instead.

Note that we explicitly specify `+stable` for all Cargo commands because even if we run client
packages with Nightly, we never want to run the linker or runner on Nightly. Unless we are testing
the linker or runner, in which case we don't go through the shim.

Unfortunately the `*.exe` files are shipped as binary blobs in the Git repo. This is important
because we aim to have a zero-setup workspace for contributors. To ensure reproducibility we
generate the files in CI and check for binary equivalence.

While in theory the binary blobs could be produced locally, reproducible builds across machines and
setups are [currently not possible in Rust](https://github.com/rust-lang/rust/issues/129080).
Therefor the binary blobs will be generated in CI and can be downloaded from there.
