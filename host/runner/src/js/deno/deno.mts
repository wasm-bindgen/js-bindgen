import { type FsBackend, run, Stream } from "../shared/shared.mjs"
import { colorText } from "../shared/shared-terminal.mjs"
import { JsBindgen } from "../imports.mts"

const fs: FsBackend = {
	writeFile(path, data) {
		Deno.writeFileSync(path, data)
	},
}

const module = await WebAssembly.compileStreaming(fetch(new URL("../wasm.wasm", import.meta.url)))

const status = await run(
	module,
	JsBindgen,
	(stream, text) => {
		function printSync(input: string, to: typeof Deno.stdout | typeof Deno.stderr) {
			let bytesWritten = 0
			const bytes = new TextEncoder().encode(input)

			while (bytesWritten < bytes.length) {
				bytesWritten += to.writeSync(bytes.subarray(bytesWritten))
			}
		}

		const output = colorText(text)

		switch (stream) {
			case Stream.Stdout:
				printSync(output, Deno.stdout)
				break
			case Stream.Stderr:
				printSync(output, Deno.stderr)
		}
	},
	fs
)

Deno.exit(status)
