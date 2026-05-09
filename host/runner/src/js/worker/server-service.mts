// eslint-disable-next-line no-var
declare var self: ServiceWorkerGlobalScope

import { createBrowserFsBackend, run } from "../shared/shared.mjs"
import { JsBindgen } from "../imports.mts"

self.addEventListener("message", async event => {
	const module = await WebAssembly.compileStreaming(fetch("../wasm.wasm"))
	const [port] = event.ports
	const fs = createBrowserFsBackend()

	await run(module, JsBindgen, (_, text) => port!.postMessage(text), fs)
	await fs.flush()

	await self.registration.unregister()
})
