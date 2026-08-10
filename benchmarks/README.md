# Benchmarks

Run the raw call benchmark with Node.js:

```console
npm install
node bench.mjs
```

Pass one or more case-insensitive substrings to run only matching benchmarks. Multiple filters are
combined with `OR`:

```console
node bench.mjs vec_u8 option_i32
```

The comparison uses the in-tree `js-bindgen` and the exact published `wasm-bindgen` version pinned
in `Cargo.toml`. The matching `wasm-bindgen` CLI must be available on `PATH`.

Warmup, batching, sampling, and statistics are handled by `mitata`. Every implementation of every
benchmark runs in a fresh process. The parent process aggregates the results after both
implementations finish, preventing one benchmark's JIT and GC state from affecting another.

The runner recreates the ignored `generated` directory on every invocation. Rust build artifacts
remain in `target` so subsequent filtered runs only rebuild changed inputs.

Benchmark functions are discovered from raw Wasm exports whose names start with `bench_`. To add a
benchmark, export the same `bench_*` function from both Rust crates. The runner infers Number,
BigInt, and reference parameters before measurement and gives every benchmark its own Wasm instance.
