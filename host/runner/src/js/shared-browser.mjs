import { runTests } from "./shared.mjs";
import { colorText } from "./shared-terminal.mjs";
export async function runBrowser() {
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
    const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"));
    const result = await runTests(module, (stream, text) => report(stream, colorText(text)));
    if (typeof result.benchBaseline === "string") {
        const response = await fetch("./benchmark-baseline", {
            method: "POST",
            headers: { "Content-Type": "text/plain" },
            body: result.benchBaseline,
        });
        if (!response.ok) {
            throw response;
        }
    }
    let status = result.success ? 0 /* Status.Ok */ : 1 /* Status.Failed */;
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
