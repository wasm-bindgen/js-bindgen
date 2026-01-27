import { createTextFormatter, installConsoleProxy } from "./shared.mjs";
import { runTests } from "./runner-core.mjs";

async function execute(port, { nocapture, filtered }) {
	const consoleProxy = installConsoleProxy();
	const tests = await (await fetch("/tests.json")).json();
	const wasmBytes = await (await fetch("/wasm")).arrayBuffer();
	const { importObject } = await import("/import.js");

	const lines = [];
	const formatter = createTextFormatter({
		nocapture,
		write(line) {
			lines.push(line);
			port.postMessage({ type: "line", line });
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
		run(testFn) {
			return withConsoleCapture(test.name, () => testFn(), event =>
				emit(event),
				consoleProxy
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

if (typeof self.onconnect !== "undefined") {
	self.onconnect = event => {
		const port = event.ports[0];
		port.onmessage = msg => {
			execute(port, msg.data).catch(error => {
				port.postMessage({ type: "report", lines: [String(error)], failed: 1 });
			});
		};
		port.start();
	};
} else {
	self.onmessage = event => {
		execute(self, event.data).catch(error => {
			self.postMessage({ type: "report", lines: [String(error)], failed: 1 });
		});
	};
}

function withConsoleCapture(name, run, emit, consoleProxy) {
	consoleProxy.setHook((level, args) => {
		const line = args.join(" ");
		const stream = level === "error" || level === "warn" ? "stderr" : "stdout";
		emit({ type: "test-output", name, line, stream, level });
	}, false);

	try {
		run();
		return { ok: true };
	} catch (error) {
		return {
			ok: false,
			stack: error.stack
		};
	} finally {
		consoleProxy.clearHook();
	}
}
