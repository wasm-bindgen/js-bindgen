/// <reference lib="webworker" />
declare var self: SharedWorkerGlobalScope

import { runTests } from "./shared.mts"

self.addEventListener("connect", async event => {
	const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"))
	const port = event.ports[0]!

	const result = await runTests(module, (_, text) => {
		port.postMessage(text)
	})

	if (typeof result.benchBaseline === "string") {
		const response = await fetch("./benchmark-baseline", {
			method: "POST",
			headers: { "Content-Type": "text/plain" },
			body: result.benchBaseline,
		})

		if (!response.ok) {
			throw new Error("failed to upload benchmark baseline")
		}
	}

	self.close()
})
