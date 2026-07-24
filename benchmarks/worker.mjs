import { readFile } from "node:fs/promises";
import { pathToFileURL } from "node:url";
import { parentPort } from "node:worker_threads";

export async function loadExport({ exportName, implementation }) {
  const module = await import(pathToFileURL(implementation.modulePath));
  const bytes = await readFile(implementation.wasmPath);
  let exports;

  if (implementation.kind === "js-bindgen") {
    const wasmModule = await WebAssembly.compile(bytes);
    const result = await new module.JsBindgen(wasmModule).instantiate();
    exports = result.instance.exports;
  } else if (implementation.kind === "wasm-bindgen") {
    exports = module.initSync({ module: bytes });
  } else {
    throw new Error(`unknown implementation: ${implementation.kind}`);
  }

  const call = exports[exportName];

  if (typeof call !== "function") {
    throw new Error(`missing Wasm export: ${exportName}`);
  }

  return call;
}

export function serve(run) {
  parentPort.on("message", message => {
    if (message !== "run") {
      throw new Error(`unexpected parent message: ${message}`);
    }

    parentPort.postMessage({
      type: "result",
      ...run(),
    });
  });
  parentPort.postMessage({ type: "ready" });
}
