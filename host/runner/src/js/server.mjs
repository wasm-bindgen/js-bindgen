import { runTests } from "./shared.mjs";
import { toOutput } from "./shared-server.mjs";
const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"));
const result = await runTests(module, (_, text) => {
    toOutput(text);
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
