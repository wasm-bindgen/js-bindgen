import testData from "../test-data.json" with { type: "json" }
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
	Ok,
	Failed,
	Abnormal,
}

export async function runTests(
	module: WebAssembly.Module,
	jsBindgenCtor: typeof JsBindgen,
	report: (stream: Stream, text: StyledText[]) => void
): Promise<Status> {
	let interceptFlag = false
	const interceptStore: string[] = []

	const CONSOLE_METHODS = ["debug", "log", "info", "warn", "error"] as const
	CONSOLE_METHODS.forEach(level => {
		const origin = console[level].bind(console)

		console[level] =
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			((...data: any[]) => {
				if (interceptFlag) {
					const stream = level === "error" || level === "warn" ? Stream.Stderr : Stream.Stdout
					const text = data.join(" ") + "\n"

					if (testData.noCapture) {
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

	const startTime = performance.now()
	report(Stream.Stdout, [
		{
			text: `\nrunning ${testData.tests.length} tests\n`,
			color: Color.Default,
		},
	])

	const failures: { name: string; error: string }[] = []
	let ignored = 0

	const newLineText = { text: "\n", color: Color.Default }
	const failedText = { text: "FAILED", color: Color.Red }
	const okText = { text: "ok", color: Color.Green }

	for (const test of testData.tests) {
		interceptStore.length = 0
		let panicMessage: string | undefined
		let panicPayload: string | undefined

		let jsBindgen
		try {
			jsBindgen = new jsBindgenCtor(module)
		} catch (error) {
			report(Stream.Stderr, [{ text: (error as Error).message, color: Color.Default }, newLineText])
			return Status.Abnormal
		}
		jsBindgen.extendImportObject({
			js_bindgen_test: {
				set_message: (message: string) => (panicMessage = message),
				set_payload: (payload: string) => (panicPayload = payload),
			},
		})
		const instance = await jsBindgen.instantiate()

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

		const testFn = instance.exports[test.importName] as () => void
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
			result = { success: false, stack: (error as Error).stack!, message: panicMessage! }
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
				typeof panicPayload === "string" &&
				!panicPayload.includes(test.shouldPanic)
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
						`      panic message: "${panicPayload}"\n` +
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

	const success = failures.length === 0 ? Status.Ok : Status.Failed
	const result = success === Status.Ok ? okText : failedText
	const passed = testData.tests.length - failures.length - ignored
	const durationMs = performance.now() - startTime
	const durationSecs = (durationMs / 1000).toFixed(2)
	output1 += "test result: "
	const output2 = `. ${passed} passed; ${failures.length} failed; ${ignored} ignored; 0 measured; ${testData.filteredCount} filtered out; finished in ${durationSecs}s\n\n`
	report(Stream.Stdout, [
		{ text: output1, color: Color.Default },
		result,
		{ text: output2, color: Color.Default },
	])

	return success
}
