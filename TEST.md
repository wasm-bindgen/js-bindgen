Manual testing commands (CI not set up yet).

Quick build (from repo root):

```sh
cd client
cargo build --example basic
cargo build --example basic --release --timings
```

Test examples:

```rust
#[js_bindgen_test::test]
fn test_wasm() {}

#[js_bindgen_test::test]
#[ignore = "hah, it works"]
fn test_ignore() {
	panic!("kaboom");
}

#[js_bindgen_test::test]
#[should_panic(expected = "kaboom")]
fn test_should_panic() {
	panic!("kaboom");
}
```

Wasm tests: Node runner (default):

```sh
cargo test --target wasm32-unknown-unknown
```

Wasm tests: Browser runner (headless):

```sh
JBG_TEST_RUNNER=browser cargo test --target wasm32-unknown-unknown
```

Wasm tests: Browser server mode (open URL in a browser):

```sh
JBG_TEST_RUNNER=server cargo test --target wasm32-unknown-unknown
```

Runner options:

- `JBG_TEST_RUNNER=node` (run in nodejs, set by default)

- `JBG_TEST_RUNNER=browser` (run in headless browser, auto detect driver)
- `JBG_TEST_GECKODRIVER=<path>` (use firefix)
- `JBG_TEST_CHROMEDRIVER=<path>` (use chrome)
- `JBG_TEST_MSEDGEDRIVER=<path>` (use edge)
- `JBG_TEST_WEBDRIVER_JSON` (set driver conf)
- `JBG_TEST_WORKER=dedicated|shared|service`

- `JBG_TEST_RUNNER=server` (serve browser runner and print URL)
- `JBG_TEST_SERVER_ADDRESS=127.0.0.1:8000` (defaults to 8000; falls back to a random port if busy)


List and filter tests:

```sh
cargo test --target wasm32-unknown-unknown -- --list
cargo test --target wasm32-unknown-unknown -- --list --format terse
cargo test --target wasm32-unknown-unknown -- some_substring
cargo test --target wasm32-unknown-unknown -- --exact full::test::name
cargo test --target wasm32-unknown-unknown -- --ignored
```

Other test flags:

```sh
cargo test --target wasm32-unknown-unknown -- --no-capture
```

Wasm tooling checks:

```sh
cd client
wasm-tools print -o examples/basic.wat target/wasm32-unknown-unknown/debug/examples/basic.wasm
wasm-tools validate examples/basic.wat
```

Shared-memory build (nightly):

```sh
cd client
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-Ctarget-feature=+atomics -Clink-arg=--shared-memory -Clink-arg=--max-memory=4294967296" \
  cargo +nightly build --example basic -Zbuild-std=panic_abort,std --release --timings
```

Wasm64 builds (nightly):

```sh
cd client
cargo +nightly build --example basic --target wasm64-unknown-unknown -Zbuild-std=panic_abort,std --release --timings
```

```sh
cd client
CARGO_TARGET_WASM64_UNKNOWN_UNKNOWN_RUSTFLAGS="-Ctarget-feature=+atomics -Clink-arg=--shared-memory -Clink-arg=--max-memory=17179869184" \
  cargo +nightly build --example basic --target wasm64-unknown-unknown -Zbuild-std=panic_abort,std --release --timings
```
