/// <reference lib="webworker" />
declare var self: DedicatedWorkerGlobalScope

import { runTests } from "./shared.mts"

const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"))
await runTests(module, (_, text) => {
	self.postMessage(text)
})

self.close()
