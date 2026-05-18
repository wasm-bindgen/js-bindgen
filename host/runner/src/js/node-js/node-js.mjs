import { open } from "node:fs/promises";
import { writeFileSync } from "node:fs";
import { run } from "../shared/shared.mjs";
import { colorText } from "../shared/shared-terminal.mjs";
import { JsBindgen } from "../imports.mjs";
const fs = {
    writeFile(path, data) {
        writeFileSync(path, data);
    },
};
const wasmFile = await open(new URL("../wasm.wasm", import.meta.url));
const wasmResponse = new Response(
// eslint-disable-next-line @typescript-eslint/no-unsafe-argument, @typescript-eslint/no-explicit-any
wasmFile.createReadStream(), {
    headers: { "Content-Type": "application/wasm" },
});
const module = await WebAssembly.compileStreaming(wasmResponse);
const status = await run(module, JsBindgen, (stream, text) => {
    const output = colorText(text);
    switch (stream) {
        case 0 /* Stream.Stdout */:
            process.stdout.write(output);
            break;
        case 1 /* Stream.Stderr */:
            process.stderr.write(output);
    }
}, fs);
process.exit(status);
