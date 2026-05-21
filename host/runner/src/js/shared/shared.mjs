import runData from "../run-data.json" with { type: "json" };
function mainMemory(module, name, importObject) {
    const value = importObject[module]?.[name];
    if (!(value instanceof WebAssembly.Memory)) {
        throw new Error(`missing main memory \`"${module}" "${name}"\` import`);
    }
    return value;
}
function mainArgs(memory, values, wasm64) {
    const WASM_PAGE_SIZE = 64 * 1024;
    const encoder = new TextEncoder();
    const args = values.map(value => encoder.encode(value));
    const pointerSize = wasm64 ? 8 : 4;
    // The `argv` array must be terminated by a null pointer.
    // Thus, in the new program, `argv[argc]` will be a null pointer.
    const argvBytes = (args.length + 1) * pointerSize;
    // The `+ 1` is for the null byte after each string. This works because
    // Wasm initializes the newly grown memory to zero by default.
    const stringBytes = args.reduce((total, arg) => total + arg.length + 1, 0);
    const totalBytes = Math.ceil((argvBytes + stringBytes) / pointerSize) * pointerSize;
    const pages = Math.ceil(totalBytes / WASM_PAGE_SIZE);
    const ptr = memory.buffer.byteLength;
    // TODO: Remove this cast once TypeScript's DOM definitions support `bigint`
    // values for `WebAssembly.Memory.grow`: microsoft/TypeScript-DOM-lib-generator#2485.
    const grow = memory.grow.bind(memory);
    grow(wasm64 ? BigInt(pages) : pages);
    const bytes = new Uint8Array(memory.buffer, ptr, totalBytes);
    const dataView = new DataView(bytes.buffer, ptr, argvBytes);
    let stringPtr = argvBytes;
    for (const [index, arg] of args.entries()) {
        const argPtr = ptr + stringPtr;
        if (wasm64) {
            dataView.setBigUint64(index * pointerSize, BigInt(argPtr), true);
        }
        else {
            dataView.setUint32(index * pointerSize, argPtr, true);
        }
        bytes.set(arg, stringPtr);
        stringPtr += arg.length + 1;
    }
    if (wasm64) {
        return { argc: args.length, argv: BigInt(ptr) };
    }
    else {
        return { argc: args.length, argv: ptr };
    }
}
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
        const importObject = jsBindgen.importObject;
        const instance = await jsBindgen.instantiate();
        return {
            importObject,
            instance,
            get panicMessage() {
                return panicMessage;
            },
            get panicPayload() {
                return panicPayload;
            },
        };
    }
    if (runData.kind === "binary") {
        const state = await instantiate();
        if (!state) {
            return 1 /* Status.Abnormal */;
        }
        const memory = mainMemory(runData.memory.module, runData.memory.name, state.importObject);
        interceptFlag = true;
        let status;
        try {
            if (runData.wasm64) {
                const { argc, argv } = mainArgs(memory, runData.args, true);
                const main = state.instance.exports["main"];
                status = main(argc, argv);
            }
            else {
                const { argc, argv } = mainArgs(memory, runData.args, false);
                const main = state.instance.exports["main"];
                status = main(argc, argv);
            }
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
