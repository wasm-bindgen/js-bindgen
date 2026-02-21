import { Stream, runTests } from "./shared.mts"
import { colorText } from "./shared-terminal.mts"

const enum Status {
	Ok,
	Failed,
	Abnormal,
}

export async function runBrowser() {
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

	const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"))
	const success = await runTests(module, (stream, text) => report(stream, colorText(text)))

	let status = success ? Status.Ok : Status.Failed

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
