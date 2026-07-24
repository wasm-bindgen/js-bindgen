# Benchmarks

Run the raw Node.js call benchmark:

```console
node bench.mjs
```

The script builds both implementations with the stable toolchain and calls their raw
`WebAssembly.Instance` exports directly. Generated JavaScript is used only to instantiate each
module.

The comparison uses the in-tree `js-bindgen` and the exact published `wasm-bindgen` version pinned
in `Cargo.toml`. The matching `wasm-bindgen` CLI must be available on `PATH`.

The workload can be adjusted with `JBG_BENCH_ITERATIONS`, `JBG_BENCH_WARMUPS`, and
`JBG_BENCH_SAMPLES`.

To add another benchmark with the same call shape, export the same function name from both Rust
crates and add one entry to `cases` in `bench.mjs`. Workloads with different inputs or outputs can
use a new worker module under `cases`.
