import { open, writeFile } from "node:fs/promises"
import testData from "./test-data.json" with { type: "json" }
import { Stream, runTests } from "./shared.mts"
import { colorText } from "./shared-terminal.mts"

const wasmFile = await open(new URL("./wasm.wasm", import.meta.url))
const wasmResponse = new Response(wasmFile.createReadStream() as any, {
	headers: { "Content-Type": "application/wasm" },
})
const module = await WebAssembly.compileStreaming(wasmResponse)

const result = await runTests(module, (stream, text) => {
	const output = colorText(text)

	switch (stream) {
		case Stream.Stdout:
			process.stdout.write(output)
			break
		case Stream.Stderr:
			process.stderr.write(output)
	}
})

if (typeof result.benchBaseline === "string" && testData.benchBaseline) {
    const path = testData.benchBaseline.path;
	await writeFile(path, result.benchBaseline)
}

process.exit(result.success ? 0 : 101)
