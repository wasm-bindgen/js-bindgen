import { workerData } from "node:worker_threads";

import { loadExport, serve } from "../worker.mjs";

const call = await loadExport(workerData);
const { iterations } = workerData;
let sink = 0;

if (call(42) !== 42) {
  throw new Error("unexpected benchmark function result");
}

serve(() => {
  let value = 0;
  const start = process.hrtime.bigint();

  for (let index = 0; index < iterations; index++) {
    value = (value + call(index)) | 0;
  }

  const end = process.hrtime.bigint();
  sink ^= value;

  return {
    elapsed: Number(end - start) / iterations,
    sink,
  };
});
