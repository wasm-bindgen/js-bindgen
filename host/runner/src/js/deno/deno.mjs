import { run } from "../shared/shared.mjs";
import { colorText, keepAlive } from "../shared/shared-terminal.mjs";
import { JsBindgen } from "../imports.mjs";
const module = await WebAssembly.compileStreaming(fetch(new URL("../wasm.wasm", import.meta.url)));
const runPromise = run(module, JsBindgen, (stream, text) => {
    function printSync(input, to) {
        let bytesWritten = 0;
        const bytes = new TextEncoder().encode(input);
        while (bytesWritten < bytes.length) {
            bytesWritten += to.writeSync(bytes.subarray(bytesWritten));
        }
    }
    const output = colorText(text);
    switch (stream) {
        case 0 /* Stream.Stdout */:
            printSync(output, Deno.stdout);
            break;
        case 1 /* Stream.Stderr */:
            printSync(output, Deno.stderr);
    }
});
const status = await keepAlive(runPromise);
Deno.exit(status);
