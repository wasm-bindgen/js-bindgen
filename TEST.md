Manual testing commands until CI is set up:

```sh
cargo test -p js-bindgen -p js-sys-macro --target x86_64-unknown-linux-gnu
```

```sh
cargo build --example basic --release --timings && wasm-tools print -o examples/basic.wat target/wasm32-unknown-unknown/release/examples/basic.wasm && wasm-tools validate examples/basic.wat
```

```sh
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-Ctarget-feature=+atomics -Clink-arg=--shared-memory -Clink-arg=--max-memory=4294967296" cargo +nightly build --example basic -Zbuild-std=panic_abort,std --release --timings && wasm-tools print -o examples/basic.wat target/wasm32-unknown-unknown/release/examples/basic.wasm && wasm-tools validate examples/basic.wat
```

```sh
cargo +nightly build --example basic --target wasm64-unknown-unknown -Zbuild-std=panic_abort,std --release --timings && wasm-tools print -o examples/basic.wat target/wasm64-unknown-unknown/release/examples/basic.wasm && wasm-tools validate examples/basic.wat
```

```sh
CARGO_TARGET_WASM64_UNKNOWN_UNKNOWN_RUSTFLAGS="-Ctarget-feature=+atomics -Clink-arg=--shared-memory -Clink-arg=--max-memory=17179869184" cargo +nightly build --example basic --target wasm64-unknown-unknown -Zbuild-std=panic_abort,std --release --timings && wasm-tools print -o examples/basic.wat target/wasm64-unknown-unknown/release/examples/basic.wasm && wasm-tools validate examples/basic.wat
```
