export async function runTests({ wasmBytes, importObject, tests, filtered, emit }) {
	const startTime = Date.now();
	emit({ type: "run-start", total: tests.length, filtered });

	const { instance } = await WebAssembly.instantiate(wasmBytes, importObject);
	const panicPayload = instance.exports.last_panic_payload;
	const panicMessage = instance.exports.last_panic_message;
	const externrefTable = resolveExternrefTable(importObject);

	let failed = 0;
	let ignored = 0;

	for (const test of tests) {
		if (test.ignore) {
			ignored += 1;
			emit({ type: "test-ignored", name: test.name, reason: test.ignore_reason });
			continue;
		}

		const testFn = instance.exports[test.name];
		if (typeof testFn !== "function") {
			emit({
				type: "test-failed",
				name: test.name,
				error: `missing export: ${test.name}`,
			});
			failed += 1;
			continue;
		}

		const result = test.run(testFn, panicPayload, panicMessage);
		const shouldPanic = Boolean(test.should_panic);
		if (shouldPanic) {
			if (result.ok) {
				emit({
					type: "test-failed",
					name: test.name,
					error: "test did not panic as expected",
					should_panic: true,
				});
				failed += 1;
				continue;
			}

			const expected = test.should_panic_reason;
			const payload = coercePanicMessage(result.panic_payload, externrefTable);
			const message = coercePanicMessage(result.panic_message, externrefTable);
			const expectedText = expected || "";
			if (expectedText && !payload.includes(expectedText)) {
				const displayPayload = escapeForDisplay(payload);
				const displayExpected = escapeForDisplay(expectedText);
				emit({
					type: "test-failed",
					name: test.name,
					error:
                        message + "\n" +
						"note: panic did not contain expected string\n" +
						`      panic message: "${displayPayload}"\n` +
						` expected substring: "${displayExpected}"`,
					should_panic: true,
				});
				failed += 1;
				continue;
			}

			emit({ type: "test-ok", name: test.name, should_panic: true });
			continue;
		}

		if (result.ok) {
			emit({ type: "test-ok", name: test.name, should_panic: false });
		} else {
			const message = coercePanicMessage(result.panic_message, externrefTable);
			emit({ type: "test-failed", name: test.name, error: message });
			failed += 1;
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
	});

	return { failed };
}

function resolveExternrefTable(importObject) {
	if (!importObject) {
		return null;
	}
	return importObject["js_sys"]["externref.table"];
}

function coercePanicMessage(value, externrefTable) {
	const ref = externrefTable.get(value);
	if (typeof externrefTable.set === "function") {
		externrefTable.set(value, null);
	}
	return String(ref);
}

function escapeForDisplay(value) {
	return String(value || "")
		.replace(/\\/g, "\\\\")
		.replace(/\r/g, "\\r")
		.replace(/\n/g, "\\n")
		.replace(/\t/g, "\\t")
		.replace(/\0/g, "\\0")
		.replace(/\x08/g, "\\b")
		.replace(/\x0c/g, "\\f");
}
