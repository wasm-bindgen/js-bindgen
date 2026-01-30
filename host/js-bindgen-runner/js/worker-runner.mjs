import { createTextFormatter } from "./shared.mjs"
import { runTests } from "./runner-core.mjs"
import consoleHook, { withConsoleCapture } from "./console-hook.mjs"

async function execute(port, { noCapture, filtered }) {
	const tests = await (await fetch("/tests.json")).json()
	const wasmBytes = await (await fetch("/wasm")).arrayBuffer()
	const { importObject } = await import("/import.js")

	const lines = []
	const formatter = createTextFormatter({
		noCapture,
		write(line) {
			lines.push(line)
			port.postMessage({ type: "line", line })
		},
	})

	function emit(event) {
		if (event.type === "test-output") {
			port.postMessage({
				type: "user-output",
				line: event.line,
				stream: event.stream,
				level: event.level || (event.stream === "stderr" ? "error" : "log"),
			})
		}
		formatter.onEvent(event)
	}

	const testInputs = tests.map(test => ({
		...test,
		run(testFn) {
			return withConsoleCapture({
				name: test.name,
				run: () => testFn(),
				emit,
				consoleHook,
				forwardToConsole: false,
			})
		},
	}))

	const result = await runTests({
		wasmBytes,
		importObject,
		tests: testInputs,
		filtered,
		emit,
	})

	port.postMessage({ type: "report", lines, failed: result.failed })
}

if (typeof self.onconnect !== "undefined") {
	self.onconnect = event => {
		const port = event.ports[0]
		port.onmessage = msg => {
			execute(port, msg.data).catch(error => {
				port.postMessage({ type: "report", lines: [String(error)], failed: 1 })
			})
		}
		port.start()
	}
} else {
	self.onmessage = event => {
		execute(self, event.data).catch(error => {
			self.postMessage({ type: "report", lines: [String(error)], failed: 1 })
		})
	}
}
