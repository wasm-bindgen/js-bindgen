import { do_not_optimize } from "mitata";

export function createBenchmark({ call }) {
  const input = {};

  if (call(input) !== input) {
    throw new Error("unexpected benchmark function result");
  }

  return function* () {
    let result;

    yield () => {
      result = call(input);
    };

    do_not_optimize(result);
  };
}
