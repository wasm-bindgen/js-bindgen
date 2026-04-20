import { Color, runTests } from "../shared/shared.mjs"
import { toOutput } from "./shared-server.mts"
import { importJsBindgen } from "../shared/shared-import.mjs"

const module = await WebAssembly.compileStreaming(fetch("../wasm.wasm"))
const jsBindgenCtor = await importJsBindgen()

if (jsBindgenCtor instanceof Error) {
	toOutput([{ text: jsBindgenCtor.message + "\n", color: Color.Default }])
} else {
	await runTests(module, jsBindgenCtor, (_, text) => {
		toOutput(text)
	})
}
