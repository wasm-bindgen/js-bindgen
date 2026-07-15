import { Stream, run } from "../shared/shared.mjs"
import { colorText } from "../shared/shared-terminal.mjs"
import { JsBindgen } from "../imports.mts"

const wasmFile = Bun.file(new URL("../wasm.wasm", import.meta.url))
const wasmResponse = new Response(wasmFile, {
	headers: { "Content-Type": "application/wasm" },
})
const module = await WebAssembly.compileStreaming(wasmResponse)

let pendingWrite = Promise.resolve()
const status = await run(module, JsBindgen, (stream, text) => {
	const output = colorText(text)
	const destination = stream === Stream.Stdout ? Bun.stdout : Bun.stderr

	pendingWrite = pendingWrite.then(async () => {
		await Bun.write(destination, output)
	})
})

await pendingWrite
process.exit(status)
