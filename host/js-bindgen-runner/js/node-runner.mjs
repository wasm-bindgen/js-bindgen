import fs from "node:fs/promises";
import { pathToFileURL } from "node:url";
import { createTextFormatter } from "./shared.mjs";
import { runTests } from "./runner-core.mjs";
import consoleHook, { withConsoleCapture } from "./console-hook.mjs";

const wasmPath = process.env.JS_BINDGEN_WASM;
const importsPath = process.env.JS_BINDGEN_IMPORTS;
const testsPath = process.env.JS_BINDGEN_TESTS_PATH;
const nocapture = process.env.JS_BINDGEN_NOCAPTURE === "1";
const filtered = Number(process.env.JS_BINDGEN_FILTERED || "0");

if (!wasmPath || !importsPath || !testsPath) {
	console.error("missing test runner environment");
	process.exit(1);
}

const { importObject } = await import(pathToFileURL(importsPath));
const wasmBytes = await fs.readFile(wasmPath);
const testsText = await fs.readFile(testsPath, "utf8");
const tests = JSON.parse(testsText);

const baseLog = consoleHook.base.log;
const baseError = consoleHook.base.error;
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
		return withConsoleCapture({
			name: test.name,
			run: () => testFn(),
			emit: event => formatter.onEvent(event),
			consoleHook,
			forwardToConsole: false,
		});
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
