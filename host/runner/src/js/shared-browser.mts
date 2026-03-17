import { Stream, Status, runTests } from "./shared.mts"
import { colorText } from "./shared-terminal.mts"
import type { JsBindgen } from "./imports.mts"

export async function runBrowser(jsBindgenCtor: typeof JsBindgen | Error) {
	let fetchOrder = 0
	let fetchRunning = 0
	let fetchError = false
	let fetchWaiting = false
	let fetchResolve: () => void
	const fetchWaiter = new Promise<void>(resolve => {
		fetchResolve = resolve
	})

	function report(stream: Stream, text: string) {
		fetchRunning++

		fetch("./report", {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({ order: fetchOrder++, stream, line: text }),
		})
			.then(result => {
				if (!result.ok) {
					throw result
				}
			})
			.catch(error => {
				fetchError = true
				throw error
			})
			.finally(() => {
				fetchRunning--

				if (fetchWaiting && fetchRunning === 0) {
					fetchResolve()
				}
			})
	}

	let status

	if (jsBindgenCtor instanceof Error) {
		report(Stream.Stderr, jsBindgenCtor.message + "\n")
		status = Status.Abnormal
	} else {
		status = await WebAssembly.compileStreaming(fetch("./wasm.wasm")).then(
			module => runTests(module, jsBindgenCtor, (stream, text) => report(stream, colorText(text))),
			error => {
				report(Stream.Stderr, (error as Error).message + "\n")

				return Status.Abnormal
			}
		)
	}

	if (fetchRunning !== 0) {
		fetchWaiting = true
		await fetchWaiter
	}

	if (fetchError) {
		status = Status.Abnormal
	}

	await fetch("/finished", {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(status),
	})
}
