/// <reference lib="webworker" />
import { runTests } from "./shared.mjs";
const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"));
await runTests(module, (_, text) => {
    self.postMessage(text);
});
self.close();
