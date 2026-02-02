import { open } from "node:fs/promises";
import { runTests } from "./shared.mjs";
import { colorText } from "./shared-terminal.mjs";
const wasmFile = await open(new URL("./wasm.wasm", import.meta.url));
const wasmResponse = new Response(wasmFile.createReadStream(), {
    headers: { "Content-Type": "application/wasm" },
});
const module = await WebAssembly.compileStreaming(wasmResponse);
const success = await runTests(module, (stream, text) => {
    const output = colorText(text);
    switch (stream) {
        case 0 /* Stream.Stdout */:
            process.stdout.write(output);
            break;
        case 1 /* Stream.Stderr */:
            process.stderr.write(output);
    }
});
process.exit(success ? 0 : 101);
