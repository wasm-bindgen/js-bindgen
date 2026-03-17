import { runTests } from "./shared.mjs";
import { JsBindgen } from "./imports.mjs";
self.addEventListener("message", async (event) => {
    const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"));
    const [port] = event.ports;
    await runTests(module, JsBindgen, (_, text) => {
        self.clients.get;
        port.postMessage(text);
    });
    self.registration.unregister();
});
