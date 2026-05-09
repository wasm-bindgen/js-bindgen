import runData from "../run-data.json" with { type: "json" }
import type { JsBindgen } from "../imports.mts"

export const enum Stream {
	Stdout,
	Stderr,
}

export type StyledText = { text: string; color: Color }

export const enum Color {
	Default,
	Green,
	Yellow,
	Red,
}

export const enum Status {
	Ok = 0,
    // See https://github.com/rust-lang/cargo/blob/0.95.0/src/cargo/ops/cargo_test.rs#L421.
	Failed = 101,
	Abnormal = 1,
}

export async function run(
	module: WebAssembly.Module,
	jsBindgenCtor: typeof JsBindgen,
	report: (stream: Stream, text: StyledText[]) => void
): Promise<number> {
	let interceptFlag = false
	const interceptStore: string[] = []
	const newLineText = { text: "\n", color: Color.Default }
	const failedText = { text: "FAILED", color: Color.Red }
	const okText = { text: "ok", color: Color.Green }

	const CONSOLE_METHODS = ["debug", "log", "info", "warn", "error"] as const
	CONSOLE_METHODS.forEach(level => {
		const origin = console[level].bind(console)

		console[level] =
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			((...data: any[]) => {
				if (interceptFlag) {
					const stream = level === "error" || level === "warn" ? Stream.Stderr : Stream.Stdout
					const text = data.join(" ") + "\n"

					if (runData.kind === "binary" || runData.noCapture) {
						report(stream, [{ text, color: Color.Default }])
					} else {
						interceptStore.push(text)
					}
				} else {
					// eslint-disable-next-line @typescript-eslint/no-unsafe-argument
					origin(...data)
				}
			}).bind(console)
	})

	async function instantiate() {
		let panicMessage: string | undefined
		let panicPayload: string | undefined
		let jsBindgen

		try {
			jsBindgen = new jsBindgenCtor(module)
		} catch (error) {
			report(Stream.Stderr, [{ text: (error as Error).message, color: Color.Default }, newLineText])
			return
		}

		jsBindgen.extendImportObject({
			js_bindgen_test: {
				set_message: (message: string) => (panicMessage = message),
				set_payload: (payload: string) => (panicPayload = payload),
			},
		})
		const instance = await jsBindgen.instantiate()

		return { instance, panicMessage, panicPayload }
	}

	if (runData.kind === "binary") {
		const state = await instantiate()

		if (!state) {
			return Status.Abnormal
		}

		interceptFlag = true
		let status: number

		try {
			if (runData.wasm64) {
				const main = state.instance.exports["main"] as (argc: number, argv: bigint) => number
				status = main(0, 0n)
			} else {
				const main = state.instance.exports["main"] as (argc: number, argv: number) => number
				status = main(0, 0)
			}
		} catch (error) {
			const message = state.panicMessage ?? (error as Error).message
			const stack = (error as Error).stack!
			report(Stream.Stderr, [{ text: message + "\n" + stack + "\n", color: Color.Default }])

			status = Status.Failed
		} finally {
			interceptFlag = false
		}

		return status
	}

	const startTime = performance.now()
	report(Stream.Stdout, [
		{
			text: `\nrunning ${runData.tests.length} tests\n`,
			color: Color.Default,
		},
	])

	const failures: { name: string; error: string }[] = []
	let ignored = 0

	for (const test of runData.tests) {
		interceptStore.length = 0
		const state = await instantiate()

		if (!state) {
			return Status.Abnormal
		}

		const testText = { text: `test ${test.name} ... `, color: Color.Default }

		if (test.ignore) {
			ignored += 1

			if (typeof test.ignore === "string") {
				report(Stream.Stdout, [
					testText,
					{ text: `ignored, ${test.ignore}`, color: Color.Yellow },
					newLineText,
				])
			} else {
				report(Stream.Stdout, [testText, { text: "ignored", color: Color.Yellow }, newLineText])
			}

			continue
		}

		const testFn = state.instance.exports[test.importName] as () => void
		let result: { success: true } | { success: false; stack: string; message: string }

		if (test.shouldPanic) {
			report(Stream.Stdout, [
				{ text: `test ${test.name} - should panic ... `, color: Color.Default },
			])
		} else {
			report(Stream.Stdout, [testText])
		}

		interceptFlag = true

		try {
			testFn()
			result = { success: true }
		} catch (error) {
			result = {
				success: false,
				stack: (error as Error).stack!,
				message: state.panicMessage ?? (error as Error).message,
			}
		}

		interceptFlag = false

		if (test.shouldPanic) {
			if (result.success) {
				report(Stream.Stdout, [failedText, newLineText])
				const stdout = interceptStore.join("")

				failures.push({ name: test.name, error: stdout + "note: test did not panic as expected" })
				continue
			}

			if (
				typeof test.shouldPanic === "string" &&
				typeof state.panicPayload === "string" &&
				!state.panicPayload.includes(test.shouldPanic)
			) {
				report(Stream.Stdout, [failedText, newLineText])
				let stdout = interceptStore.join("")

				if (stdout.length !== 0) {
					stdout += "\n"
				}

				failures.push({
					name: test.name,
					error:
						stdout +
						result.message +
						"\n" +
						result.stack +
						"\n" +
						"note: panic did not contain expected string\n" +
						`      panic message: "${state.panicPayload}"\n` +
						` expected substring: "${test.shouldPanic}"`,
				})
				continue
			}

			report(Stream.Stdout, [okText, newLineText])
			continue
		} else if (result.success) {
			report(Stream.Stdout, [okText, newLineText])
		} else {
			report(Stream.Stdout, [failedText, newLineText])
			let stdout = interceptStore.join("")

			if (stdout.length !== 0) {
				stdout += "\n"
			}

			failures.push({
				name: test.name,
				error: stdout + result.message + "\n" + result.stack,
			})
		}
	}

	let output1 = "\n"

	if (failures.length > 0) {
		output1 += "failures:\n\n"

		for (const failure of failures) {
			output1 += `---- ${failure.name} stdout ----\n` + failure.error + "\n"
		}

		output1 += "\nfailures:\n"

		for (const failure of failures) {
			output1 += `    ${failure.name}\n`
		}

		output1 += "\n"
	}

	const status = failures.length === 0 ? Status.Ok : Status.Failed
	const result = status === Status.Ok ? okText : failedText
	const passed = runData.tests.length - failures.length - ignored
	const durationMs = performance.now() - startTime
	const durationSecs = (durationMs / 1000).toFixed(2)
	output1 += "test result: "
	const output2 = `. ${passed} passed; ${failures.length} failed; ${ignored} ignored; 0 measured; ${runData.filteredCount} filtered out; finished in ${durationSecs}s\n\n`
	report(Stream.Stdout, [
		{ text: output1, color: Color.Default },
		result,
		{ text: output2, color: Color.Default },
	])

	return status
}
