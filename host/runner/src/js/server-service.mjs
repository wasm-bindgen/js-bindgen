/// <reference lib="webworker" />
import { runTests } from "./shared.mjs";
self.addEventListener("message", async (event) => {
    const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"));
    const [port] = event.ports;
    const result = await runTests(module, (_, text) => {
        self.clients.get;
        port.postMessage(text);
    });
    if (typeof result.benchBaseline === "string") {
        const response = await fetch("./benchmark-baseline", {
            method: "POST",
            headers: { "Content-Type": "text/plain" },
            body: result.benchBaseline,
        });
        if (!response.ok) {
            throw new Error("failed to upload benchmark baseline");
        }
    }
    self.registration.unregister();
});
