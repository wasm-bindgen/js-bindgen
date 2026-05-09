// eslint-disable-next-line no-var
declare var self: SharedWorkerGlobalScope

import { run } from "../shared/shared.mjs"
import { importJsBindgen } from "../shared/shared-import.mjs"

self.addEventListener("connect", async event => {
	const module = await WebAssembly.compileStreaming(fetch("../wasm.wasm"))
	const port = event.ports[0]!

	const jsBindgenCtor = await importJsBindgen()

	if (jsBindgenCtor instanceof Error) {
		port.postMessage(jsBindgenCtor.message + "\n")
	} else {
		await run(module, jsBindgenCtor, (_, text) => {
			port.postMessage(text)
		})
	}

	self.close()
})
