## Currently Working on

- `#[js_sys]`:
  - Major re-factoring
  - Another pass on namespacing
- More Primitive types:
  - Expand `JsString`
  - Expand `JsArray`
  - `JsNumber`
  - `BigInt`
- Basic runner

## Example

There is a very basic working example in `client/examples/basic.rs`. To run it:

```sh
cd client
cargo build --example basic
```

Now host `client` with your favorite HTTP server and navigate to `examples/basic.html`. Also give
`wasm64-unknown-unknown` a try!
