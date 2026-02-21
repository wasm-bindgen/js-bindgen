import { runTests, Stream } from "./shared.mts"
import { colorText } from "./shared-terminal.mts"

const module = await WebAssembly.compileStreaming(fetch(new URL("./wasm.wasm", import.meta.url)))

const success = await runTests(module, (stream, text) => {
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
})

Deno.exit(success ? 0 : 101)
