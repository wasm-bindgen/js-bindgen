// eslint-disable-next-line no-var
declare var self: ServiceWorkerGlobalScope

import { runTests } from "../shared/shared.mjs"
import { JsBindgen } from "../imports.mts"

self.addEventListener("message", async event => {
	const module = await WebAssembly.compileStreaming(fetch("../wasm.wasm"))
	const [port] = event.ports

	await runTests(module, JsBindgen, (_, text) => port!.postMessage(text))

	await self.registration.unregister()
})
