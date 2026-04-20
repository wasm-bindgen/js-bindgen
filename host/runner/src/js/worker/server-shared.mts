// eslint-disable-next-line no-var
declare var self: SharedWorkerGlobalScope

import { runTests } from "../shared/shared.mjs"
import { importJsBindgen } from "../shared/shared-import.mjs"

self.addEventListener("connect", async event => {
	const module = await WebAssembly.compileStreaming(fetch("../wasm.wasm"))
	const port = event.ports[0]!

	const jsBindgenCtor = await importJsBindgen()

	if (jsBindgenCtor instanceof Error) {
		port.postMessage(jsBindgenCtor.message + "\n")
	} else {
		await runTests(module, jsBindgenCtor, (_, text) => {
			port.postMessage(text)
		})
	}

	self.close()
})
