/// <reference lib="webworker" />
import { runTests } from "./shared.mjs";
self.addEventListener("message", async (event) => {
    const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"));
    const [port] = event.ports;
    await runTests(module, (_, text) => {
        self.clients.get;
        port.postMessage(text);
    });
    self.registration.unregister();
});
