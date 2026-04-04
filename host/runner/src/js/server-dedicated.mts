declare var self: DedicatedWorkerGlobalScope

import { runTests } from "./shared.mts"
import { importJsBindgen } from "./shared-import.mts"

const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"))
const jsBindgenCtor = await importJsBindgen()

if (jsBindgenCtor instanceof Error) {
	self.postMessage(jsBindgenCtor.message + "\n")
} else {
	await runTests(module, jsBindgenCtor, (_, text) => {
		self.postMessage(text)
	})
}

self.close()
