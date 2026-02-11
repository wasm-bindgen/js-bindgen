import testData from "./test-data.json" with { type: "json" };
import { JsBindgen } from "./imports.mjs";
export async function runTests(module, report) {
    let interceptFlag = false;
    const interceptStore = [];
    const CONSOLE_METHODS = ["debug", "log", "info", "warn", "error"];
    CONSOLE_METHODS.forEach(level => {
        const origin = console[level].bind(console);
        console[level] = ((...data) => {
            if (interceptFlag) {
                const stream = level === "error" || level === "warn" ? 1 /* Stream.Stderr */ : 0 /* Stream.Stdout */;
                const text = data.join(" ") + "\n";
                if (testData.noCapture) {
                    report(stream, [{ text, color: 0 /* Color.Default */ }]);
                }
                else {
                    interceptStore.push(text);
                }
            }
            else {
                origin(...data);
            }
        }).bind(console);
        return [level, origin];
    });
    const startTime = performance.now();
    report(0 /* Stream.Stdout */, [
        {
            text: `\nrunning ${testData.tests.length} tests\n`,
            color: 0 /* Color.Default */,
        },
    ]);
    let failures = [];
    let ignored = 0;
    let panicPayload;
    let panicMessage;
    const newLineText = { text: "\n", color: 0 /* Color.Default */ };
    const failedText = { text: "FAILED", color: 3 /* Color.Red */ };
    const okText = { text: "ok", color: 1 /* Color.Green */ };
    for (const test of testData.tests) {
        interceptStore.length = 0;
        panicPayload = undefined;
        panicMessage = undefined;
        const jsBindgen = new JsBindgen(module);
        jsBindgen.extendImportObject({
            js_bindgen_test: {
                set_payload: (payload) => (panicPayload = payload),
                set_message: (message) => (panicMessage = message),
            },
        });
        const instance = await jsBindgen.instantiate();
        const testText = { text: `test ${test.name} ... `, color: 0 /* Color.Default */ };
        if (test.ignore) {
            ignored += 1;
            if (typeof test.ignore === "string") {
                report(0 /* Stream.Stdout */, [
                    testText,
                    { text: `ignored, ${test.ignore}`, color: 2 /* Color.Yellow */ },
                    newLineText,
                ]);
            }
            else {
                report(0 /* Stream.Stdout */, [testText, { text: `ignored`, color: 2 /* Color.Yellow */ }, newLineText]);
            }
            continue;
        }
        const testFn = instance.exports[test.importName];
        let result;
        if (test.shouldPanic) {
            report(0 /* Stream.Stdout */, [
                { text: `test ${test.name} - should panic ... `, color: 0 /* Color.Default */ },
            ]);
        }
        else {
            report(0 /* Stream.Stdout */, [testText]);
        }
        interceptFlag = true;
        try {
            testFn();
            result = { success: true };
        }
        catch (error) {
            result = { success: false, stack: error.stack };
        }
        interceptFlag = false;
        if (test.shouldPanic) {
            if (result.success) {
                report(0 /* Stream.Stdout */, [failedText, newLineText]);
                let stdout = interceptStore.join("");
                failures.push({ name: test.name, error: stdout + "note: test did not panic as expected" });
                continue;
            }
            if (typeof test.shouldPanic == "string" &&
                typeof panicPayload !== "undefined" &&
                !panicPayload.includes(test.shouldPanic)) {
                report(0 /* Stream.Stdout */, [failedText, newLineText]);
                let stdout = interceptStore.join("");
                if (stdout.length !== 0) {
                    stdout += "\n";
                }
                failures.push({
                    name: test.name,
                    error: stdout +
                        panicMessage +
                        "\n" +
                        result.stack +
                        "\n" +
                        "note: panic did not contain expected string\n" +
                        `      panic message: "${panicPayload}"\n` +
                        ` expected substring: "${test.shouldPanic}"`,
                });
                continue;
            }
            report(0 /* Stream.Stdout */, [okText, newLineText]);
            continue;
        }
        else if (result.success) {
            report(0 /* Stream.Stdout */, [okText, newLineText]);
        }
        else {
            report(0 /* Stream.Stdout */, [failedText, newLineText]);
            let stdout = interceptStore.join("");
            if (stdout.length !== 0) {
                stdout += "\n";
            }
            failures.push({
                name: test.name,
                error: stdout + panicMessage + "\n" + result.stack,
            });
        }
    }
    let output1 = "\n";
    if (failures.length > 0) {
        output1 += "failures:\n\n";
        for (const failure of failures) {
            output1 += `---- ${failure.name} stdout ----\n` + failure.error + "\n";
        }
        output1 += "\nfailures:\n";
        for (const failure of failures) {
            output1 += `    ${failure.name}\n`;
        }
        output1 += "\n";
    }
    let success = failures.length === 0;
    const result = success ? okText : failedText;
    const passed = testData.tests.length - failures.length - ignored;
    const durationMs = performance.now() - startTime;
    const durationSecs = (durationMs / 1000).toFixed(2);
    output1 += "test result: ";
    const output2 = `. ${passed} passed; ${failures.length} failed; ${ignored} ignored; 0 measured; ${testData.filteredCount} filtered out; finished in ${durationSecs}s\n\n`;
    report(0 /* Stream.Stdout */, [
        { text: output1, color: 0 /* Color.Default */ },
        result,
        { text: output2, color: 0 /* Color.Default */ },
    ]);
    return success;
}
