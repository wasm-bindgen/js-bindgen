import { runTests } from "../shared/shared.mjs";
import { importJsBindgen } from "../shared/shared-import.mjs";
const module = await WebAssembly.compileStreaming(fetch("../wasm.wasm"));
const jsBindgenCtor = await importJsBindgen();
if (jsBindgenCtor instanceof Error) {
    self.postMessage(jsBindgenCtor.message + "\n");
}
else {
    await runTests(module, jsBindgenCtor, (_, text) => {
        self.postMessage(text);
    });
}
self.close();
