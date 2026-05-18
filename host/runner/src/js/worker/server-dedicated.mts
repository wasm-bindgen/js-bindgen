// eslint-disable-next-line no-var
declare var self: DedicatedWorkerGlobalScope

import { createBrowserFsBackend, run } from "../shared/shared.mjs"
import { importJsBindgen } from "../shared/shared-import.mjs"

const module = await WebAssembly.compileStreaming(fetch("../wasm.wasm"))
const jsBindgenCtor = await importJsBindgen()

if (jsBindgenCtor instanceof Error) {
	self.postMessage(jsBindgenCtor.message + "\n")
} else {
	const fs = createBrowserFsBackend()
	await run(
		module,
		jsBindgenCtor,
		(_, text) => {
			self.postMessage(text)
		},
		fs
	)
	await fs.flush()
}

self.close()
