import fs from "node:fs/promises";
import { pathToFileURL } from "node:url";
import { createTextFormatter } from "./shared.mjs";
import { runTests } from "./runner-core.mjs";

const wasmPath = process.env.JS_BINDGEN_WASM;
const importsPath = process.env.JS_BINDGEN_IMPORTS;
const testsJson = process.env.JS_BINDGEN_TESTS;
const nocapture = process.env.JS_BINDGEN_NOCAPTURE === "1";
const filtered = Number(process.env.JS_BINDGEN_FILTERED || "0");

if (!wasmPath || !importsPath || !testsJson) {
	console.error("missing test runner environment");
	process.exit(1);
}

const { importObject } = await import(pathToFileURL(importsPath));
const wasmBytes = await fs.readFile(wasmPath);
const tests = JSON.parse(testsJson);

const baseLog = console.log;
const baseError = console.error;
const formatter = createTextFormatter({
	nocapture,
	write(line, stream) {
		if (stream === "stderr") {
			baseError(line);
		} else {
			baseLog(line);
		}
	},
});

function emit(event) {
	formatter.onEvent(event);
}

const testInputs = tests.map(test => ({
	...test,
	run(testFn) {
		return withConsoleCapture(test.name, () => testFn());
	},
}));

const result = await runTests({
	wasmBytes,
	importObject,
	tests: testInputs,
	filtered,
	emit,
});

process.exit(result.failed === 0 ? 0 : 1);

function withConsoleCapture(name, run) {
	function emitOutput(line, stream, level) {
		emit({ type: "test-output", name, line, stream, level });
	}

	const originalLog = console.log;
	const originalError = console.error;
	const originalWarn = console.warn;
	const originalInfo = console.info;
	const originalDebug = console.debug;
	console.log = (...args) => emitOutput(args.join(" "), "stdout", "log");
	console.error = (...args) => emitOutput(args.join(" "), "stderr", "error");
	console.warn = (...args) => emitOutput(args.join(" "), "stderr", "warn");
	console.info = (...args) => emitOutput(args.join(" "), "stdout", "info");
	console.debug = (...args) => emitOutput(args.join(" "), "stdout", "debug");

	try {
		run();
		return { ok: true };
	} catch (error) {
		return {
			ok: false,
			stack: error.stack
		};
	} finally {
		console.log = originalLog;
		console.error = originalError;
		console.warn = originalWarn;
		console.info = originalInfo;
		console.debug = originalDebug;
	}
}
