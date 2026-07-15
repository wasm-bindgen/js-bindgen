import { open } from "node:fs/promises"
import { writeFileSync } from "node:fs"
import { Stream, type FsBackend, run } from "../shared/shared.mjs"
import { colorText } from "../shared/shared-terminal.mjs"
import { JsBindgen } from "../imports.mts"

const fs: FsBackend = {
	writeFile(path, data) {
		writeFileSync(path, data)
	},
}

const wasmFile = await open(new URL("../wasm.wasm", import.meta.url))
const wasmResponse = new Response(
	// eslint-disable-next-line @typescript-eslint/no-unsafe-argument, @typescript-eslint/no-explicit-any
	wasmFile.createReadStream() as any,
	{
		headers: { "Content-Type": "application/wasm" },
	}
)
const module = await WebAssembly.compileStreaming(wasmResponse)

const status = await run(
	module,
	JsBindgen,
	(stream, text) => {
		const output = colorText(text)

		switch (stream) {
			case Stream.Stdout:
				process.stdout.write(output)
				break
			case Stream.Stderr:
				process.stderr.write(output)
		}
	},
	fs
)

process.exit(status)
