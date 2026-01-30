export async function runTests({ wasmBytes, importObject, tests, filtered, emit }) {
	const startTime = Date.now()
	emit({ type: "run-start", total: tests.length, filtered })

	let failed = 0
	let ignored = 0

	for (const test of tests) {
		const { instance } = await WebAssembly.instantiate(wasmBytes, importObject)
		const panicPayload = instance.exports.last_panic_payload
		const panicMessage = instance.exports.last_panic_message
		const externrefTable = resolveExternrefTable(importObject)

		if (test.ignore) {
			ignored += 1
			emit({
				type: "test-ignored",
				name: test.name,
				reason: typeof test.ignore == "string" ? test.ignore : undefined,
			})
			continue
		}

		const testFn = instance.exports[test.name]
		if (typeof testFn !== "function") {
			emit({
				type: "test-failed",
				name: test.name,
				error: `missing export: ${test.name}`,
			})
			failed += 1
			continue
		}

		const result = test.run(testFn)
		const shouldPanic = test.shouldPanic
		if (shouldPanic) {
			if (result.ok) {
				emit({
					type: "test-failed",
					name: test.name,
					error: "test did not panic as expected",
					shouldPanic: true,
				})
				failed += 1
				continue
			}

			const expectedText = typeof test.shouldPanic == "string" ? test.shouldPanic : undefined
			const payload = coercePanicMessage(panicPayload(), externrefTable)
			const message = coercePanicMessage(panicMessage(), externrefTable)

			if (expectedText && !payload.includes(expectedText)) {
				const displayPayload = escapeForDisplay(payload)
				const displayExpected = escapeForDisplay(expectedText)
				emit({
					type: "test-failed",
					name: test.name,
					error:
						message +
						"\n" +
						result.stack +
						"\n" +
						"note: panic did not contain expected string\n" +
						`      panic message: "${displayPayload}"\n` +
						` expected substring: "${displayExpected}"`,
					shouldPanic: true,
				})
				failed += 1
				continue
			}

			emit({ type: "test-ok", name: test.name, shouldPanic: true })
			continue
		}

		if (result.ok) {
			emit({ type: "test-ok", name: test.name, shouldPanic: false })
		} else {
			const message = coercePanicMessage(panicMessage(), externrefTable)
			emit({ type: "test-failed", name: test.name, error: message + "\n" + result.stack })
			failed += 1
		}
	}

	emit({
		type: "run-end",
		status: failed === 0 ? "ok" : "FAILED",
		passed: tests.length - failed - ignored,
		failed,
		ignored,
		filtered,
		duration_ms: Date.now() - startTime,
	})

	return { failed }
}

function resolveExternrefTable(importObject) {
	return importObject["js_sys"]["externref.table"]
}

function coercePanicMessage(value, externrefTable) {
	const ref = externrefTable.get(value)
	externrefTable.set(value, null)
	return String(ref)
}

function escapeForDisplay(value) {
	return String(value || "")
		.replace(/\r/g, "\\r")
		.replace(/\n/g, "\\n")
		.replace(/\t/g, "\\t")
}
