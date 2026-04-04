import { runTests } from "./shared.mjs";
import { colorText } from "./shared-terminal.mjs";
export async function runBrowser(jsBindgenCtor) {
    let fetchOrder = 0;
    let fetchRunning = 0;
    let fetchError = false;
    let fetchWaiting = false;
    let fetchResolve;
    const fetchWaiter = new Promise(resolve => {
        fetchResolve = resolve;
    });
    function report(stream, text) {
        fetchRunning++;
        fetch("./report", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ order: fetchOrder++, stream, line: text }),
        })
            .then(result => {
            if (!result.ok) {
                throw result;
            }
        })
            .catch(error => {
            fetchError = true;
            throw error;
        })
            .finally(() => {
            fetchRunning--;
            if (fetchWaiting && fetchRunning === 0) {
                fetchResolve();
            }
        });
    }
    let status;
    if (jsBindgenCtor instanceof Error) {
        report(1 /* Stream.Stderr */, jsBindgenCtor.message + "\n");
        status = 2 /* Status.Abnormal */;
    }
    else {
        status = await WebAssembly.compileStreaming(fetch("./wasm.wasm")).then(module => runTests(module, jsBindgenCtor, (stream, text) => report(stream, colorText(text))), error => {
            report(1 /* Stream.Stderr */, error.message + "\n");
            return 2 /* Status.Abnormal */;
        });
    }
    if (fetchRunning !== 0) {
        fetchWaiting = true;
        await fetchWaiter;
    }
    if (fetchError) {
        status = 2 /* Status.Abnormal */;
    }
    await fetch("/finished", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(status),
    });
}
