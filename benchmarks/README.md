# Benchmarks

Run the raw call benchmark with Node.js or Deno:

```console
npm install
node --no-compilation-cache bench.mjs
deno run -A --v8-flags=--no-compilation-cache bench.mjs
```

Select cases by passing one or more name fragments:

```console
node --no-compilation-cache bench.mjs js_value
node --no-compilation-cache bench.mjs option result
```

Fragments are matched case-insensitively against display names and export names.

The comparison uses the in-tree `js-bindgen` and the exact published `wasm-bindgen` version pinned
in `Cargo.toml`. The matching `wasm-bindgen` CLI must be available on `PATH`.

Warmup, batching, sampling, and statistics are handled by `mitata`.

To add another benchmark with the same call shape, export the same function name from both Rust
crates and add one entry to `cases` in `bench.mjs`. Workloads with different inputs or outputs can
use a new benchmark module under `cases`.
