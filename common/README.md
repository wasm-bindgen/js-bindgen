# Description

`js-bindgen` uses a custom linker and runner. However, during development we can't use these
binaries pre-installed or pre-compiled, we need the one being worked on now!

The issue is that specifying a custom linker via
[`target.<triple>.linker`](https://doc.rust-lang.org/1.92.0/cargo/reference/config.html#targettriplelinker)
requires a path to a binary and can't contain arbitrary commands like `cargo run`.

To solve this we introduce a simple script file that calls `cargo run -p js-bindgen-ld` for us. To
make sure this works cross-platform, we make use of some clever tricks you can observe by taking a
look at the file yourself.
