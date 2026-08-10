import { spawnSync } from "node:child_process"
import { readFile, rm } from "node:fs/promises"
import { dirname, join } from "node:path"
import process from "node:process"
import { fileURLToPath, pathToFileURL } from "node:url"

import { bench, do_not_optimize, run } from "mitata"

const benchmarkPath = fileURLToPath(import.meta.url)
const benchmarkDirectory = dirname(benchmarkPath)
const repositoryDirectory = join(benchmarkDirectory, "..")
const targetDirectory = join(benchmarkDirectory, "target")
const generatedDirectory = join(benchmarkDirectory, "generated")
const benchmarkPrefix = "bench_"
const exceptionHandling = process.env.JBG_BENCH_NO_EH !== "1"
const rustflagsVariable = "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS"
const rustflags = [
	process.env[rustflagsVariable],
	exceptionHandling && "-Awarnings -Ctarget-feature=+exception-handling",
]
	.filter(Boolean)
	.join(" ")
const wasmCargoEnvironment = {
	[rustflagsVariable]: rustflags,
}
const workerImplementation = process.env.JBG_BENCH_IMPLEMENTATION
const workerBenchmark = process.env.JBG_BENCHMARK

const filters = process.argv.slice(2).map(filter => filter.toLowerCase())

function execute(command, args, options = {}) {
	const result = spawnSync(command, args, {
		cwd: benchmarkDirectory,
		env: { ...process.env, ...options.env },
		stdio: "inherit",
	})

	if (result.error) {
		throw result.error
	}

	if (result.status !== 0) {
		throw new Error(`${command} ${args.join(" ")} failed with status ${result.status}`)
	}
}

function executeForOutput(command, args, options = {}) {
	const result = spawnSync(command, args, {
		cwd: benchmarkDirectory,
		encoding: "utf8",
		env: { ...process.env, ...options.env },
		maxBuffer: 16 * 1024 * 1024,
		stdio: ["ignore", "pipe", "inherit"],
	})

	if (result.error) {
		throw result.error
	}

	if (result.status !== 0) {
		throw new Error(`${command} ${args.join(" ")} failed with status ${result.status}`)
	}

	return result.stdout
}

function cargo(args, options) {
	execute("cargo", args, options)
}

async function build() {
	await rm(generatedDirectory, { force: true, recursive: true })
	const toolchain = exceptionHandling ? ["+nightly"] : []

	cargo(
		[
			...toolchain,
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
					...wasmCargoEnvironment,
					CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER: join(
						repositoryDirectory,
						"host/cargo-shim/linker"
				),
			},
		}
	)

	const jsBindgenInput = join(
		targetDirectory,
		"wasm32-unknown-unknown/release/js_bindgen_benchmark.wasm"
	)
	const jsBindgenOutput = join(generatedDirectory, "js-bindgen")

	cargo([
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
	])

	cargo(
		[
			...toolchain,
			"build",
			"--quiet",
			"--package",
			"wasm-bindgen-benchmark",
			"--release",
			"--target",
			"wasm32-unknown-unknown",
		],
		{ env: wasmCargoEnvironment }
	)

	const wasmBindgenInput = join(
		targetDirectory,
		"wasm32-unknown-unknown/release/wasm_bindgen_benchmark.wasm"
	)
	const wasmBindgenOutput = join(generatedDirectory, "wasm-bindgen")

	execute("wasm-bindgen", [
		wasmBindgenInput,
		"--target",
		"web",
		"--out-dir",
		wasmBindgenOutput,
		"--no-typescript",
	])
}

let instanceId = 0

async function loadImplementation(implementation) {
	const moduleUrl = pathToFileURL(implementation.modulePath)
	moduleUrl.searchParams.set("instance", String(instanceId++))

	const module = await import(moduleUrl)
	const bytes = await readFile(implementation.wasmPath)

	if (implementation.kind === "js-bindgen") {
		const wasmModule = await WebAssembly.compile(bytes)
		const result = await new module.JsBindgen(wasmModule).instantiate()
		return {
			raw: result.instance.exports,
			wrapped: result.exports,
		}
	}

	if (implementation.kind === "wasm-bindgen") {
		return {
			raw: module.initSync({ module: bytes }),
			wrapped: module,
		}
	}

	throw new Error(`unknown implementation: ${implementation.kind}`)
}
const implementations = [
	{
		name: "js-bindgen",
		kind: "js-bindgen",
		modulePath: join(generatedDirectory, "js-bindgen/js_bindgen_benchmark.mjs"),
		wasmPath: join(generatedDirectory, "js-bindgen/js_bindgen_benchmark.wasm"),
	},
	{
		name: "wasm-bindgen",
		kind: "wasm-bindgen",
		modulePath: join(generatedDirectory, "wasm-bindgen/wasm_bindgen_benchmark.js"),
		wasmPath: join(generatedDirectory, "wasm-bindgen/wasm_bindgen_benchmark_bg.wasm"),
	},
]

function compareBenchmarks(left, right) {
	return left.localeCompare(right)
}

// Wasm functions expose their arity but not their parameter types. Start with
// Number and retry the parameter that rejected it as BigInt. Parameters that
// never coerce the probe are reference values.
async function inferArguments(exportName, call, allowFailure = false) {
	const kinds = Array(call.length).fill("number")

	while (true) {
		const coerced = Array(call.length).fill(false)
		let lastCoerced = -1
		let asynchronous = false
		let result
		let fails = false
		const probes = kinds.map((kind, index) => ({
			[Symbol.toPrimitive]() {
				coerced[index] = true
				lastCoerced = index
				return kind === "bigint" ? 42n : 42
			},
		}))

		try {
			result = call(...probes)
		} catch (error) {
			if (
				error instanceof TypeError &&
				lastCoerced >= 0 &&
				kinds[lastCoerced] !== "bigint"
			) {
				kinds[lastCoerced] = "bigint"
				continue
			}

			if (!allowFailure) {
				throw new Error(`cannot infer parameters for ${exportName}`, {
					cause: error,
				})
			}

			fails = true
		}

		if (!fails && typeof result?.then === "function") {
			asynchronous = true

			try {
				result = await result
			} catch (error) {
				if (!allowFailure) {
					throw new Error(`cannot infer result for ${exportName}`, {
						cause: error,
					})
				}

				fails = true
			}
		}

		let bigintIndex = 0

		return {
			inputs: kinds.map((kind, index) => {
				if (!coerced[index]) {
					return {}
				}

				if (kind === "bigint") {
					return bigintIndex++ === 0 ? 42n : 0n
				}

				return 42
			}),
			kinds: kinds.map((kind, index) => (coerced[index] ? kind : "reference")),
			asynchronous,
			result,
			fails,
		}
	}
}

async function discoverBenchmarks(implementation) {
	const module = new WebAssembly.Module(await readFile(implementation.wasmPath))
	return WebAssembly.Module.exports(module)
		.filter(item => item.kind === "function" && item.name.startsWith(benchmarkPrefix))
		.map(item => item.name)
		.sort(compareBenchmarks)
}

let benchmarkId = 0

function createBenchmark(call, inputs, fails, asynchronous) {
	const parameterCount = inputs.length
	const id = benchmarkId++
	const parameters = Array.from({ length: parameterCount }, (_, index) => `arg${index}`)
	const invocation = `call(${parameters.join(", ")})`
	const await_ = asynchronous ? "await " : ""
	const measuredCall = fails
		? `
            try {
              result = ${await_}${invocation};
            } catch (error) {
              result = error;
            }`
		: `result = ${await_}${invocation};`
	const setup = parameters
		.map(
			(_, index) => `
        [${index}]() {
          return inputs[${index}];
        },`
		)
		.join("")

	// Compile a separate call site for every implementation. Reusing the same
	// factory shares V8 optimization feedback between otherwise independent
	// benchmarks and makes the result depend on registration order.
	return Function(
		"call",
		"inputs",
		"doNotOptimize",
		`
      return function* benchmark${id}() {
        let result;

        yield {${setup}
          ${asynchronous ? "async " : ""}bench(${parameters.join(", ")}) {
            ${measuredCall}
          },
        };

        doNotOptimize(result);
      };
    `
	)(call, inputs, do_not_optimize)
}

async function runWorker() {
	const implementation = implementations.find(({ kind }) => kind === workerImplementation)

	if (!implementation) {
		throw new Error(`unknown benchmark implementation: ${workerImplementation}`)
	}

	const wrappedExports = await loadImplementation(implementation)
	const wrappedCall = wrappedExports.wrapped[workerBenchmark]

	if (typeof wrappedCall !== "function") {
		throw new Error(`missing JS export: ${implementation.name}:${workerBenchmark}`)
	}

	const wrapped = await inferArguments(workerBenchmark, wrappedCall, true)
	const returnsReference =
		(typeof wrapped.result === "object" && wrapped.result !== null) ||
		typeof wrapped.result === "function"
	let useWrapper =
		wrapped.asynchronous || wrapped.fails || wrapped.kinds.includes("reference")
	let raw

	if (!useWrapper) {
		const rawExports = await loadImplementation(implementation)
		const rawCall = rawExports.raw[workerBenchmark]

		if (typeof rawCall !== "function") {
			throw new Error(`missing Wasm export: ${implementation.name}:${workerBenchmark}`)
		}

		raw = await inferArguments(workerBenchmark, rawCall)
		useWrapper = Array.isArray(raw.result) && returnsReference
	}

	// Argument inference runs user code and can initialize queues or perturb
	// owned table slots. Measure a fresh instance after all probing is complete.
	const measuredExports = await loadImplementation(implementation)
	const call = (useWrapper ? measuredExports.wrapped : measuredExports.raw)[workerBenchmark]
	const { asynchronous, fails, inputs, kinds } = useWrapper ? wrapped : raw
	bench(implementation.name, createBenchmark(call, inputs, fails, asynchronous))

	const result = await run({ format: "quiet", throw: true })
	const trial = result.benchmarks[0]
	const measurement = trial?.runs[0]

	if (!measurement?.stats) {
		throw new Error(`benchmark failed: ${implementation.name}:${workerBenchmark}`)
	}

	const { debug: _, samples: __, ...stats } = measurement.stats
	process.stdout.write(
		JSON.stringify({
				context: {
					arch: result.context.arch,
					cpu: result.context.cpu,
					exceptionHandling,
					runtime: result.context.runtime,
					version: result.context.version,
			},
			implementation: implementation.name,
			asynchronous,
			fails,
			kinds,
			stats,
		})
	)
}

function runCase(implementation, exportName) {
	const output = executeForOutput(process.execPath, [...process.execArgv, benchmarkPath], {
		env: {
			JBG_BENCHMARK: exportName,
			JBG_BENCH_IMPLEMENTATION: implementation.kind,
		},
	})

	try {
		return JSON.parse(output)
	} catch (error) {
		throw new Error(`invalid benchmark output from ${implementation.name}:${exportName}`, {
			cause: error,
		})
	}
}

function formatTime(nanoseconds) {
	if (nanoseconds < 1) {
		return `${(nanoseconds * 1000).toFixed(2)} ps`
	}

	if (nanoseconds < 1000) {
		return `${nanoseconds.toFixed(2)} ns`
	}

	if (nanoseconds < 1_000_000) {
		return `${(nanoseconds / 1000).toFixed(2)} µs`
	}

	if (nanoseconds < 1_000_000_000) {
		return `${(nanoseconds / 1_000_000).toFixed(2)} ms`
	}

	return `${(nanoseconds / 1_000_000_000).toFixed(2)} s`
}

function printContext(context) {
	console.log(`clk: ~${context.cpu.freq.toFixed(2)} GHz`)
	console.log(`cpu: ${context.cpu.name}`)
	console.log(
		`runtime: ${context.runtime}${context.version ? ` ${context.version}` : ""} (${context.arch})`
	)
	console.log(`exception-handling: ${context.exceptionHandling ? "enabled" : "disabled"}`)
}

function printResults(name, results) {
	console.log("")
	console.log(`• ${name}`)
	console.log("-".repeat(96))

	for (const { implementation, stats } of results) {
		const average = `${formatTime(stats.avg)}/iter`.padStart(15)
		const range = `(${formatTime(stats.min)} … ${formatTime(stats.max)})`.padStart(25)
		const percentiles = `${formatTime(stats.p75)} / ${formatTime(stats.p99)}`.padStart(20)
		console.log(`${implementation.padEnd(18)}${average} ${range} ${percentiles}`)
	}

	const baseline = results.find(({ implementation }) => implementation === "js-bindgen")
	const comparisons = []
	for (const result of results) {
		if (result === baseline) {
			continue
		}

		const baselineIsFaster = baseline.stats.avg <= result.stats.avg
		const ratio = baselineIsFaster
			? result.stats.avg / baseline.stats.avg
			: baseline.stats.avg / result.stats.avg
		console.log("")
		console.log("summary")
		console.log(
			`  js-bindgen ${ratio.toFixed(2)}x ${
				baselineIsFaster ? "faster" : "slower"
			} than ${result.implementation}`
		)

		comparisons.push({
			baseline: baseline.stats.avg,
			implementation: result.implementation,
			name,
			other: result.stats.avg,
			ratio,
			slower: !baselineIsFaster,
		})
	}

	return comparisons
}

function color(text, code) {
	if (!process.stdout.isTTY) {
		return text
	}

	return `\u001B[${code}m${text}\u001B[0m`
}

function printComparisons(comparisons) {
	console.log("")
	console.log("js-bindgen comparison")
	console.log("-".repeat(96))

	for (const comparison of comparisons) {
		const result =
			`${comparison.name.padEnd(48)} ${comparison.ratio.toFixed(2)}x ` +
			`${comparison.slower ? "slower" : "faster"}  ` +
			`(${formatTime(comparison.baseline)} vs ${formatTime(comparison.other)} ` +
			`${comparison.implementation})`
		console.log(color(result, comparison.slower ? 31 : 32))
	}
}

async function runCoordinator() {
	await build()

	const discoveredBenchmarks = await Promise.all(implementations.map(discoverBenchmarks))
	const benchmarks = discoveredBenchmarks[0]

	if (benchmarks.length === 0) {
		throw new Error(`no ${benchmarkPrefix} exports found`)
	}

	for (let index = 1; index < discoveredBenchmarks.length; index++) {
		if (benchmarks.join("\n") !== discoveredBenchmarks[index].join("\n")) {
			throw new Error(
				`${implementations[index].name} exports do not match ${implementations[0].name}`
			)
		}
	}

	const selectedBenchmarks = benchmarks.filter(exportName => {
		if (filters.length === 0) {
			return true
		}

		return filters.some(filter => exportName.toLowerCase().includes(filter))
	})

	if (selectedBenchmarks.length === 0) {
		throw new Error(`no benchmark matched: ${filters.join(", ")}`)
	}

	let printedContext = false
	const comparisons = []
	for (const exportName of selectedBenchmarks) {
		let expectedAsynchronous
		let expectedKinds
		let expectedFailure
		const results = []

		for (const implementation of implementations) {
			const result = runCase(implementation, exportName)

			if (
				expectedAsynchronous !== undefined &&
				expectedAsynchronous !== result.asynchronous
			) {
				throw new Error(
					`async behavior mismatch for ${exportName}: ${expectedAsynchronous} != ${result.asynchronous}`
				)
			}

			if (expectedKinds && expectedKinds.join() !== result.kinds.join()) {
				throw new Error(
					`parameter ABI mismatch for ${exportName}: ${expectedKinds.join()} != ${result.kinds.join()}`
				)
			}

			if (expectedFailure !== undefined && expectedFailure !== result.fails) {
				throw new Error(
					`failure behavior mismatch for ${exportName}: ${expectedFailure} != ${result.fails}`
				)
			}

			expectedAsynchronous = result.asynchronous
			expectedKinds = result.kinds
			expectedFailure = result.fails
			results.push(result)

			if (!printedContext) {
				printContext(result.context)
				printedContext = true
			}
		}

		comparisons.push(...printResults(exportName, results))
	}

	printComparisons(comparisons)
}

if (workerImplementation === undefined && workerBenchmark === undefined) {
	await runCoordinator()
} else if (workerImplementation !== undefined && workerBenchmark !== undefined) {
	await runWorker()
} else {
	throw new Error("incomplete benchmark worker configuration")
}
