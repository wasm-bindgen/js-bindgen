import { do_not_optimize } from "mitata";

export function createBenchmark({ call }) {
  if (call(42) !== 42) {
    throw new Error("unexpected benchmark function result");
  }

  return function* () {
    let result;

    yield () => {
      result = call(42);
    };

    do_not_optimize(result);
  };
}
