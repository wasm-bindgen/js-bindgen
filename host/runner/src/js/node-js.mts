import { open } from "node:fs/promises"
import { Stream, runTests } from "./shared.mts"
import { colorText } from "./shared-terminal.mts"

const wasmFile = await open(new URL("./wasm.wasm", import.meta.url))
const wasmResponse = new Response(wasmFile.createReadStream() as any, {
	headers: { "Content-Type": "application/wasm" },
})
const module = await WebAssembly.compileStreaming(wasmResponse)

const success = await runTests(module, (stream, text) => {
	const output = colorText(text)

	switch (stream) {
		case Stream.Stdout:
			process.stdout.write(output)
			break
		case Stream.Stderr:
			process.stderr.write(output)
	}
})

process.exit(success ? 0 : 101)
