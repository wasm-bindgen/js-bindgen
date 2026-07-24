import { workerData } from "node:worker_threads";

import { loadExport, serve } from "../worker.mjs";

const call = await loadExport(workerData);
const { iterations } = workerData;
const input = [0x0123456789abcdefn, 0xfedcba9876543210n];
const output = input.map(value => BigInt.asIntN(64, value));
let sink = 0n;

const expected = call(...input);

if (
  !Array.isArray(expected) ||
  expected[0] !== output[0] ||
  expected[1] !== output[1]
) {
  throw new Error("unexpected benchmark function result");
}

serve(() => {
  let result;
  const start = process.hrtime.bigint();

  for (let index = 0; index < iterations; index++) {
    result = call(...input);
  }

  const end = process.hrtime.bigint();
  sink ^= result[0] ^ result[1];

  return {
    elapsed: Number(end - start) / iterations,
    sink,
  };
});
