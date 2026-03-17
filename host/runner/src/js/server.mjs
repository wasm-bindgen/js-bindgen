import { runTests } from "./shared.mjs";
import { toOutput } from "./shared-server.mjs";
import { importJsBindgen } from "./shared-import.mjs";
const module = await WebAssembly.compileStreaming(fetch("./wasm.wasm"));
const jsBindgenCtor = await importJsBindgen();
if (jsBindgenCtor instanceof Error) {
    toOutput([{ text: jsBindgenCtor.message + "\n", color: 0 /* Color.Default */ }]);
}
else {
    await runTests(module, jsBindgenCtor, (_, text) => {
        toOutput(text);
    });
}
