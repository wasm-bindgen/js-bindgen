import { runTests } from "./shared.mts"
import { toOutput } from "./shared-server.mts"

const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"))
await runTests(module, (_, text) => {
	toOutput(text)
})
