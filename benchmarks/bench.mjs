import { spawnSync } from "node:child_process";
import { once } from "node:events";
import { rm } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { Worker } from "node:worker_threads";

const benchmarkDirectory = dirname(fileURLToPath(import.meta.url));
const repositoryDirectory = join(benchmarkDirectory, "..");
const targetDirectory = join(benchmarkDirectory, "target");
const generatedDirectory = join(benchmarkDirectory, "generated");

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

function count(name, fallback) {
  const value = Number.parseInt(process.env[name] ?? fallback, 10);

  if (!Number.isSafeInteger(value) || value <= 0) {
    throw new Error(`${name} must be a positive integer`);
  }

  return value;
}

await build();

const iterations = count("JBG_BENCH_ITERATIONS", "20000000");
const warmups = count("JBG_BENCH_WARMUPS", "5");
const samples = count("JBG_BENCH_SAMPLES", "15");
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
const cases = [
  {
    name: "i32 identity",
    exportName: "i32_identity",
    worker: new URL("./cases/i32.mjs", import.meta.url),
  },
];

class Runner {
  constructor(worker, implementation) {
    this.worker = worker;
    this.implementation = implementation;
  }

  static async create(testCase, implementation) {
    const worker = new Worker(testCase.worker, {
      workerData: {
        exportName: testCase.exportName,
        implementation,
        iterations,
      },
    });
    const [message] = await once(worker, "message");

    if (message.type !== "ready") {
      throw new Error(`unexpected worker message: ${message.type}`);
    }

    return new Runner(worker, implementation);
  }

  async run() {
    this.worker.postMessage("run");
    const [message] = await once(this.worker, "message");

    if (message.type !== "result") {
      throw new Error(`unexpected worker message: ${message.type}`);
    }

    return message.elapsed;
  }

  close() {
    return this.worker.terminate();
  }
}

console.log(`Node.js ${process.version}`);
console.log(`${iterations.toLocaleString()} calls per sample`);

for (const testCase of cases) {
  const runners = await Promise.all(
    implementations.map(implementation =>
      Runner.create(testCase, implementation),
    ),
  );
  const results = Object.fromEntries(
    implementations.map(({ name }) => [name, []]),
  );

  for (let round = 0; round < warmups + samples; round++) {
    const offset = round % runners.length;

    for (let index = 0; index < runners.length; index++) {
      const runner = runners[(index + offset) % runners.length];
      const elapsed = await runner.run();

      if (round >= warmups) {
        results[runner.implementation.name].push(elapsed);
      }
    }
  }

  console.log(`\n${testCase.name}`);

  for (const implementation of implementations) {
    const values = results[implementation.name].toSorted(
      (left, right) => left - right,
    );
    const median = values[Math.floor(values.length / 2)];
    const low = values[Math.floor(values.length / 4)];
    const high = values[Math.floor((values.length * 3) / 4)];

    console.log(
      `${implementation.name.padEnd(12)} ${median.toFixed(4)} ns/call  ` +
        `[${low.toFixed(4)}, ${high.toFixed(4)}]`,
    );
  }

  await Promise.all(runners.map(runner => runner.close()));
}
