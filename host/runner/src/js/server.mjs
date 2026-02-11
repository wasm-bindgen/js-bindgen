import { runTests } from "./shared.mjs";
import { toOutput } from "./shared-server.mjs";
const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"));
await runTests(module, (_, text) => {
    toOutput(text);
});
