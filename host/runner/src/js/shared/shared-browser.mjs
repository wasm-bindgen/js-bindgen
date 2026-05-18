import { createBrowserFsBackend, run } from "./shared.mjs";
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
        fetch("../report", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ order: fetchOrder++, report: { stream, line: text } }),
        })
            .then(result => {
            if (!result.ok) {
                throw new Error(`fetch failed with status ${result.status}`);
            }
        })
            .catch((error) => {
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
        status = 1 /* Status.Abnormal */;
    }
    else {
        const fs = createBrowserFsBackend();
        status = await WebAssembly.compileStreaming(fetch("../wasm.wasm")).then(module => run(module, jsBindgenCtor, (stream, text) => report(stream, colorText(text)), fs), (error) => {
            report(1 /* Stream.Stderr */, error.message + "\n");
            return 1 /* Status.Abnormal */;
        });
        await fs.flush();
    }
    if (fetchRunning !== 0) {
        fetchWaiting = true;
        await fetchWaiter;
    }
    // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
    if (fetchError) {
        fetchOrder = 0;
        status = 1 /* Status.Abnormal */;
    }
    await fetch("../finished", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ status, messages: fetchOrder }),
    });
}
