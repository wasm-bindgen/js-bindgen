import runData from "../run-data.json" with { type: "json" };
export async function run(module, jsBindgenCtor, report) {
    let interceptFlag = false;
    const interceptStore = [];
    const newLineText = { text: "\n", color: 0 /* Color.Default */ };
    const failedText = { text: "FAILED", color: 3 /* Color.Red */ };
    const okText = { text: "ok", color: 1 /* Color.Green */ };
    const CONSOLE_METHODS = ["debug", "log", "info", "warn", "error"];
    CONSOLE_METHODS.forEach(level => {
        const origin = console[level].bind(console);
        console[level] =
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            ((...data) => {
                if (interceptFlag) {
                    const stream = level === "error" || level === "warn" ? 1 /* Stream.Stderr */ : 0 /* Stream.Stdout */;
                    const text = data.join(" ") + "\n";
                    if (runData.kind === "binary" || runData.noCapture) {
                        report(stream, [{ text, color: 0 /* Color.Default */ }]);
                    }
                    else {
                        interceptStore.push(text);
                    }
                }
                else {
                    // eslint-disable-next-line @typescript-eslint/no-unsafe-argument
                    origin(...data);
                }
            }).bind(console);
    });
    async function instantiate() {
        let panicMessage;
        let panicPayload;
        let jsBindgen;
        try {
            jsBindgen = new jsBindgenCtor(module);
        }
        catch (error) {
            report(1 /* Stream.Stderr */, [{ text: error.message, color: 0 /* Color.Default */ }, newLineText]);
            return;
        }
        jsBindgen.extendImportObject({
            js_bindgen_test: {
                set_message: (message) => (panicMessage = message),
                set_payload: (payload) => (panicPayload = payload),
            },
        });
        const instance = await jsBindgen.instantiate();
        return { instance, panicMessage, panicPayload };
    }
    if (runData.kind === "binary") {
        const state = await instantiate();
        if (!state) {
            return 1 /* Status.Abnormal */;
        }
        interceptFlag = true;
        let status;
        try {
            const main = state.instance.exports["main"];
            status = main(0, 0);
        }
        catch (error) {
            const message = state.panicMessage ?? error.message;
            const stack = error.stack;
            report(1 /* Stream.Stderr */, [{ text: message + "\n" + stack + "\n", color: 0 /* Color.Default */ }]);
            status = 101 /* Status.Failed */;
        }
        finally {
            interceptFlag = false;
        }
        return status;
    }
    const startTime = performance.now();
    report(0 /* Stream.Stdout */, [
        {
            text: `\nrunning ${runData.tests.length} tests\n`,
            color: 0 /* Color.Default */,
        },
    ]);
    const failures = [];
    let ignored = 0;
    for (const test of runData.tests) {
        interceptStore.length = 0;
        const state = await instantiate();
        if (!state) {
            return 1 /* Status.Abnormal */;
        }
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
                report(0 /* Stream.Stdout */, [testText, { text: "ignored", color: 2 /* Color.Yellow */ }, newLineText]);
            }
            continue;
        }
        const testFn = state.instance.exports[test.importName];
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
            result = {
                success: false,
                stack: error.stack,
                message: state.panicMessage ?? error.message,
            };
        }
        interceptFlag = false;
        if (test.shouldPanic) {
            if (result.success) {
                report(0 /* Stream.Stdout */, [failedText, newLineText]);
                const stdout = interceptStore.join("");
                failures.push({ name: test.name, error: stdout + "note: test did not panic as expected" });
                continue;
            }
            if (typeof test.shouldPanic === "string" &&
                typeof state.panicPayload === "string" &&
                !state.panicPayload.includes(test.shouldPanic)) {
                report(0 /* Stream.Stdout */, [failedText, newLineText]);
                let stdout = interceptStore.join("");
                if (stdout.length !== 0) {
                    stdout += "\n";
                }
                failures.push({
                    name: test.name,
                    error: stdout +
                        result.message +
                        "\n" +
                        result.stack +
                        "\n" +
                        "note: panic did not contain expected string\n" +
                        `      panic message: "${state.panicPayload}"\n` +
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
                error: stdout + result.message + "\n" + result.stack,
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
    const status = failures.length === 0 ? 0 /* Status.Ok */ : 101 /* Status.Failed */;
    const result = status === 0 /* Status.Ok */ ? okText : failedText;
    const passed = runData.tests.length - failures.length - ignored;
    const durationMs = performance.now() - startTime;
    const durationSecs = (durationMs / 1000).toFixed(2);
    output1 += "test result: ";
    const output2 = `. ${passed} passed; ${failures.length} failed; ${ignored} ignored; 0 measured; ${runData.filteredCount} filtered out; finished in ${durationSecs}s\n\n`;
    report(0 /* Stream.Stdout */, [
        { text: output1, color: 0 /* Color.Default */ },
        result,
        { text: output2, color: 0 /* Color.Default */ },
    ]);
    return status;
}
