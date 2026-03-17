declare var self: ServiceWorkerGlobalScope

import { runTests } from "./shared.mts"
import { JsBindgen } from "./imports.mts"

self.addEventListener("message", async event => {
	const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"))
	const [port] = event.ports

	await runTests(module, JsBindgen, (_, text) => {
		self.clients.get
		port!.postMessage(text)
	})

	self.registration.unregister()
})
