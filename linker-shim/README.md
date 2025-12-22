# Description

`js-bindgen` uses a custom linker. However, during development we can't use this linker pre-installed or pre-compiled, we need the one being worked on now!

The issue is that specifying that specifying a custom linker via [`target.<triple>.linker`](https://doc.rust-lang.org/1.92.0/cargo/reference/config.html#targettriplelinker) requires a path to a binary and can't contain arbitrary commands like `cargo run`.

To solve this we introduce a simple script files that calls `cargo run -p js-bindgen-linker` for us. To make sure this works cross-platform, we make use of the feature on Windows to run binaries without specifying their extension. Now our linker is just called `linker` but will call different files depending on the operating system.

- For Linux and MacOS the shell script `linker` is provided.
- For Windows the batch file `linker.cmd` is provided.
