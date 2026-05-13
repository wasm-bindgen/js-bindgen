#!/usr/bin/env sh
cargo rustc -p wabii --target wasm32-unknown-unknown --crate-type staticlib --release
cargo +nightly rustc -p wabii --target wasm64-unknown-unknown --crate-type staticlib -Zbuild-std=panic_abort,std --release

cp target/wasm32-unknown-unknown/release/libwabii.a wabi-sdk/wasm32
cp target/wasm64-unknown-unknown/release/libwabii.a wabi-sdk/wasm64
