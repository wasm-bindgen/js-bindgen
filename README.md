## Example

There is a very basic working example in `client/examples/basic.rs`. To run it:

```sh
cd client
cargo build --example basic
```

Now host `client` with your favorite HTTP server and navigate to `examples/basic.html`. Also give
`wasm64-unknown-unknown` a try!

## Test

You can run parts of the CI locally:

```sh
cd dev
# Run `cargo build` with various flags:
cargo run -- build
# Run `cargo test` with various flags:
cargo run -- test
```
