import { workerData } from "node:worker_threads";

import { loadExport, serve } from "../worker.mjs";

const call = await loadExport(workerData);
const { iterations } = workerData;
const input = 42;
let sink = 0;

if (call(input) !== input) {
  throw new Error("unexpected benchmark function result");
}

serve(() => {
  let result;
  const start = process.hrtime.bigint();

  for (let index = 0; index < iterations; index++) {
    result = call(input);
  }

  const end = process.hrtime.bigint();
  sink ^= result;

  return {
    elapsed: Number(end - start) / iterations,
    sink,
  };
});
