import { spawnSync } from "node:child_process";
import { readFile, rm } from "node:fs/promises";
import { dirname, join } from "node:path";
import process from "node:process";
import { fileURLToPath, pathToFileURL } from "node:url";

import { bench, group, run, summary } from "mitata";

const benchmarkDirectory = dirname(fileURLToPath(import.meta.url));
const repositoryDirectory = join(benchmarkDirectory, "..");
const targetDirectory = join(benchmarkDirectory, "target");
const generatedDirectory = join(benchmarkDirectory, "generated");

const filters = process.argv.slice(2).map(filter => filter.toLowerCase());

function execute(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: benchmarkDirectory,
    env: { ...process.env, ...options.env },
    stdio: "inherit",
  });

  if (result.error) {
    throw result.error;
  }

  if (result.status !== 0) {
    throw new Error(
      `${command} ${args.join(" ")} failed with status ${result.status}`,
    );
  }
}

function cargo(args, options) {
  execute("cargo", args, options);
}

async function build() {
  await rm(generatedDirectory, { force: true, recursive: true });

  cargo(
    [
      "+stable",
      "build",
      "--quiet",
      "--package",
      "js-bindgen-benchmark",
      "--release",
      "--target",
      "wasm32-unknown-unknown",
    ],
    {
      env: {
        CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER: join(
          repositoryDirectory,
          "host/cargo-shim/linker",
        ),
      },
    },
  );

  const jsBindgenInput = join(
    targetDirectory,
    "wasm32-unknown-unknown/release/js_bindgen_benchmark.wasm",
  );
  const jsBindgenOutput = join(generatedDirectory, "js-bindgen");

  cargo([
    "+stable",
    "run",
    "--quiet",
    "--manifest-path",
    join(repositoryDirectory, "host/Cargo.toml"),
    "--package",
    "js-bindgen-cli",
    "--",
    jsBindgenInput,
    "--out-dir",
    jsBindgenOutput,
  ]);

  cargo([
    "+stable",
    "build",
    "--quiet",
    "--package",
    "wasm-bindgen-benchmark",
    "--release",
    "--target",
    "wasm32-unknown-unknown",
  ]);

  const wasmBindgenInput = join(
    targetDirectory,
    "wasm32-unknown-unknown/release/wasm_bindgen_benchmark.wasm",
  );
  const wasmBindgenOutput = join(generatedDirectory, "wasm-bindgen");

  execute("wasm-bindgen", [
    wasmBindgenInput,
    "--target",
    "web",
    "--out-dir",
    wasmBindgenOutput,
    "--no-typescript",
  ]);
}

async function loadImplementation(implementation) {
  const module = await import(pathToFileURL(implementation.modulePath));
  const bytes = await readFile(implementation.wasmPath);

  if (implementation.kind === "js-bindgen") {
    const wasmModule = await WebAssembly.compile(bytes);
    const result = await new module.JsBindgen(wasmModule).instantiate();
    return result.instance.exports;
  }

  if (implementation.kind === "wasm-bindgen") {
    return module.initSync({ module: bytes });
  }

  throw new Error(`unknown implementation: ${implementation.kind}`);
}

await build();

const implementations = [
  {
    name: "js-bindgen",
    kind: "js-bindgen",
    modulePath: join(
      generatedDirectory,
      "js-bindgen/js_bindgen_benchmark.mjs",
    ),
    wasmPath: join(
      generatedDirectory,
      "js-bindgen/js_bindgen_benchmark.wasm",
    ),
  },
  {
    name: "wasm-bindgen",
    kind: "wasm-bindgen",
    modulePath: join(
      generatedDirectory,
      "wasm-bindgen/wasm_bindgen_benchmark.js",
    ),
    wasmPath: join(
      generatedDirectory,
      "wasm-bindgen/wasm_bindgen_benchmark_bg.wasm",
    ),
  },
];
const loadedImplementations = await Promise.all(
  implementations.map(async implementation => ({
    ...implementation,
    exports: await loadImplementation(implementation),
  })),
);
const cases = [
  {
    name: "i32 identity",
    exportName: "i32_identity",
    module: new URL("./cases/i32.mjs", import.meta.url),
  },
  {
    name: "u128 identity, small (indirect return)",
    exportName: "u128_identity",
    module: new URL("./cases/u128.mjs", import.meta.url),
    input: [42n, 0n],
  },
  {
    name: "u128 identity, wide (indirect return)",
    exportName: "u128_identity",
    module: new URL("./cases/u128.mjs", import.meta.url),
    input: [0xffffffffffffffffn, 0xffffffffffffffffn],
  },
  {
    name: "Option<i32>::Some (sentinel return)",
    exportName: "option_i32_identity",
    module: new URL("./cases/option-i32.mjs", import.meta.url),
  },
  {
    name: "Result<i32, JsValue>::Ok",
    exportName: "result_i32_identity",
    module: new URL("./cases/result-i32.mjs", import.meta.url),
  },
  {
    name: "JsValue identity",
    exportName: "js_value_identity",
    module: new URL("./cases/js-value.mjs", import.meta.url),
  },
  {
    name: "i32 identity (import roundtrip)",
    exportName: "import_i32_identity",
    module: new URL("./cases/i32.mjs", import.meta.url),
  },
  {
    name: "u128 identity, small (import roundtrip)",
    exportName: "import_u128_identity",
    module: new URL("./cases/u128.mjs", import.meta.url),
    input: [42n, 0n],
  },
  {
    name: "u128 identity, wide (import roundtrip)",
    exportName: "import_u128_identity",
    module: new URL("./cases/u128.mjs", import.meta.url),
    input: [0xffffffffffffffffn, 0xffffffffffffffffn],
  },
  {
    name: "Option<i32>::Some (import roundtrip)",
    exportName: "import_option_i32_identity",
    module: new URL("./cases/option-i32.mjs", import.meta.url),
  },
  {
    name: "Result<i32, JsValue>::Ok (import roundtrip)",
    exportName: "import_result_i32_identity",
    module: new URL("./cases/result-i32.mjs", import.meta.url),
  },
  {
    name: "JsValue identity (import roundtrip)",
    exportName: "import_js_value_identity",
    module: new URL("./cases/js-value.mjs", import.meta.url),
  },
];

const selectedCases = cases.filter(testCase => {
  if (filters.length === 0) {
    return true;
  }

  const name = testCase.name.toLowerCase();
  const exportName = testCase.exportName.toLowerCase();
  return filters.some(filter => name.includes(filter) || exportName.includes(filter));
});

if (selectedCases.length === 0) {
  throw new Error(`no benchmark matched: ${filters.join(", ")}`);
}

for (const testCase of selectedCases) {
  await group(testCase.name, async () => {
    await summary(async () => {
      for (const implementation of loadedImplementations) {
        const moduleUrl = new URL(testCase.module);
        moduleUrl.searchParams.set("implementation", implementation.kind);
        const module = await import(moduleUrl);
        const call = implementation.exports[testCase.exportName];

        if (typeof call !== "function") {
          throw new Error(
            `missing Wasm export: ${implementation.name}:${testCase.exportName}`,
          );
        }

        const benchmark = module.createBenchmark({
          call,
          implementation,
          input: testCase.input,
        });

        if (typeof benchmark !== "function") {
          throw new Error(`invalid benchmark case: ${testCase.module}`);
        }

        bench(implementation.name, benchmark).baseline(
          implementation.kind === "js-bindgen",
        );
      }
    });
  });
}

await run({ throw: true });
