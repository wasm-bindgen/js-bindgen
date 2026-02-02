/// <reference lib="webworker" />
declare var self: SharedWorkerGlobalScope

import { runTests } from "./shared.mts"

self.addEventListener("connect", async event => {
	const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"))
	const port = event.ports[0]!

	await runTests(module, (_, text) => {
		port.postMessage(text)
	})

	self.close()
})
