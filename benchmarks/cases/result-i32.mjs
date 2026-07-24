import { workerData } from "node:worker_threads";

import { loadExport, serve } from "../worker.mjs";

const call = await loadExport(workerData);
const { implementation, iterations } = workerData;
const input = 42;
let sink = 0;

function resultSlots(result) {
  if (!Array.isArray(result)) {
    throw new Error("unexpected benchmark function result");
  }

  if (implementation.kind === "js-bindgen") {
    if (
      result[0] !== undefined ||
      result[1] !== 0 ||
      result[2] !== input
    ) {
      throw new Error("unexpected benchmark function result");
    }

    return [result[1], result[2]];
  }

  if (
    result[0] !== input ||
    result[1] !== 0 ||
    result[2] !== 0
  ) {
    throw new Error("unexpected benchmark function result");
  }

  return [result[2], result[0]];
}

resultSlots(call(input));

serve(() => {
  let result;
  const start = process.hrtime.bigint();

  for (let index = 0; index < iterations; index++) {
    result = call(input);
  }

  const end = process.hrtime.bigint();
  const [tag, value] = resultSlots(result);
  sink ^= tag ^ value;

  return {
    elapsed: Number(end - start) / iterations,
    sink,
  };
});
