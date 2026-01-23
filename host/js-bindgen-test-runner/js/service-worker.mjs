import { createTextFormatter } from "./shared.mjs";
import { runTests } from "./runner-core.mjs";
import { importObject } from "/import.js";

self.addEventListener("message", event => {
	const port = event.ports && event.ports[0];
	if (!port) {
		return;
	}
	execute(port, event.data).catch(error => {
		port.postMessage({ type: "report", lines: [String(error)], failed: 1 });
	});
});

async function execute(port, { nocapture, filtered }) {
	const tests = await (await fetch("/tests.json")).json();
	const wasmBytes = await (await fetch("/wasm")).arrayBuffer();
	const lines = [];
	const formatter = createTextFormatter({
		nocapture,
		write(line) {
			lines.push(line);
			if (nocapture) {
				port.postMessage({ type: "line", line });
			}
		},
	});

	function emit(event) {
		if (event.type === "test-output") {
			port.postMessage({
				type: "user-output",
				line: event.line,
				stream: event.stream,
				level: event.level || (event.stream === "stderr" ? "error" : "log"),
			});
		}
		formatter.onEvent(event);
	}

	const testInputs = tests.map(test => ({
		...test,
		run(testFn, panicPayload, panicMessage) {
			return withConsoleCapture(test.name, () => testFn(), panicPayload, panicMessage, event =>
				emit(event)
			);
		},
	}));

	const result = await runTests({
		wasmBytes,
		importObject,
		tests: testInputs,
		filtered,
		emit,
	});

	port.postMessage({ type: "report", lines, failed: result.failed });
}

function withConsoleCapture(name, run, panicPayload, panicMessage, emit) {
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
		return { ok: false, panic_payload: panicPayload(), panic_message: panicMessage() };
	} finally {
		console.log = originalLog;
		console.error = originalError;
		console.warn = originalWarn;
		console.info = originalInfo;
		console.debug = originalDebug;
	}
}
